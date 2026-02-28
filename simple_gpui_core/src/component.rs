use crate::component_shared::{ParseMode, build_field_defs, collect_component_body};
use crate::methods;
use case::CaseExt;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

pub(crate) fn component_impl(item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let fn_name = func.sig.ident.clone();
    let struct_name = format_ident!("{}", fn_name.to_string().to_camel());

    let parsed = match collect_component_body(&func, ParseMode::Stateful) {
        Ok(parsed) => parsed,
        Err(err) => return err,
    };

    let mut field_defs = build_field_defs(&parsed.properties, false);

    if !parsed.subscribes.is_empty() {
        field_defs.push(quote! { _subscriptions: Vec<Subscription> })
    }

    let function_new = methods::generate_new_method(&parsed.properties, &parsed.subscribes);
    let function_setters = methods::generate_set_method(&parsed.properties);

    let inputs = &func.sig.inputs;
    let output = &func.sig.output;
    let new_stmts = &parsed.new_stmts;
    let body = quote!({
        #(#new_stmts)*
    });

    quote! {
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
    }
    .into()
}
