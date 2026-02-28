mod extractors;
mod methods;
mod component;
mod component_stateless;
mod component_shared;

use proc_macro::TokenStream;
use syn::{Expr, Ident};

pub(crate) fn upsert_property(
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
    component::component_impl(item)
}

/// attribute proc macro
#[proc_macro_attribute]
pub fn component_stateless(_attr: TokenStream, item: TokenStream) -> TokenStream {
    component_stateless::component_stateless_impl(item)
}
