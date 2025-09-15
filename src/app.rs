use leptos::{html::Div, prelude::*};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use leptos_use::{
    core::Position, use_draggable_with_options, use_window, UseDraggableOptions, UseDraggableReturn,
};

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
        <Stylesheet id="leptos" href="/pkg/samba_canaster_web.css"/>

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
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let fixed_el = NodeRef::<Div>::new();

    let inner_width = use_window()
        .as_ref()
        .map(|w| w.inner_width().unwrap().as_f64().unwrap())
        .unwrap_or(0.0);

    let UseDraggableReturn {
        x: fixed_x,
        y: fixed_y,
        style: fixed_style,
        ..
    } = use_draggable_with_options(
        fixed_el,
        UseDraggableOptions::default()
            .initial_value(Position {
                x: inner_width / 2.2,
                y: 100.0,
            })
            .prevent_default(true),
    );

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <div
            node_ref=fixed_el
            class="fixed px-4 py-2 border border-gray-400/30 rounded shadow hover:shadow-lg bg-[--bg] select-none cursor-move z-30"
            // class="card card-slot"
            style=move || format!("touch-action: none; {}", fixed_style())
            >
                "Fixed 👋 Drag me!"
                <div class="text-sm opacity-50">I am {move || fixed_x().round()} , {move || fixed_y().round()}</div>
        </div>
    }
}
