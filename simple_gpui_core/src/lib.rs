mod extractors;
mod methods;

use crate::extractors::{extract_component_property, extract_subscribe, extract_with_context};
use case::CaseExt;
use proc_macro::TokenStream;
use std::collections::HashMap;
use quote::{format_ident, quote};
use syn::{Expr, Ident, ItemFn, Stmt, parse_macro_input, parse_quote};

/// attribute proc macro
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse the input function
    let func = parse_macro_input!(item as ItemFn);
    let fn_name = func.sig.ident.clone();
    let struct_name = format_ident!("{}", fn_name.to_string().to_camel());

    // Collect properties and build new statements for the body
    let mut properties: HashMap<Ident, (syn::Type, Option<Expr>)> = HashMap::new();
    let mut new_stmts: Vec<Stmt> = Vec::new();
    let mut subscribes: Vec<(Ident, Expr)> = Vec::new();

    for stmt in &func.block.stmts {
        if let Some((ident, ty, init_expr)) = extract_component_property(stmt) {
            properties.insert(ident.clone(), (ty.clone(), init_expr.clone()));
        } else if let Some((ident, expr)) = extract_subscribe(stmt) {
            subscribes.push((ident.clone(), expr.clone()));
        } else if extract_with_context(stmt) {
            properties.insert(format_ident!("cx"), (parse_quote!(&mut Context<Self>), None));
            properties.insert(format_ident!("window"), (parse_quote!(&mut Window), None));
        } else {
            new_stmts.push(stmt.clone());
        }
    }

    // Build tokens for struct properties in fields
    let mut field_defs: Vec<proc_macro2::TokenStream> = properties
        .iter()
        .filter_map(|(ident, (ty, _init))| {
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
