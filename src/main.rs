use std::cell::{RefCell};
use std::{env, io};
use std::fs::File;
use std::io::{BufRead, Read};
use std::rc::Rc;
use crate::bit_reader::{Reader, ReaderMethods};
// use bitvec::prelude::*;

mod reader;
mod demo;
mod parser;
mod field;
mod serializer;
mod huffman_tree;
mod qfloat;
mod class;
mod entities;
mod string_table;
mod field_path;
mod bit_reader;

use reader::*;
use crate::entities::{Entity, EntityOperations};
use crate::parser::{External, Stampede};


struct ExternalParser {
    tick: i32,
    time: i64,
}

impl ExternalParser {
    fn new() -> Self {
        ExternalParser {
            tick: 0,
            time: 0,
        }
    }
}

impl External for ExternalParser {
    // fn on_packet(&self, ctx: &Stampede, p: EDemoCommands, msg: &Vec<u8>) {
    // }
    //
    // fn on_net_message(&self, ctx: &Stampede, p: NET_Messages, msg: &Vec<u8>) {
    // }
    //
    // fn on_svc_message(&self, ctx: &Stampede, p: SVC_Messages, msg: &Vec<u8>) {
    // }
    //
    // fn on_base_user_message(&self, ctx: &Stampede, p: EBaseUserMessages, msg: &Vec<u8>) {
    // }
    //
    // fn on_base_entity_message(&self, ctx: &Stampede, p: EBaseEntityMessages, msg: &Vec<u8>) {
    // }
    //
    // fn on_base_game_event(&self, ctx: &Stampede, p: EBaseGameEvents, msg: &Vec<u8>) {
    // }

    fn on_tick_start(&mut self, ctx: &Stampede) {
        if let Some(grp) = ctx.get_first_entity_by_class_name("CDOTAGamerulesProxy") {
            if let Some(old_time) = grp.get_property_by_name("m_pGameRules.m_flGameStartTime") {
                // println!("{:?}", grp.map());
                // panic!();
                // println!("{:?}", grp.get_property_by_name("m_pGameRules.m_nPauseStartTick").unwrap());
                let is_paused = grp.get_property_by_name("m_pGameRules.m_bGamePaused").unwrap().as_bool();
                let time_tick = match is_paused {
                    true => grp.get_property_by_name("m_pGameRules.m_nPauseStartTick").unwrap().as_signed(),
                    false => self.tick as i64
                };
                let paused_ticks = if let Some(x) = grp.get_property_by_name("m_pGameRules.m_nTotalPausedTicks") {
                    x.as_signed()
                } else {
                    0
                };
                self.time = ((time_tick - paused_ticks) as f32 / 30.0).round() as i64;
            } else {
                self.time = 0;
            };
        }
    }

    fn on_entity(&self, ctx: &Stampede, p: &EntityOperations, e: &Entity) {

        // println!("{:?}", ctx.get_first_entity_by_class_name("CDOTAGamerulesProxy").unwrap());
        if let EntityOperations::Entered = p {}
    }
}


fn main() {
    let working_dir = env::current_dir().unwrap();
    let replay_path = working_dir.to_str().unwrap().to_owned() + "\\replays\\7391247517_310492352.dem";
    let mut buf = vec![];


    let mut x = io::stdin().lock();
    x.read_to_end(&mut buf).expect("TODO: panic message");
    // let file = File::open(replay_path);
    // file.unwrap().read_to_end(&mut buf).unwrap();

    let reader = Reader::new(buf.as_slice());

    let external = Rc::new(RefCell::new(ExternalParser::new()));
    let mut parser = Stampede::new(reader, Rc::clone(&external));
    parser.process();
    println!("{}", external.borrow().time);
}
