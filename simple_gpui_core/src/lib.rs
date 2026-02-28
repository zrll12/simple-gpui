mod extractors;
mod methods;

use crate::extractors::{
    extract_component_property, extract_observe, extract_subscribe, extract_with_context,
};
use case::CaseExt;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Ident, ItemFn, Stmt, parse_macro_input, parse_quote};

fn upsert_property(
    properties: &mut Vec<(Ident, syn::Type, Option<Expr>)>,
    ident: Ident,
    ty: syn::Type,
    init: Option<Expr>,
) -> Result<(), TokenStream> {
    if properties.iter().any(|(name, _, _)| name == &ident) {
        Err(
            syn::Error::new(ident.span(), format!("Property {} already exists", ident))
                .into_compile_error()
                .into(),
        )
    } else {
        properties.push((ident, ty, init));
        Ok(())
    }
}

/// attribute proc macro
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse the input function
    let func = parse_macro_input!(item as ItemFn);
    let fn_name = func.sig.ident.clone();
    let struct_name = format_ident!("{}", fn_name.to_string().to_camel());

    // Collect properties and build new statements for the body
    let mut properties: Vec<(Ident, syn::Type, Option<Expr>)> = Vec::new();
    let mut new_stmts: Vec<Stmt> = Vec::new();
    let mut subscribes: Vec<(Ident, Expr)> = Vec::new();
    let mut observe_index: usize = 0;

    for stmt in &func.block.stmts {
        if let Some((ident, ty, init_expr)) = extract_component_property(stmt) {
            if let Err(err) = upsert_property(&mut properties, ident, ty, init_expr) {
                return err;
            }
        } else if let Some((state_ty, callback)) = extract_observe(stmt) {
            observe_index += 1;
            let ident = format_ident!("_ob_{}", observe_index);
            let ty: syn::Type = parse_quote!(Subscription);
            let init_expr: Expr = parse_quote!(
                cx.observe_global_in::<#state_ty>(window, #callback)
            );
            if let Err(err) = upsert_property(&mut properties, ident, ty, Some(init_expr)) {
                return err;
            }
        } else if let Some((ident, expr)) = extract_subscribe(stmt) {
            subscribes.push((ident.clone(), expr.clone()));
        } else if extract_with_context(stmt) {
            for (ident, ty) in [
                (format_ident!("cx"), parse_quote!(&mut Context<Self>)),
                (format_ident!("window"), parse_quote!(&mut Window)),
            ] {
                if let Err(err) = upsert_property(&mut properties, ident, ty, None) {
                    return err;
                }
            }
        } else {
            new_stmts.push(stmt.clone());
        }
    }

    // Build tokens for struct properties in fields
    let mut field_defs: Vec<proc_macro2::TokenStream> = properties
        .iter()
        .filter_map(|(ident, ty, _init)| {
            let is_runtime_param = ident == "cx" || ident == "window";
            if is_runtime_param {
                None
            } else {
                Some(quote! {
                    #ident: #ty
                })
            }
        })
        .collect();

    if subscribes.len() > 0 {
        field_defs.push(quote! { _subscriptions: Vec<Subscription> })
    }

    // Generate methods
    let function_new = methods::generate_new_method(&properties, &subscribes);
    let function_setters = methods::generate_set_method(&properties);

    let inputs = &func.sig.inputs;
    let output = &func.sig.output;
    let body = quote!({
        #(#new_stmts)*
    });

    let output_tokens = quote! {
        pub struct #struct_name {
            #(#field_defs),*
        }

        impl #struct_name {
            #function_new
            #function_setters
        }

        impl Render for #struct_name {
            fn render(&mut self, #inputs) #output {
                #body
            }
        }
    };

    output_tokens.into()
}
