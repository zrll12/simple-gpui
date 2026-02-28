use gpui::*;
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::*;
use simple_gpui::component_property;
use simple_gpui_core::component;

#[component]
// Window and cx here are not the same as in the property definition. Maybe you can rename them?
fn hello_world(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    init_with_context!();
    component_property!(input_state: Entity<InputState> = cx.new(|cx| InputState::new(window, cx).placeholder("Enter your name")));
    component_property!(text: SharedString = SharedString::new("World"));
    subscribe!(input_state, |view, _state, event, _window, cx| match event {
        InputEvent::Change => {
            let value = input_state.read(cx).value();
            view.text = value.clone();
            cx.notify()
        }
        _ => {}
    });

    v_flex()
        .p_5()
        .gap_2()
        .size_full()
        .items_center()
        .justify_center()
        .child(Input::new(&self.input_state))
        .child(format!("Hello, {}!", &self.text))
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(800.), px(600.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| HelloWorld::new(cx, window));
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .unwrap();

            Ok::<_, ()>(())
        })
        .detach();
    });
}
