
use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use simple_gpui::component_property;
use simple_gpui_core::component;

#[component]
fn hello_world(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    component_property!(text: SharedString = SharedString::new("World1"));
    component_property!(text2: SharedString);

    div()
        .flex()
        .flex_col()
        .gap_3()
        .bg(rgb(0x505050))
        .size(px(500.0))
        .justify_center()
        .items_center()
        .shadow_lg()
        .border_1()
        .border_color(rgb(0x0000ff))
        .text_xl()
        .text_color(rgb(0xffffff))
        .child(format!("Hello, {}!\nAnd hello {}", &self.text, &self.text2))
        .child(
            div()
                .flex()
                .gap_2()
                .child(div().size_8().bg(gpui::red()))
                .child(div().size_8().bg(gpui::green()))
                .child(div().size_8().bg(gpui::blue()))
                .child(div().size_8().bg(gpui::yellow()))
                .child(div().size_8().bg(gpui::black()))
                .child(div().size_8().bg(gpui::white())),
        )
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {

                cx.new(|_| HelloWorld::new(SharedString::new("HHU")))
            },
        )
        .unwrap();
    });
}
