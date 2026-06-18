use super::*;
use crate::entity::field::{Decode, Encode, FieldPath, FieldState, FieldValue, Skip};
use crate::proto::{CSvcMsgPacketEntities, Message};
use crate::reader::{FieldPathCodec, SliceReader};
use crate::stream::copy::{bit_position, copy_original_bits};
use crate::stream::field_path::FieldOp;
use crate::writer::{BitsWriter, BitstreamWriter};
use std::rc::Rc;

pub(super) const ENTITY_REWRITE_BUFFER_LEN: usize = 8192;

pub(super) struct FieldReplacement {
    serializer: Rc<crate::entity::field::Serializer>,
    fp: FieldPath,
    value: FieldValue,
    value_start: usize,
    value_end: usize,
}

pub(super) struct DecodedEntityField {
    fp: FieldPath,
    name: Rc<str>,
    value_start: usize,
    value_end: usize,
}

impl<'a, R, W> DemoWriter<'a, R, W>
where
    R: BitsReader + MessageReader,
    W: Write + Seek,
{
    #[inline]
    fn clear_entity_paths(&mut self) {
        self.entity_rewrite_paths.clear();
    }

    #[inline]
    fn push_entity_path(&mut self, fp: FieldPath) {
        self.entity_rewrite_paths.push(fp);
    }

    #[inline]
    fn entity_paths(&self) -> &[FieldPath] {
        &self.entity_rewrite_paths
    }

    #[inline]
    fn clear_decoded_fields(&mut self) {
        self.entity_decoded_fields.clear();
    }

    #[inline]
    fn push_decoded_field(&mut self, field: DecodedEntityField) {
        self.entity_decoded_fields.push(field);
    }

    #[inline]
    fn push_field_replacement(&mut self, replacement: FieldReplacement) {
        self.entity_replacements.push(replacement);
    }
}

impl<'a, R, W> DemoWriter<'a, R, W>
where
    R: BitsReader + MessageReader,
    W: Write + Seek,
{
    pub(crate) fn rewrite_svc_packet_entities(
        &mut self,
        msg: &[u8],
    ) -> Result<Option<Vec<u8>>, ParserError> {
        let mut packet_entities = CSvcMsgPacketEntities::decode(msg)?;
        let Some(entity_data) = packet_entities.entity_data.as_deref() else {
            return Ok(None);
        };

        let (rewritten, changed) =
            self.rewrite_entity_data(entity_data, packet_entities.updated_entries())?;
        if changed {
            packet_entities.entity_data = Some(rewritten);
            packet_entities.serialized_entities = None;
            Ok(Some(packet_entities.encode_to_vec()))
        } else {
            Ok(None)
        }
    }

    fn rewrite_entity_data(
        &mut self,
        entity_data: &[u8],
        updated_entries: i32,
    ) -> Result<(Vec<u8>, bool), ParserError> {
        let mut reader = SliceReader::new(entity_data);
        self.entity_replacements.clear();
        let mut index = usize::MAX;
        let path_reader = self.field_path_codec.clone();

        for _ in 0..updated_entries {
            let delta = reader.read_ubit_var();
            index = index.wrapping_add((delta + 1) as usize);

            let cmd = reader.read_bits(2);

            if cmd == 1 {
                continue;
            }

            match EntityEvents::from_cmd(cmd) {
                EntityEvents::Created => {
                    self.rewrite_entity_created(&mut reader, &path_reader, index)?;
                }
                EntityEvents::Updated => {
                    self.rewrite_entity_updated(&mut reader, &path_reader, index)?;
                }
                EntityEvents::Deleted => {
                    self.parser.context.entities.entities_vec[index].index = u32::MAX;
                }
            }
        }

        if self.entity_replacements.is_empty() {
            return Ok((Vec::new(), false));
        }

        let mut out = Vec::with_capacity(entity_data.len());
        let mut writer = BitstreamWriter::new(&mut out);
        let mut copy_start = 0;
        let mut replacements = std::mem::take(&mut self.entity_replacements);
        for replacement in replacements.drain(..) {
            copy_original_bits(
                entity_data,
                copy_start,
                replacement.value_start - copy_start,
                &mut writer,
            )?;
            replacement
                .serializer
                .get_decoder(&replacement.fp)
                .encode(&mut writer, &replacement.value)?;
            copy_start = replacement.value_end;
        }
        self.entity_replacements = replacements;
        copy_original_bits(
            entity_data,
            copy_start,
            entity_data.len() * 8 - copy_start,
            &mut writer,
        )?;
        writer.flush()?;
        drop(writer);
        Ok((out, true))
    }

    fn rewrite_entity_created(
        &mut self,
        reader: &mut SliceReader<'_>,
        path_reader: &FieldPathCodec,
        index: usize,
    ) -> Result<(), ParserError> {
        let class_id = reader.read_bits(self.parser.context.classes.class_id_size) as i32;

        let serial = reader.read_bits(17);

        let _ = reader.read_var_u32();

        let class = self
            .parser
            .context
            .classes
            .get_by_id_rc(class_id as usize)
            .clone();

        let state = self.entity_baseline_state(class_id, &class.serializer);
        let mut entity = Entity::new(index as u32, serial, class, state);

        let track = self.should_track_entity(EntityEvents::Created, &entity);
        let rewrite = self.should_rewrite_entity(EntityEvents::Created, &entity);
        if track || rewrite {
            self.rewrite_fields(
                reader,
                path_reader,
                EntityEvents::Created,
                &mut entity,
                track,
                rewrite,
            )?;
        } else {
            self.skip_original_fields(reader, path_reader, &entity);
        }
        if !track {
            entity.state = FieldState::default();
        }
        self.parser.context.entities.entities_vec[index] = entity;

        Ok(())
    }

    fn rewrite_entity_updated(
        &mut self,
        reader: &mut SliceReader<'_>,
        path_reader: &FieldPathCodec,
        index: usize,
    ) -> Result<(), ParserError> {
        let class = self.parser.context.entities.entities_vec[index]
            .class
            .clone();
        let placeholder = Entity {
            index: u32::MAX,
            serial: 0,
            class,
            state: FieldState::default(),
        };
        let mut entity = std::mem::replace(
            &mut self.parser.context.entities.entities_vec[index],
            placeholder,
        );
        let track = self.should_track_entity(EntityEvents::Updated, &entity);
        let rewrite = self.should_rewrite_entity(EntityEvents::Updated, &entity);
        if track || rewrite {
            self.rewrite_fields(
                reader,
                path_reader,
                EntityEvents::Updated,
                &mut entity,
                track,
                rewrite,
            )?;
        } else {
            self.skip_original_fields(reader, path_reader, &entity);
        }
        if !track {
            entity.state = FieldState::default();
        }
        self.parser.context.entities.entities_vec[index] = entity;

        Ok(())
    }

    fn skip_original_fields(
        &mut self,
        reader: &mut SliceReader<'_>,
        path_reader: &FieldPathCodec,
        entity: &Entity,
    ) {
        self.clear_entity_paths();
        let mut fp = FieldPath::default();

        loop {
            reader.refill();
            let op = path_reader.read_op(reader);
            if let FieldOp::FieldPathEncodeFinish = op {
                break;
            }
            op.execute(reader, &mut fp);
            self.push_entity_path(fp);
        }

        for fp in self.entity_paths().iter().copied() {
            entity.class.serializer.get_decoder(&fp).skip(reader);
        }
    }

    fn rewrite_fields(
        &mut self,
        reader: &mut SliceReader<'_>,
        path_reader: &FieldPathCodec,
        event: EntityEvents,
        entity: &mut Entity,
        track: bool,
        rewrite: bool,
    ) -> Result<(), ParserError> {
        self.clear_entity_paths();
        let mut fp = FieldPath::default();

        loop {
            reader.refill();
            let op = path_reader.read_op(reader);
            if op == FieldOp::FieldPathEncodeFinish {
                break;
            }
            op.execute(reader, &mut fp);
            self.push_entity_path(fp);
        }

        if !rewrite {
            for fp in self.entity_paths().iter() {
                let decoder = entity.class.serializer.get_decoder(fp);
                let value = decoder.decode(reader);
                entity.state.set(fp, value);
            }
            return Ok(());
        }

        if !track {
            let mut paths = std::mem::take(&mut self.entity_rewrite_paths);
            for fp in paths.iter().copied() {
                let name = entity.class.serializer.get_name(&fp);
                let decoder = entity.class.serializer.get_decoder(&fp);
                let value_start = bit_position(reader);
                let value = decoder.decode(reader);
                let value_end = bit_position(reader);

                if let Some(next_value) = self.replace_entity_field(event, entity, &name, &value) {
                    entity.state.set(&fp, next_value.clone());
                    self.push_field_replacement(FieldReplacement {
                        serializer: entity.class.serializer.clone(),
                        fp,
                        value: next_value,
                        value_start,
                        value_end,
                    });
                }
            }
            paths.clear();
            self.entity_rewrite_paths = paths;
            return Ok(());
        }

        self.clear_decoded_fields();
        let mut paths = std::mem::take(&mut self.entity_rewrite_paths);
        for fp in paths.iter().copied() {
            let name = entity.class.serializer.get_name(&fp);
            let decoder = entity.class.serializer.get_decoder(&fp);
            let value_start = bit_position(reader);
            let value = decoder.decode(reader);
            let value_end = bit_position(reader);
            entity.state.set(&fp, value);
            self.push_decoded_field(DecodedEntityField {
                fp,
                name,
                value_start,
                value_end,
            });
        }
        paths.clear();
        self.entity_rewrite_paths = paths;

        let mut decoded_fields = std::mem::take(&mut self.entity_decoded_fields);
        for field in decoded_fields.drain(..) {
            let Some(value) = entity.state.get_value(&field.fp) else {
                continue;
            };
            let replacement = self.replace_entity_field(event, entity, &field.name, value);

            if let Some(next_value) = replacement {
                entity.state.set(&field.fp, next_value.clone());
                self.push_field_replacement(FieldReplacement {
                    serializer: entity.class.serializer.clone(),
                    fp: field.fp,
                    value: next_value,
                    value_start: field.value_start,
                    value_end: field.value_end,
                });
            }
        }
        self.entity_decoded_fields = decoded_fields;

        Ok(())
    }

    fn entity_baseline_state(
        &mut self,
        class_id: i32,
        serializer: &crate::entity::field::Serializer,
    ) -> FieldState {
        self.parser
            .context
            .baselines
            .states
            .entry(class_id)
            .or_insert_with(|| {
                let mut state = FieldState::default();
                if let Some(baseline) = self.parser.context.baselines.baselines.get(&class_id) {
                    self.parser.field_reader.read_fields(
                        &mut SliceReader::new(baseline.as_ref()),
                        serializer,
                        &mut state,
                    );
                }
                state
            })
            .clone()
    }
}
