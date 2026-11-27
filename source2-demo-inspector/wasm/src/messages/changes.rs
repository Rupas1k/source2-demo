use crate::session::format_field_value;
use serde::Serialize;
use source2_demo::prelude::*;

#[derive(Serialize)]
pub struct EntityChange {
    index: u32,
    handle: u32,
    class: String,
    operation: &'static str,
    class_id: i32,
    fields: Vec<FieldChange>,
}

#[derive(Serialize)]
struct FieldChange {
    path: Vec<u16>,
    name: String,
    kind: String,
    #[serde(rename = "decodedAs")]
    decoded_as: &'static str,
    before: Option<String>,
    after: Option<String>,
}

pub struct TableChange {
    pub id: i32,
    pub name: String,
    pub created: bool,
    pub rows: Vec<StringTableRow>,
}

#[derive(Default)]
struct PacketTarget {
    target: (usize, usize),
    command: usize,
    child: usize,
    matched: bool,
    collecting: bool,
    finished: bool,
}

impl PacketTarget {
    fn new(command: usize, child: usize) -> Self {
        Self {
            target: (command, child),
            ..Default::default()
        }
    }

    fn is_target_command(&self) -> bool {
        self.command == self.target.0
    }

    fn track_message(&mut self, relevant: bool) {
        self.collecting = (self.command, self.child) == self.target && relevant;
        self.matched |= self.collecting;
        self.child += 1;
    }

    fn finish_command(&mut self) -> ObserverResult {
        if self.is_target_command() {
            self.finished = true;
            anyhow::bail!("Packet inspection complete");
        }
        self.command += 1;
        self.child = 0;
        self.collecting = false;
        Ok(())
    }
}

#[derive(Default)]
struct EntityChanges {
    packet: PacketTarget,
    changes: Vec<EntityChange>,
    fields: Vec<FieldChange>,
}

#[observer]
#[uses_string_tables]
impl EntityChanges {
    #[wants_full_packet_snapshot]
    fn is_target_snapshot(&self) -> bool {
        self.packet.is_target_command()
    }

    #[on_message]
    fn track_packet_message(&mut self, id: u32) -> ObserverResult {
        self.packet
            .track_message(id == SvcMessages::SvcPacketEntities as u32);
        Ok(())
    }

    #[on_message]
    fn track_demo_command(&mut self, _: EDemoCommands) -> ObserverResult {
        self.packet.finish_command()
    }

    #[wants_entity_field_updates]
    fn is_collecting_fields(&self) -> bool {
        self.packet.collecting
    }

    #[on_field_update]
    fn track_field_updates(&mut self, updates: &[EntityFieldUpdate]) -> ObserverResult {
        self.fields.extend(updates.iter().map(|update| FieldChange {
            path: update.path.clone(),
            name: update.name.clone(),
            kind: update.field_type.clone(),
            decoded_as: update.after.type_name(),
            before: update.before.as_ref().map(format_field_value),
            after: Some(format_field_value(&update.after)),
        }));
        Ok(())
    }

    #[on_entity]
    fn track_entity(&mut self, event: EntityEvents, entity: &Entity) -> ObserverResult {
        if self.packet.collecting {
            if matches!(event, EntityEvents::Deleted) {
                self.fields
                    .extend(entity.fields().into_iter().filter_map(|field| {
                        Some(FieldChange {
                            path: field.path,
                            name: field.name,
                            kind: field.field_type,
                            decoded_as: field.decoded_type.unwrap_or("None"),
                            before: Some(format_field_value(field.value?)),
                            after: None,
                        })
                    }));
            }
            self.changes.push(EntityChange {
                index: entity.index(),
                handle: entity.handle(),
                class: entity.class().name().to_owned(),
                class_id: entity.class().id(),
                fields: std::mem::take(&mut self.fields),
                operation: match event {
                    EntityEvents::Created => "Created",
                    EntityEvents::Updated => "Updated",
                    EntityEvents::Deleted => "Deleted",
                },
            });
        }
        Ok(())
    }
}

#[derive(Default)]
struct StringTableChanges {
    packet: PacketTarget,
    creating: bool,
    table: Option<TableChange>,
}

#[observer]
impl StringTableChanges {
    #[wants_full_packet_snapshot]
    fn is_target_snapshot(&self) -> bool {
        self.packet.is_target_command()
    }

    #[on_message]
    fn track_packet_message(&mut self, id: u32) -> ObserverResult {
        self.creating = id == SvcMessages::SvcCreateStringTable as u32;
        self.packet.track_message(matches!(
            SvcMessages::try_from(id as i32),
            Ok(SvcMessages::SvcCreateStringTable | SvcMessages::SvcUpdateStringTable)
        ));
        Ok(())
    }

    #[on_message]
    fn track_demo_command(&mut self, _: EDemoCommands) -> ObserverResult {
        self.packet.finish_command()
    }

    #[on_string_table]
    fn track_string_table(&mut self, table: &StringTable, modified: &[i32]) -> ObserverResult {
        if self.packet.collecting {
            self.table = Some(TableChange {
                id: table.index(),
                name: table.name().to_owned(),
                created: self.creating,
                rows: modified
                    .iter()
                    .map(|&index| table.get_row(index as usize).cloned())
                    .collect::<Result<_, _>>()?,
            });
        }
        Ok(())
    }
}

pub fn changes(file: &[u8], command: usize, child: usize) -> Result<Vec<EntityChange>, String> {
    let state = inspect(
        file,
        EntityChanges {
            packet: PacketTarget::new(command, child),
            ..Default::default()
        },
        |observer| &observer.packet,
        "Entity changes are unavailable for this packet snapshot.",
    )?;
    Ok(state.changes)
}

pub fn table_changes(file: &[u8], command: usize, child: usize) -> Result<TableChange, String> {
    let state = inspect(
        file,
        StringTableChanges {
            packet: PacketTarget::new(command, child),
            ..Default::default()
        },
        |observer| &observer.packet,
        "String-table changes are unavailable for this packet snapshot.",
    )?;
    state
        .table
        .ok_or_else(|| "String-table entries are unavailable for this packet.".to_owned())
}

fn inspect<T: Observer + Default>(
    file: &[u8],
    observer: T,
    packet: impl FnOnce(&T) -> &PacketTarget,
    unavailable: &str,
) -> Result<T, String> {
    let mut parser = Parser::from_slice(file).map_err(|error| error.to_string())?;
    let observer = parser.add_observer(observer);
    let result = parser.run_to_end();
    let mut state = observer.borrow_mut();
    let packet = packet(&state);
    if !packet.finished {
        result.map_err(|error| error.to_string())?;
    }
    if !packet.matched {
        return Err(unavailable.into());
    }
    Ok(std::mem::take(&mut *state))
}
