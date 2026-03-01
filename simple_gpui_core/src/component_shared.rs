use crate::extractors::{
    extract_component_property, extract_observe, extract_subscribe, extract_subscribe_in,
    extract_with_context,
};
use crate::upsert_property;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Ident, ItemFn, Stmt, parse_quote};

pub(crate) struct ParsedComponentBody {
    pub(crate) properties: Vec<(Ident, syn::Type, Option<Expr>)>,
    pub(crate) new_stmts: Vec<Stmt>,
    pub(crate) subscriptions: Vec<Expr>,
}

#[derive(Clone, Copy)]
pub(crate) enum ParseMode {
    Stateful,
    Stateless,
}

pub(crate) fn collect_component_body(
    func: &ItemFn,
    mode: ParseMode,
) -> Result<ParsedComponentBody, TokenStream> {
    let mut properties: Vec<(Ident, syn::Type, Option<Expr>)> = Vec::new();
    let mut new_stmts: Vec<Stmt> = Vec::new();
    let mut subscriptions: Vec<Expr> = Vec::new();

    for stmt in &func.block.stmts {
        match extract_component_property(stmt) {
            Ok(Some((ident, ty, init_expr))) => {
                if let Err(err) = upsert_property(&mut properties, ident, ty, init_expr) {
                    return Err(err);
                }
                continue;
            }
            Err(err) => {
                return Err(err.into_compile_error().into());
            }
            Ok(None) => {}
        }

        if let Some((state_ty, callback)) = extract_observe(stmt) {
            if matches!(mode, ParseMode::Stateless) {
                return Err(syn::Error::new_spanned(
                    stmt,
                    "observe! is not supported in #[component_stateless]. Please migrate this component to #[component] and then use observe!.",
                )
                .into_compile_error()
                .into());
            }

            let init_expr: Expr = parse_quote!(
                cx.observe_global_in::<#state_ty>(window, #callback)
            );
            subscriptions.push(init_expr);
            continue;
        }

        if let Some((ident, expr)) = extract_subscribe(stmt) {
            if matches!(mode, ParseMode::Stateless) {
                return Err(syn::Error::new_spanned(
                    stmt,
                    "subscribe! is not supported in #[component_stateless]. Please migrate this component to #[component] and then use subscribe!.",
                )
                .into_compile_error()
                .into());
            }

            let subscription_expr: Expr = parse_quote! {
                {
                    let __subscribe_target = #ident.clone();
                    let #ident = __subscribe_target.clone();
                    cx.subscribe(&__subscribe_target, #expr)
                }
            };
            subscriptions.push(subscription_expr);
            continue;
        }

        if let Some((ident, expr)) = extract_subscribe_in(stmt) {
            if matches!(mode, ParseMode::Stateless) {
                return Err(syn::Error::new_spanned(
                    stmt,
                    "subscribe_in! is not supported in #[component_stateless]. Please migrate this component to #[component] and then use subscribe_in!.",
                )
                .into_compile_error()
                .into());
            }

            let subscription_expr: Expr = parse_quote! {
                {
                    let __subscribe_target = #ident.clone();
                    let #ident = __subscribe_target.clone();
                    cx.subscribe_in(&__subscribe_target, window, #expr)
                }
            };
            subscriptions.push(subscription_expr);
            continue;
        }

        if extract_with_context(stmt) {
            if matches!(mode, ParseMode::Stateless) {
                return Err(syn::Error::new_spanned(
                    stmt,
                    "init_with_context! is not supported in #[component_stateless], use #[component] instead",
                )
                .into_compile_error()
                .into());
            }

            for (ident, ty) in [
                (format_ident!("cx"), parse_quote!(&mut Context<Self>)),
                (format_ident!("window"), parse_quote!(&mut Window)),
            ] {
                if let Err(err) = upsert_property(&mut properties, ident, ty, None) {
                    return Err(err);
                }
            }
            continue;
        }

        new_stmts.push(stmt.clone());
    }

    Ok(ParsedComponentBody {
        properties,
        new_stmts,
        subscriptions,
    })
}

pub(crate) fn build_field_defs(
    properties: &[(Ident, syn::Type, Option<Expr>)],
    include_runtime_params: bool,
) -> Vec<proc_macro2::TokenStream> {
    properties
        .iter()
        .filter_map(|(ident, ty, _init)| {
            let is_runtime_param = ident == "cx" || ident == "window";
            if !include_runtime_params && is_runtime_param {
                None
            } else {
                Some(quote! {
                    #ident: #ty
                })
            }
        })
        .collect()
}
