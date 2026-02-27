use gpui::*;
use gpui_component::input::{InputEvent, InputState, TextInput};
use gpui_component::tab::{Tab, TabBar};
use gpui_component::*;
use simple_gpui_core::component;

#[component]
fn hello_world(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    init_with_context!();
    component_property!(input_state: Entity<InputState> = cx.new(|cx| InputState::new(window, cx).validate(|s, _| s.parse::<f32>().is_ok())));
    component_property!(c: f32 = 0.);
    component_property!(f: f32 = 32.);
    component_property!(selected: usize = 0);
    subscribe!(input_state, |view, _state, event, _window, cx| match event {
        InputEvent::Change => {
            let value = input_state.read(cx).value();
            if let Ok(c) = value.parse::<f32>() {
                if view.selected == 0 {
                    view.c = c;
                    view.f = (c * 9. / 5.) + 32.;
                } else if view.selected == 1 {
                    view.f = c;
                    view.c = (c - 32.) * 5. / 9.;
                }
                cx.notify()
            }
        }
        _ => {}
    });

    v_flex()
        .p_5()
        .gap_2()
        .size_full()
        .items_center()
        .justify_center()
        .child(
            h_flex()
                .items_center()
                .justify_center()
                .child("Select Input Unit: ")
                .child(
                    TabBar::new("segmented-tabs")
                        .segmented()
                        .selected_index(self.selected)
                        .on_click(_cx.listener(|view, index, _, cx| {
                            view.selected = *index;
                            if *index == 0 { // F -> C
                                view.c = view.f;
                                view.f = view.c * 9. / 5. + 32.;
                            } else if *index == 1 { // C -> F
                                view.f = view.c;
                                view.c = view.f - 32. * 5. / 9.;
                            }
                            cx.notify();
                        }))
                        .child(Tab::new("°C"))
                        .child(Tab::new("°F")),
                ),
        )
        .child(TextInput::new(&self.input_state))
        .child(format!("Celsius: {:.2} °C", &self.c))
        .child(format!("Fahrenheit: {:.2} °F", &self.f))
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(800.), px(600.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| HelloWorld::new(cx, window));
                cx.new(|cx| Root::new(view.into(), window, cx))
            })
            .unwrap();

            Ok::<_, ()>(())
        })
        .detach();
    });
}
