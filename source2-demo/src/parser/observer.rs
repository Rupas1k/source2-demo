use crate::parser::Context;
use crate::proto::*;
use crate::{Entity, EntityEvents, GameEvent, StringTable};

#[cfg(feature = "dota")]
use crate::event::CombatLogEntry;

/// Result type for observers ([`anyhow::Result`])
pub type ObserverResult = anyhow::Result<()>;

/// A trait defining methods for handling game event and protobuf messages. Can
/// be attached to [`crate::Parser`] instance with [`crate::Parser::register_observer`]
/// method.
#[allow(unused_variables)]
pub trait Observer {
    fn on_demo_command(
        &mut self,
        ctx: &Context,
        msg_type: EDemoCommands,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_net_message(
        &mut self,
        ctx: &Context,
        msg_type: NetMessages,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_svc_message(
        &mut self,
        ctx: &Context,
        msg_type: SvcMessages,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_base_user_message(
        &mut self,
        ctx: &Context,
        msg_type: EBaseUserMessages,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_base_game_event(
        &mut self,
        ctx: &Context,
        msg_type: EBaseGameEvents,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_tick_start(&mut self, ctx: &Context) -> ObserverResult {
        Ok(())
    }

    fn on_tick_end(&mut self, ctx: &Context) -> ObserverResult {
        Ok(())
    }

    fn on_entity(&mut self, ctx: &Context, event: EntityEvents, entity: &Entity) -> ObserverResult {
        Ok(())
    }

    fn on_game_event(&mut self, ctx: &Context, ge: &GameEvent) -> ObserverResult {
        Ok(())
    }

    fn on_string_table(
        &mut self,
        ctx: &Context,
        st: &StringTable,
        modified: &[i32],
    ) -> ObserverResult {
        Ok(())
    }

    fn on_stop(&mut self, ctx: &Context) -> ObserverResult {
        Ok(())
    }

    #[cfg(feature = "dota")]
    fn on_combat_log(&mut self, ctx: &Context, cle: &CombatLogEntry) -> ObserverResult {
        Ok(())
    }

    #[cfg(feature = "dota")]
    fn on_dota_user_message(
        &mut self,
        ctx: &Context,
        msg_type: EDotaUserMessages,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    #[cfg(feature = "deadlock")]
    fn on_citadel_game_event(
        &mut self,
        ctx: &Context,
        msg_type: ECitadelGameEvents,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }

    #[cfg(feature = "deadlock")]
    fn on_citadel_user_message(
        &mut self,
        ctx: &Context,
        msg_type: CitadelUserMessageIds,
        msg: &[u8],
    ) -> ObserverResult {
        Ok(())
    }
}
