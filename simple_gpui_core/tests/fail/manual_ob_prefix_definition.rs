use simple_gpui_core::component_stateless;

#[component_stateless]
fn manual_ob_prefix_definition() {
    component_property!(_ob_1: usize = 1);
}

fn main() {}
