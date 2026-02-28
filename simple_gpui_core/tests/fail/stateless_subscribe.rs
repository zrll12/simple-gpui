use simple_gpui_core::component_stateless;

#[component_stateless]
fn stateless_subscribe() {
    subscribe!(state, |_, _, _, _, _| {});
}

fn main() {}
