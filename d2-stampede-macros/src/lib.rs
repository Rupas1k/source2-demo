extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemImpl, ItemStruct, ItemFn, Type};


#[proc_macro_attribute]
pub fn observer(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = &input.ident;

    let fields = if let syn::Fields::Named(fields) = &input.fields {
        fields.named.iter().map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;
            quote! {
                #field_name: #field_type,
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };

    let fields_default = if let syn::Fields::Named(fields) = &input.fields {
        fields.named.iter().map(|field| {
            let field_name = &field.ident;
            quote! {
                #field_name: Default::default(),
            }
        }).collect::<Vec<_>>()
    } else {
        vec![]
    };

    let expanded = quote! {
        struct #struct_name {
            #(#fields),*
            dota_handlers: std::collections::HashMap<EDotaUserMessages, Vec<Box<dyn Fn(&mut Self, &Context, &[u8]) -> ObserverResult>>>,
            base_handlers: std::collections::HashMap<EBaseUserMessages, Vec<Box<dyn Fn(&mut Self, &Context, &[u8]) -> ObserverResult>>>,
        }

        impl Default for #struct_name {
            fn default() -> Self {
                let mut instance = Self {
                    dota_handlers: std::collections::HashMap::new(),
                    base_handlers: std::collections::HashMap::new(),
                    #(#fields_default),*
                };
                instance.register_handlers();
                instance
            }
        }

        impl Observer for #struct_name {
            fn on_dota_user_message(
                &mut self,
                ctx: &Context,
                msg_type: EDotaUserMessages,
                msg: &[u8],
            ) -> ObserverResult {
                self.handle_dota_user_message(ctx, msg_type, msg)
            }

            // fn on_base_user_message(
            //     &mut self,
            //     ctx: &Context,
            //     msg_type: EBaseUserMessages,
            //     msg: &[u8],
            // ) -> ObserverResult {
            //     self.handle_base_user_message(ctx, msg_type, msg)
            // }
        }
    };

    TokenStream::from(expanded)
}

#[inline]
fn get_enum_from_struct(struct_name: &str) -> proc_macro2::TokenStream {
    match struct_name {
        "CDotaUserMsgChatMessage" => quote! { EDotaUserMessages::DotaUmChatMessage },
        // "CDotaUserMsgCombatLogData" => "EDotaUserMessages::DotaUmCombatLogData",
        // "CDotaUserMsgCombatLogData2" => "EDotaUserMessages::DotaUmCombatLogData2",
        // "CDotaUserMsgCreateLinearProjectile" => "EDotaUserMessages::DotaUmCreateLinearProjectile",
        // "CDotaUserMsgDestroyLinearProjectile" => "EDotaUserMessages::DotaUmDestroyLinearProjectile",
        // "CDotaUserMsgGlobalLightColor" => "EDotaUserMessages::DotaUmGlobalLightColor",
        // "CDotaUserMsgGlobalLightDirection" => "EDotaUserMessages::DotaUmGlobalLightDirection",
        // "CDotaUserMsgHeroEffect" => "EDotaUserMessages::DotaUmHeroEffect",
        // "CDotaUserMsgModifyUnitMotionController" => "EDotaUserMessages::DotaUmModifyUnitMotionController",
        // "CDotaUserMsgParticleManager" => "EDotaUserMessages::DotaUmParticleManager",
        // "CDotaUserMsgPing" => "EDotaUserMessages::DotaUmPing",
        // "CDotaUserMsgRemoveLinearProjectile" => "EDotaUserMessages::DotaUmRemoveLinearProjectile",
        // "CDotaUserMsgSelection" => "EDotaUserMessages::DotaUmSelection",
        // "CDotaUserMsgSpectatorPlayerClick" => "EDotaUserMessages::DotaUmSpectatorPlayerClick",
        // "CDotaUserMsgTutorialTipInfo" => "EDotaUserMessages::DotaUmTutorialTipInfo",
        // "CDotaUserMsgUpdateHealth" => "EDotaUserMessages::DotaUmUpdateHealth",
        // "CDotaUserMsgUpdateMana" => "EDotaUserMessages::DotaUmUpdateMana",
        // "CDotaUserMsgUpdateMultipleEntities" => "EDotaUserMessages::DotaUmUpdateMultipleEntities",
        // "CDotaUserMsgUpdateMultipleEntities2" => "EDotaUserMessages::DotaUmUpdateMultipleEntities2",
        // "CDotaUserMsgUpdateMultipleEntities3" => "EDotaUserMessages::DotaUmUpdateMultipleEntities3",
        // "CDotaUserMsgWorldLine" => "EDotaUserMessages::DotaUmWorldLine",
        _ => panic!("Unknown message type: {}", struct_name)
    }
}

#[proc_macro_attribute]
pub fn observermethods(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = &input.self_ty;
    let mut register_handlers_body = quote!();

    for item in &input.items {
        if let syn::ImplItem::Fn(method) = item {
            for attr in &method.attrs {
                if attr.path().is_ident("on_dota_user_message") {
                    if let Some(arg_type) = get_message_type(method) {
                        let enum_type = get_enum_from_struct(&arg_type.to_token_stream().to_string());
                        let method_name = &method.sig.ident;
                        register_handlers_body = quote! {
                            #register_handlers_body
                            self.register_dota_handler(#enum_type, |slf: &mut Self, ctx, msg| -> ObserverResult {
                                if let Ok(message) = #arg_type::decode(msg) {
                                    slf.#method_name(ctx, message)?;
                                }
                                Ok(())
                            });
                        };
                    }
                }
                // else if attr.path().is_ident("on_base_user_message") {
                //     if let Some(arg_type) = get_message_type(method) {
                //         let method_name = &method.sig.ident;
                //         register_handlers_body = quote! {
                //             #register_handlers_body
                //             self.register_base_handler(#arg_type::default(), |slf: &Self, ctx, msg| {
                //                 if let Ok(message) = #arg_type::decode(msg) {
                //                     slf.#method_name(ctx, message);
                //                 }
                //             });
                //         };
                //     }
                // }
            }
        }
    }

    let expanded = quote! {
        impl #struct_name {
            fn register_dota_handler<F>(&mut self, msg_type: EDotaUserMessages, handler: F)
            where
                F: Fn(&mut Self, &Context, &[u8]) -> ObserverResult + 'static,
            {
                let handlers = self.dota_handlers.entry(msg_type).or_insert_with(Vec::new);
                handlers.push(Box::new(handler));
            }

            fn register_base_handler<F>(&mut self, msg_type: EBaseUserMessages, handler: F)
            where
                F: Fn(&mut Self, &Context, &[u8]) -> ObserverResult + 'static,
            {
                let handlers = self.base_handlers.entry(msg_type).or_insert_with(Vec::new);
                handlers.push(Box::new(handler));
            }

            #[inline]
            fn handle_dota_user_message(
                &mut self,
                ctx: &Context,
                msg_type: EDotaUserMessages,
                msg: &[u8],
            ) -> ObserverResult {
                let self_ptr = self as *mut Self;

                if let Some(v) = self.dota_handlers.get(&msg_type) {
                    for h in v.iter() {
                        unsafe { h(self_ptr.as_mut().unwrap(), ctx, msg).unwrap() }
                    }
                }

                Ok(())
            }

            fn handle_base_user_message(
                &mut self,
                ctx: &Context,
                msg_type: EBaseUserMessages,
                msg: &[u8],
            ) -> ObserverResult {
                // let x = self.base_handlers.get(&msg_type);
                // if let Some(handler) = x {
                //     handler(self, ctx, msg)?;
                // }
                Ok(())
            }

            fn register_handlers(&mut self) {
                #register_handlers_body
            }
        }

        #input
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn on_dota_user_message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::from(parse_macro_input!(item as ItemFn).to_token_stream())
}
#[proc_macro_attribute]
pub fn on_base_user_message(_attr: TokenStream, item: TokenStream) -> TokenStream {
    TokenStream::from(parse_macro_input!(item as ItemFn).to_token_stream())
}

fn get_message_type(method: &syn::ImplItemFn) -> Option<&Type> {
    method.sig.inputs.iter().nth(2).and_then(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            Some(&*pat_type.ty)
        } else {
            None
        }
    })
}