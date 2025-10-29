
// This macro is not necessarily used, only for dismissing ide errors.
#[macro_export]
macro_rules! component_property {
    ($($t:tt)*) => {
        unreachable!("component_property! should only be used inside a #[component] function, and in ident: type = expr or ident: type form");
    };
}