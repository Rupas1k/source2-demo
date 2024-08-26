mod protobuf_map;

use crate::protobuf_map::get_enum_from_struct;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, FnArg, ItemImpl, Type};

#[proc_macro_attribute]
pub fn observer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = &input.self_ty;

    #[cfg(feature = "dota")]
    let mut on_combat_log_body = quote!();
    #[cfg(feature = "dota")]
    let mut on_dota_user_message_body = quote!();

    #[cfg(feature = "citadel")]
    let mut on_citadel_user_message_body = quote!();
    #[cfg(feature = "citadel")]
    let mut on_citadel_game_event_body = quote!();

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

    for item in &input.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            for attr in &method.attrs {
                if attr.path().is_ident("on_message") {
                    check_second_arg_is_context(method);
                    if let Some((arg_type, is_reference)) = get_message_type(method) {
                        let enum_type = get_enum_from_struct(&arg_type.to_token_stream().to_string());
                        let call_message = if is_reference {
                            quote! { self.#method_name(ctx, &message)?; }
                        } else {
                            quote! { self.#method_name(ctx, message)?; }
                        };
                        match enum_type.to_token_stream().to_string().split("::").collect::<Vec<_>>()[0].trim() {
                            #[cfg(feature = "dota")]
                            "EDotaUserMessages" => {
                                on_dota_user_message_body = quote! {
                                    #on_dota_user_message_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            #call_message
                                        }
                                    }
                                };
                            }
                            #[cfg(feature = "citadel")]
                            "CitadelUserMessageIds" => {
                                on_citadel_user_message_body = quote! {
                                    #on_citadel_user_message_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            #call_message
                                        }
                                    }
                                };
                            }
                            #[cfg(feature = "citadel")]
                            "ECitadelGameEvents" => {
                                on_citadel_game_event_body = quote! {
                                    #on_citadel_game_event_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            #call_message
                                        }
                                    }
                                };
                            }
                            "EBaseUserMessages" => {
                                on_base_user_message_body = quote! {
                                    #on_base_user_message_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            #call_message
                                        }
                                    }
                                };
                            }
                            "SvcMessages" => {
                                on_svc_message_body = quote! {
                                    #on_svc_message_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            #call_message
                                        }
                                    }
                                };
                            }
                            "EBaseGameEvents" => {
                                on_base_game_event_body = quote! {
                                    #on_base_game_event_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            #call_message
                                        }
                                    }
                                };
                            }
                            "NetMessages" => {
                                on_net_message_body = quote! {
                                    #on_net_message_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            #call_message
                                        }
                                    }
                                };
                            }
                            "EDemoCommands" => {
                                on_demo_command_body = quote! {
                                    #on_demo_command_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            #call_message
                                        }
                                    }
                                };
                            }
                            x => unreachable!("{}", x),
                        }
                    } else {
                        panic!("Message handler must have a message argument")
                    }
                }

                if attr.path().is_ident("on_tick_start") {
                    check_second_arg_is_context(method);
                    on_tick_start_body = quote! {
                        #on_tick_start_body
                        self.#method_name(ctx)?;
                    };
                }

                if attr.path().is_ident("on_tick_end") {
                    check_second_arg_is_context(method);
                    on_tick_end_body = quote! {
                        #on_tick_end_body
                        self.#method_name(ctx)?;
                    };
                }

                if attr.path().is_ident("on_entity") {
                    check_second_arg_is_context(method);
                    on_entity_body = quote! {
                        #on_entity_body
                        self.#method_name(ctx, event, entity)?;
                    };
                }

                if attr.path().is_ident("on_game_event") {
                    check_second_arg_is_context(method);

                    on_game_event_body = if let Some(event_name) = attr.parse_args::<syn::LitStr>().ok() {
                        quote! {
                            #on_game_event_body
                            if ge.name() == #event_name {
                                self.#method_name(ctx, ge)?;
                            }
                        }
                    } else {
                        quote! {
                            #on_game_event_body
                            self.#method_name(ctx, ge)?;
                        }
                    }
                }

                if attr.path().is_ident("on_string_table") {
                    check_second_arg_is_context(method);

                    on_string_table_body = if let Some(table_name) = attr.parse_args::<syn::LitStr>().ok() {
                        quote! {
                            #on_string_table_body
                            if table.name() == #table_name {
                                self.#method_name(ctx, table, modified)?;
                            }
                        }
                    } else {
                        quote! {
                            #on_string_table_body
                            self.#method_name(ctx, table, modified)?;
                        }
                    };
                }

                #[cfg(feature = "dota")]
                if attr.path().is_ident("on_combat_log") {
                    check_second_arg_is_context(method);
                    on_combat_log_body = quote! {
                        #on_combat_log_body
                        self.#method_name(ctx, cle)?;
                    };
                }
            }
        }
    }

    let specific_methods = {
        #[allow(unused)]
        let mut methods = quote! {};

        #[cfg(feature = "dota")]
        {
            methods.extend(quote! {
                fn on_dota_user_message(
                    &mut self,
                    ctx: &Context,
                    msg_type: EDotaUserMessages,
                    msg: &[u8],
                ) -> ObserverResult {
                    #on_dota_user_message_body
                    Ok(())
                }

                fn on_combat_log(
                    &mut self,
                    ctx: &Context,
                    cle: &CombatLogEntry,
                ) -> ObserverResult {
                    #on_combat_log_body
                    Ok(())
                }
            });
        }

        #[cfg(feature = "citadel")]
        {
            methods.extend(quote! {
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
        }

        methods
    };

    let expanded = quote! {
        impl Observer for #struct_name {
            #specific_methods

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
        }

        #input
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn on_message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn on_tick_start(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn on_tick_end(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn on_entity(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn on_game_event(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn on_string_table(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[cfg(feature = "dota")]
#[proc_macro_attribute]
pub fn on_combat_log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

fn get_message_type(method: &syn::ImplItemFn) -> Option<(Type, bool)> {
    method.sig.inputs.iter().nth(2).and_then(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let Type::Reference(x) = pat_type.ty.as_ref() {
                if x.mutability.is_some() {
                    panic!("Mutable reference not supported")
                }
                Some((*x.elem.clone(), true))
            } else {
                Some((*pat_type.ty.clone(), false))
            }
        } else {
            None
        }
    })
}

fn check_second_arg_is_context(method: &syn::ImplItemFn) {
    if let Some(FnArg::Typed(pat_type)) = method.sig.inputs.iter().nth(1) {
        if let Type::Reference(type_reference) = pat_type.ty.as_ref() {
            let elem_type = type_reference.elem.as_ref();
            if let Type::Path(type_path) = elem_type {
                if let Some(segment) = type_path.path.segments.first() {
                    if segment.ident == "Context" && type_reference.mutability.is_none() {
                        return;
                    }
                }
            }
        }
    }
    panic!("The second argument must be of type &Context");
}
