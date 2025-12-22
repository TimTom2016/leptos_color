use crate::components::alpha::Alpha;
use crate::components::hue::Hue;
use crate::components::saturation::Saturation;
use crate::theme::Theme;
use csscolorparser::Color;
use leptos::html::Div;
use leptos::logging::warn;
use leptos::prelude::*;
use leptos_use::{UseCssVarOptions, use_css_var_with_options};
mod style {
    leptos_styling::style_sheet!(
        color_picker_style,
        "./src/components/color_picker.css",
        "leptos_color"
    );
}
use style::color_picker_style;
/// A comprehensive color picker component.
///
/// This component provides a full-featured color picker with saturation/value selection,
/// hue selection, alpha selection (optional), and input fields for hex, RGB, and alpha values.
///
/// # Props
///
/// * `theme`: A `MaybeSignal<Theme>` representing the theme for the component. Defaults to `Theme::default()`.
/// * `color`: A `Signal<Color>` representing the current color value.
/// * `hide_alpha`: An optional `MaybeSignal<bool>` to hide the alpha channel controls.
/// * `hide_hex`: An optional `MaybeSignal<bool>` to hide the hexadecimal color input.
/// * `hide_rgb`: An optional `MaybeSignal<bool>` to hide the RGB color inputs.
/// * `on_change`: A `Callback<Color>` that is called when the color value changes.
///
/// # Features
///
/// - Saturation/Value selector: A large area for selecting color saturation and value.
/// - Hue selector: A slider for selecting the hue of the color.
/// - Alpha selector: An optional slider for selecting the alpha (transparency) of the color.
/// - Hex input: An input field for entering or displaying the color in hexadecimal format.
/// - RGB inputs: Separate input fields for red, green, and blue color components.
/// - Alpha input: An optional input field for the alpha value.
///
/// # Behavior
///
/// - The component uses CSS variables to manage and update color values efficiently.
/// - It reacts to changes in the `color` signal and updates all UI elements accordingly.
/// - User interactions with any part of the color picker (saturation area, hue slider, alpha slider, or input fields)
///   trigger the `on_change` callback with the updated color.
///
/// # Example
///
/// ```rust
/// use leptos::*;
/// use csscolorparser::Color;
///
/// #[component]
/// fn App() -> impl IntoView {
///     let (color, set_color) = create_signal(Color::from_rgba8(255, 0, 0, 255));
///
///     view! {
///         <ColorPicker
///             color=color
///             on_change=move |new_color| set_color.set(new_color)
///         />
///     }
/// }
/// ```
///
/// This example creates a `ColorPicker` component with a default red color and updates the color
/// when any change is made in the picker.
#[component]
pub fn ColorPicker(
    #[prop(into, default=Theme::default().into())] theme: Signal<Theme>,
    #[prop(into)] color: Signal<Color>,
    #[prop(into, optional)] hide_alpha: Signal<bool>,
    #[prop(into, optional)] hide_hex: Signal<bool>,
    #[prop(into, optional)] hide_rgb: Signal<bool>,
    #[prop(into)] on_change: Callback<Color>,
) -> impl IntoView {
    let el = NodeRef::<Div>::new();
    let (hue, set_hue) = use_css_var_with_options(
        "--lpc-hue",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("0")
            .observe(false),
    );

    // Define the other CSS variables
    let (red, set_red) = use_css_var_with_options(
        "--lpc-red",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("0")
            .observe(false),
    );

    let (green, set_green) = use_css_var_with_options(
        "--lpc-green",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("0")
            .observe(false),
    );

    let (blue, set_blue) = use_css_var_with_options(
        "--lpc-blue",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("0")
            .observe(false),
    );

    let (hex, set_hex) = use_css_var_with_options(
        "--lpc-hex",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("#000000")
            .observe(false),
    );

    let (alpha, set_alpha) = use_css_var_with_options(
        "--lpc-alpha",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("1")
            .observe(false),
    );

    let (rgba, set_rgba) = use_css_var_with_options(
        "--lpc-rgba",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("rgba(0, 0, 0, 1)")
            .observe(false),
    );

    let (hue_pointer, set_hue_pointer) = use_css_var_with_options(
        "--lpc-hue-pointer",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("0%")
            .observe(false),
    );

    let (alpha_pointer, set_alpha_pointer) = use_css_var_with_options(
        "--lpc-alpha-pointer",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("100%")
            .observe(false),
    );

    let (saturation_pointer_top, set_saturation_pointer_top) = use_css_var_with_options(
        "--lpc-saturation-pointer-top",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("calc(100% - 6px)")
            .observe(false),
    );

    let (saturation_pointer_left, set_saturation_pointer_left) = use_css_var_with_options(
        "--lpc-saturation-pointer-left",
        UseCssVarOptions::default()
            .target(el)
            .initial_value("calc(0% - 6px)")
            .observe(false),
    );

    // React to color changes and update CSS variables
    Effect::new(move |_| {
        color.track();
        let hsla = color.get().to_hsla();
        let rgba = color.get().to_rgba8();
        let alpha = rgba[3];
        let hex = color.get().to_hex_string();
        let hsva = color.get().to_hsva();

        set_hue.set((hsla[0] as u16).to_string());
        set_red.set(rgba[0].to_string());
        set_green.set(rgba[1].to_string());
        set_blue.set(rgba[2].to_string());
        set_hex.set(hex);
        set_alpha.set(alpha.to_string());
        set_rgba.set(format!(
            "rgba({}, {}, {}, {})",
            rgba[0],
            rgba[1],
            rgba[2],
            (alpha as f32 / 255.0)
        ));
        set_hue_pointer.set(format!("{}%", (hsla[0] * 100.0 / 360.0).round()));
        set_alpha_pointer.set(format!("{}%", (alpha as f32 / 255.0 * 100.0).round()));
        set_saturation_pointer_top.set(format!("calc({}% - 6px)", -(hsva[2] * 100.0) + 100.0));
        set_saturation_pointer_left.set(format!("calc({}% - 6px)", (hsva[1] * 100.0).round()));
    });

    view! {
        <div node_ref={el} class=color_picker_style::LEPTOS_COLOR_CONTAINER style=move || theme.with(|value| value.to_style())>
            <Saturation on_change=move |left: f64,top: f64| {
                let mut hsva = color.get().to_hsva();
                hsva[2] = (1.0 - top) as f32;
                hsva[1] = left as f32;
                if hsva[2] <= 0.0 {
                    hsva[2] = 0.001;
                }
                if hsva[1] <= 0.0 {
                    hsva[1] = 0.001;
                }
                on_change.run(Color::from_hsva(hsva[0], hsva[1], hsva[2], hsva[3]));
            }/>
            <div class=color_picker_style::LEPTOS_COLOR_FLEX>
                <div class=color_picker_style::LEPTOS_COLOR_VALUE_WRAPPER>
                    <div class=color_picker_style::LEPTOS_COLOR_CHECKBOARD>
                        <div class=color_picker_style::LEPTOS_COLOR_VALUE />
                    </div>
                </div>
                <div class=color_picker_style::LEPTOS_COLOR_RANGES>
                    <Hue on_change=move |left,_| {
                        let hsla = color.get().to_hsla();
                        on_change.run(Color::from_hsla((left*360.0) as f32, hsla[1], hsla[2], hsla[3]));
                    } />
                    <Show
                        when=move || { !hide_alpha.get()}
                      >
                      <Alpha on_change=move |left,_| {
                          let mut color = color.get();
                          color.a = left as f32;
                          on_change.run(color);
                      }/>
                    </Show>
                </div>
            </div>

            <div class=color_picker_style::LEPTOS_COLOR_INPUTS>
                <Show
                    when=move || { !hide_hex.get()}
                >
                <label class=color_picker_style::LEPTOS_COLOR_LABEL>
                    <div class=color_picker_style::LEPTOS_COLOR_WRAPPER>
                        <span class=color_picker_style::LEPTOS_COLOR_PREFIX>"#"</span>

                        <input
                        class=color_picker_style::LEPTOS_COLOR_INPUT
                        type="text"
                        name="hex"
                        style:width="54px"
                        on:blur={move |ev| {
                            match event_target_value(&ev).parse::<Color>() {
                                Ok(new_color) => on_change.run(new_color),
                                Err(_) => {},
                            }
                        }}
                        on:change={move |ev| {
                            match event_target_value(&ev).parse::<Color>() {
                                Ok(new_color) => on_change.run(new_color),
                                Err(_) => {},
                            }
                        }}
                        prop:value={move || hex.get().replace("#", "")}
                        maxlength={6}
                        />
                        </div>
                        <span>"Hex"</span>
                    </label>
                    <div style="display: flex"/>
                </Show>
                <Show
                    when=move || { !hide_rgb.get()}
                >
                <label class=color_picker_style::LEPTOS_COLOR_LABEL>
                    <div class=color_picker_style::LEPTOS_COLOR_WRAPPER>
                        <input
                            class=color_picker_style::LEPTOS_COLOR_INPUT
                            prop:value=red
                            name="red"
                            type="number"
                            style:width="42px"
                            min={0}
                            max={255}
                            step={1}
                            autocomplete="off"
                            on:change={move |ev| {
                                match event_target_value(&ev).parse::<u8>() {
                                    Ok(value) => {
                                        let mut color = color.get();
                                        color.r = value as f32 / 255.0;
                                        on_change.run(color);
                                    },
                                    Err(_) => todo!(),
                                }
                            }}
                        />

                            </div>
                        <span>"R"</span>
                    </label>
                <label class=color_picker_style::LEPTOS_COLOR_LABEL>
                    <div class=color_picker_style::LEPTOS_COLOR_WRAPPER>
                        <input
                            class=color_picker_style::LEPTOS_COLOR_INPUT
                            prop:value=green
                            name="green"
                            type="number"
                            style:width="42px"
                            min={0}
                            max={255}
                            step={1}
                            autocomplete="off"
                            on:change={move |ev| {
                                match event_target_value(&ev).parse::<u8>() {
                                    Ok(value) => {
                                        let mut color = color.get();
                                        color.g = value as f32 / 255.0;
                                        on_change.run(color);
                                    },
                                    Err(_) => todo!(),
                                }
                            }}
                        />
                    </div>
                    <span>"G"</span>
                </label>
                <label class=color_picker_style::LEPTOS_COLOR_LABEL>
                    <div class=color_picker_style::LEPTOS_COLOR_WRAPPER>
                        <input
                            class=color_picker_style::LEPTOS_COLOR_INPUT
                            prop:value=blue
                            name="blue"
                            type="number"
                            style:width="42px"
                            min={0}
                            max={255}
                            step={1}
                            autocomplete="off"
                            on:change={move |ev| {
                                match event_target_value(&ev).parse::<u8>() {
                                    Ok(value) => {
                                        let mut color = color.get();
                                        color.b = value as f32 / 255.0;
                                        on_change.run(color);
                                    },
                                    Err(_) => {},
                                }
                            }}
                        />
                    </div>
                    <span>"B"</span>
                </label>
                </Show>
                <Show
                    when=move || { !hide_alpha.get()}
                >
                <label class=color_picker_style::LEPTOS_COLOR_LABEL>
                    <div class=color_picker_style::LEPTOS_COLOR_WRAPPER>
                    <input
                        class=color_picker_style::LEPTOS_COLOR_INPUT
                        prop:value=alpha
                        name="alpha"
                        type="number"
                        style:width="42px"
                        min={0}
                        max={255}
                        step={1}
                        autocomplete="off"
                        on:change={move |ev| {
                            match event_target_value(&ev).parse::<u8>() {
                                Ok(value) => {
                                    let mut color = color.get();
                                    color.a = value as f32 / 255.0;
                                    on_change.run(color);
                                },
                                Err(_) => {},
                            }
                        }}/>
                    </div>
                    <span>"Alpha"</span>
                </label>
                </Show>
            </div>
        </div>
    }
}
