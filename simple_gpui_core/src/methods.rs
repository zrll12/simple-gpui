use quote::{format_ident, quote};

// Generates an new methods, all fields without initializers will be required as parameters.
pub fn generate_new_method(
    properties: &Vec<(proc_macro2::Ident, syn::Type, Option<syn::Expr>)>,
) -> proc_macro2::TokenStream {
    let mut no_initiated_fields = vec![];

    let field_inits: Vec<proc_macro2::TokenStream> = properties
        .iter()
        .map(|(ident, ty, init)| match init {
            Some(expr) => quote! { #ident: #expr },
            None => {
                no_initiated_fields.push((ident, ty));
                quote! { #ident }
            }
        })
        .collect();

    let func_params: Vec<proc_macro2::TokenStream> = no_initiated_fields
        .iter()
        .map(|(ident, ty)| {
            quote! { #ident: #ty }
        })
        .collect();

    quote! {
        pub fn new(#(#func_params),*) -> Self {
            Self {
                #(#field_inits),*
            }
        }
    }
}

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