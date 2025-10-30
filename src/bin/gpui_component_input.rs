use gpui::*;
use gpui_component::input::{InputState, TextInput};
use gpui_component::*;
use simple_gpui_core::component;
// pub struct Example {
//     input_state: Entity<InputState>,
//     display_text: SharedString,
//
//     /// We need to keep the subscriptions alive with the Example entity.
//     ///
//     /// So if the Example entity is dropped, the subscriptions are also dropped.
//     /// This is important to avoid memory leaks.
//     _subscriptions: Vec<Subscription>,
// }
//
// impl Example {
//     fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
//         let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("Enter your name"));
//
//         let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
//             let input_state = input_state.clone();
//             move |this, _, ev: &InputEvent, _window, cx| match ev {
//                 InputEvent::Change => {
//                     let value = input_state.read(cx).value();
//                     this.display_text = format!("Hello, {}!", value).into();
//                     cx.notify()
//                 }
//                 _ => {}
//             }
//         })];
//
//         Self {
//             input_state,
//             display_text: SharedString::default(),
//             _subscriptions,
//         }
//     }
// }
//
// impl Render for Example {
//     fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
//         v_flex()
//             .p_5()
//             .gap_2()
//             .size_full()
//             .items_center()
//             .justify_center()
//             .child(TextInput::new(&self.input_state))
//             .child(self.display_text.clone())
//     }
// }

#[component]
fn hello_world(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    with_context!();
    with_window!();
    component_property!(input_state: Entity<InputState> = cx.new(|cx| InputState::new(window, cx).placeholder("Enter your name")));
    component_property!(text: SharedString = SharedString::new("World"));

    v_flex()
        .p_5()
        .gap_2()
        .size_full()
        .items_center()
        .justify_center()
        .child(TextInput::new(&self.input_state))
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
                cx.new(|cx| Root::new(view.into(), window, cx))
            })
            .unwrap();

            Ok::<_, ()>(())
        })
        .detach();
    });
}
