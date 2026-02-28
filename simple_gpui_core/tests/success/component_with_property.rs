use simple_gpui_core::component;

trait Render {
    fn render(&mut self);
}

#[allow(unused_macros)]
macro_rules! component_property {
    ($($t:tt)*) => {};
}

#[component]
fn component_with_property() {
    component_property!(count: usize = 1);
}

fn main() {
    let _ = ComponentWithProperty::new();
}
