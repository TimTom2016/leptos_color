use csscolorparser::Color;
use leptos::logging::warn;
use leptos::prelude::*;

use crate::hooks::use_position::{use_position, UsePositionProps};
mod style {
    leptos_styling::style_sheet!(
        alpha_style,
        "./src/components/alpha.css",
        "leptos_color_alpha"
    );
}
use style::alpha_style;
/// A component for selecting the alpha (transparency) value of a color.
///
/// This component provides a horizontal slider that allows users to select
/// the alpha value of a color, ranging from fully transparent to fully opaque.
///
/// # Props
///
/// * `on_change`: A `Callback<(f64, f64)>` that is called when the selected position changes.
///   The callback receives a tuple of (left, top) values, where:
///   - `left` represents the alpha value (0 = fully transparent, 1 = fully opaque)
///   - `top` is not used for this component but is included for consistency with other components
///
/// # Behavior
///
/// - The component renders a horizontal bar with a checkered background to represent transparency.
/// - Users can click, tap, or drag along this bar to select an alpha value.
/// - The component uses the `use_position` hook to handle mouse and touch interactions.
/// - As the user interacts with the component, the `on_change` callback is triggered with
///   the new position values.
///
/// # Styling
///
/// The component includes its own CSS styles, which are mounted using the `mount_style` function.
/// The styles define the appearance of the alpha slider, including the checkered background
/// that represents transparency.
///
/// # Example
///
/// ```rust
/// use leptos::*;
///
/// #[component]
/// fn ColorPicker() -> impl IntoView {
///     let (alpha, set_alpha) = create_signal(1.0);
///
///     view! {
///         <Alpha
///             on_change=move |(a, _)| {
///                 set_alpha.set(a);
///             }
///         />
///         <p>"Alpha: " {move || alpha.get()}</p>
///     }
/// }
/// ```
///
/// This example creates an `Alpha` component and displays the selected alpha value.

#[component]
pub fn Alpha(#[prop(into)] on_change: Callback<(f64, f64)>) -> impl IntoView {
    let handle_move = Callback::new(move |(left, top): (f64, f64)| on_change.run((left, top)));

    // Use the `use_position` hook to get the ref and handle_start function
    let (ref_div, handle_start) = use_position(UsePositionProps {
        on_move: handle_move.clone(),
    });
    view! {
        <div class=alpha_style::LEPTOS_COLOR_ALPHA_CONTAINER node_ref={ref_div} on:touchstart=move |ev| {
            handle_start.run(ev.into())} on:mousedown=move |ev| {
            handle_start.run(ev.into())}>
            <div class=alpha_style::LEPTOS_COLOR_ALPHA_ALPHA/>
            <div class=alpha_style::LEPTOS_COLOR_ALPHA_CHECKBOARD />
            <div class=alpha_style::LEPTOS_COLOR_ALPHA_POINTER>
                <div class=alpha_style::LEPTOS_COLOR_ALPHA_SLIDER />
            </div>
        </div>
    }
}
