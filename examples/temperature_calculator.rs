use gpui::*;
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::tab::{Tab, TabBar};
use gpui_component::*;
use simple_gpui_core::component;

#[component]
fn calculator(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    init_with_context!();
    component_entity!(input_state: InputState = InputState::new(window, cx).validate(|s, _| s.parse::<f32>().is_ok()));
    component_property!(input: f32 = 0.);
    component_property!(selected: usize = 0);
    subscribe!(input_state, |view, _state, event, _window, cx| {
        if let InputEvent::Change = event {
            let value = input_state.read(cx).value();
            if let Ok(c) = value.parse::<f32>() {
                view.input = c;
                cx.notify()
            }
        }
    });

    fn get_celsius(input: f32, selected: usize) -> f32 {
        if selected == 0 {
            input
        } else {
            (input - 32.) * 5. / 9.
        }
    }

    fn get_fahrenheit(input: f32, selected: usize) -> f32 {
        if selected == 0 {
            input * 9. / 5. + 32.
        } else {
            input
        }
    }

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
                            cx.notify();
                        }))
                        .child(Tab::new().label("°C"))
                        .child(Tab::new().label("°F")),
                ),
        )
        .child(Input::new(&self.input_state))
        .child(format!("Celsius: {:.2} °C", get_celsius(self.input, self.selected)))
        .child(format!("Fahrenheit: {:.2} °F", get_fahrenheit(self.input, self.selected)))
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(300.), px(200.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| Calculator::new(cx, window));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .unwrap();

            Ok::<_, ()>(())
        })
        .detach();
    });
}
