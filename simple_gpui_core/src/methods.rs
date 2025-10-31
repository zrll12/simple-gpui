use quote::{format_ident, quote};

// Generates a new methods, all fields without initializers will be required as parameters.
pub fn generate_new_method(
    properties: &Vec<(proc_macro2::Ident, syn::Type, Option<syn::Expr>)>,
    temp_properties: &Vec<(proc_macro2::Ident, syn::Type)>,
    subscribes: &Vec<(proc_macro2::Ident, syn::Expr)>,
    with_context: bool,
) -> proc_macro2::TokenStream {
    let mut no_initiated_fields = vec![];
    let mut initiated_fields = vec![];

    let mut field_inits: Vec<proc_macro2::TokenStream> = properties
        .iter()
        .map(|(ident, ty, init)| {
            match init {
                Some(expr) => {
                    initiated_fields.push((ident, ty, expr));
                }
                None => {
                    no_initiated_fields.push((ident, ty));
                }
            }
            quote! { #ident }
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
    let temp_params: Vec<proc_macro2::TokenStream> = temp_properties
        .iter()
        .map(|(ident, ty)| {
            quote! { #ident: #ty }
        })
        .collect();
    func_params.extend(temp_params);
    if subscribes.len() > 0 && with_context {
        func_params.push(quote! { cx: &mut Context<Self> });
        func_params.push(quote! { window: &mut Window });
    }

    let var_inits: Vec<proc_macro2::TokenStream> = initiated_fields
        .iter()
        .map(|(ident, ty, init)| {
            quote! { let #ident: #ty = #init; }
        })
        .collect();

    let subscribe_inits: Vec<proc_macro2::TokenStream> = subscribes
        .iter()
        .map(|(ident, expr)| {
            quote! { cx.subscribe_in(&#ident, window, #expr) }
        })
        .collect();

    let subscriptions_init = if subscribes.len() > 0 {
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
    properties: &Vec<(proc_macro2::Ident, syn::Type, Option<syn::Expr>)>,
) -> proc_macro2::TokenStream {
    let functions = properties
        .iter()
        .map(|(ident, ty, _init)| {
            let method_name = format_ident!("{}", ident);
            quote! {
                pub fn #method_name(mut self, value: #ty) -> Self {
                    self.#ident = value;
                    self
                }
            }
        })
        .collect::<Vec<_>>();
    quote! {
        #(#functions)*
    }
}
