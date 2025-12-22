use leptos::ev::{Event, UiEvent, mousemove, mouseup, touchend, touchmove};
use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::{use_document, use_event_listener};
use std::ops::Deref;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent, TouchEvent};
#[derive(Clone)]
pub struct UsePositionProps {
    pub on_move: Callback<(f64, f64), ()>,
}

enum MoveType {
    Mouse,
    Touch,
}
/// A custom hook for handling position-based interactions in a component.
///
/// This hook provides functionality for tracking and responding to mouse and touch
/// interactions within a specified element, typically used for draggable or
/// position-sensitive components like color pickers or sliders.
///
/// # Arguments
///
/// * `props`: `UsePositionProps` - A struct containing the configuration for the hook.
///
/// # Returns
///
/// A tuple containing:
/// 1. A `NodeRef<Div>` that should be attached to the target element.
/// 2. A `Callback<UiEvent>` that should be used to handle the start of an interaction (mousedown or touchstart).
///
/// # UsePositionProps
///
/// ```rust
/// #[derive(Clone)]
/// pub struct UsePositionProps {
///     pub on_move: Callback<(f64, f64), ()>,
/// }
/// ```
///
/// * `on_move`: A callback that is triggered when the position changes. It receives a tuple of (x, y)
///   coordinates, normalized to the range [0, 1] relative to the element's dimensions.
///
/// # Behavior
///
/// - Tracks mouse and touch interactions within the target element.
/// - Normalizes the position to values between 0 and 1 for both x and y coordinates.
/// - Handles dragging behavior, including starting, moving, and ending drag operations.
/// - Attaches necessary event listeners dynamically when dragging starts and removes them when it ends.
/// - Works with both mouse and touch events for broad device compatibility.
///
/// # Example
///
/// ```rust
/// use leptos::*;
///
/// #[component]
/// fn PositionTracker() -> impl IntoView {
///     let (position, set_position) = create_signal((0.0, 0.0));
///
///     let props = UsePositionProps {
///         on_move: Callback::new(move |pos| set_position.set(pos)),
///     };
///
///     let (ref_div, handle_start) = use_position(props);
///
///     view! {
///         <div
///             ref=ref_div
///             on:mousedown=handle_start
///             on:touchstart=handle_start
///             style="width: 200px; height: 200px; background-color: #f0f0f0;"
///         >
///             "Drag here"
///         </div>
///         <p>"Position: " {move || format!("({:.2}, {:.2})", position.get().0, position.get().1)}</p>
///     }
/// }
/// ```
///
/// This example creates a draggable area that tracks and displays the current position.
pub fn use_position(props: UsePositionProps) -> (NodeRef<Div>, Callback<UiEvent>) {
    let (dragging, set_dragging) = signal(false);
    let ref_div = NodeRef::<Div>::new();

    let limit = |value: f64| -> f64 { value.min(1.0).max(0.0) };

    let get_position = move |e: &Event| -> Option<(f64, f64)> {
        if let Some(div) = ref_div.get_untracked() {
            let rect = Element::from(div.deref().clone()).get_bounding_client_rect();
            let (width, height) = (rect.width(), rect.height());

            let (client_x, client_y) = if let Some(mouse_event) = e.dyn_ref::<MouseEvent>() {
                (mouse_event.client_x() as f64, mouse_event.client_y() as f64)
            } else if let Some(touch_event) = e.dyn_ref::<TouchEvent>() {
                if let Some(touch) = touch_event.touches().item(0) {
                    (touch.client_x() as f64, touch.client_y() as f64)
                } else {
                    return None;
                }
            } else {
                return None;
            };
            Some((
                limit((client_x - rect.left()) / width),
                limit((client_y - rect.top()) / height),
            ))
        } else {
            None
        }
    };

    let handle_move = {
        let on_move = props.on_move.clone();
        move |move_type: MoveType, e: Event| {
            if matches!(move_type, MoveType::Mouse) {
                e.prevent_default();
            }
            if let Some(pos) = get_position(&e) {
                on_move.run(pos);
            }
        }
    };

    let handle_start = move |e: UiEvent| {
        set_dragging.set(true);
        if let Some(pos) = get_position(&e) {
            props.on_move.run(pos);
        }
    };

    let handle_end = move || {
        set_dragging.set(false);
    };

    Effect::new(move |_| {
        let is_dragging = dragging.get();
        if is_dragging {
            let _ = use_event_listener(use_document(), mousemove, move |evt| {
                handle_move(MoveType::Mouse, evt.into());
            });
            let _ = use_event_listener(use_document(), mouseup, move |_| {
                handle_end();
            });
            let _ = use_event_listener(use_document(), touchmove, move |evt| {
                handle_move(MoveType::Touch, evt.into());
            });
            let _ = use_event_listener(use_document(), touchend, move |_| {
                handle_end();
            });
        };
    });

    (ref_div, Callback::new(handle_start))
}
