use std::cell::{RefCell};
use std::{env, io};
use std::fs::File;
use std::io::{BufRead, Read};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use protobuf::Message;
use protogen::dota_shared_enums::{CMsgDOTACombatLogEntry, DOTA_COMBATLOG_TYPES};
use protogen::dota_usermessages::{CDOTAUserMsg_ChatEvent, CDOTAUserMsg_ChatMessage, EDotaUserMessages};
use protogen::netmessages::SVC_Messages;
use crate::combat_log::CombatLog;
use crate::reader::Reader;
mod parser;
mod field;
mod serializer;
mod huffman_tree;
mod qfloat;
mod class;
mod entitiy;
mod string_table;
mod field_path;
mod reader;
mod field_op;
mod field_patch;
mod field_decoder;
mod field_state;
mod field_reader;
mod field_type;
mod combat_log;

use crate::entitiy::{Entity, EntityOperation};
use crate::parser::{Visitor, Stampede};

use mimalloc::MiMalloc;
use protogen::dota_usermessages::EDotaUserMessages::DOTA_UM_ChatMessage;
use protogen::usermessages::{CUserMessageSayText2, EBaseUserMessages};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

struct ExternalParser {
    tick: u32,
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

impl Visitor for ExternalParser {
    fn on_base_user_message(&self, ctx: &Stampede<Self>, p: EBaseUserMessages, msg: &Vec<u8>) {
        if p == EBaseUserMessages::UM_SayText2 {
            let message = CUserMessageSayText2::parse_from_bytes(msg).unwrap();

            println!("{} said: {}", message.param1(), message.param2())
        }
    }

    fn on_tick_start(&mut self, ctx: &Stampede<Self>) {
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
                    234
                };
                self.time = ((time_tick - paused_ticks) as f32 / 30.0).round() as i64;
            } else {
                self.time = grp.get_property_by_name("m_pGameRules.m_fGameTime").unwrap().as_float() as i64;
            };
        }
        self.tick = ctx.tick;
    }

    // fn on_entity(&self, ctx: &Stampede, p: &EntityOperations, e: &Entity) {
    //
    //     // println!("{:?}", ctx.get_first_entity_by_class_name("CDOTAGamerulesProxy").unwrap());
    //     if let EntityOperations::Entered = p {}
    // }
}


fn main() {
    let working_dir = env::current_dir().unwrap();
    let replay_path = working_dir.to_str().unwrap().to_owned() + "\\replays\\4080778303_443839809.dem";
    let mut buf = vec![];


    let mut x = io::stdin().lock();
    x.read_to_end(&mut buf).expect("TODO: panic message");
    // let file = File::open(replay_path);
    // file.unwrap().read_to_end(&mut buf).unwrap();

    let parser = Stampede::new(&buf, ExternalParser::new()).process();
    // let mut threads: Vec<JoinHandle<ExternalParser>> = vec![];
    // for _ in 0..4 {
    //     let b = buf.clone();
    //     threads.push(std::thread::spawn(move || {
    //         Stampede::new(&b, ExternalParser::new()).process()
    //     }));
    // }
    // for thread in threads {
    //     let x = thread.join().expect("TODO: panic message");
    //     println!("{}", x.tick)
    // }
    println!("{}", parser.tick);
    println!("{}", parser.time);
}
