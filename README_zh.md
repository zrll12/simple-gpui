# Simple GPUI

[English](./README.md)

### 快速开始

在您的 `Cargo.toml` 中添加:

```toml
[dependencies]
simple-gpui = "0.1.0"
gpui = "0.2.2"
gpui-component = "0.3.1"
```

然后运行示例:

```bash
cargo run --example hello_world
```

最小示例:

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

完整文档、指南与 API 说明请查看 Wiki:

https://github.com/zrll12/simple-gpui/wiki
