use leptos::logging::log;
use leptos::prelude::*;
use leptos_color::components::color_input::ColorInput;
use leptos_color::Color;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_styling::StyleSheets;
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <StyleSheets/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/basic-ssr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let color = RwSignal::new(Color::new(1.0, 1.0, 1.0, 1.0));
    let on_change = Callback::new(move |x: Color| {
        log!("{:?}", x);
        color.set(x)
    });
    view! {
        <h1>"Welcome to Leptos Color!"</h1>
        <div style="height: 500px; width: 500px; display: flex; align-items: center; justify-content: center;">
            <ColorInput color=color on_change=on_change></ColorInput>
        </div>
    }
}
