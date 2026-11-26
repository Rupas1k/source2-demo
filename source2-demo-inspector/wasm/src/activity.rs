use prost::Message;
use source2_demo::prelude::*;
use source2_demo::proto::{CDemoFullPacket, CDemoStringTables, EDemoCommands};
use source2_demo::StringTable;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

type Point = [u32; 4];

#[derive(Default)]
struct Track(Vec<Point>);

impl Track {
    fn record(&mut self, ctx: &Context, count: u32, created: bool, deleted: bool) {
        let tick = ctx.tick() as i32;
        let tick = tick.max(0) as u32;
        if !self.0.last().is_some_and(|point| point[0] == tick) {
            self.0.push([tick, 0, 0, 0]);
        }
        let point = self.0.last_mut().unwrap();
        point[1] += count;
        point[2] += u32::from(created);
        point[3] += u32::from(deleted);
    }

    fn encode(&self) -> Vec<u32> {
        self.0.iter().flatten().copied().collect()
    }
}

#[derive(Default)]
struct TableActivity {
    writes: Track,
    rows: HashMap<i32, Track>,
    packets: Vec<[u32; 3]>,
}

#[derive(Default)]
struct EntityActivity {
    track: Track,
    packets: Vec<[u32; 3]>,
}

#[derive(Default)]
struct ActivityObserver {
    entity_updates: Track,
    entities: HashMap<u32, EntityActivity>,
    command: u32,
    child: u32,
    table_writes: Track,
    tables: HashMap<String, TableActivity>,
    baseline_writes: Track,
    baselines: HashMap<i32, Track>,
}

#[observer]
impl ActivityObserver {
    #[on_message]
    fn track_packet_message(&mut self, _: u32) -> ObserverResult {
        self.child += 1;
        Ok(())
    }

    #[on_message]
    fn track_demo_command(
        &mut self,
        ctx: &Context,
        command: EDemoCommands,
        payload: &[u8],
    ) -> ObserverResult {
        let snapshot = match command {
            EDemoCommands::DemStringTables => Some(CDemoStringTables::decode(payload)?),
            EDemoCommands::DemFullPacket => CDemoFullPacket::decode(payload)?.string_table,
            _ => None,
        };
        if let Some(snapshot) = snapshot {
            let location = [(ctx.tick() as i32).max(0) as u32, self.command, u32::MAX];
            for table in snapshot.tables {
                let activity = self
                    .tables
                    .entry(table.table_name().to_owned())
                    .or_default();
                if activity.packets.last() != Some(&location) {
                    activity.packets.push(location);
                }
            }
        }
        self.command += 1;
        self.child = 0;
        Ok(())
    }

    #[on_entity]
    fn track_entity(
        &mut self,
        ctx: &Context,
        event: EntityEvents,
        entity: &Entity,
    ) -> ObserverResult {
        let created = event == EntityEvents::Created;
        let deleted = event == EntityEvents::Deleted;
        self.entity_updates.record(ctx, 1, created, deleted);
        let activity = self.entities.entry(entity.handle()).or_default();
        activity.track.record(ctx, 1, created, deleted);
        if let Some(child) = self.child.checked_sub(1) {
            let packet = [(ctx.tick() as i32).max(0) as u32, self.command, child];
            if activity.packets.last() != Some(&packet) {
                activity.packets.push(packet);
            }
        }
        Ok(())
    }

    #[on_string_table]
    fn track_string_table(
        &mut self,
        ctx: &Context,
        table: &StringTable,
        modified: &[i32],
    ) -> ObserverResult {
        if modified.is_empty() {
            return Ok(());
        }

        let is_baseline = table.name() == "instancebaseline";
        self.table_writes
            .record(ctx, modified.len() as u32, false, false);
        let activity = self.tables.entry(table.name().to_owned()).or_default();
        if let Some(child) = self.child.checked_sub(1) {
            let location = [(ctx.tick() as i32).max(0) as u32, self.command, child];
            if activity.packets.last() != Some(&location) {
                activity.packets.push(location);
            }
        }
        activity
            .writes
            .record(ctx, modified.len() as u32, false, false);
        for &index in modified {
            activity
                .rows
                .entry(index)
                .or_default()
                .record(ctx, 1, false, false);
            if is_baseline {
                let row = table.get_row(index as usize)?;
                if let Ok(class) = row.key().parse::<i32>() {
                    self.baseline_writes.record(ctx, 1, false, false);
                    self.baselines
                        .entry(class)
                        .or_default()
                        .record(ctx, 1, false, false);
                }
            }
        }
        Ok(())
    }
}

#[wasm_bindgen]
pub struct ReplayActivityIndex {
    activity: ActivityObserver,
}

#[wasm_bindgen]
impl ReplayActivityIndex {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Vec<u8>) -> Result<ReplayActivityIndex, JsError> {
        let mut parser = Parser::from_slice(&bytes)?;
        let observer = parser.register_observer::<ActivityObserver>();
        parser.run_to_end()?;
        let activity = std::mem::take(&mut *observer.borrow_mut());
        Ok(Self { activity })
    }

    pub fn entities(&self, handle: Option<u32>) -> Vec<u32> {
        match handle {
            Some(handle) => self
                .activity
                .entities
                .get(&handle)
                .map(|activity| activity.track.encode())
                .unwrap_or_default(),
            None => self.activity.entity_updates.encode(),
        }
    }

    #[wasm_bindgen(js_name = "entityOccurrences")]
    pub fn entity_occurrences(&self, handle: u32) -> Vec<u32> {
        self.activity
            .entities
            .get(&handle)
            .map(|activity| activity.packets.iter().flatten().copied().collect())
            .unwrap_or_default()
    }

    pub fn baselines(&self, class_id: Option<i32>) -> Vec<u32> {
        match class_id {
            Some(class_id) => self
                .activity
                .baselines
                .get(&class_id)
                .map(Track::encode)
                .unwrap_or_default(),
            None => self.activity.baseline_writes.encode(),
        }
    }

    #[wasm_bindgen(js_name = "stringTableOccurrences")]
    pub fn string_table_occurrences(&self, name: &str) -> Vec<u32> {
        let Some(activity) = self.activity.tables.get(name) else {
            return Vec::new();
        };
        let mut packets = activity.packets.clone();
        packets.sort_unstable_by_key(|entry| (entry[1], entry[2].wrapping_add(1)));
        packets.into_iter().flatten().collect()
    }

    #[wasm_bindgen(js_name = "stringTables")]
    pub fn string_tables(&self, name: Option<String>, row: Option<i32>) -> Vec<u32> {
        match name {
            Some(name) => self
                .activity
                .tables
                .get(&name)
                .and_then(|table| match row {
                    Some(index) => table.rows.get(&index),
                    None => Some(&table.writes),
                })
                .map(Track::encode)
                .unwrap_or_default(),
            None => self.activity.table_writes.encode(),
        }
    }
}
