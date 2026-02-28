use core::str;

use gpui::*;
use gpui_component::input::{Input, InputState};
use gpui_component::*;
use rfd::AsyncFileDialog;
use simple_gpui_core::component;

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.set_global(AppState::default());
        cx.on_action(new_file);
        cx.on_action(open_file);
        cx.activate(true);
        set_app_menus(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(800.), px(600.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| Editor::new(cx, window));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .unwrap();

            Ok::<_, ()>(())
        })
        .detach();
    });
}

#[component]
fn editor(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    init_with_context!();
    component_property!(input_state: Entity<InputState> = cx.new(|cx| InputState::new(window, cx).multi_line(true)));
    observe!(AppState, |page, window, cx| {
        let app_state = cx.global::<AppState>();
        let content = app_state.content.clone();
        println!("AppState changed: content={}", content);
        page.input_state.update(cx, |input, cx| {
            input.set_value(content, window, cx)
        });

        cx.notify();
    });

    v_flex()
        .p_5()
        .gap_2()
        .size_full()
        .items_center()
        .justify_center()
        .child(Input::new(&self.input_state))
}

actions!(set_menus, [NewFile, OpenFile]);
#[derive(Default)]
struct AppState {
    content: SharedString,
}
impl Global for AppState {}

pub fn set_app_menus(cx: &mut App) {
    cx.set_menus(vec![Menu {
        name: "Project".into(),
        items: vec![
            MenuItem::action("New File", NewFile),
            MenuItem::action("Open File", OpenFile),
        ],
    }]);
}

pub fn new_file(_: &NewFile, cx: &mut App) {
    println!("Creating new file...");
    let app_state = cx.global_mut::<AppState>();
    app_state.content = SharedString::new("");
}

pub fn open_file(_: &OpenFile, cx: &mut App) {
    cx.spawn(async move |cx| {
        let file = AsyncFileDialog::new()
            .add_filter("config file", &["json", "toml"])
            .add_filter("all files", &["*"])
            .set_directory(".")
            .pick_file()
            .await;
        let Some(file) = file else {
            return;
        };
        let path = file.path().to_path_buf();
        let content =
            std::fs::read_to_string(path).unwrap_or_else(|_| "Failed to read file".into());
        let _ = cx.update(|cx| {
            let app_state = cx.global_mut::<AppState>();
            app_state.content = content.into();
        });
    })
    .detach();
}
