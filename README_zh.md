# Simple GPUI

[English](./README.md)

### 概述

Simple GPUI 是一个 Rust 库,为构建 GPUI 应用程序提供了简化的基于组件的框架。它通过过程宏提供了一种声明式的方法来创建 UI 组件,使 GPUI 开发更加直观和符合人体工程学。

### 特性

- **组件宏**:使用 `#[component]` 属性宏简化组件创建
- **响应式属性**:定义组件属性并自动生成 getter/setter 方法
- **事件订阅**:使用 `subscribe!` 宏轻松处理事件
- **上下文管理**:使用 `init_with_context!` 简化上下文访问
- **类型安全**:完整的 Rust 类型安全和编译时保证

### 安装

在您的 `Cargo.toml` 中添加:

```toml
[dependencies]
simple-gpui = "0.1.0"
gpui = "0.2.2"
gpui-component = "0.3.1"
```

### 快速开始

这是一个简单的 "Hello World" 示例:

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

### 核心概念

#### 组件属性

使用 `component_property!` 宏定义组件属性:

```rust
#[component]
fn my_component(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    // 带有默认值的属性
    component_property!(count: i32 = 0);
    
    // 不带默认值的属性(必须在构造时设置)
    component_property!(name: SharedString);
    
    div().child(format!("{}: {}", self.name, self.count))
}
```

#### 事件订阅

使用 `subscribe!` 宏订阅实体的事件:

```rust
#[component]
fn input_example(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    init_with_context!();
    component_property!(input_state: Entity<InputState> = cx.new(|cx| InputState::new(window, cx)));
    component_property!(text: SharedString = SharedString::new(""));
    
    subscribe!(input_state, {
        let input_state = input_state.clone();
        move |this, _, ev: &InputEvent, _window, cx| {
            match ev {
                InputEvent::Change => {
                    this.text = input_state.read(cx).value();
                    cx.notify()
                }
                _ => {}
            }
        }
    });
    
    v_flex()
        .child(TextInput::new(&self.input_state))
        .child(format!("您输入了: {}", &self.text))
}
```

#### 上下文访问

当您需要在属性初始化期间访问 window 或 context 时,使用 `init_with_context!()`:

```rust
#[component]
fn my_component(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    init_with_context!();
    // 现在您可以在属性初始化器中使用 'window' 和 'cx'
    component_property!(state: Entity<MyState> = cx.new(|cx| MyState::new(window, cx)));
    
    div().child("内容")
}
```

### 示例

仓库包含几个示例:

1. **hello_world.rs** - 带属性的基本组件
2. **gpui_component_input.rs** - 带事件订阅的输入处理
3. **temperature_caculator.rs** - 带标签和输入验证的温度转换器

运行示例:

```bash
cargo run --example hello_world
cargo run --example gpui_component_input
cargo run --example temperature_caculator
```

### 项目结构

```
simple-gpui/
├── src/                    # 主库导出
├── simple_gpui_core/       # 核心过程宏
│   ├── src/
│   │   ├── lib.rs         # 组件宏实现
│   │   ├── extractors.rs  # 宏解析逻辑
│   │   └── methods.rs     # 方法代码生成
├── examples/               # 示例应用
└── Cargo.toml
```

### 工作原理

`#[component]` 宏将您的函数转换为一个结构体,包括:

1. **生成的结构体**,为每个 `component_property!` 生成字段
2. **new() 方法**用于初始化
3. **Setter 方法**用于每个属性
4. **Render trait 实现**使用您的函数体
5. **订阅管理**用于事件处理器

### 要求

- Rust 2024 版或更高版本
- GPUI 0.2.2+
- gpui-component 0.3.1+

### 贡献

欢迎贡献!请随时提交 Pull Request。
