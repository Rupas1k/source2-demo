use std::cell::RefCell;
use std::rc::Rc;

use crate::try_observers;
use d2_stampede::prelude::*;
use d2_stampede::proto::*;

#[derive(Default)]
pub struct Chat {
    observers: Vec<Rc<RefCell<dyn ChatObserver + 'static>>>,
}

impl Chat {
    pub fn register_observer(&mut self, obs: Rc<RefCell<dyn ChatObserver + 'static>>) {
        self.observers.push(obs);
    }
}

impl Observer for Chat {
    fn on_dota_user_message(
        &mut self,
        ctx: &Context,
        msg_type: EDotaUserMessages,
        msg: &[u8],
    ) -> ObserverResult {
        match msg_type {
            EDotaUserMessages::DotaUmChatEvent => {
                let chat_event = CDotaUserMsgChatEvent::decode(msg)?;
                try_observers!(self, on_chat_event(ctx, &chat_event))
            }
            EDotaUserMessages::DotaUmChatMessage => {
                let chat_msg = CDotaUserMsgChatMessage::decode(msg)?;
                try_observers!(self, on_all_chat_message(ctx, &chat_msg))
            }
            EDotaUserMessages::DotaUmChatWheel => {
                let chat_wheel = CDotaUserMsgChatWheel::decode(msg)?;
                try_observers!(self, on_chat_wheel(ctx, &chat_wheel))
            }
            _ => Ok(()),
        }
    }
}

#[allow(unused_variables)]
pub trait ChatObserver {
    fn on_chat_event(&mut self, ctx: &Context, event: &CDotaUserMsgChatEvent) -> ObserverResult {
        Ok(())
    }

    fn on_all_chat_message(
        &mut self,
        ctx: &Context,
        event: &CDotaUserMsgChatMessage,
    ) -> ObserverResult {
        Ok(())
    }

    fn on_chat_wheel(&mut self, ctx: &Context, event: &CDotaUserMsgChatWheel) -> ObserverResult {
        Ok(())
    }
}
