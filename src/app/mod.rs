use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::{Router, Routes, Route}, path};

pub mod components;
pub mod pages;

#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1, maximum-scale=1"
                />
                <Stylesheet id="leptos" href="/pkg/issue-bevy-with-lazy-routes.css"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "404 - Page not found">
                <Route path=path!("/") view=pages::home::HomePage/>
                <Route path=path!("/about") view=pages::about::AboutPage/>
            </Routes>
        </Router>
    }
}