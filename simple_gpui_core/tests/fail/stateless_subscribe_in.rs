use simple_gpui_core::component_stateless;

#[component_stateless]
fn stateless_subscribe_in() {
    subscribe_in!(state, |_, _, _, _, _| {});
}

fn main() {}
