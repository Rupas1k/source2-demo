use std::{env, io};
use std::io::{Read};

use stampede::prelude::{Entity, EntityEvent, Message, Observer, Parser};
use stampede::proto::{
    EBaseUserMessages,
    CUserMessageSayText2
};


struct MyObserver {
    tick: u32,
    time: i64,
}

impl MyObserver {
    fn new() -> Self {
        MyObserver {
            tick: 0,
            time: 0,
        }
    }
}
impl Observer for MyObserver {
    fn on_base_user_message(&mut self, ctx: &Parser<Self>, p: EBaseUserMessages, msg: &[u8]) {
        if p == EBaseUserMessages::UmSayText2 {
            let message = CUserMessageSayText2::decode(msg).unwrap();

            println!("{} said: {}", message.param1(), message.param2())
        }
    }

    fn on_tick_start(&mut self, ctx: &Parser<Self>) {
        if let Some(grp) = ctx.get_first_entity_by_class_name("CDOTAGamerulesProxy") {
            if let Some(old_time) = grp.get_property_by_name("m_pGameRules.m_flGameStartTime") {
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

    fn on_entity(&mut self, ctx: &Parser<Self>, event: EntityEvent, e: &Entity) {
        if event == EntityEvent::Updated
            && e.class.borrow().name.as_ref() == "CDOTA_NPC_Observer_Ward"
            && e.get_property_by_name("m_lifeState").unwrap().as_unsigned() == 1 {

            println!("Obs ward killed {:?}", e.map());

        }
    }

    // fn on_message(&mut self, ctx: &Stampede<Self>, msg_type: MessageType, msg: &[u8]) {
    //     if let MessageType::(x) = msg_type {
    //         println!("123");
    //     }
    // }

    // fn on_entity(&self, ctx: &Stampede, p: &EntityOperations, e: &Entity) {
    //
    //     // println!("{:?}", ctx.get_first_entity_by_class_name("CDOTAGamerulesProxy").unwrap());
    //     if let EntityOperations::Entered = p {}
    // }
}


fn main() -> io::Result<()> {
    let working_dir = env::current_dir()?;
    let replay_path = working_dir.to_str().unwrap().to_owned() + "\\replays\\4080778303_443839809.dem";
    let mut buf = vec![];


    let mut x = io::stdin().lock();
    x.read_to_end(&mut buf).expect("TODO: panic message");
    // let mut file = File::open(replay_path)?;
    // file.read_to_end(&mut buf).unentwrap();

    let parser = Parser::new(&buf, MyObserver::new()).process();
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

    Ok(())
}
