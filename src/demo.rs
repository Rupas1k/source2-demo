use protobuf::{Enum};
use protogen::gameevents::EBaseGameEvents;
use protogen::netmessages::SVC_Messages;
use protogen::networkbasetypes::*;


#[derive(Debug, Eq, PartialEq)]
pub struct PendingMessage {
    pub tick: u32,
    pub msg_type: i32,
    pub buf: Vec<u8>
}


// impl PartialOrd<Self> for PendingMessage {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(&other))
//     }
// }
//
//
// impl Ord for PendingMessage {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match self.tick.cmp(&other.tick) {
//             Ordering::Greater => Ordering::Less,
//             Ordering::Less => Ordering::Greater,
//             Ordering::Equal => {
//                 match self.priority().cmp(&other.priority()) {
//                     Ordering::Greater => Ordering::Greater,
//                     Ordering::Less => Ordering::Less,
//                     Ordering::Equal => self.order.cmp(&other.order).reverse()
//                 }
//             }
//         }
//     }
// }

impl PendingMessage {
    pub fn new (tick: u32, msg_type: i32, buf: Vec<u8>) -> Self {
        PendingMessage {
            tick,
            msg_type,
            buf
        }
    }

    fn priority(&self) -> i8 {
        match self.msg_type {
            x if x == NET_Messages::net_Tick.value()
                || x == SVC_Messages::svc_CreateStringTable.value()
                || x == SVC_Messages::svc_UpdateStringTable.value()
                || x == NET_Messages::net_SpawnGroup_Load.value() => { -10 }
            x if x == SVC_Messages::svc_PacketEntities.value() => { 5 }
            x if x == EBaseGameEvents::GE_Source1LegacyGameEvent.value() => { 10 }
            _ => { 0 }
        }
    }
}

// #[derive(Debug)]
// pub struct PendingMessages {
//     pub messages: Vec<PendingMessage>
// }

// impl PendingMessages {
//     pub fn new () -> Self {
//         PendingMessages {
//             messages: Vec::new()
//         }
//     }
//
//     pub fn push(&mut self, m: PendingMessage) {
//         self.messages.push(m);
//     }
//
//     pub fn len (&self) -> usize {
//         self.messages.len()
//     }
//
//     pub fn swap (&mut self, i: usize, j: usize) {
//         self.messages.swap(i, j);
//     }
//
//     pub fn pop(&mut self) -> Option<PendingMessage> {
//         self.messages.pop()
//     }
//
//     pub fn less(&self, i: usize, j: usize) -> bool {
//         match self.messages[i].tick.cmp(&self.messages[j].tick) {
//             std::cmp::Ordering::Greater => false,
//             std::cmp::Ordering::Less => true,
//             std::cmp::Ordering::Equal => self.messages[i].priority() < self.messages[j].priority(),
//         }
//     }
//
//     pub fn sort(&mut self) {
//         self.messages.sort_by(|a, b| a.priority().cmp(&b.priority()))
//     }
// }
