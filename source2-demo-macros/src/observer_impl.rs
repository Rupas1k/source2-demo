use crate::protobuf_map::get_enum_from_struct;
#[cfg(any(feature = "dota", feature = "citadel"))]
use crate::type_utils::is_combat_log_type;
use crate::type_utils::{
    canonical_message_type, is_context_type, is_entity_events_ref_type, is_entity_events_type, is_entity_type, is_game_event_type, is_message_type,
    is_message_type_ref, is_modified_indices_type, is_payload_type, is_string_table_type, referenced_or_owned_type, stringify_type,
};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parser;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::{FnArg, Ident, ItemImpl, Token, Type};

fn attr_is(attr: &syn::Attribute, name: &str) -> bool {
    attr.path().segments.last().is_some_and(|segment| segment.ident == name)
}

pub(crate) fn expand_observer(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (mut mode_all, mut errors) = parse_observer_mode(attr);

    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = &input.self_ty;

    let mut interests = quote!(::source2_demo::Interests::empty());
    macro_rules! add_flag {
        ($flag:ident) => {
            interests = quote!(#interests | ::source2_demo::Interests::$flag);
        };
    }

    for a in &input.attrs {
        if attr_is(a, "uses_all") {
            mode_all = true;
        }
        if attr_is(a, "uses_entities") {
            add_flag!(ENTITY_STATE);
        }
        if attr_is(a, "uses_string_tables") {
            add_flag!(STRING_TABLE_STATE);
        }
        if attr_is(a, "uses_game_events") {
            add_flag!(BASE_GAME_EVENT);
        }
        #[cfg(any(feature = "dota", feature = "citadel"))]
        if attr_is(a, "uses_combat_log") {
            add_flag!(STRING_TABLE_STATE);
            #[cfg(feature = "dota")]
            add_flag!(COMBAT_LOG_ENTRIES);
            #[cfg(feature = "citadel")]
            add_flag!(CITADEL_COMBAT_LOG_ENTRIES);
        }
    }

    let mut has_demo = false;
    let mut has_net = false;
    let mut has_svc = false;
    let mut has_base_um = false;
    let mut has_base_ge = false;
    let mut has_tick_start = false;
    let mut has_tick_end = false;
    let mut has_entity = false;
    let mut has_entity_track = false;
    let mut has_string_table = false;
    let mut has_string_table_track = false;
    let mut has_stop = false;

    #[cfg(feature = "dota")]
    let mut has_dota_um = false;
    #[cfg(feature = "dota")]
    let mut has_combat_log = false;

    #[cfg(feature = "citadel")]
    let mut has_cita_um = false;
    #[cfg(feature = "citadel")]
    let mut has_citadel_combat_log = false;
    #[cfg(feature = "citadel")]
    let mut has_cita_ge = false;

    #[cfg(feature = "cs2")]
    let mut has_cs2_um = false;
    #[cfg(feature = "cs2")]
    let mut has_cs2_ge = false;

    #[cfg(feature = "dota")]
    let mut on_combat_log_body = quote!();
    #[cfg(feature = "dota")]
    let mut on_dota_user_message_body = quote!();

    #[cfg(feature = "citadel")]
    let mut on_citadel_user_message_body = quote!();
    #[cfg(feature = "citadel")]
    let mut on_citadel_combat_log_body = quote!();
    #[cfg(feature = "citadel")]
    let mut on_citadel_game_event_body = quote!();

    #[cfg(feature = "cs2")]
    let mut on_cs2_user_message_body = quote!();
    #[cfg(feature = "cs2")]
    let mut on_cs2_game_event_body = quote!();

    let mut on_svc_message_body = quote!();
    let mut on_net_message_body = quote!();
    let mut on_base_game_event_body = quote!();
    let mut on_demo_command_body = quote!();
    let mut on_base_user_message_body = quote!();
    let mut on_tick_start_body = quote!();
    let mut on_tick_end_body = quote!();
    let mut on_entity_body = quote!();
    let mut on_game_event_body = quote!();
    let mut on_string_table_body = quote!();
    let mut on_stop_body = quote!();

    for item in &input.items {
        if let syn::ImplItem::Fn(method) = item {
            for a in &method.attrs {
                if attr_is(a, "uses_all") {
                    mode_all = true;
                }
                if attr_is(a, "uses_entities") {
                    add_flag!(ENTITY_STATE);
                }
                if attr_is(a, "uses_string_tables") {
                    add_flag!(STRING_TABLE_STATE);
                }
                if attr_is(a, "uses_game_events") {
                    add_flag!(BASE_GAME_EVENT);
                }
                #[cfg(any(feature = "dota", feature = "citadel"))]
                if attr_is(a, "uses_combat_log") {
                    add_flag!(STRING_TABLE_STATE);
                    #[cfg(feature = "dota")]
                    add_flag!(COMBAT_LOG_ENTRIES);
                    #[cfg(feature = "citadel")]
                    add_flag!(CITADEL_COMBAT_LOG_ENTRIES);
                }
            }

            for attr in &method.attrs {
                let method_name = method.sig.ident.clone();

                if let Some(segment) = attr.path().segments.last() {
                    match segment.ident.to_string().as_str() {
                        "on_tick_start" => {
                            has_tick_start = true;
                            match observer_context_args(method, "#[on_tick_start]") {
                                Ok(args) => on_tick_start_body.extend(quote! {
                                    self.#method_name(#args)?;
                                }),
                                Err(error) => errors.extend(error.to_compile_error()),
                            }
                        }
                        "on_tick_end" => {
                            has_tick_end = true;
                            match observer_context_args(method, "#[on_tick_end]") {
                                Ok(args) => on_tick_end_body.extend(quote! {
                                    self.#method_name(#args)?;
                                }),
                                Err(error) => errors.extend(error.to_compile_error()),
                            }
                        }
                        "on_stop" => {
                            has_stop = true;
                            match observer_context_args(method, "#[on_stop]") {
                                Ok(args) => on_stop_body.extend(quote! {
                                    self.#method_name(#args)?;
                                }),
                                Err(error) => errors.extend(error.to_compile_error()),
                            }
                        }
                        #[cfg(any(feature = "dota", feature = "citadel"))]
                        "on_combat_log" => match observer_combat_log_args(method) {
                            Ok((citadel, args)) => {
                                has_string_table = true;
                                let call = quote! { self.#method_name(#args)?; };

                                if citadel {
                                    #[cfg(feature = "citadel")]
                                    {
                                        has_citadel_combat_log = true;
                                        on_citadel_combat_log_body.extend(call);
                                    }
                                    #[cfg(not(feature = "citadel"))]
                                    errors.extend(
                                        syn::Error::new_spanned(method, "Citadel combat logs require the deadlock feature").to_compile_error(),
                                    );
                                } else {
                                    #[cfg(feature = "dota")]
                                    {
                                        has_combat_log = true;
                                        on_combat_log_body.extend(call);
                                    }
                                    #[cfg(not(feature = "dota"))]
                                    errors.extend(syn::Error::new_spanned(method, "Dota combat logs require the dota feature").to_compile_error());
                                }
                            }
                            Err(error) => errors.extend(error.to_compile_error()),
                        },
                        "on_entity" => {
                            has_entity = true;
                            has_entity_track = true;
                            match observer_entity_args(method) {
                                Ok(args) => {
                                    on_entity_body.extend(if let Ok(entity_class) = attr.parse_args::<syn::LitStr>() {
                                        quote! {
                                            if entity.class().name() == #entity_class {
                                                self.#method_name(#args)?;
                                            }
                                        }
                                    } else {
                                        quote! {
                                            self.#method_name(#args)?;
                                        }
                                    });
                                }
                                Err(error) => errors.extend(error.to_compile_error()),
                            }
                        }
                        "on_game_event" => match observer_game_event_args(method) {
                            Ok(args) => {
                                on_game_event_body.extend(if let Ok(event_name) = attr.parse_args::<syn::LitStr>() {
                                    quote! {
                                        if ge.name() == #event_name {
                                            self.#method_name(#args)?;
                                        }
                                    }
                                } else {
                                    quote! {
                                        self.#method_name(#args)?;
                                    }
                                });
                            }
                            Err(error) => errors.extend(error.to_compile_error()),
                        },
                        "on_string_table" => {
                            has_string_table = true;
                            has_string_table_track = true;
                            match observer_string_table_args(method) {
                                Ok(args) => {
                                    on_string_table_body.extend(if let Ok(table_name) = attr.parse_args::<syn::LitStr>() {
                                        quote! {
                                            if table.name() == #table_name {
                                                self.#method_name(#args)?;
                                            }
                                        }
                                    } else {
                                        quote! {
                                            self.#method_name(#args)?;
                                        }
                                    });
                                }
                                Err(error) => errors.extend(error.to_compile_error()),
                            }
                        }
                        "on_message" => match observer_message_args(method) {
                            Ok((kind, args)) => {
                                let (root, call) = match kind {
                                    ObserverMessageKind::Decoded { arg_type, is_ref } => {
                                        let enum_type = match get_enum_from_struct(arg_type.to_token_stream().to_string().as_str()) {
                                            Ok(enum_type) => enum_type,
                                            Err(error) => {
                                                errors.extend(syn::Error::new_spanned(&arg_type, error.to_string()).to_compile_error());
                                                continue;
                                            }
                                        };
                                        let type_string = enum_type.to_token_stream().to_string();
                                        let root = type_string.split("::").collect::<Vec<_>>()[0].trim().to_string();
                                        let message_arg = if is_ref {
                                            quote! { &message }
                                        } else {
                                            quote! { message }
                                        };
                                        let args = args
                                            .into_iter()
                                            .map(|arg| match arg {
                                                ObserverArg::Context => quote! { ctx },
                                                ObserverArg::Message => message_arg.clone(),
                                                ObserverArg::MessageType => {
                                                    quote! { compile_error!("internal macro error: unexpected message type arg") }
                                                }
                                                ObserverArg::Payload => quote! { compile_error!("internal macro error: unexpected payload arg") },
                                            })
                                            .collect::<Vec<_>>();
                                        (
                                            root,
                                            quote! {
                                                if msg_type == #enum_type {
                                                    if let Ok(message) = #arg_type::decode(msg) {
                                                        self.#method_name(#(#args),*)?;
                                                    }
                                                }
                                            },
                                        )
                                    }
                                    ObserverMessageKind::Raw { enum_type, is_ref } => {
                                        let type_string = enum_type.to_token_stream().to_string();
                                        let root = canonical_message_type(&type_string).unwrap_or(type_string);
                                        let message_type_arg = if is_ref {
                                            quote! { &msg_type }
                                        } else {
                                            quote! { msg_type }
                                        };
                                        let args = args
                                            .into_iter()
                                            .map(|arg| match arg {
                                                ObserverArg::Context => quote! { ctx },
                                                ObserverArg::MessageType => message_type_arg.clone(),
                                                ObserverArg::Payload => quote! { msg },
                                                ObserverArg::Message => {
                                                    quote! { compile_error!("internal macro error: unexpected protobuf message arg") }
                                                }
                                            })
                                            .collect::<Vec<_>>();
                                        (
                                            root,
                                            quote! {
                                                self.#method_name(#(#args),*)?;
                                            },
                                        )
                                    }
                                };

                                macro_rules! extend {
                                    ($body: ident) => {
                                        $body.extend(call.clone())
                                    };
                                }

                                let known_message_root = match root.as_str() {
                                    "EDemoCommands" => {
                                        has_demo = true;
                                        true
                                    }
                                    "EBaseUserMessages" => {
                                        has_base_um = true;
                                        true
                                    }
                                    "EBaseGameEvents" => {
                                        has_base_ge = true;
                                        true
                                    }
                                    "SvcMessages" => {
                                        has_svc = true;
                                        true
                                    }
                                    "NetMessages" => {
                                        has_net = true;
                                        true
                                    }
                                    #[cfg(feature = "dota")]
                                    "EDotaUserMessages" => {
                                        has_dota_um = true;
                                        true
                                    }
                                    #[cfg(feature = "citadel")]
                                    "CitadelUserMessageIds" => {
                                        has_cita_um = true;
                                        true
                                    }
                                    #[cfg(feature = "citadel")]
                                    "ECitadelGameEvents" => {
                                        has_cita_ge = true;
                                        true
                                    }
                                    #[cfg(feature = "cs2")]
                                    "ECstrike15UserMessages" => {
                                        has_cs2_um = true;
                                        true
                                    }
                                    #[cfg(feature = "cs2")]
                                    "ECsgoGameEvents" => {
                                        has_cs2_ge = true;
                                        true
                                    }
                                    _ => false,
                                };
                                if !known_message_root {
                                    errors.extend(syn::Error::new_spanned(&method.sig, "unknown #[on_message] message type").to_compile_error());
                                    continue;
                                }

                                match root.as_str() {
                                    "EDemoCommands" => extend!(on_demo_command_body),
                                    "EBaseUserMessages" => extend!(on_base_user_message_body),
                                    "EBaseGameEvents" => extend!(on_base_game_event_body),
                                    "SvcMessages" => extend!(on_svc_message_body),
                                    "NetMessages" => extend!(on_net_message_body),

                                    #[cfg(feature = "dota")]
                                    "EDotaUserMessages" => extend!(on_dota_user_message_body),
                                    #[cfg(feature = "citadel")]
                                    "CitadelUserMessageIds" => extend!(on_citadel_user_message_body),
                                    #[cfg(feature = "citadel")]
                                    "ECitadelGameEvents" => extend!(on_citadel_game_event_body),
                                    #[cfg(feature = "cs2")]
                                    "ECstrike15UserMessages" => extend!(on_cs2_user_message_body),
                                    #[cfg(feature = "cs2")]
                                    "ECsgoGameEvents" => extend!(on_cs2_game_event_body),

                                    _ => {}
                                }
                            }
                            Err(error) => errors.extend(error.to_compile_error()),
                        },
                        _ => {}
                    }
                }
            }
        }
    }

    #[allow(unused_mut)]
    let mut obs_body = quote! {
        fn on_base_user_message(
            &mut self,
            ctx: &Context,
            msg_type: EBaseUserMessages,
            msg: &[u8],
        ) -> ObserverResult {
            #on_base_user_message_body
            Ok(())
        }

        fn on_svc_message(
            &mut self,
            ctx: &Context,
            msg_type: SvcMessages,
            msg: &[u8],
        ) -> ObserverResult {
            #on_svc_message_body
            Ok(())
        }

        fn on_net_message(
            &mut self,
            ctx: &Context,
            msg_type: NetMessages,
            msg: &[u8],
        ) -> ObserverResult {
            #on_net_message_body
            Ok(())
        }

        fn on_base_game_event(
            &mut self,
            ctx: &Context,
            msg_type: EBaseGameEvents,
            msg: &[u8],
        ) -> ObserverResult {
            #on_base_game_event_body
            Ok(())
        }

        fn on_demo_command(
            &mut self,
            ctx: &Context,
            msg_type: EDemoCommands,
            msg: &[u8],
        ) -> ObserverResult {
            #on_demo_command_body
            Ok(())
        }

        fn on_tick_start(
            &mut self,
            ctx: &Context,
        ) -> ObserverResult {
            #on_tick_start_body
            Ok(())
        }

        fn on_tick_end(
            &mut self,
            ctx: &Context,
        ) -> ObserverResult {
            #on_tick_end_body
            Ok(())
        }

        fn on_entity(
            &mut self,
            ctx: &Context,
            event: EntityEvents,
            entity: &Entity,
        ) -> ObserverResult {
            #on_entity_body
            Ok(())
        }

        fn on_game_event(
            &mut self,
            ctx: &Context,
            ge: &GameEvent
        ) -> ObserverResult {
            #on_game_event_body
            Ok(())
        }

        fn on_string_table(
            &mut self,
            ctx: &Context,
            table: &StringTable,
            modified: &[i32]
        ) -> ObserverResult {
            #on_string_table_body
            Ok(())
        }

        fn on_stop(
            &mut self,
            ctx: &Context,
        ) -> ObserverResult {
            #on_stop_body
            Ok(())
        }
    };

    #[cfg(feature = "dota")]
    obs_body.extend(quote! {
        fn on_combat_log(
            &mut self,
            ctx: &Context,
            cle: &CombatLogEntry
        ) -> ObserverResult {
            #on_combat_log_body
            Ok(())
        }

        fn on_dota_user_message(
            &mut self,
            ctx: &Context,
            msg_type: EDotaUserMessages,
            msg: &[u8],
        ) -> ObserverResult {
            #on_dota_user_message_body
            Ok(())
        }
    });

    #[cfg(feature = "citadel")]
    obs_body.extend(quote! {
        fn on_citadel_combat_log(
            &mut self,
            ctx: &Context,
            cle: &::source2_demo::CitadelCombatLogEntry,
        ) -> ObserverResult {
            #on_citadel_combat_log_body
            Ok(())
        }

        fn on_citadel_user_message(
            &mut self,
            ctx: &Context,
            msg_type: CitadelUserMessageIds,
            msg: &[u8],
        ) -> ObserverResult {
            #on_citadel_user_message_body
            Ok(())
        }

        fn on_citadel_game_event(
            &mut self,
            ctx: &Context,
            msg_type: ECitadelGameEvents,
            msg: &[u8],
        ) -> ObserverResult {
            #on_citadel_game_event_body
            Ok(())
        }
    });

    #[cfg(feature = "cs2")]
    obs_body.extend(quote! {
        fn on_cs2_user_message(
            &mut self,
            ctx: &Context,
            msg_type: ECstrike15UserMessages,
            msg: &[u8],
        ) -> ObserverResult {
            #on_cs2_user_message_body
            Ok(())
        }

        fn on_cs2_game_event(
            &mut self,
            ctx: &Context,
            msg_type: ECsgoGameEvents,
            msg: &[u8],
        ) -> ObserverResult {
            #on_cs2_game_event_body
            Ok(())
        }
    });

    macro_rules! add_if { ($cond:expr, $flag:ident) => {
        if $cond { interests = quote!(#interests | ::source2_demo::Interests::$flag); }
    }}

    if mode_all {
        interests = quote!(::source2_demo::Interests::all());
    }

    add_if!(has_demo, DEMO_MESSAGE);
    add_if!(has_net, NET_MESSAGE);
    add_if!(has_svc, SVC_MESSAGE);
    add_if!(has_base_um, BASE_USER_MESSAGE);
    add_if!(has_base_ge, BASE_GAME_EVENT);
    add_if!(has_tick_start, TICK_START);
    add_if!(has_tick_end, TICK_END);
    add_if!(has_entity, ENTITY_STATE);
    add_if!(has_entity_track, ENTITY_EVENTS);
    add_if!(has_string_table, STRING_TABLE_STATE);
    add_if!(has_string_table_track, STRING_TABLE_ENTRIES);
    add_if!(has_stop, REPLAY_END);

    #[cfg(feature = "dota")]
    add_if!(has_dota_um, DOTA_USER_MESSAGE);
    #[cfg(feature = "dota")]
    add_if!(has_combat_log, COMBAT_LOG_ENTRIES);

    #[cfg(feature = "citadel")]
    add_if!(has_cita_um, CITADEL_USER_MESSAGE);
    #[cfg(feature = "citadel")]
    add_if!(has_citadel_combat_log, CITADEL_COMBAT_LOG_ENTRIES);
    #[cfg(feature = "citadel")]
    add_if!(has_cita_ge, CITADEL_GAME_EVENT);

    #[cfg(feature = "cs2")]
    add_if!(has_cs2_um, CS2_USER_MESSAGE);
    #[cfg(feature = "cs2")]
    add_if!(has_cs2_ge, CS2_GAME_EVENT);

    let ret = quote! {
        impl Observer for #struct_name {
            fn interests(&self) -> ::source2_demo::Interests { #interests }
            #obs_body
        }
        #input
        #errors
    };

    TokenStream::from(ret)
}

fn parse_observer_mode(attr: TokenStream) -> (bool, proc_macro2::TokenStream) {
    let mut mode_all = false;
    let mut errors = quote!();

    if attr.is_empty() {
        return (mode_all, errors);
    }

    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    match parser.parse(attr) {
        Ok(idents) => {
            for id in idents {
                if id == "all" {
                    mode_all = true;
                } else {
                    errors.extend(syn::Error::new_spanned(id, "expected `all`").to_compile_error());
                }
            }
        }
        Err(error) => {
            errors.extend(error.to_compile_error());
        }
    }

    (mode_all, errors)
}

enum ObserverArg {
    Context,
    Message,
    MessageType,
    Payload,
}

enum ObserverMessageKind {
    Decoded { arg_type: Type, is_ref: bool },
    Raw { enum_type: Type, is_ref: bool },
}

fn observer_context_args(method: &syn::ImplItemFn, attr_name: &str) -> syn::Result<proc_macro2::TokenStream> {
    let mut args = Vec::new();
    for input in method.sig.inputs.iter().skip(1) {
        let FnArg::Typed(pat_type) = input else {
            continue;
        };
        let type_string = stringify_type(pat_type.ty.as_ref());
        if is_context_type(&type_string) {
            args.push(quote! { ctx });
        } else {
            return Err(syn::Error::new_spanned(pat_type, format!("{attr_name} supports only `&Context`")));
        }
    }
    Ok(quote! { #(#args),* })
}

fn observer_entity_args(method: &syn::ImplItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let mut args = Vec::new();
    for input in method.sig.inputs.iter().skip(1) {
        let FnArg::Typed(pat_type) = input else {
            continue;
        };
        let type_string = stringify_type(pat_type.ty.as_ref());
        let arg = if is_context_type(&type_string) {
            quote! { ctx }
        } else if is_entity_events_type(&type_string) {
            quote! { event }
        } else if is_entity_events_ref_type(&type_string) {
            quote! { &event }
        } else if is_entity_type(&type_string) {
            quote! { entity }
        } else {
            return Err(syn::Error::new_spanned(pat_type, "unsupported #[on_entity] argument"));
        };
        args.push(arg);
    }
    Ok(quote! { #(#args),* })
}

fn observer_game_event_args(method: &syn::ImplItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let mut args = Vec::new();
    for input in method.sig.inputs.iter().skip(1) {
        let FnArg::Typed(pat_type) = input else {
            continue;
        };
        let type_string = stringify_type(pat_type.ty.as_ref());
        let arg = if is_context_type(&type_string) {
            quote! { ctx }
        } else if is_game_event_type(&type_string) {
            quote! { ge }
        } else {
            return Err(syn::Error::new_spanned(pat_type, "unsupported #[on_game_event] argument"));
        };
        args.push(arg);
    }
    Ok(quote! { #(#args),* })
}

fn observer_string_table_args(method: &syn::ImplItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let mut args = Vec::new();
    for input in method.sig.inputs.iter().skip(1) {
        let FnArg::Typed(pat_type) = input else {
            continue;
        };
        let type_string = stringify_type(pat_type.ty.as_ref());
        let arg = if is_context_type(&type_string) {
            quote! { ctx }
        } else if is_string_table_type(&type_string) {
            quote! { table }
        } else if is_modified_indices_type(&type_string) {
            quote! { modified }
        } else {
            return Err(syn::Error::new_spanned(pat_type, "unsupported #[on_string_table] argument"));
        };
        args.push(arg);
    }
    Ok(quote! { #(#args),* })
}

#[cfg(any(feature = "dota", feature = "citadel"))]
fn observer_combat_log_args(method: &syn::ImplItemFn) -> syn::Result<(bool, proc_macro2::TokenStream)> {
    let mut args = Vec::new();
    let mut citadel = None;
    for input in method.sig.inputs.iter().skip(1) {
        let FnArg::Typed(pat_type) = input else {
            continue;
        };
        let type_string = stringify_type(pat_type.ty.as_ref());
        let arg = if is_context_type(&type_string) {
            quote! { ctx }
        } else if is_combat_log_type(&type_string)
            || matches!(
                type_string.as_str(),
                "& CitadelCombatLogEntry" | "& source2_demo :: CitadelCombatLogEntry" | "& :: source2_demo :: CitadelCombatLogEntry"
            )
        {
            let is_citadel = type_string.contains("CitadelCombatLogEntry");
            if citadel.replace(is_citadel).is_some() {
                return Err(syn::Error::new_spanned(pat_type, "expected one combat log entry argument"));
            }
            quote! { cle }
        } else {
            return Err(syn::Error::new_spanned(pat_type, "unsupported #[on_combat_log] argument"));
        };
        args.push(arg);
    }
    Ok((citadel.unwrap_or(cfg!(not(feature = "dota"))), quote! { #(#args),* }))
}

fn observer_message_args(method: &syn::ImplItemFn) -> syn::Result<(ObserverMessageKind, Vec<ObserverArg>)> {
    let mut message = None;
    let mut message_type = None;
    let mut payload = false;
    let mut args = Vec::new();

    for input in method.sig.inputs.iter().skip(1) {
        let FnArg::Typed(pat_type) = input else {
            continue;
        };
        let type_string = stringify_type(pat_type.ty.as_ref());
        if is_context_type(&type_string) {
            args.push(ObserverArg::Context);
            continue;
        }

        if is_message_type(&type_string) || is_message_type_ref(&type_string) {
            if message_type.is_some() {
                return Err(syn::Error::new_spanned(pat_type, "#[on_message] supports only one message type argument"));
            }
            let (arg_type, is_ref) = referenced_or_owned_type(pat_type.ty.as_ref());
            message_type = Some((arg_type, is_ref));
            args.push(ObserverArg::MessageType);
            continue;
        }

        if is_payload_type(&type_string) {
            if payload {
                return Err(syn::Error::new_spanned(pat_type, "#[on_message] supports only one raw payload argument"));
            }
            payload = true;
            args.push(ObserverArg::Payload);
            continue;
        }

        if message.is_some() {
            return Err(syn::Error::new_spanned(
                pat_type,
                "#[on_message] supports only `&Context` and one protobuf message argument",
            ));
        }

        let (arg_type, is_ref) = referenced_or_owned_type(pat_type.ty.as_ref());
        message = Some((arg_type, is_ref));
        args.push(ObserverArg::Message);
    }

    match (message, message_type, payload) {
        (Some((arg_type, is_ref)), None, false) => Ok((ObserverMessageKind::Decoded { arg_type, is_ref }, args)),
        (None, Some((enum_type, is_ref)), true) => Ok((ObserverMessageKind::Raw { enum_type, is_ref }, args)),
        (Some(_), Some(_), _) => Err(syn::Error::new_spanned(
            &method.sig,
            "#[on_message] cannot mix protobuf and raw message arguments",
        )),
        (Some(_), None, true) => Err(syn::Error::new_spanned(
            &method.sig,
            "#[on_message] raw payload handlers need a message type argument",
        )),
        (None, Some(_), false) => Err(syn::Error::new_spanned(
            &method.sig,
            "#[on_message] raw handlers need a `&[u8]` payload argument",
        )),
        (None, None, _) => Err(syn::Error::new_spanned(
            &method.sig,
            "#[on_message] needs a protobuf message argument or raw message type plus `&[u8]`",
        )),
    }
}
