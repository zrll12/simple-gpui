use simple_gpui_core::component_stateless;

#[component_stateless]
fn duplicate_property_definition() {
    component_property!(count: usize = 1);
    component_property!(count: usize = 2);
}

fn main() {}
