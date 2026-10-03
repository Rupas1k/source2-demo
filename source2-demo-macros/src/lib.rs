#![doc = include_str!("../README.md")]
//!
//! # Procedural Macros for Source 2 Replay Parser
//!
//! This crate provides procedural macros that simplify implementing the `Observer`
//! and `DemoRewriter` traits.
//!
//! ## Overview
//!
//! The macros automate much of the boilerplate required for handling replay events:
//! - Automatically implement the Observer trait
//! - Generate protobuf message decoding
//! - Set up interest flags based on used methods
//! - Handle optional `Context` parameters
//!
//! Handler attributes may be imported and used as single-segment attributes,
//! such as `#[on_message]`, or written with a path, such as
//! `#[source2_demo_macros::on_message]`.
//!
//! ## Main Macros
//!
//! ### `#[observer]` - Main implementation macro
//!
//! Automatically implements the Observer trait with all necessary methods.
//! Takes an impl block with event handlers and generates the full implementation.
//!
//! ```no_run
//! # use source2_demo::prelude::*;
//! #[derive(Default)]
//! struct MyObserver;
//!
//! #[observer]
//! impl MyObserver {
//!     #[on_tick_start]
//!     fn on_tick(&mut self, ctx: &Context) -> ObserverResult {
//!         println!("Tick: {}", ctx.tick());
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ### `#[rewriter]` - Demo writer implementation macro
//!
//! Automatically implements the `DemoRewriter` trait for replay rewriting.
//! Use it when you want to drop, replace, or mutate demo messages while the
//! writer emits a new replay. Packet message handlers can receive either the raw
//! packet payload or a decoded protobuf message. The protobuf form infers the
//! packet id from the message type, just like `#[on_message]`.
//!
//! #### Drop a decoded packet message
//!
//! ```no_run
//! # use source2_demo::prelude::*;
//! # use source2_demo::proto::CDotaUserMsgChatMessage;
//! # use source2_demo::writer::*;
//! struct RemoveChat;
//!
//! #[rewriter]
//! impl RemoveChat {
//!     #[rewrite_packet_message]
//!     fn remove_chat(
//!         &mut self,
//!         _message: CDotaUserMsgChatMessage,
//!     ) -> Result<MessageRewrite, ParserError> {
//!         Ok(MessageRewrite::Drop)
//!     }
//! }
//! ```
//!
//! #### Mutate and re-encode a decoded packet message
//!
//! ```no_run
//! # use source2_demo::prelude::*;
//! # use source2_demo::proto::CDotaUserMsgChatMessage;
//! # use source2_demo::writer::*;
//! struct RedactChat;
//!
//! #[rewriter]
//! impl RedactChat {
//!     #[rewrite_packet_message]
//!     fn redact_chat(
//!         &mut self,
//!         message: &mut CDotaUserMsgChatMessage,
//!     ) -> Result<MessageRewrite, ParserError> {
//!         message.message_text = Some("[redacted]".to_string());
//!         Ok(MessageRewrite::Rewrite)
//!     }
//! }
//! ```
//!
//! #### Rewrite a string table entry
//!
//! ```no_run
//! # use source2_demo::prelude::*;
//! # use source2_demo::proto::CMsgPlayerInfo;
//! # use source2_demo::writer::*;
//! struct AnonymizeUserInfo;
//!
//! #[rewriter]
//! impl AnonymizeUserInfo {
//!     #[rewrite_string_table_entry]
//!     fn rewrite_userinfo(
//!         &mut self,
//!         table_name: &str,
//!         entry: &mut StringTableEntryUpdate,
//!     ) -> Result<(), ParserError> {
//!         if table_name == "userinfo" {
//!             if let Some(value) = entry.value_mut() {
//!                 let mut player = CMsgPlayerInfo::decode(value.as_slice())?;
//!                 player.name = Some("Anonymous".to_string());
//!                 *value = player.encode_to_vec();
//!             }
//!         }
//!         Ok(())
//!     }
//! }
//! ```
//!
//! #### Rewrite decoded entity fields
//!
//! ```no_run
//! # use source2_demo::prelude::*;
//! # use source2_demo::writer::*;
//! struct RemoveSteamIds;
//!
//! #[rewriter]
//! impl RemoveSteamIds {
//!     #[rewrite_field(class = "CDOTA_PlayerResource", field = ends_with("m_iPlayerSteamID"))]
//!     fn remove_steam_id(&mut self, _value: u64) -> u64 {
//!         0
//!     }
//! }
//! ```
//!
//! ### Event Handler Macros
//!
//! These attributes mark methods as event handlers within an #[observer] impl:
//!
//! - `#[on_message]` - Handles protobuf messages
//! - `#[on_tick_start]` - Called at tick start
//! - `#[on_tick_end]` - Called at tick end
//! - `#[on_entity]` - Called for entity changes
//! - `#[on_game_event]` - Called for game events
//! - `#[on_string_table]` - Called for string table updates
//! - `#[on_stop]` - Called when replay ends
//! - `#[on_combat_log]` - Called for Dota 2 or Deadlock combat log entries
//!
//! ### Trait Markers
//!
//! These mark impl blocks with which data types to track:
//!
//! - `#[uses_all]` - Track all supported data
//! - `#[uses_entities]` - Track entities
//! - `#[uses_string_tables]` - Track string tables
//! - `#[uses_game_events]` - Track game events
//! - `#[uses_combat_log]` - Track combat log for enabled games
//!
//! ## How the Macros Work
//!
//! The `#[observer]` macro:
//!
//! 1. Scans all methods for event handler attributes
//! 2. Collects parameter types to generate interest flags
//! 3. Creates decoding logic for protobuf messages
//! 4. Generates the full `Observer` trait implementation
//! 5. Handles optional parameters (Context is optional)
//!
//! ## Complete Example
//!
//! ```no_run
//! use source2_demo::prelude::*;
//!
//! #[derive(Default)]
//! struct GameAnalyzer {
//!     tick_count: u32,
//!     entity_count: u32,
//! }
//!
//! #[observer]
//! #[uses_entities]
//! impl GameAnalyzer {
//!     #[on_tick_start]
//!     fn on_tick(&mut self, ctx: &Context) -> ObserverResult {
//!         self.tick_count += 1;
//!         Ok(())
//!     }
//!
//!     #[on_entity]
//!     fn on_entity_create(&mut self, event: EntityEvents, entity: &Entity) -> ObserverResult {
//!         if event == EntityEvents::Created {
//!             self.entity_count += 1;
//!         }
//!         Ok(())
//!     }
//!
//!     #[on_message]
//!     fn on_chat(&mut self, msg: CDotaUserMsgChatMessage) -> ObserverResult {
//!         println!("Chat: {}", msg.message_text());
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Key Features
//!
//! - **Automatic Interest Management**: Macro automatically determines which interests
//!   to set based on which handler methods are defined
//! - **Optional Context**: Pass `ctx: &Context` to any handler or omit it
//! - **Automatic Message Decoding**: `#[on_message]` handlers automatically decode
//!   protobuf messages based on parameter type
//! - **Filtering Attributes**: Filter events with string arguments:
//!   - `#[on_entity("CDOTA_Unit_Hero_Axe")]` - Only heroes named Axe
//!   - `#[on_game_event("player_death")]` - Only death events
//!   - `#[on_string_table("userinfo")]` - Only userinfo table updates
//! - **Game-Specific Messages**: Use `#[on_message]` with Dota 2, Deadlock, or
//!   CS2 protobuf message types

mod observer_impl;
mod protobuf_map;
mod rewriter_impl;
mod type_utils;

use proc_macro::TokenStream;

#[allow(unused_mut)]
#[proc_macro_attribute]
/// Implements the Observer trait for your struct.
///
/// This is the main macro that ties everything together. Apply it to an `impl` block
/// that contains event handler methods marked with `#[on_*]` attributes.
///
/// # What It Does
///
/// - Automatically implements the Observer trait
/// - Generates code to call all handler methods at appropriate times
/// - Sets up interest flags based on which handlers are defined
/// - Decodes protobuf messages and passes them to handlers
/// - Filters events based on optional string arguments
///
/// # Modes
///
/// Use `#[observer]` to infer interests from handlers and `#[uses_*]` markers.
/// Use `#[observer(all)]` or `#[uses_all]` to enable every interest.
///
/// # Handler Attributes
///
/// Use these attributes inside the impl block to mark event handlers:
///
/// - `#[on_tick_start]` - Called at the start of each tick
/// - `#[on_tick_end]` - Called at the end of each tick
/// - `#[on_entity]` - Called when entities change
/// - `#[on_entity("ClassName")]` - Only for specific entity classes
/// - `#[on_message]` - Called for protobuf messages (type inferred from param)
/// - `#[on_game_event]` - Called for all game events
/// - `#[on_game_event("event_name")]` - Only for specific events
/// - `#[on_string_table]` - Called when string tables update
/// - `#[on_string_table("table_name")]` - Only for specific tables
/// - `#[on_stop]` - Called when replay ends
/// - `#[on_combat_log]` - Called for Dota 2 or Deadlock combat log entries
///
/// # Trait Attributes
///
/// Apply these to the `impl` block or individual methods to enable tracking:
///
/// - `#[uses_all]` - Enable all tracking
/// - `#[uses_entities]` - Enable entity tracking
/// - `#[uses_string_tables]` - Enable string table tracking
/// - `#[uses_game_events]` - Enable game event tracking
/// - `#[uses_combat_log]` - Enable combat log tracking for enabled games
///
/// # Parameter Guidelines
///
/// Handlers can receive these parameters. Most are optional; `#[on_message]`
/// requires either one protobuf message or a supported message enum plus
/// `&[u8]` payload.
/// - `ctx: &Context` - Current replay state (always optional)
/// - Specific parameters depending on the handler:
///   - `event: EntityEvents` or `event: &EntityEvents` (on_entity)
///   - `entity: &Entity` (on_entity)
///   - `ge: &GameEvent` (on_game_event)
///   - `table: &StringTable` (on_string_table)
///   - `modified: &[i32]` (on_string_table)
///   - `cle: &CombatLogEntry` (on_combat_log)
///   - A protobuf message, or a supported message enum plus `&[u8]` payload
///     (on_message)
///
/// Handler parameters can appear in any order. Handlers return
/// `ObserverResult`.
///
/// # Examples
///
/// ## Basic observer
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// #[derive(Default)]
/// struct BasicObserver;
///
/// #[observer]
/// impl BasicObserver {
///     #[on_tick_start]
///     fn on_tick_start(&mut self, ctx: &Context) -> ObserverResult {
///         println!("Tick: {}", ctx.tick());
///         Ok(())
///     }
/// }
/// ```
///
/// ## With entity tracking
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// #[derive(Default)]
/// struct EntityTracker;
///
/// #[observer]
/// #[uses_entities]
/// impl EntityTracker {
///     #[on_entity]
///     fn on_hero_created(&mut self, event: EntityEvents, entity: &Entity) -> ObserverResult {
///         if event == EntityEvents::Created && entity.class().name().starts_with("CDOTA_Unit_Hero_") {
///             println!("Hero created: {}", entity.class().name());
///         }
///         Ok(())
///     }
/// }
/// ```
///
/// ## With multiple handlers
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// #[derive(Default)]
/// struct ComplexObserver {
///     ticks: u32,
///     messages: u32,
/// }
///
/// #[observer]
/// impl ComplexObserver {
///     #[on_tick_start]
///     fn on_tick(&mut self, ctx: &Context) -> ObserverResult {
///         self.ticks += 1;
///         Ok(())
///     }
///
///     #[on_message]
///     fn on_chat(&mut self, msg: CDotaUserMsgChatMessage) -> ObserverResult {
///         self.messages += 1;
///         println!("Message: {}", msg.message_text());
///         Ok(())
///     }
///
///     #[on_stop]
///     fn on_replay_end(&mut self) -> ObserverResult {
///         println!("Total ticks: {}, messages: {}", self.ticks, self.messages);
///         Ok(())
///     }
/// }
/// ```
///
/// ## With game event filtering
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// #[derive(Default)]
/// struct DeathTracker {
///     deaths: u32,
/// }
///
/// #[observer]
/// impl DeathTracker {
///     #[on_game_event("player_death")]
///     fn on_death(&mut self, ctx: &Context, ge: &GameEvent) -> ObserverResult {
///         self.deaths += 1;
///         Ok(())
///     }
/// }
/// ```
///
/// # Interest Flags
///
/// The macro automatically determines which interest flags to set based on which
/// handlers are defined. For example:
/// - If you have `#[on_tick_start]`, `Interests::TICK_START` is added
/// - If you have `#[on_entity]`, both `Interests::ENTITY_STATE` and
///   `Interests::ENTITY_EVENTS` are added
/// - If you have `#[on_message]` handlers, appropriate message interest flags are added
///
/// You can also use trait attributes like `#[uses_entities]` to manually ensure
/// certain interests are set.
pub fn observer(attr: TokenStream, item: TokenStream) -> TokenStream {
    observer_impl::expand_observer(attr, item)
}

/// Implements the `DemoRewriter` trait for your struct.
///
/// Apply this to an inherent `impl` block and mark methods with writer callback
/// attributes. The macro builds the `RewriteInterests` mask from those methods
/// so the writer only decodes the parts of the replay your rewrite needs.
///
/// # Callback Types
///
/// - `#[rewrite_demo_message]` sees outer `EDemoCommands` payloads before
///   packet-level decoding.
/// - `#[rewrite_packet_message]` sees messages inside demo packets. It can take
///   raw `(msg_type, payload)` arguments or a decoded protobuf message.
/// - `#[rewrite_packet_messages]` can mutate the final decoded packet message
///   list after individual packet-message callbacks run.
/// - `#[rewrite_demo_string_tables]`, `#[rewrite_string_table_entry]`,
///   `#[rewrite_svc_create_string_table]`, and
///   `#[rewrite_svc_update_string_table]` target string-table data at different
///   levels of decoding.
/// - `#[rewrite_field]` and `#[replace_entity_field]` replace decoded entity
///   field values, while `#[should_rewrite_entity]` filters which entities enter
///   that path and `#[should_track_entity]` filters which entities retain state.
///
/// # Return Values
///
/// Message callbacks return `Result<MessageRewrite, ParserError>`.
/// `Keep` leaves the current payload alone, `Drop` removes it, `Replace(bytes)`
/// writes explicit bytes, and `Rewrite` re-encodes a mutable decoded protobuf
/// argument when that callback supports it. List, entry, and filter callbacks
/// use the return type of the matching `DemoRewriter` method.
///
/// # Examples
///
/// ## Drop a decoded packet message
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # use source2_demo::proto::CDotaUserMsgChatMessage;
/// # use source2_demo::writer::*;
/// struct RemoveChat;
///
/// #[rewriter]
/// impl RemoveChat {
///     #[rewrite_packet_message]
///     fn remove_chat(
///         &mut self,
///         _msg: CDotaUserMsgChatMessage,
///     ) -> Result<MessageRewrite, ParserError> {
///         Ok(MessageRewrite::Drop)
///     }
/// }
/// ```
///
/// ## Mutate and re-encode a decoded packet message
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # use source2_demo::proto::CDotaUserMsgChatMessage;
/// # use source2_demo::writer::*;
/// struct RedactChat;
///
/// #[rewriter]
/// impl RedactChat {
///     #[rewrite_packet_message]
///     fn redact_chat(
///         &mut self,
///         message: &mut CDotaUserMsgChatMessage,
///     ) -> Result<MessageRewrite, ParserError> {
///         message.message_text = Some("[redacted]".to_string());
///         Ok(MessageRewrite::Rewrite)
///     }
/// }
/// ```
///
/// ## Rewrite a string table entry
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # use source2_demo::proto::CMsgPlayerInfo;
/// # use source2_demo::writer::*;
/// struct AnonymizeUserInfo;
///
/// #[rewriter]
/// impl AnonymizeUserInfo {
///     #[rewrite_string_table_entry]
///     fn rewrite_userinfo(
///         &mut self,
///         table_name: &str,
///         entry: &mut StringTableEntryUpdate,
///     ) -> Result<(), ParserError> {
///         if table_name == "userinfo" {
///             if let Some(value) = entry.value_mut() {
///                 let mut player = CMsgPlayerInfo::decode(value.as_slice())?;
///                 player.name = Some("Anonymous".to_string());
///                 *value = player.encode_to_vec();
///             }
///         }
///         Ok(())
///     }
/// }
/// ```
///
/// ## Rewrite decoded entity fields
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # use source2_demo::writer::*;
/// struct RemoveSteamIds;
///
/// #[rewriter]
/// impl RemoveSteamIds {
///     #[rewrite_field(class = "CDOTA_PlayerResource", field = ends_with("m_iPlayerSteamID"))]
///     fn remove_steam_id(&mut self, _value: u64) -> u64 {
///         0
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn rewriter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    rewriter_impl::expand_rewriter(item)
}

/// Marks a method as a protobuf message handler.
///
/// Use this to handle specific protobuf message types. The message type is inferred
/// from the method's parameter type, which should be a mapped protobuf message
/// struct.
///
/// The method will automatically decode binary message data and call your handler
/// with the decoded message object.
///
/// # Parameters
///
/// A decoded handler can receive:
///
/// - `ctx: &Context` (optional)
/// - One protobuf message by value or shared reference
///
/// A raw handler can instead receive:
///
/// - `ctx: &Context` (optional)
/// - One supported message enum by value or shared reference
/// - `payload: &[u8]`
///
/// Parameters can appear in any order.
/// Return `ObserverResult`.
///
/// # Supported Message Types
///
/// - `CDotaUserMsgChatMessage` and other Dota 2 messages
/// - `CCitadelUserMsgChatMsg` and other Deadlock messages
/// - `CCsUsrMsgVguiMenu` and other CS2 messages
/// - Other mapped protobuf message types
///
/// # Examples
///
/// ## Handle Dota 2 chat messages
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_message]
/// fn on_chat(&mut self, ctx: &Context, msg: CDotaUserMsgChatMessage) -> ObserverResult {
///     println!("[{}] {}", ctx.tick(), msg.message_text());
///     Ok(())
/// }
/// # }
/// ```
///
/// ## Handle without context
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_message]
/// fn on_chat(&mut self, msg: CDotaUserMsgChatMessage) -> ObserverResult {
///     println!("Message: {}", msg.message_text());
///     Ok(())
/// }
/// # }
/// ```
///
/// ## Handle by reference
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_message]
/// fn on_chat(&mut self, msg: &CDotaUserMsgChatMessage) -> ObserverResult {
///     println!("Message: {}", msg.message_text());
///     Ok(())
/// }
/// # }
/// ```
///
/// ## Handle raw messages
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_message]
/// fn on_raw(&mut self, msg_type: SvcMessages, payload: &[u8]) -> ObserverResult {
///     println!("{msg_type:?}: {} bytes", payload.len());
///     Ok(())
/// }
/// # }
/// ```
#[proc_macro_attribute]
pub fn on_message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Rewrites an outer demo command payload.
///
/// Use this for callbacks that operate before packet-level decoding, such as
/// dropping or replacing whole `EDemoCommands` messages. Raw handlers can
/// receive `ctx: &Context`, `tick: u32`, `msg_type: EDemoCommands`, and
/// `payload: &[u8]`, in any supported callback-argument order.
///
/// Return `Result<MessageRewrite, ParserError>`.
#[proc_macro_attribute]
pub fn rewrite_demo_message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Rewrites an individual message inside a demo packet.
///
/// A raw handler can receive packet metadata and payload bytes. A typed handler
/// can receive a protobuf message; the macro infers the packet message id from
/// that protobuf type. Mutable protobuf arguments may return
/// `MessageRewrite::Rewrite` to re-encode the modified message.
///
/// # Parameters
///
/// A raw handler can receive `ctx: &Context`, `tick: u32`, `msg_type: i32`,
/// and `payload: &[u8]`. A typed handler can receive `ctx: &Context`,
/// `tick: u32`, and one protobuf message by value, shared reference, or mutable
/// reference. Parameters can appear in any order.
///
/// Return `Result<MessageRewrite, ParserError>`.
///
/// # Examples
///
/// ## Drop a decoded protobuf message
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # use source2_demo::proto::CDotaUserMsgChatMessage;
/// # use source2_demo::writer::*;
/// # struct MyRewriter;
/// # impl MyRewriter {
/// #[rewrite_packet_message]
/// fn remove_chat(&mut self, _msg: CDotaUserMsgChatMessage) -> Result<MessageRewrite, ParserError> {
///     Ok(MessageRewrite::Drop)
/// }
/// # }
/// ```
///
/// ## Mutate a decoded protobuf message
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # use source2_demo::proto::CDotaUserMsgChatMessage;
/// # use source2_demo::writer::*;
/// # struct MyRewriter;
/// # impl MyRewriter {
/// #[rewrite_packet_message]
/// fn redact_chat(
///     &mut self,
///     msg: &mut CDotaUserMsgChatMessage,
/// ) -> Result<MessageRewrite, ParserError> {
///     msg.message_text = Some("[redacted]".to_string());
///     Ok(MessageRewrite::Rewrite)
/// }
/// # }
/// ```
#[proc_macro_attribute]
pub fn rewrite_packet_message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Mutates the decoded packet message list after per-message rewrites.
///
/// Use this when the rewrite needs packet-wide context, such as appending a new
/// `PacketMessage`, removing several messages together, or reordering messages
/// within one packet.
///
/// The handler can receive `ctx: &Context`, `tick: u32`, and
/// `messages: &mut Vec<PacketMessage>` in any order. Return
/// `Result<(), ParserError>`.
#[proc_macro_attribute]
pub fn rewrite_packet_messages(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Rewrites a decoded `CDemoStringTables` outer demo message.
///
/// This callback sees the full demo string-table message. Return
/// `MessageRewrite::Rewrite` after mutating the decoded message, or
/// `MessageRewrite::Replace(bytes)` to provide encoded bytes directly.
///
/// The handler can receive `ctx: &Context`, `tick: u32`, and
/// `message: &mut CDemoStringTables` in any order. Return
/// `Result<MessageRewrite, ParserError>`.
#[proc_macro_attribute]
pub fn rewrite_demo_string_tables(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Rewrites one decoded string table entry update.
///
/// Use this for targeted string-table edits such as rewriting `userinfo` rows.
/// The handler can receive `ctx: &Context`, `tick: u32`, `table_name: &str`,
/// and `entry: &mut StringTableEntryUpdate` in any order. Return
/// `Result<(), ParserError>`.
///
/// # Examples
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # use source2_demo::proto::CMsgPlayerInfo;
/// # use source2_demo::writer::*;
/// # struct MyRewriter;
/// # impl MyRewriter {
/// #[rewrite_string_table_entry]
/// fn anonymize_userinfo(
///     &mut self,
///     table_name: &str,
///     entry: &mut StringTableEntryUpdate,
/// ) -> Result<(), ParserError> {
///     if table_name == "userinfo" {
///         if let Some(value) = entry.value_mut() {
///             let mut player = CMsgPlayerInfo::decode(value.as_slice())?;
///             player.name = Some("Anonymous".to_string());
///             *value = player.encode_to_vec();
///         }
///     }
///     Ok(())
/// }
/// # }
/// ```
#[proc_macro_attribute]
pub fn rewrite_string_table_entry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Rewrites a decoded `svc_CreateStringTable` packet message.
///
/// Return `MessageRewrite::Rewrite` after mutating the decoded
/// `CSvcMsgCreateStringTable`, or return `Replace` / `Drop` for explicit output
/// control.
///
/// The handler can receive `ctx: &Context`, `tick: u32`, and
/// `message: &mut CSvcMsgCreateStringTable` in any order. Return
/// `Result<MessageRewrite, ParserError>`.
#[proc_macro_attribute]
pub fn rewrite_svc_create_string_table(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Rewrites a decoded `svc_UpdateStringTable` packet message.
///
/// This is useful for incremental string-table updates that arrive inside
/// packet data. Return `MessageRewrite::Rewrite` after mutating the decoded
/// `CSvcMsgUpdateStringTable`.
///
/// The handler can receive `ctx: &Context`, `tick: u32`, and
/// `message: &mut CSvcMsgUpdateStringTable` in any order. Return
/// `Result<MessageRewrite, ParserError>`.
#[proc_macro_attribute]
pub fn rewrite_svc_update_string_table(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Replaces decoded entity field values with custom logic.
///
/// This low-level form receives the current field as `&FieldValue` and returns
/// `Option<FieldValue>`. Return `Some` to replace the value; return `None` to
/// leave it unchanged or let later generated checks continue.
///
/// The handler can receive `ctx: &Context`, `event: EntityEvents`,
/// `entity: &Entity`, `field_name: &str`, and `value: &FieldValue` in any
/// order.
#[proc_macro_attribute]
pub fn replace_entity_field(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Replaces decoded entity field values with class and field filters.
///
/// # Attribute Arguments
///
/// - `class = ...` is required.
/// - `field = ...` is optional.
///
/// Both filters accept an exact string literal or the `exact`, `starts_with`,
/// `ends_with`, `contains`, `any`, `all`, and `not` predicates.
///
/// Field-specific handlers may use typed values such as `u64`, `bool`, or
/// `String`. Class-only handlers must use `FieldValue` or `&FieldValue`
/// because there is no field predicate to determine one concrete value type.
///
/// # Handler Parameters
///
/// A handler must receive one field value parameter. Field-specific handlers
/// support `&str`, `String`, `&String`, `bool`, `f32`, `i8`, `i16`, `i32`,
/// `i64`, `u8`, `u16`, `u32`, `u64`, `[f32; 2]`, `[f32; 3]`, `[f32; 4]`,
/// `FieldValue`, and `&FieldValue`. Class-only handlers must use `FieldValue`
/// or `&FieldValue`.
///
/// A handler can also receive any of these optional parameters:
///
/// - `ctx: &Context`
/// - `event: EntityEvents`
/// - `entity: &Entity`
/// - `field_name: &str`
///
/// Parameters can appear in any order except that `field_name: &str` must come
/// before the field value parameter. This matters because `&str` can itself be
/// the field value type.
///
/// # Return Values
///
/// Return a replacement value directly to always rewrite the field. Return
/// `Option<T>` to rewrite with `Some(value)` or leave the field unchanged with
/// `None`. The replacement must convert into `FieldValue`.
///
/// # Examples
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # use source2_demo::writer::*;
/// # struct MyRewriter;
/// # impl MyRewriter {
/// #[rewrite_field(class = "CDOTA_PlayerResource", field = ends_with("m_iPlayerSteamID"))]
/// fn remove_steam_id(&mut self, _value: u64) -> u64 {
///     0
/// }
/// # }
/// ```
#[proc_macro_attribute]
pub fn rewrite_field(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Filters which entities enter the entity field rewrite path.
///
/// Return `false` to skip all entity-field replacement callbacks for that
/// entity. Use this to keep broad `#[rewrite_field]` handlers from forcing the
/// writer to decode and re-encode unrelated entity classes.
///
/// The handler can receive `ctx: &Context`, `event: EntityEvents`, and
/// `entity: &Entity` in any order. Return `bool`.
#[proc_macro_attribute]
pub fn should_rewrite_entity(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Filters which entities retain decoded field state while rewriting.
///
/// Return `false` to skip state decoding when the entity also does not enter
/// the rewrite path. The default is `true`, preserving complete entity state.
///
/// The handler can receive `ctx: &Context`, `event: EntityEvents`, and
/// `entity: &Entity` in any order. Return `bool`.
#[proc_macro_attribute]
pub fn should_track_entity(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a method as a tick-start handler.
///
/// This handler is called at the beginning of each game tick. Use it for
/// per-tick logic like updating game state, tracking changes, or generating output.
///
/// # Parameters
///
/// The handler can receive:
/// - `ctx: &Context` (optional) - Current replay state including tick number
///
/// Return `ObserverResult`.
///
/// # When It's Called
///
/// Called right after the tick number is incremented in the Context, but before
/// any other tick-specific events are processed.
///
/// # Examples
///
/// ## Track entity state every tick
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_tick_start]
/// fn on_tick_start(&mut self, ctx: &Context) -> ObserverResult {
///     if ctx.tick() % 30 == 0 {  // Every second (30 ticks/sec)
///         println!("Ticks processed: {}", ctx.tick());
///     }
///     Ok(())
/// }
/// # }
/// ```
///
/// ## Without context
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_tick_start]
/// fn on_tick_start(&mut self) -> ObserverResult {
///     // Do something without needing context
///     Ok(())
/// }
/// # }
/// ```
#[proc_macro_attribute]
pub fn on_tick_start(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a method as a tick-end handler.
///
/// This handler is called at the end of each game tick. Use it to finalize
/// per-tick calculations, output results, or reset temporary state.
///
/// # Parameters
///
/// The handler can receive:
/// - `ctx: &Context` (optional) - Current replay state
///
/// Return `ObserverResult`.
///
/// # When It's Called
///
/// Called after all events for the current tick have been processed.
///
/// # Examples
///
/// ## Flush buffered data at end of tick
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs {
/// #     buffer: Vec<String>,
/// # }
/// # impl MyObs {
/// #[on_tick_end]
/// fn on_tick_end(&mut self, ctx: &Context) -> ObserverResult {
///     // Output buffered data
///     for item in self.buffer.drain(..) {
///         println!("[{}] {}", ctx.tick(), item);
///     }
///     Ok(())
/// }
/// # }
/// ```
#[proc_macro_attribute]
pub fn on_tick_end(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a method as an entity event handler.
///
/// This handler is called when entities are created, updated, or deleted.
/// Optionally filter to specific entity class names.
///
/// # Parameters
///
/// The handler can receive:
/// - `ctx: &Context` (optional) - Current replay state
/// - `event: EntityEvents` or `event: &EntityEvents` (optional) - Type of entity event
/// - `entity: &Entity` (optional) - The entity that changed
///
/// Parameters can appear in any order. Return `ObserverResult`.
///
/// # Filtering
///
/// Pass a class name to only handle entities of that type:
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_entity("CDOTA_Unit_Hero_Axe")]
/// fn on_axe(&mut self, entity: &Entity) -> ObserverResult {
///     // Only called for Axe hero entity
///     Ok(())
/// }
/// # }
/// ```
///
/// # Examples
///
/// ## Track all entity changes
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs {
/// #     created: usize,
/// # }
/// # impl MyObs {
/// #[on_entity]
/// fn on_entity(&mut self, event: EntityEvents, entity: &Entity) -> ObserverResult {
///     if event == EntityEvents::Created {
///         self.created += 1;
///     }
///     Ok(())
/// }
/// # }
/// ```
///
/// ## Track specific entity type
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_entity("CDOTA_PlayerResource")]
/// fn on_player_resource(&mut self, ctx: &Context, entity: &Entity) -> ObserverResult {
///     // Only called when player resource entity updates
///     Ok(())
/// }
/// # }
/// ```
#[proc_macro_attribute]
pub fn on_entity(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a method as a game event handler.
///
/// This handler is called when game events occur (kills, deaths, item purchases, etc.).
/// Optionally filter to specific event names.
///
/// # Parameters
///
/// The handler can receive:
/// - `ctx: &Context` (optional) - Current replay state
/// - `ge: &GameEvent` (optional) - The game event
///
/// Parameters can appear in any order. Return `ObserverResult`.
///
/// # Filtering
///
/// Pass an event name to only handle that event type:
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_game_event("player_death")]
/// fn on_death(&mut self, ge: &GameEvent) -> ObserverResult {
///     // Only called for player_death events
///     Ok(())
/// }
/// # }
/// ```
///
/// # Examples
///
/// ## Handle all game events
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_game_event]
/// fn on_event(&mut self, ctx: &Context, ge: &GameEvent) -> ObserverResult {
///     println!("[{}] Event: {}", ctx.tick(), ge.name());
///     Ok(())
/// }
/// # }
/// ```
///
/// ## Handle specific event type
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs {
/// #     kill_count: u32,
/// # }
/// # impl MyObs {
/// #[on_game_event("dota_player_kill")]
/// fn on_kill(&mut self, ge: &GameEvent) -> ObserverResult {
///     self.kill_count += 1;
///     Ok(())
/// }
/// # }
/// ```
#[proc_macro_attribute]
pub fn on_game_event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a method as a string table update handler.
///
/// This handler is called when string tables are updated. String tables contain
/// game data like player names, modifiers, effects, etc.
/// Optionally filter to specific table names.
///
/// # Parameters
///
/// The handler can receive:
/// - `ctx: &Context` (optional) - Current replay state
/// - `table: &StringTable` (optional) - The updated string table
/// - `modified: &[i32]` (optional) - Indices of rows that were modified
///
/// Parameters can appear in any order. Return `ObserverResult`.
///
/// # Filtering
///
/// Pass a table name to only handle that table:
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_string_table("userinfo")]
/// fn on_userinfo(&mut self, table: &StringTable, modified: &[i32]) -> ObserverResult {
///     // Only called when userinfo table updates
///     Ok(())
/// }
/// # }
/// ```
///
/// # Examples
///
/// ## Track all string table updates
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_string_table]
/// fn on_table_update(&mut self, ctx: &Context, table: &StringTable, modified: &[i32]) -> ObserverResult {
///     println!("[{}] Table {} updated: {} rows", ctx.tick(), table.name(), modified.len());
///     Ok(())
/// }
/// # }
/// ```
///
/// ## Monitor specific table
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_string_table("ActiveModifiers")]
/// fn on_modifiers(&mut self, table: &StringTable, modified: &[i32]) -> ObserverResult {
///     println!("Active modifiers changed: {} rows", modified.len());
///     Ok(())
/// }
/// # }
/// ```
#[proc_macro_attribute]
pub fn on_string_table(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a method as a replay stop handler.
///
/// This handler is called when the replay ends (CDemoStop message).
/// Use it to finalize results, output statistics, or clean up resources.
///
/// # Parameters
///
/// The handler can receive:
/// - `ctx: &Context` (optional) - Final replay state
///
/// Return `ObserverResult`.
///
/// # Examples
///
/// ## Output final statistics
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs {
/// #     ticks: u32,
/// # }
/// # impl MyObs {
/// #[on_stop]
/// fn on_stop(&mut self, ctx: &Context) -> ObserverResult {
///     println!("Replay ended at tick {}", ctx.tick());
///     println!("Total ticks processed: {}", self.ticks);
///     Ok(())
/// }
/// # }
/// ```
///
/// ## Without context
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # struct MyObs;
/// # impl MyObs {
/// #[on_stop]
/// fn on_stop(&mut self) -> ObserverResult {
///     println!("Replay complete");
///     Ok(())
/// }
/// # }
/// ```
#[proc_macro_attribute]
pub fn on_stop(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks a method as a Dota 2 or Deadlock combat log handler.
///
/// Use `&CombatLogEntry` for Dota 2 or `&CitadelCombatLogEntry` for Deadlock.
/// Both handler types can coexist when both game features are enabled.
///
/// This handler is called whenever a combat log entry is generated.
/// Combat log entries include damage, healing, kills, abilities, items, etc.
///
/// # Parameters
///
/// The handler can receive:
/// - `ctx: &Context` (optional) - Current replay state
/// - `cle: &CombatLogEntry` (optional) - The combat log entry
///
/// Parameters can appear in any order. Return `ObserverResult`.
///
/// # Requires Feature
///
/// Only available when the `dota` feature is enabled.
///
/// # Examples
///
/// ## Track damage in real-time
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # use source2_demo::proto::DotaCombatlogTypes;
/// # struct MyObs {
/// #     total_damage: u32,
/// # }
/// # impl MyObs {
/// #[on_combat_log]
/// fn on_damage(&mut self, ctx: &Context, cle: &CombatLogEntry) -> ObserverResult {
///     if cle.r#type() == DotaCombatlogTypes::DotaCombatlogDamage {
///         if let Ok(damage) = cle.value() {
///             self.total_damage += damage;
///         }
///     }
///     Ok(())
/// }
/// # }
/// ```
///
/// ## Track kills
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// # use source2_demo::proto::DotaCombatlogTypes;
/// # struct MyObs {
/// #     kills: u32,
/// # }
/// # impl MyObs {
/// #[on_combat_log]
/// fn on_kill(&mut self, cle: &CombatLogEntry) -> ObserverResult {
///     if cle.r#type() == DotaCombatlogTypes::DotaCombatlogDeath {
///         self.kills += 1;
///     }
///     Ok(())
/// }
/// # }
/// ```
#[cfg(any(feature = "dota", feature = "citadel"))]
#[proc_macro_attribute]
pub fn on_combat_log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks the impl block or individual method to enable all tracking.
///
/// This is equivalent to `#[observer(all)]`.
///
/// # Examples
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// #[observer]
/// #[uses_all]
/// impl MyObs {
/// # fn dummy(&mut self) -> ObserverResult { Ok(()) }
/// }
/// ```
#[proc_macro_attribute]
pub fn uses_all(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks the impl block to enable entity tracking.
///
/// When applied to an impl block or individual method, automatically enables
/// the `ENTITY_STATE` interest flag so entities are tracked during parsing.
///
/// # Examples
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// #[observer]
/// #[uses_entities]
/// impl MyObs {
///     // Now you can use #[on_entity] handlers
/// # fn dummy(&mut self) -> ObserverResult { Ok(()) }
/// }
/// ```
#[proc_macro_attribute]
pub fn uses_entities(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks the impl block to enable string table tracking.
///
/// When applied to an impl block or individual method, automatically enables
/// the `STRING_TABLE_STATE` interest flag so string tables are tracked during parsing.
///
/// # Examples
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// #[observer]
/// #[uses_string_tables]
/// impl MyObs {
///     // Now you can use #[on_string_table] handlers
/// # fn dummy(&mut self) -> ObserverResult { Ok(()) }
/// }
/// ```
#[proc_macro_attribute]
pub fn uses_string_tables(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks the impl block to enable game event tracking.
///
/// When applied to an impl block or individual method, automatically enables
/// the `BASE_GAME_EVENT` interest flag so game events are tracked during parsing.
///
/// # Examples
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// #[observer]
/// #[uses_game_events]
/// impl MyObs {
///     // Now you can use #[on_game_event] handlers
/// # fn dummy(&mut self) -> ObserverResult { Ok(()) }
/// }
/// ```
#[proc_macro_attribute]
pub fn uses_game_events(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Marks the impl block to enable combat log tracking for enabled games.
///
/// When applied to an impl block or individual method, automatically enables
/// the `COMBAT_LOG_ENTRIES` and `STRING_TABLE_STATE` interest flags for combat
/// log parsing.
///
/// # Requires Feature
///
/// Available when the `dota` or `citadel` macro feature is enabled.
///
/// # Examples
///
/// ```no_run
/// # use source2_demo::prelude::*;
/// #[observer]
/// #[uses_combat_log]
/// impl MyObs {
///     // Now you can use #[on_combat_log] handlers
/// # fn dummy(&mut self) -> ObserverResult { Ok(()) }
/// }
/// ```
#[cfg(any(feature = "dota", feature = "citadel"))]
#[proc_macro_attribute]
pub fn uses_combat_log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
