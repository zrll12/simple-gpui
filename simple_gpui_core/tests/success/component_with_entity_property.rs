use simple_gpui_core::component;

trait Render {
    fn render(&mut self);
}

struct Entity<T>(T);

#[allow(unused_macros)]
macro_rules! component_entity {
    ($($t:tt)*) => {};
}

#[component]
fn component_with_entity_property() {
    component_entity!(count: usize = Entity(1));
}

fn main() {
    let _ = ComponentWithEntityProperty::new();
}
