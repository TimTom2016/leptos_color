use csscolorparser::Color;
use leptos::logging::warn;
use leptos::prelude::*;

use crate::hooks::use_position::{UsePositionProps, use_position};
mod style {
    leptos_styling::style_sheet!(
        saturation_style,
        "./src/components/saturation.css",
        "leptos_color"
    );
}
use style::saturation_style;

/// A component for selecting color saturation and value.
///
/// This component provides a 2D area where the user can select the saturation and value
/// of a color. The horizontal axis represents saturation, and the vertical axis represents value.
///
/// # Props
///
/// * `on_change`: A `Callback<(f64, f64)>` that is called when the selected position changes.
///   The callback receives a tuple of (left, top) values, where both are in the range [0, 1].
///   - `left` represents the saturation (0 = unsaturated, 1 = fully saturated)
///   - `top` represents the value (0 = full value/brightness, 1 = no value/black)
///
/// # Behavior
///
/// - The component renders a square area with a white-to-transparent gradient overlaid on
///   a black-to-transparent gradient to create a saturation-value selection field.
/// - Users can click, tap, or drag within this area to select a color.
/// - The component uses the `use_position` hook to handle mouse and touch interactions.
/// - As the user interacts with the component, the `on_change` callback is triggered with
///   the new position values.
///
/// # Styling
///
/// The component includes its own CSS styles, which are mounted using the `mount_style` function.
/// It also injects additional styles for the saturation and value gradients.
///
/// # Example
///
/// ```rust
/// use leptos::*;
///
/// #[component]
/// fn ColorPicker() -> impl IntoView {
///     let (saturation, set_saturation) = create_signal(0.5);
///     let (value, set_value) = create_signal(0.5);
///
///     view! {
///         <Saturation
///             on_change=move |(s, v)| {
///                 set_saturation.set(s);
///                 set_value.set(1.0 - v); // Invert v because top=0 is full value
///             }
///         />
///         <p>"Saturation: " {move || saturation.get()}</p>
///         <p>"Value: " {move || value.get()}</p>
///     }
/// }
/// ```
///
/// This example creates a `Saturation` component and displays the selected saturation and value.
#[component]
pub fn Saturation(#[prop(into)] on_change: Callback<(f64, f64)>) -> impl IntoView {
    // Closure that handles the position move
    let handle_move = Callback::new(move |(left, top): (f64, f64)| on_change.run((left, top)));

    // Use the `use_position` hook to get the ref and handle_start function
    let (ref_div, handle_start) = use_position(UsePositionProps {
        on_move: handle_move.clone(),
    });
    view! {
        <div node_ref={ref_div} class=saturation_style::LEPTOS_COLOR_COLOR on:touchstart=move |ev| {
            handle_start.run(ev.into());} on:mousedown=move |ev| {
            handle_start.run(ev.into());}>
            <div class=format!("{} {}",saturation_style::SATURATION_WHITE,saturation_style::LEPTOS_COLOR_GRADIENT)>
            <div class=format!("{} {}",saturation_style::SATURATION_BLACK,saturation_style::LEPTOS_COLOR_GRADIENT) />
            <div class=saturation_style::LEPTOS_COLOR_POINTER>
                <div class=saturation_style::LEPTOS_COLOR_CIRCLE />
            </div>
            </div>
        </div>
    }
}
