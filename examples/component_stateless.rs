use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size
};
use simple_gpui_core::{component, component_stateless};

#[component_stateless]
fn badge(_window: &mut Window, _cx: &mut App) -> impl IntoElement {
    component_property!(text: SharedString = SharedString::new("New"));

    div()
        .px_2()
        .py_1()
        .rounded_md()
        .bg(rgb(0xff0000))
        .text_color(rgb(0xffffff))
        .text_sm()
        .child(self.text.clone())
}

#[component]
fn demo(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_center()
        .size_full()
        .child(Badge::new())
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Demo::new()),
        )
        .unwrap();
    });
}
