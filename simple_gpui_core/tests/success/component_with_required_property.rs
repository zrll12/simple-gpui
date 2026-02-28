use simple_gpui_core::component;

trait Render {
    fn render(&mut self);
}

#[allow(unused_macros)]
macro_rules! component_property {
    ($($t:tt)*) => {};
}

#[component]
fn component_with_required_property() {
    component_property!(title: String);
}

fn main() {
    let _ = ComponentWithRequiredProperty::new(String::from("hello"));
}
