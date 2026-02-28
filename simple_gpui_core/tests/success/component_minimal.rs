use simple_gpui_core::component;

trait Render {
    fn render(&mut self);
}

#[component]
fn component_minimal() {}

fn main() {}
