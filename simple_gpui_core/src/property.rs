use proc_macro2::Ident;
use syn::{Expr, Stmt};

/// Extracts a statement like:
///   component_property!(name: Type = expr);
///   component_property!(name: Type);
/// returns (ident, ty, expr)
pub fn extract_component_property(stmt: &Stmt) -> Option<(Ident, syn::Type, Option<Expr>)> {
    if let Stmt::Macro(mac_stmt) = stmt {
        let mac = &mac_stmt.mac;
        if mac.path.is_ident("component_property") {
            use syn::parse::{Parse, ParseStream, Result};

            struct Prop {
                name: Ident,
                colon_token: syn::token::Colon,
                ty: syn::Type,
                eq_token: Option<syn::token::Eq>,
                init: Option<Expr>,
            }

            impl Parse for Prop {
                fn parse(input: ParseStream) -> Result<Self> {
                    let name: Ident = input.parse()?;
                    let colon_token: syn::token::Colon = input.parse()?;
                    let ty: syn::Type = input.parse()?;
                    let eq_token: Option<syn::token::Eq> = input.parse().ok();
                    let init: Option<Expr> = input.parse().ok();
                    Ok(Prop {
                        name,
                        colon_token,
                        ty,
                        eq_token,
                        init,
                    })
                }
            }

            let tokens = mac.tokens.clone();
            if let Ok(prop) = syn::parse2::<Prop>(tokens) {
                return Some((prop.name, prop.ty, prop.init));
            }
        }
    }
    None
}
