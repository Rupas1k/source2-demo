use crate::entity::field::*;
use crate::entity::Class;
use crate::error::ParserError;
use crate::parser::demo::DemoMessages;
use crate::proto::*;
use crate::reader::Reader;
use crate::{Parser, StringTableRow};
use hashbrown::HashMap;
use std::rc::Rc;

pub trait DemoCommands {
    fn dem_send_tables(&mut self, send_tables: CDemoSendTables) -> Result<(), ParserError>;

    fn dem_class_info(&mut self, class_info: CDemoClassInfo) -> Result<(), ParserError>;

    fn dem_packet(&mut self, demo_packet: CDemoPacket) -> Result<(), ParserError>;

    fn dem_full_packet(&mut self, full_packet: CDemoFullPacket) -> Result<(), ParserError>;

    fn dem_string_tables(&mut self, string_tables: CDemoStringTables) -> Result<(), ParserError>;

    fn dem_stop(&mut self) -> Result<(), ParserError> {
        Ok(())
    }
}

impl DemoCommands for Parser<'_> {
    fn dem_send_tables(&mut self, send_tables: CDemoSendTables) -> Result<(), ParserError> {
        let serializers = &mut self.context.serializers;

        let mut reader = Reader::new(send_tables.data());
        let amount = reader.read_var_u32();
        let buf = reader.read_bytes(amount);

        let fs = CSvcMsgFlattenedSerializer::decode(buf.as_slice())?;

        let resolve = |p: Option<i32>| -> Box<str> {
            if let Some(i) = p {
                return fs.symbols[i as usize].clone().into();
            }
            "".into()
        };

        let mut fields: Vec<Rc<Field>> = vec![];
        let mut field_types: HashMap<Box<str>, Rc<FieldType>> = HashMap::default();

        for s in fs.serializers.iter() {
            let serializer_name = fs.symbols[s.serializer_name_sym() as usize].clone();
            let mut serializer = Serializer::default();

            for i in s.fields_index.iter().map(|x| *x as usize) {
                let current_field = &fs.fields[i];
                let field_serializer_name = resolve(current_field.field_serializer_name_sym);

                if i >= fields.len() {
                    let var_type_str = resolve(current_field.var_type_sym);
                    let var_name = resolve(current_field.var_name_sym);

                    let current_field_serializer = serializers.get(&field_serializer_name).cloned();

                    let field_type = field_types
                        .entry(var_type_str.clone())
                        .or_insert_with(|| FieldType::new(var_type_str.clone().as_ref()).into())
                        .clone();

                    let properties = FieldProperties {
                        encoder: match var_name.as_ref() {
                            "m_flSimulationTime" | "m_flAnimTime" => Some(FieldEncoder::SimTime),
                            "m_flRuneTime" => Some(FieldEncoder::RuneTime),
                            _ => FieldEncoder::from_str(&resolve(current_field.var_encoder_sym)),
                        },
                        encoder_flags: current_field.encode_flags(),
                        bit_count: current_field.bit_count(),
                        low_value: current_field.low_value(),
                        high_value: current_field.high_value(),
                    };

                    let model = if let Some(serializer) = current_field_serializer {
                        if field_type.pointer {
                            FieldModel::FixedTable(serializer)
                        } else {
                            FieldModel::VariableTable(serializer)
                        }
                    } else if field_type.count.is_some_and(|x| x > 0)
                        && field_type.base.as_ref() != "char"
                    {
                        FieldModel::FixedArray
                    } else if field_type.base.as_ref() == "CUtlVector"
                        || field_type.base.as_ref() == "CNetworkUtlVectorBase"
                        || field_type.base.as_ref() == "CUtlVectorEmbeddedNetworkVar"
                    {
                        FieldModel::VariableArray(FieldDecoder::from_field(
                            field_type.generic.as_ref().unwrap(),
                            properties,
                        ))
                    } else {
                        FieldModel::Simple
                    };

                    let decoder = match model {
                        FieldModel::Simple | FieldModel::FixedArray => {
                            FieldDecoder::from_field(&field_type, properties)
                        }
                        FieldModel::VariableArray(_) => FieldDecoder::Unsigned32,
                        FieldModel::FixedTable(_) => FieldDecoder::Boolean,
                        FieldModel::VariableTable(_) => FieldDecoder::Unsigned32,
                    };

                    let field = Field {
                        var_name,
                        field_type,
                        model,

                        decoder,
                    };
                    fields.push(field.into());
                }
                serializer.fields.push(fields[i].clone());
            }
            serializers.insert(serializer_name.into(), serializer.into());
        }
        Ok(())
    }

    fn dem_class_info(&mut self, class_info: CDemoClassInfo) -> Result<(), ParserError> {
        for class in class_info.classes {
            let class_id = class.class_id();
            let network_name = class.network_name();
            let serializer = self.context.serializers[network_name].clone();
            let class = Rc::new(Class::new(class_id, network_name.into(), serializer));

            self.context.classes.classes_vec.push(class.clone());
            self.context
                .classes
                .classes_by_name
                .insert(network_name.into(), class);
        }
        Ok(())
    }

    fn dem_packet(&mut self, packet: CDemoPacket) -> Result<(), ParserError> {
        let mut packet_reader = Reader::new(packet.data());
        while packet_reader.bytes_remaining() != 0 {
            let msg_type = packet_reader.read_ubit_var() as i32;
            let size = packet_reader.read_var_u32();
            let msg_buf = packet_reader.read_bytes(size);

            #[cfg(any(
                all(feature = "dota", feature = "deadlock"),
                all(not(feature = "dota"), not(feature = "deadlock"))
            ))]
            {
                if let Ok(msg) = SvcMessages::try_from(msg_type) {
                    self.on_svc_message(msg, &msg_buf)?;
                } else if let Ok(msg) = EBaseUserMessages::try_from(msg_type) {
                    self.on_base_user_message(msg, &msg_buf)?;
                } else if let Ok(msg) = EBaseGameEvents::try_from(msg_type) {
                    self.on_base_game_event(msg, &msg_buf)?;
                } else if let Ok(msg) = NetMessages::try_from(msg_type) {
                    self.on_net_message(msg, &msg_buf)?;
                }
                continue;
            }

            #[cfg(feature = "dota")]
            if let Ok(msg) = EDotaUserMessages::try_from(msg_type) {
                self.on_dota_user_message(msg, &msg_buf)?;
            } else if let Ok(msg) = SvcMessages::try_from(msg_type) {
                self.on_svc_message(msg, &msg_buf)?;
            } else if let Ok(msg) = EBaseUserMessages::try_from(msg_type) {
                self.on_base_user_message(msg, &msg_buf)?;
            } else if let Ok(msg) = EBaseGameEvents::try_from(msg_type) {
                self.on_base_game_event(msg, &msg_buf)?;
            } else if let Ok(msg) = NetMessages::try_from(msg_type) {
                self.on_net_message(msg, &msg_buf)?;
            }

            #[cfg(feature = "deadlock")]
            if let Ok(msg) = CitadelUserMessageIds::try_from(msg_type) {
                self.on_citadel_user_message(msg, &msg_buf)?;
            } else if let Ok(msg) = ECitadelGameEvents::try_from(msg_type) {
                self.on_citadel_game_event(msg, &msg_buf)?;
            } else if let Ok(msg) = SvcMessages::try_from(msg_type) {
                self.on_svc_message(msg, &msg_buf)?;
            } else if let Ok(msg) = EBaseUserMessages::try_from(msg_type) {
                self.on_base_user_message(msg, &msg_buf)?;
            } else if let Ok(msg) = EBaseGameEvents::try_from(msg_type) {
                self.on_base_game_event(msg, &msg_buf)?;
            } else if let Ok(msg) = NetMessages::try_from(msg_type) {
                self.on_net_message(msg, &msg_buf)?;
            }
        }

        Ok(())
    }

    fn dem_full_packet(&mut self, full_packet: CDemoFullPacket) -> Result<(), ParserError> {
        if self.context.last_full_packet_tick == u32::MAX || self.skip_deltas {
            self.dem_string_tables(full_packet.string_table.unwrap())?;
            self.dem_packet(full_packet.packet.unwrap())?;
        }

        self.context.last_full_packet_tick = self.context.tick;

        Ok(())
    }

    fn dem_string_tables(&mut self, msg: CDemoStringTables) -> Result<(), ParserError> {
        for table in msg.tables.iter() {
            let x = self
                .context
                .string_tables
                .get_by_name_mut(table.table_name())?;

            x.items
                .resize_with(table.items.len(), StringTableRow::default);
            for (i, item) in table.items.iter().enumerate() {
                x.items[i].index = i as i32;
                x.items[i].key = item.str().to_string();
                x.items[i].value = Rc::new(item.data().to_vec()).into();
                if table.table_name() == "instancebaseline" {
                    self.context.baselines.add_baseline(
                        item.str().parse().unwrap(),
                        x.items[i].value.as_ref().unwrap().clone(),
                    );
                }
            }
        }

        Ok(())
    }

    fn dem_stop(&mut self) -> Result<(), ParserError> {
        self.on_stop()?;
        Ok(())
    }
}
