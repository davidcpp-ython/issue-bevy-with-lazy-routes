use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About - Lazy Route Test"/>
        
        <div class="page-container">
            <h1>"About Page"</h1>
            <p>"This is a simple about page loaded via lazy routing."</p>
            
            <section>
                <h2>"Test Information"</h2>
                <p>"This project demonstrates an issue with Bevy canvas components when using lazy routes in Leptos 0.8.6."</p>
                <ul>
                    <li>"SSR is enabled with hydration"</li>
                    <li>"Bevy only runs on the client (target_arch = wasm32)"</li>
                    <li>"All routes are lazy-loaded with code splitting"</li>
                </ul>
            </section>
            
            <nav class="page-navigation">
                <a href="/">"← Back to Home"</a>
            </nav>
        </div>
    }
}