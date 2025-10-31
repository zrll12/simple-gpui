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

/// Extracts a subscribe like:
///   subscribe!(state_ident, impl FnMut(&mut T, &Entity<Emitter>, &Evt, &mut Window, &mut Context<T>) + 'static);
/// returns (ident, closure_expr)
pub fn extract_subscribe(stmt: &Stmt) -> Option<(Ident, Expr)> {
    if let Stmt::Macro(mac_stmt) = stmt {
        let mac = &mac_stmt.mac;
        if mac.path.is_ident("subscribe") {
            use syn::Token;
            use syn::parse::{Parse, ParseStream, Result};

            struct Subscribe {
                ident: Ident,
                _comma: Token![,],
                closure: Expr,
            }

            impl Parse for Subscribe {
                fn parse(input: ParseStream) -> Result<Self> {
                    let ident: Ident = input.parse()?;
                    let _comma: Token![,] = input.parse()?;
                    let closure: Expr = input.parse()?;
                    Ok(Subscribe {
                        ident,
                        _comma,
                        closure,
                    })
                }
            }

            let tokens = mac.tokens.clone();
            if let Ok(sub) = syn::parse2::<Subscribe>(tokens) {
                return Some((sub.ident, sub.closure));
            }
        }
    }
    None
}

/// Extracts a statement like:
///   with_context_in_init!();
/// returns true if found
pub fn extract_with_context(stmt: &Stmt) -> bool {
    if let Stmt::Macro(mac_stmt) = stmt {
        let mac = &mac_stmt.mac;
        if mac.path.is_ident("init_with_context") {
            return true;
        }
    }
    false
}
