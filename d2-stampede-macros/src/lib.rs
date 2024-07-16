mod protobuf_map;

use crate::protobuf_map::get_enum_from_struct;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemImpl, Type};

#[proc_macro_attribute]
pub fn observer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = &input.self_ty;

    let mut on_dota_user_message_body = quote!();
    let mut on_base_user_message_body = quote!();
    let mut on_svc_message_body = quote!();
    let mut on_net_message_body = quote!();
    let mut on_base_game_event_body = quote!();
    let mut on_demo_command_body = quote!();
    let mut on_tick_start_body = quote!();
    let mut on_tick_end_body = quote!();
    let mut on_entity_body = quote!();
    let mut on_combat_log_body = quote!();

    for item in &input.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            for attr in &method.attrs {
                if attr.path().is_ident("on_message") {
                    if let Some(arg_type) = get_message_type(method) {
                        let enum_type = get_enum_from_struct(&arg_type.to_token_stream().to_string());
                        match enum_type.to_token_stream().to_string().split("::").collect::<Vec<_>>()[0].trim() {
                            "EDotaUserMessages" => {
                                on_dota_user_message_body = quote! {
                                    #on_dota_user_message_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            self.#method_name(ctx, message)?;
                                        }
                                    }
                                };
                            }
                            "EBaseUserMessages" => {
                                on_base_user_message_body = quote! {
                                    #on_base_user_message_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            self.#method_name(ctx, message)?;
                                        }
                                    }
                                };
                            }
                            "SvcMessages" => {
                                on_svc_message_body = quote! {
                                    #on_svc_message_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            self.#method_name(ctx, message)?;
                                        }
                                    }
                                };
                            }
                            "EBaseGameEvents" => {
                                on_base_game_event_body = quote! {
                                    #on_base_game_event_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            self.#method_name(ctx, message)?;
                                        }
                                    }
                                };
                            }
                            "NetMessages" => {
                                on_net_message_body = quote! {
                                    #on_net_message_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            self.#method_name(ctx, message)?;
                                        }
                                    }
                                };
                            }
                            "EDemoCommands" => {
                                on_demo_command_body = quote! {
                                    #on_demo_command_body
                                    if msg_type == #enum_type {
                                        if let Ok(message) = #arg_type::decode(msg) {
                                            self.#method_name(ctx, message)?;
                                        }
                                    }
                                };
                            }
                            x => unreachable!("{}", x),
                        }
                    }
                }

                if attr.path().is_ident("on_tick_start") {
                    on_tick_start_body = quote! {
                        #on_tick_start_body
                        self.#method_name(ctx)?;
                    };
                }

                if attr.path().is_ident("on_tick_end") {
                    on_tick_end_body = quote! {
                        #on_tick_end_body
                        self.#method_name(ctx)?;
                    };
                }


                if attr.path().is_ident("on_entity") {
                    on_entity_body = quote! {
                        #on_entity_body
                        self.#method_name(ctx, event, entity)?;
                    };
                }

                if attr.path().is_ident("on_combat_log") {
                    on_combat_log_body = quote! {
                        #on_combat_log_body
                        self.#method_name(ctx, cle)?;
                    };
                }
            }
        }
    }

    let expanded = quote! {
        impl Observer for #struct_name {
            fn on_dota_user_message(
                &mut self,
                ctx: &Context,
                msg_type: EDotaUserMessages,
                msg: &[u8],
            ) -> ObserverResult {
                #on_dota_user_message_body
                Ok(())
            }

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
            
            fn on_combat_log(
                &mut self,
                ctx: &Context,
                cle: &CombatLogEntry,
            ) -> ObserverResult {
                #on_combat_log_body
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
pub fn on_combat_log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

fn get_message_type(method: &syn::ImplItemFn) -> Option<&Type> {
    method.sig.inputs.iter().nth(2).and_then(|arg| if let syn::FnArg::Typed(pat_type) = arg { Some(&*pat_type.ty) } else { None })
}
