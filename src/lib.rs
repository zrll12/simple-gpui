pub use simple_gpui_core::*;

// This macro is not necessarily used, only for dismissing ide errors.
#[macro_export]
macro_rules! component_property {
    ($($t:tt)*) => {
        unreachable!("component_property! should only be used inside a #[component] or #[component_stateless] function, and in ident: type = expr or ident: type form");
    };
}

#[macro_export]
macro_rules! component_entity {
    ($($t:tt)*) => {
        unreachable!("component_entity! should only be used inside a #[component] or #[component_stateless] function, and in ident: type = expr or ident: type form")
    };
}

#[macro_export]
macro_rules! init_with_context {
    ($($t:tt)*) => {
        unreachable!("use_context! should only be used inside a #[component] function, and in ident: type = expr or ident: type form");
    };
}

#[macro_export]
macro_rules! subscribe {
    ($($t:tt)*) => {
        unreachable!("subscribe! should only be used inside a #[component] function, and in subscribe!(entity, closure) form (maps to cx.subscribe_in)");
    };
}

#[macro_export]
macro_rules! observe {
    ($($t:tt)*) => {
        unreachable!("observe! should only be used inside a #[component] function, and in observe!(Type, |self, window, cx| {}) form");
    };
}