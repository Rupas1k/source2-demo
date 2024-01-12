use std::cell::{Ref, RefCell, RefMut};
use std::env;
use std::fs::{File};
use std::io::Read;
use std::ops::Deref;
use protobuf::{Enum, Message};
use protogen::demo::*;
use protogen::dota_usermessages::{EDotaUserMessages};
use protogen::gameevents::EBaseGameEvents;
use protogen::netmessages::SVC_Messages;
use protogen::networkbasetypes::{CNETMsg_Tick, NET_Messages};
use protogen::usermessages::{CUserMessageItemPickup, EBaseEntityMessages, EBaseUserMessages};

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

use reader::*;
use crate::entities::{Entity, EntityOperations};
use crate::field::{Field, FieldPath};
use crate::parser::{External, Stampede};



struct ExternalParser {
    tick: i32,
}

impl ExternalParser {
    fn new() -> Self {
        ExternalParser {
            tick: 0
        }
    }
}

impl External for ExternalParser {
    fn on_packet(&self, ctx: &Stampede, p: EDemoCommands, msg: &Vec<u8>) {
    }

    fn on_net_message(&self, ctx: &Stampede, p: NET_Messages, msg: &Vec<u8>) {
    }

    fn on_svc_message(&self, ctx: &Stampede, p: SVC_Messages, msg: &Vec<u8>) {
    }

    fn on_base_user_message(&self, ctx: &Stampede, p: EBaseUserMessages, msg: &Vec<u8>) {
    }

    fn on_base_entity_message(&self, ctx: &Stampede, p: EBaseEntityMessages, msg: &Vec<u8>) {
    }

    fn on_base_game_event(&self, ctx: &Stampede, p: EBaseGameEvents, msg: &Vec<u8>) {
    }

    fn on_tick_start(&self, ctx: &Stampede) {
        if let Some(grp) = ctx.get_first_entity_by_class_name("CDOTAGamerulesProxy") {
            // let _ = 1;
            let r = grp.get_property_by_name("m_pGameRules.m_flGameStartTime").unwrap();
            // let mut fp = FieldPath::new();
            // grp.class.borrow().get_field_path_for_name(&mut fp, "m_pGameRules.m_flGameStartTime");
            // let r = *grp.get_property_for_field_path(&mut fp).unwrap().as_float().unwrap();
            // grp.fp_cache.insert("m_pGameRules.m_flGameStartTime".to_string(), fp);
            // if r > 0.0 {
            //     tick += 1;
            //     // println!("{r}")
            // }
            // let tick = *grp.map()["m_pGameRules.m_flGameStartTime"]
            //     .as_ref()
            //     .unwrap()
            //     .as_float()
            //     .unwrap();
            // if tick > 0.0 {
            //     // println!("{tick}");
        }
    }

    fn on_entity(&self, ctx: &Stampede, p: &EntityOperations, e: &Entity) {

        // println!("{:?}", ctx.get_first_entity_by_class_name("CDOTAGamerulesProxy").unwrap());
        if let EntityOperations::Entered = p {
        }
    }
}


fn main() {
    let working_dir = env::current_dir().unwrap();
    // let replay_path = working_dir.to_str().unwrap().to_owned() + "\\replays\\7391247517_310492352.dem";
    let replay_path = working_dir.to_str().unwrap().to_owned() + "\\replays\\7391247517_310492352.dem";

    // let mut stream = FileStream::open(replay_path).unwrap();
    // // let mut reader = BinaryReader::new(&mut stream, Endian::Big);

    let mut file = File::open(&replay_path).unwrap();
    // let mmap = unsafe { Mmap::map(&file).unwrap() };

    let mut buf = Vec::<u8>::new();
    file.read_to_end(&mut buf);

    let reader = Reader::new(buf);

    let mut external = ExternalParser::new();
    let mut parser = Stampede::new(reader, &mut external);
    parser.process();
}
