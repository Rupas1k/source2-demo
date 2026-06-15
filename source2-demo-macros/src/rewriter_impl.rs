use crate::protobuf_map::get_enum_from_struct;
use crate::type_utils::{
    is_context_type, is_demo_command_type, is_entity_events_type, is_entity_type, is_field_value_ref_type, is_field_value_type,
    is_packet_messages_type, is_raw_field_value_type, is_rewrite_field_value_type, is_string_table_entry_type, stringify_type,
};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::{Expr, FnArg, Ident, ItemImpl, Lit, Token, Type};

fn path_is(path: &syn::Path, name: &str) -> bool {
    path.segments.last().is_some_and(|segment| segment.ident == name)
}

pub(crate) fn expand_rewriter(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_name = &input.self_ty;

    let mut interests = quote!(::source2_demo::writer::RewriteInterests::empty());
    macro_rules! add_flag {
        ($flag:ident) => {
            interests = quote!(#interests | ::source2_demo::writer::RewriteInterests::$flag);
        };
    }

    let mut rewrite_demo_message_body = quote!();
    let mut rewrite_packet_message_body = quote!();
    let mut rewrite_packet_messages_body = quote!();
    let mut rewrite_demo_string_tables_body = quote!();
    let mut rewrite_string_table_entry_body = quote!();
    let mut rewrite_svc_create_string_table_body = quote!();
    let mut rewrite_svc_update_string_table_body = quote!();
    let mut replace_entity_field_body = quote!();
    let mut should_rewrite_entity_body = quote!();
    let mut should_track_entity_body = quote!();

    for item in &input.items {
        let syn::ImplItem::Fn(method) = item else {
            continue;
        };

        let method_name = method.sig.ident.clone();
        for attr in &method.attrs {
            let Some(segment) = attr.path().segments.last() else {
                continue;
            };

            match segment.ident.to_string().as_str() {
                "rewrite_demo_message" => {
                    add_flag!(DEMO_MESSAGE);
                    if packet_message_method_is_raw(method) {
                        let args = writer_callback_method_args(method, quote! {});
                        rewrite_demo_message_body.extend(quote! {
                            match self.#method_name(#args)? {
                                ::source2_demo::writer::MessageRewrite::Keep => {}
                                rewrite => return Ok(rewrite),
                            }
                        });
                    } else {
                        match packet_message_method_type(method) {
                            Ok((arg_type, is_ref, is_mut_ref)) => {
                                if let Err(error) = extend_rewrite_demo_message_body(
                                    &mut rewrite_demo_message_body,
                                    method,
                                    &method_name,
                                    &arg_type,
                                    is_ref,
                                    is_mut_ref,
                                ) {
                                    rewrite_demo_message_body.extend(error.to_compile_error());
                                }
                            }
                            Err(error) => {
                                rewrite_demo_message_body.extend(error.to_compile_error());
                            }
                        }
                    }
                }
                "rewrite_packet_message" => {
                    add_flag!(PACKET_MESSAGE);
                    if packet_message_method_is_raw(method) {
                        let args = writer_callback_method_args(method, quote! {});
                        rewrite_packet_message_body.extend(quote! {
                            match self.#method_name(#args)? {
                                ::source2_demo::writer::MessageRewrite::Keep => {}
                                rewrite => return Ok(rewrite),
                            }
                        });
                    } else {
                        match packet_message_method_type(method) {
                            Ok((arg_type, is_ref, is_mut_ref)) => {
                                if let Err(error) = extend_rewrite_packet_message_body(
                                    &mut rewrite_packet_message_body,
                                    method,
                                    &method_name,
                                    &arg_type,
                                    is_ref,
                                    is_mut_ref,
                                ) {
                                    rewrite_packet_message_body.extend(error.to_compile_error());
                                }
                            }
                            Err(error) => {
                                rewrite_packet_message_body.extend(error.to_compile_error());
                            }
                        }
                    }
                }
                "rewrite_packet_messages" => {
                    add_flag!(PACKET_MESSAGES);
                    let args = writer_callback_method_args(method, quote! {});
                    rewrite_packet_messages_body.extend(quote! {
                        self.#method_name(#args)?;
                    });
                }
                "rewrite_demo_string_tables" => {
                    add_flag!(DEMO_STRING_TABLES);
                    let args = writer_callback_method_args(method, quote! { message });
                    rewrite_demo_string_tables_body.extend(quote! {
                        match self.#method_name(#args)? {
                            ::source2_demo::writer::MessageRewrite::Keep => {}
                            rewrite => return Ok(rewrite),
                        }
                    });
                }
                "rewrite_string_table_entry" => {
                    add_flag!(STRING_TABLE_ENTRIES);
                    let args = writer_callback_method_args(method, quote! {});
                    rewrite_string_table_entry_body.extend(quote! {
                        self.#method_name(#args)?;
                    });
                }
                "rewrite_svc_create_string_table" => {
                    add_flag!(SVC_CREATE_STRING_TABLE);
                    let args = writer_callback_method_args(method, quote! { message });
                    rewrite_svc_create_string_table_body.extend(quote! {
                        match self.#method_name(#args)? {
                            ::source2_demo::writer::MessageRewrite::Keep => {}
                            rewrite => return Ok(rewrite),
                        }
                    });
                }
                "rewrite_svc_update_string_table" => {
                    add_flag!(SVC_UPDATE_STRING_TABLE);
                    let args = writer_callback_method_args(method, quote! { message });
                    rewrite_svc_update_string_table_body.extend(quote! {
                        match self.#method_name(#args)? {
                            ::source2_demo::writer::MessageRewrite::Keep => {}
                            rewrite => return Ok(rewrite),
                        }
                    });
                }
                "replace_entity_field" => {
                    add_flag!(ENTITY_FIELDS);
                    let args = replace_entity_field_method_args(method);
                    replace_entity_field_body.extend(quote! {
                        if let Some(value) = self.#method_name(#args) {
                            return Some(value);
                        }
                    });
                }
                "rewrite_field" => {
                    add_flag!(ENTITY_FIELDS);
                    match rewrite_field_body(attr, method) {
                        Ok(tokens) => replace_entity_field_body.extend(tokens),
                        Err(error) => replace_entity_field_body.extend(error.to_compile_error()),
                    }
                }
                "should_rewrite_entity" => {
                    add_flag!(ENTITY_FIELDS);
                    let args = should_rewrite_entity_method_args(method);
                    should_rewrite_entity_body.extend(quote! {
                        if !self.#method_name(#args) {
                            return false;
                        }
                    });
                }
                "should_track_entity" => {
                    add_flag!(ENTITY_FIELDS);
                    let args = should_entity_method_args(method, "should_track_entity");
                    should_track_entity_body.extend(quote! {
                        if !self.#method_name(#args) {
                            return false;
                        }
                    });
                }
                _ => {}
            }
        }
    }

    let ret = quote! {
        impl ::source2_demo::writer::DemoRewriter for #struct_name {
            fn interests(&self) -> ::source2_demo::writer::RewriteInterests {
                #interests
            }

            fn rewrite_demo_message(
                &mut self,
                ctx: &::source2_demo::Context,
                tick: u32,
                msg_type: ::source2_demo::proto::EDemoCommands,
                payload: &[u8],
            ) -> Result<::source2_demo::writer::MessageRewrite, ::source2_demo::error::ParserError> {
                #rewrite_demo_message_body
                Ok(::source2_demo::writer::MessageRewrite::Keep)
            }

            fn rewrite_packet_message(
                &mut self,
                ctx: &::source2_demo::Context,
                tick: u32,
                msg_type: i32,
                payload: &[u8],
            ) -> Result<::source2_demo::writer::MessageRewrite, ::source2_demo::error::ParserError> {
                #rewrite_packet_message_body
                Ok(::source2_demo::writer::MessageRewrite::Keep)
            }

            fn rewrite_packet_messages(
                &mut self,
                ctx: &::source2_demo::Context,
                tick: u32,
                messages: &mut Vec<::source2_demo::writer::PacketMessage>,
            ) -> Result<(), ::source2_demo::error::ParserError> {
                #rewrite_packet_messages_body
                Ok(())
            }

            fn rewrite_demo_string_tables(
                &mut self,
                ctx: &::source2_demo::Context,
                tick: u32,
                message: &mut ::source2_demo::proto::CDemoStringTables,
            ) -> Result<::source2_demo::writer::MessageRewrite, ::source2_demo::error::ParserError> {
                #rewrite_demo_string_tables_body
                Ok(::source2_demo::writer::MessageRewrite::Keep)
            }

            fn rewrite_string_table_entry(
                &mut self,
                ctx: &::source2_demo::Context,
                tick: u32,
                table_name: &str,
                entry: &mut ::source2_demo::writer::StringTableEntryUpdate,
            ) -> Result<(), ::source2_demo::error::ParserError> {
                #rewrite_string_table_entry_body
                Ok(())
            }

            fn rewrite_svc_create_string_table(
                &mut self,
                ctx: &::source2_demo::Context,
                tick: u32,
                message: &mut ::source2_demo::proto::CSvcMsgCreateStringTable,
            ) -> Result<::source2_demo::writer::MessageRewrite, ::source2_demo::error::ParserError> {
                #rewrite_svc_create_string_table_body
                Ok(::source2_demo::writer::MessageRewrite::Keep)
            }

            fn rewrite_svc_update_string_table(
                &mut self,
                ctx: &::source2_demo::Context,
                tick: u32,
                message: &mut ::source2_demo::proto::CSvcMsgUpdateStringTable,
            ) -> Result<::source2_demo::writer::MessageRewrite, ::source2_demo::error::ParserError> {
                #rewrite_svc_update_string_table_body
                Ok(::source2_demo::writer::MessageRewrite::Keep)
            }

            fn replace_entity_field(
                &mut self,
                ctx: &::source2_demo::Context,
                event: ::source2_demo::EntityEvents,
                entity: &::source2_demo::Entity,
                field_name: &str,
                value: &::source2_demo::FieldValue,
            ) -> Option<::source2_demo::FieldValue> {
                #replace_entity_field_body
                None
            }

            fn should_rewrite_entity(
                &mut self,
                ctx: &::source2_demo::Context,
                event: ::source2_demo::EntityEvents,
                entity: &::source2_demo::Entity,
            ) -> bool {
                #should_rewrite_entity_body
                true
            }

            fn should_track_entity(
                &mut self,
                ctx: &::source2_demo::Context,
                event: ::source2_demo::EntityEvents,
                entity: &::source2_demo::Entity,
            ) -> bool {
                #should_track_entity_body
                true
            }

        }
        #input
    };

    TokenStream::from(ret)
}

fn rewrite_field_body(attr: &syn::Attribute, method: &syn::ImplItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let method_name = &method.sig.ident;
    let mut class = None;
    let mut field = None;

    attr.parse_nested_meta(|meta| {
        if path_is(&meta.path, "class") {
            let value = meta.value()?;
            class = Some(value.parse::<Expr>()?);
            Ok(())
        } else if path_is(&meta.path, "field") {
            let value = meta.value()?;
            field = Some(value.parse::<Expr>()?);
            Ok(())
        } else {
            Err(meta.error("expected `class = ...` or `field = ...`"))
        }
    })?;

    let class = class.ok_or_else(|| syn::Error::new_spanned(attr, "missing `class = ...`"))?;
    let class_predicate = string_predicate(&quote! { entity.class().name() }, &class)?;
    let class_only = field.is_none();
    let field_predicate = match field {
        Some(field) => string_predicate(&quote! { field_name }, &field)?,
        None => quote! { true },
    };
    if class_only {
        ensure_rewrite_field_raw_value(method)?;
    }
    let value_predicate = rewrite_field_value_predicate(method)?;
    let args = rewrite_field_method_args(method)?;

    Ok(quote! {
        if (#class_predicate) && (#field_predicate) && (#value_predicate) {
            let replacement = self.#method_name(#args);
            if let Some(value) = ::source2_demo::FieldRewriteResult::into_field_rewrite_result(replacement) {
                return Some(value);
            }
        }
    })
}

fn string_predicate(target: &proc_macro2::TokenStream, expr: &Expr) -> syn::Result<proc_macro2::TokenStream> {
    match expr {
        Expr::Lit(expr_lit) => {
            let Lit::Str(value) = &expr_lit.lit else {
                return Err(syn::Error::new_spanned(expr, "expected string literal"));
            };
            Ok(quote! { #target == #value })
        }
        Expr::Call(call) => {
            let Expr::Path(path) = call.func.as_ref() else {
                return Err(syn::Error::new_spanned(&call.func, "expected predicate name"));
            };
            let Some(ident) = path.path.get_ident() else {
                return Err(syn::Error::new_spanned(&path.path, "expected predicate name"));
            };
            let name = ident.to_string();
            match name.as_str() {
                "exact" => {
                    let value = single_string_arg(expr, &call.args)?;
                    Ok(quote! { #target == #value })
                }
                "starts_with" => {
                    let value = single_string_arg(expr, &call.args)?;
                    Ok(quote! { #target.starts_with(#value) })
                }
                "ends_with" => {
                    let value = single_string_arg(expr, &call.args)?;
                    Ok(quote! { #target.ends_with(#value) })
                }
                "contains" => {
                    let value = single_string_arg(expr, &call.args)?;
                    Ok(quote! { #target.contains(#value) })
                }
                "any" => {
                    if call.args.is_empty() {
                        return Err(syn::Error::new_spanned(expr, "`any` needs at least one argument"));
                    }
                    let predicates = call
                        .args
                        .iter()
                        .map(|arg| string_predicate(target, arg))
                        .collect::<syn::Result<Vec<_>>>()?;
                    Ok(quote! { false #( || (#predicates) )* })
                }
                "all" => {
                    if call.args.is_empty() {
                        return Err(syn::Error::new_spanned(expr, "`all` needs at least one argument"));
                    }
                    let predicates = call
                        .args
                        .iter()
                        .map(|arg| string_predicate(target, arg))
                        .collect::<syn::Result<Vec<_>>>()?;
                    Ok(quote! { true #( && (#predicates) )* })
                }
                "not" => {
                    if call.args.len() != 1 {
                        return Err(syn::Error::new_spanned(expr, "`not` needs exactly one argument"));
                    }
                    let Some(arg) = call.args.first() else {
                        return Err(syn::Error::new_spanned(expr, "`not` needs exactly one argument"));
                    };
                    let predicate = string_predicate(target, arg)?;
                    Ok(quote! { !(#predicate) })
                }
                _ => Err(syn::Error::new_spanned(
                    ident,
                    "unknown predicate; expected exact, starts_with, ends_with, contains, any, all, or not",
                )),
            }
        }
        _ => Err(syn::Error::new_spanned(expr, "expected string literal or predicate call")),
    }
}

fn single_string_arg<'a>(expr: &Expr, args: &'a Punctuated<Expr, Token![,]>) -> syn::Result<&'a syn::LitStr> {
    if args.len() != 1 {
        return Err(syn::Error::new_spanned(expr, "predicate needs exactly one argument"));
    }
    let Some(arg) = args.first() else {
        return Err(syn::Error::new_spanned(expr, "predicate needs exactly one argument"));
    };
    let Expr::Lit(expr_lit) = arg else {
        return Err(syn::Error::new_spanned(expr, "predicate argument must be a string literal"));
    };
    let Lit::Str(value) = &expr_lit.lit else {
        return Err(syn::Error::new_spanned(expr, "predicate argument must be a string literal"));
    };
    Ok(value)
}

fn rewrite_field_method_args(method: &syn::ImplItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let value_arg_index = rewrite_field_value_arg_index(method)?;
    let mut args = Vec::new();

    for (index, input) in method.sig.inputs.iter().enumerate().skip(1) {
        let FnArg::Typed(pat_type) = input else {
            continue;
        };
        if index == value_arg_index {
            args.push(rewrite_field_value_arg(pat_type.ty.as_ref())?);
            continue;
        }

        let type_string = stringify_type(pat_type.ty.as_ref());
        let arg = if is_entity_events_type(&type_string) {
            quote! { event }
        } else if is_context_type(&type_string) {
            quote! { ctx }
        } else if is_entity_type(&type_string) {
            quote! { entity }
        } else if type_string == "& str" {
            quote! { field_name }
        } else if is_field_value_ref_type(&type_string) {
            quote! { value }
        } else {
            return Err(syn::Error::new_spanned(pat_type, "unsupported #[rewrite_field] argument"));
        };
        args.push(arg);
    }

    Ok(quote! { #(#args),* })
}

fn replace_entity_field_method_args(method: &syn::ImplItemFn) -> proc_macro2::TokenStream {
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
        } else if is_entity_type(&type_string) {
            quote! { entity }
        } else if type_string == "& str" {
            quote! { field_name }
        } else if is_field_value_ref_type(&type_string) {
            quote! { value }
        } else {
            quote! { compile_error!("unsupported #[replace_entity_field] argument") }
        };
        args.push(arg);
    }

    quote! { #(#args),* }
}

fn should_rewrite_entity_method_args(method: &syn::ImplItemFn) -> proc_macro2::TokenStream {
    should_entity_method_args(method, "should_rewrite_entity")
}

fn should_entity_method_args(method: &syn::ImplItemFn, attribute: &str) -> proc_macro2::TokenStream {
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
        } else if is_entity_type(&type_string) {
            quote! { entity }
        } else {
            let message = format!("unsupported #[{attribute}] argument");
            quote! { compile_error!(#message) }
        };
        args.push(arg);
    }

    quote! { #(#args),* }
}

fn rewrite_field_value_arg_index(method: &syn::ImplItemFn) -> syn::Result<usize> {
    for (index, input) in method.sig.inputs.iter().enumerate().skip(1).rev() {
        let FnArg::Typed(pat_type) = input else {
            continue;
        };
        let type_string = stringify_type(pat_type.ty.as_ref());
        if is_rewrite_field_value_type(&type_string) {
            return Ok(index);
        }
    }

    Err(syn::Error::new_spanned(&method.sig, "#[rewrite_field] needs a value argument"))
}

fn ensure_rewrite_field_raw_value(method: &syn::ImplItemFn) -> syn::Result<()> {
    let value_arg_index = rewrite_field_value_arg_index(method)?;
    let Some(FnArg::Typed(pat_type)) = method.sig.inputs.iter().nth(value_arg_index) else {
        return Ok(());
    };
    let type_string = stringify_type(pat_type.ty.as_ref());
    if is_raw_field_value_type(&type_string) {
        Ok(())
    } else {
        Err(syn::Error::new_spanned(
            pat_type,
            "class-only #[rewrite_field] handlers must use FieldValue",
        ))
    }
}

fn rewrite_field_value_arg(ty: &Type) -> syn::Result<proc_macro2::TokenStream> {
    let type_string = stringify_type(ty);
    match type_string.as_str() {
        "& str" => Ok(quote! { <&::source2_demo::FieldValue as ::std::convert::TryInto<String>>::try_into(value).ok()?.as_str() }),
        "String" => Ok(quote! { <&::source2_demo::FieldValue as ::std::convert::TryInto<String>>::try_into(value).ok()? }),
        "& String" => Ok(quote! { &<&::source2_demo::FieldValue as ::std::convert::TryInto<String>>::try_into(value).ok()? }),
        value if is_field_value_ref_type(value) => Ok(quote! { value }),
        value if is_field_value_type(value) => Ok(quote! { value.clone() }),
        _ => Ok(quote! { <&::source2_demo::FieldValue as ::std::convert::TryInto<#ty>>::try_into(value).ok()? }),
    }
}

fn rewrite_field_value_predicate(method: &syn::ImplItemFn) -> syn::Result<proc_macro2::TokenStream> {
    let value_arg_index = rewrite_field_value_arg_index(method)?;
    let Some(FnArg::Typed(pat_type)) = method.sig.inputs.iter().nth(value_arg_index) else {
        return Ok(quote! { true });
    };
    let ty = pat_type.ty.as_ref();
    let type_string = stringify_type(ty);
    match type_string.as_str() {
        value if is_raw_field_value_type(value) => Ok(quote! { true }),
        "& str" | "String" | "& String" => Ok(quote! {
            <&::source2_demo::FieldValue as ::std::convert::TryInto<String>>::try_into(value).is_ok()
        }),
        _ => Ok(quote! {
            <&::source2_demo::FieldValue as ::std::convert::TryInto<#ty>>::try_into(value).is_ok()
        }),
    }
}

fn writer_callback_method_args(method: &syn::ImplItemFn, message_arg: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut args = Vec::new();

    for input in method.sig.inputs.iter().skip(1) {
        let FnArg::Typed(pat_type) = input else {
            continue;
        };
        let type_string = stringify_type(pat_type.ty.as_ref());
        let arg = writer_callback_arg(&type_string, message_arg.clone());
        args.push(arg);
    }

    quote! { #(#args),* }
}

fn writer_callback_arg(type_string: &str, message_arg: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    if is_context_type(type_string) {
        quote! { ctx }
    } else if type_string == "u32" {
        quote! { tick }
    } else if type_string == "i32" || is_demo_command_type(type_string) {
        quote! { msg_type }
    } else if type_string == "& [u8]" {
        quote! { payload }
    } else if type_string == "& str" {
        quote! { table_name }
    } else if is_packet_messages_type(type_string) {
        quote! { messages }
    } else if is_string_table_entry_type(type_string) {
        quote! { entry }
    } else {
        message_arg
    }
}

fn packet_message_method_is_raw(method: &syn::ImplItemFn) -> bool {
    packet_message_first_payload_arg(method).is_some_and(|type_string| type_string == "i32" || type_string == "& [u8]")
}

fn packet_message_method_type(method: &syn::ImplItemFn) -> syn::Result<(Type, bool, bool)> {
    for input in method.sig.inputs.iter().skip(1) {
        let FnArg::Typed(pat_type) = input else {
            continue;
        };
        let type_string = stringify_type(pat_type.ty.as_ref());
        if is_context_type(&type_string) || type_string == "u32" {
            continue;
        }
        if let Type::Reference(reference) = pat_type.ty.as_ref() {
            return Ok((*reference.elem.clone(), true, reference.mutability.is_some()));
        }
        return Ok((*pat_type.ty.clone(), false, false));
    }
    Err(syn::Error::new_spanned(&method.sig, "#[rewrite_packet_message] needs a message argument"))
}

fn packet_message_first_payload_arg(method: &syn::ImplItemFn) -> Option<String> {
    method.sig.inputs.iter().skip(1).find_map(|input| {
        let FnArg::Typed(pat_type) = input else {
            return None;
        };
        let type_string = stringify_type(pat_type.ty.as_ref());
        if is_context_type(&type_string) || type_string == "u32" {
            None
        } else {
            Some(type_string)
        }
    })
}

fn extend_rewrite_packet_message_body(
    body: &mut proc_macro2::TokenStream,
    method: &syn::ImplItemFn,
    method_name: &Ident,
    arg_type: &Type,
    is_ref: bool,
    is_mut_ref: bool,
) -> syn::Result<()> {
    let enum_type = get_enum_from_struct(arg_type.to_token_stream().to_string().as_str())
        .map_err(|error| syn::Error::new_spanned(arg_type, error.to_string()))?;
    let message_arg = if is_ref {
        if is_mut_ref {
            quote! { &mut message }
        } else {
            quote! { &message }
        }
    } else {
        quote! { message }
    };
    let method_args = writer_callback_method_args(method, message_arg);

    if is_ref {
        body.extend(quote! {
            if msg_type == #enum_type as i32 {
                let mut message = <#arg_type as ::source2_demo::proto::Message>::decode(payload)?;
                match self.#method_name(#method_args)? {
                    ::source2_demo::writer::MessageRewrite::Keep => {}
                    ::source2_demo::writer::MessageRewrite::Rewrite => {
                        return Ok(::source2_demo::writer::MessageRewrite::Replace(
                            ::source2_demo::proto::Message::encode_to_vec(&message),
                        ));
                    }
                    rewrite => return Ok(rewrite),
                }
            }
        });
    } else {
        body.extend(quote! {
            if msg_type == #enum_type as i32 {
                let message = <#arg_type as ::source2_demo::proto::Message>::decode(payload)?;
                match self.#method_name(#method_args)? {
                    ::source2_demo::writer::MessageRewrite::Keep
                    | ::source2_demo::writer::MessageRewrite::Rewrite => {}
                    rewrite => return Ok(rewrite),
                }
            }
        });
    }
    Ok(())
}

fn extend_rewrite_demo_message_body(
    body: &mut proc_macro2::TokenStream,
    method: &syn::ImplItemFn,
    method_name: &Ident,
    arg_type: &Type,
    is_ref: bool,
    is_mut_ref: bool,
) -> syn::Result<()> {
    let enum_type = get_enum_from_struct(arg_type.to_token_stream().to_string().as_str())
        .map_err(|error| syn::Error::new_spanned(arg_type, error.to_string()))?;
    let message_arg = if is_ref {
        if is_mut_ref {
            quote! { &mut message }
        } else {
            quote! { &message }
        }
    } else {
        quote! { message }
    };
    let method_args = writer_callback_method_args(method, message_arg);

    if is_ref {
        body.extend(quote! {
            if msg_type == ::source2_demo::proto::#enum_type {
                let mut message = <#arg_type as ::source2_demo::proto::Message>::decode(payload)?;
                match self.#method_name(#method_args)? {
                    ::source2_demo::writer::MessageRewrite::Keep => {}
                    ::source2_demo::writer::MessageRewrite::Rewrite => {
                        return Ok(::source2_demo::writer::MessageRewrite::Replace(
                            ::source2_demo::proto::Message::encode_to_vec(&message),
                        ));
                    }
                    rewrite => return Ok(rewrite),
                }
            }
        });
    } else {
        body.extend(quote! {
            if msg_type == ::source2_demo::proto::#enum_type {
                let message = <#arg_type as ::source2_demo::proto::Message>::decode(payload)?;
                match self.#method_name(#method_args)? {
                    ::source2_demo::writer::MessageRewrite::Keep
                    | ::source2_demo::writer::MessageRewrite::Rewrite => {}
                    rewrite => return Ok(rewrite),
                }
            }
        });
    }
    Ok(())
}
