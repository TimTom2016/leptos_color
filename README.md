# Leptos Color

[![Crates.io](https://img.shields.io/crates/v/leptos_color.svg)](https://crates.io/crates/leptos_color)
[![docs.rs](https://docs.rs/leptos_color/badge.svg)](https://docs.rs/leptos_color/)

Leptos Color is a simple crate that provides a color picker component for [Leptos](https://github.com/leptos-rs/leptos), a Rust web framework. It allows easy integration of color selection functionality in Leptos applications.\
Heavily inspired by [React Pick Color](https://www.npmjs.com/package/react-pick-color)
## Features

- **Color Picker**: A customizable color picker component.
- **Color Input**: An input field with an attached color picker.
- **Theme Support**: Customizable theming options.
- **Flexible Configuration**: Options to hide specific color input types (alpha, hex, RGB).

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
leptos_color = "0.8.0"
```

## Usage

### Basic Color Picker

```rust
use leptos::*;
use leptos_color::{components::color_picker::ColorPicker, Color};

#[component]
fn App() -> impl IntoView {
    let (color, set_color) = create_signal(Color::new(1.0, 1.0, 1.0, 1.0));

    view! {
        <ColorPicker
            color=color
            on_change=move |new_color| set_color.set(new_color)
        />
    }
}
```

### Color Input

```rust
use leptos::*;
use leptos_color::{components::color_input::ColorInput, Color};

#[component]
fn App() -> impl IntoView {
    let color = create_rw_signal(Color::new(1.0, 1.0, 1.0, 1.0));

    view! {
        <ColorInput
            color=color
            on_change=move |new_color| color.set(new_color)
        />
    }
}
```

## Configuration Options

- `theme`: Customize the appearance of the color picker.
- `hide_alpha`: Hide the alpha (opacity) input.
- `hide_hex`: Hide the hexadecimal color input.
- `hide_rgb`: Hide the RGB color inputs.

## Examples

Check the `examples/basic-ssr` directory for a complete example of how to use Leptos Color in a server-side rendered application.

## Feature Flags

- `default`: Includes the `color_input` feature.
- `csr`: Client-side rendering support.
- `ssr`: Server-side rendering support.
- `hydrate`: Hydration support.
- `color_input`: Enables the ColorInput component.

## Documentation

For more detailed information, check out the [API documentation](https://docs.rs/leptos_color/).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
