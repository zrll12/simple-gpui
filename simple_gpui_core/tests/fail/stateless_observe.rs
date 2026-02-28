use simple_gpui_core::component_stateless;

#[component_stateless]
fn stateless_observe() {
    observe!(AppState, |_, _, _| {});
}

fn main() {}
