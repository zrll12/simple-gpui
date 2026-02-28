use quote::{format_ident, quote};

fn is_runtime_context_param(ident: &proc_macro2::Ident) -> bool {
    let name = ident.to_string();
    name == "cx" || name == "window"
}

fn is_observe_property(ident: &proc_macro2::Ident) -> bool {
    ident.to_string().starts_with("_ob_")
}

// Generates a new methods, all fields without initializers will be required as parameters.
pub fn generate_new_method(
    properties: &[(proc_macro2::Ident, syn::Type, Option<syn::Expr>)],
    subscribes: &[(proc_macro2::Ident, syn::Expr)],
) -> proc_macro2::TokenStream {
    let mut no_initiated_fields = vec![];
    let mut initiated_fields = vec![];

    let mut field_inits: Vec<proc_macro2::TokenStream> = properties
        .iter()
        .filter_map(|(ident, ty, init)| {
            if is_runtime_context_param(ident) {
                return None;
            }
            match init {
                Some(expr) => {
                    initiated_fields.push((ident, ty, expr));
                }
                None => {
                    no_initiated_fields.push((ident, ty));
                }
            }
            Some(quote! { #ident })
        })
        .collect();
    if subscribes.len() > 0 {
        field_inits.push(quote! { _subscriptions })
    }

    let mut func_params: Vec<proc_macro2::TokenStream> = no_initiated_fields
        .iter()
        .map(|(ident, ty)| {
            quote! { #ident: #ty }
        })
        .collect();
    let context_params = properties
        .iter()
        .filter(|(ident, _ty, _init)| is_runtime_context_param(ident))
        .map(|(ident, ty, _init)| {
            quote! { #ident: #ty }
        })
        .collect::<Vec<_>>();
    func_params.extend(context_params);

    let var_inits: Vec<proc_macro2::TokenStream> = initiated_fields
        .iter()
        .map(|(ident, ty, init)| {
            quote! { let #ident: #ty = #init; }
        })
        .collect();

    let subscribe_inits: Vec<proc_macro2::TokenStream> = subscribes
        .iter()
        .map(|(ident, expr)| {
            quote! {
                {
                    let __subscribe_target = #ident.clone();
                    let #ident = __subscribe_target.clone();
                    cx.subscribe_in(&__subscribe_target, window, #expr)
                }
            }
        })
        .collect();

    let subscriptions_init = if !subscribes.is_empty() {
        quote! {
            let _subscriptions: Vec<Subscription> = vec![
                #(#subscribe_inits),*
            ];
        }
    } else {
        quote! {}
    };

    quote! {
        pub fn new(#(#func_params),*) -> Self {
            #(#var_inits)*
            #subscriptions_init
            Self {
                #(#field_inits),*
            }
        }
    }
}

// Generates setter methods for each property.
pub fn generate_set_method(
    properties: &[(proc_macro2::Ident, syn::Type, Option<syn::Expr>)],
) -> proc_macro2::TokenStream {
    let functions = properties
        .iter()
        .filter_map(|(ident, ty, _init)| {
            if is_runtime_context_param(ident) {
                return None;
            }
            if is_observe_property(ident) {
                return None;
            }
            let method_name = format_ident!("{}", ident);
            Some(quote! {
                pub fn #method_name(mut self, value: #ty) -> Self {
                    self.#ident = value;
                    self
                }
            })
        })
        .collect::<Vec<_>>();
    quote! {
        #(#functions)*
    }
}
