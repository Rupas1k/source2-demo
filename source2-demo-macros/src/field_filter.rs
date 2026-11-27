use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, Lit, Meta, Token};

fn path_is(path: &syn::Path, name: &str) -> bool {
    path.segments.last().is_some_and(|segment| segment.ident == name)
}

pub(crate) fn parse_field_filters(attr: &Attribute) -> syn::Result<(Option<Expr>, Option<Expr>)> {
    let mut class = None;
    let mut field = None;
    if matches!(attr.meta, Meta::Path(_)) {
        return Ok((class, field));
    }

    attr.parse_nested_meta(|meta| {
        let selected = if path_is(&meta.path, "class") {
            &mut class
        } else if path_is(&meta.path, "field") {
            &mut field
        } else {
            return Err(meta.error("expected `class = ...` or `field = ...`"));
        };
        if selected.is_some() {
            return Err(meta.error("duplicate filter"));
        }
        *selected = Some(meta.value()?.parse()?);
        Ok(())
    })?;
    Ok((class, field))
}

pub(crate) fn string_predicate(target: &TokenStream, expr: &Expr) -> syn::Result<TokenStream> {
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
