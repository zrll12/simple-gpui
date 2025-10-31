# Simple GPUI

[中文文档](./README_zh.md)

### Overview

Simple GPUI is a Rust library that provides a simplified component-based framework for building GPUI applications. It offers a declarative approach to creating UI components using procedural macros, making GPUI development more intuitive and ergonomic.

### Features

- **Component Macro**: Simplify component creation with the `#[component]` attribute macro
- **Reactive Properties**: Define component properties with automatic getter/setter generation
- **Event Subscriptions**: Easy event handling with the `subscribe!` macro
- **Context Management**: Simplified context access with `init_with_context!`
- **Type-Safe**: Full Rust type safety with compile-time guarantees

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
simple-gpui = "0.1.0"
gpui = "0.2.2"
gpui-component = "0.3.1"
```

### Quick Start

Here's a simple "Hello World" example:

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

### Core Concepts

#### Component Properties

Define component properties using the `component_property!` macro:

```rust
#[component]
fn my_component(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    // Property with default value
    component_property!(count: i32 = 0);
    
    // Property without default value (must be set during construction)
    component_property!(name: SharedString);
    
    div().child(format!("{}: {}", self.name, self.count))
}
```

#### Event Subscriptions

Subscribe to events from entities using the `subscribe!` macro:

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
        .child(format!("You typed: {}", &self.text))
}
```

#### Context Access

Use `init_with_context!()` when you need to access the window or context during property initialization:

```rust
#[component]
fn my_component(_window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
    init_with_context!();
    // Now you can use 'window' and 'cx' in property initializers
    component_property!(state: Entity<MyState> = cx.new(|cx| MyState::new(window, cx)));
    
    div().child("Content")
}
```

### Examples

The repository includes several examples:

1. **hello_world.rs** - Basic component with properties
2. **gpui_component_input.rs** - Input handling with event subscriptions
3. **temperature_caculator.rs** - Temperature converter with tabs and input validation

Run examples with:

```bash
cargo run --example hello_world
cargo run --example gpui_component_input
cargo run --example temperature_caculator
```

### Project Structure

```
simple-gpui/
├── src/                    # Main library exports
├── simple_gpui_core/       # Core procedural macros
│   ├── src/
│   │   ├── lib.rs         # Component macro implementation
│   │   ├── extractors.rs  # Macro parsing logic
│   │   └── methods.rs     # Code generation for methods
├── examples/               # Example applications
└── Cargo.toml
```

### How It Works

The `#[component]` macro transforms your function into a struct with:

1. **Generated struct** with fields for each `component_property!`
2. **new() method** for initialization
3. **Setter methods** for each property
4. **Render trait implementation** using your function body
5. **Subscription management** for event handlers

### Requirements

- Rust 2024 edition or later
- GPUI 0.2.2+
- gpui-component 0.3.1+

### Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
