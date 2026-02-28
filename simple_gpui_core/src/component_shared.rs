use crate::extractors::{
    extract_component_property, extract_observe, extract_subscribe, extract_with_context,
};
use crate::upsert_property;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Ident, ItemFn, Stmt, parse_quote};

pub(crate) struct ParsedComponentBody {
    pub(crate) properties: Vec<(Ident, syn::Type, Option<Expr>)>,
    pub(crate) new_stmts: Vec<Stmt>,
    pub(crate) subscribes: Vec<(Ident, Expr)>,
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
    let mut subscribes: Vec<(Ident, Expr)> = Vec::new();
    let mut observe_index: usize = 0;

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

            observe_index += 1;
            let ident = format_ident!("_ob_{}", observe_index);
            let ty: syn::Type = parse_quote!(Subscription);
            let init_expr: Expr = parse_quote!(
                cx.observe_global_in::<#state_ty>(window, #callback)
            );
            if let Err(err) = upsert_property(&mut properties, ident, ty, Some(init_expr)) {
                return Err(err);
            }
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

            subscribes.push((ident.clone(), expr.clone()));
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
        subscribes,
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
