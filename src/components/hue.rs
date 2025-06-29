use csscolorparser::Color;
use leptos::logging::warn;
use leptos::prelude::*;

use crate::hooks::use_position::{use_position, UsePositionProps};
mod style {
    leptos_styling::style_sheet!(hue_style, "./src/components/hue.css", "leptos_color_hue");
}
use style::hue_style;
/// A component for selecting the hue of a color.
///
/// This component provides a horizontal slider that allows users to select
/// the hue value of a color, representing the full spectrum of colors.
///
/// # Props
///
/// * `on_change`: A `Callback<(f64, f64)>` that is called when the selected position changes.
///   The callback receives a tuple of (left, top) values, where:
///   - `left` represents the hue value (0 to 1, mapping to 0° to 360° in the color wheel)
///   - `top` is not used for this component but is included for consistency with other components
///
/// # Behavior
///
/// - The component renders a horizontal bar with a gradient representing the full color spectrum.
/// - Users can click, tap, or drag along this bar to select a hue value.
/// - The component uses the `use_position` hook to handle mouse and touch interactions.
/// - As the user interacts with the component, the `on_change` callback is triggered with
///   the new position values.
///
/// # Styling
///
/// The component includes its own CSS styles, which are mounted using the `mount_style` function.
/// The styles define the appearance of the hue slider, including the color spectrum gradient.
///
/// # Example
///
/// ```rust
/// use leptos::*;
///
/// #[component]
/// fn ColorPicker() -> impl IntoView {
///     let (hue, set_hue) = create_signal(0.0);
///
///     view! {
///         <Hue
///             on_change=move |(h, _)| {
///                 set_hue.set(h * 360.0); // Convert to degrees
///             }
///         />
///         <p>"Hue: " {move || format!("{:.0}°", hue.get())}</p>
///     }
/// }
/// ```
///
/// This example creates a `Hue` component and displays the selected hue value in degrees.
#[component]
pub fn Hue(#[prop(into)] on_change: Callback<(f64, f64)>) -> impl IntoView {
    let handle_move = Callback::new(move |(left, top): (f64, f64)| on_change.run((left, top)));

    // Use the `use_position` hook to get the ref and handle_start function
    let (ref_div, handle_start) = use_position(UsePositionProps {
        on_move: handle_move.clone(),
    });
    view! {
        <div class=hue_style::LEPTOS_COLOR_HUE_CONTAINER node_ref={ref_div} on:touchstart=move |ev| {
            handle_start.run(ev.into())} on:mousedown=move |ev| {
            handle_start.run(ev.into())}>
            <div class=hue_style::LEPTOS_COLOR_HUE_POINTER>
                <div class=hue_style::LEPTOS_COLOR_HUE_SLIDER />
            </div>
        </div>
    }
}
