# Simple GPUI

[中文文档](./README_zh.md)

> **Note:** This library is still evolving alongside my gpui project, so there is a high likelihood of additional breaking changes until we reach version 1.0.0, which will be released once my gpui project is complete and everything is confirmed to work smoothly.

### Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
simple-gpui = "0.1.0"
gpui = "0.2.2"
gpui-component = "0.3.1"
```

Then run an example:

```bash
cargo run --example hello_world
```

Minimal example:

```rust
use gpui::*;
use simple_gpui_core::component;

#[component]
fn hello_world(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    component_property!(text: SharedString = SharedString::new("World"));
    
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .child(format!("Hello, {}!", &self.text))
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|cx| HelloWorld::new(cx))
        }).unwrap();
    });
}
```

For full documentation, guides, and API details, please visit:

https://github.com/zrll12/simple-gpui/wiki
