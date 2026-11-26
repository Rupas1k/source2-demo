use serde::Serialize;
use source2_demo::metadata::EntityFieldUpdate;
use source2_demo::prelude::*;
use source2_demo::proto::EDemoCommands;
use source2_demo::reader::Command;

#[derive(Serialize)]
pub struct MessageLocation {
    command: usize,
    child: usize,
    tick: i32,
}

#[derive(Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum FieldSource {
    Creation { location: MessageLocation },
    Update { location: MessageLocation },
    Baseline { class_id: i32, tick: i32 },
}

enum Origin {
    Creation(usize, usize),
    Update(usize, usize),
    Baseline(i32, usize),
}

#[derive(Default)]
struct FieldTrace {
    handle: u32,
    path: Vec<u16>,
    command: usize,
    child: usize,
    stop_command: usize,
    collecting: bool,
    finished: bool,
    write: Option<FieldValue>,
    source: Option<(Origin, FieldValue)>,
}

#[observer]
#[uses_string_tables]
impl FieldTrace {
    #[on_message]
    fn track_packet_message(&mut self, id: u32) -> ObserverResult {
        self.collecting = id == SvcMessages::SvcPacketEntities as u32;
        self.child += 1;
        Ok(())
    }

    #[on_message]
    fn track_demo_command(&mut self, _: EDemoCommands) -> ObserverResult {
        if self.command == self.stop_command {
            self.finished = true;
            anyhow::bail!("Field trace complete");
        }
        self.command += 1;
        self.child = 0;
        self.collecting = false;
        Ok(())
    }

    #[wants_entity_field_updates]
    fn is_collecting_fields(&self) -> bool {
        self.collecting
    }

    #[on_field_update]
    fn track_field_updates(
        &mut self,
        entity: &Entity,
        updates: &[EntityFieldUpdate],
    ) -> ObserverResult {
        if entity.handle() == self.handle {
            if let Some(write) = updates.iter().rev().find(|write| write.path == self.path) {
                self.write = Some(write.after.clone());
            }
        }
        Ok(())
    }

    #[on_entity]
    fn track_entity(&mut self, event: EntityEvents, entity: &Entity) -> ObserverResult {
        if entity.handle() != self.handle {
            return Ok(());
        }
        let write = self.write.take();
        match event {
            EntityEvents::Created => {
                self.source = if let Some(value) = write {
                    Some((Origin::Creation(self.command, self.child - 1), value))
                } else {
                    entity
                        .fields()
                        .into_iter()
                        .find(|field| field.path == self.path)
                        .and_then(|field| field.value)
                        .map(|value| {
                            (
                                Origin::Baseline(entity.class().id(), self.command),
                                value.clone(),
                            )
                        })
                };
            }
            EntityEvents::Updated => {
                if let Some(value) = write {
                    self.source = Some((Origin::Update(self.command, self.child - 1), value));
                }
            }
            EntityEvents::Deleted => self.source = None,
        }
        Ok(())
    }
}

pub fn find(
    file: &[u8],
    commands: &[Command],
    position: usize,
    handle: u32,
    path: Vec<u16>,
    expected: &FieldValue,
) -> Result<Option<FieldSource>, String> {
    let Some(stop_command) = commands
        .iter()
        .rposition(|command| command.range.end <= position)
    else {
        return Ok(None);
    };
    let mut parser = Parser::from_slice(file).map_err(|error| error.to_string())?;
    let trace = parser.add_observer(FieldTrace {
        handle,
        path,
        stop_command,
        ..Default::default()
    });
    let result = parser.run_to_end();
    let trace = trace.borrow();
    if !trace.finished {
        result.map_err(|error| error.to_string())?;
    }
    let tick = |command: usize| commands[command].tick.max(0);
    let location = |command, child| MessageLocation {
        command,
        child,
        tick: tick(command),
    };
    Ok(trace
        .source
        .as_ref()
        .filter(|(_, value)| value == expected)
        .map(|(origin, _)| match *origin {
            Origin::Creation(command, child) => FieldSource::Creation {
                location: location(command, child),
            },
            Origin::Update(command, child) => FieldSource::Update {
                location: location(command, child),
            },
            Origin::Baseline(class_id, command) => FieldSource::Baseline {
                class_id,
                tick: tick(command),
            },
        }))
}
