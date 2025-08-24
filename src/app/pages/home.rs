use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{lazy_route, LazyRoute};
use crate::app::components::bevy_canvas::BevyCanvas;

pub struct HomePage;

#[lazy_route]
impl LazyRoute for HomePage {
    fn data() -> Self {
        HomePage
    }
    
    fn view(this: Self) -> AnyView {
        view! {
            <Title text="Home - Bevy Canvas Test"/>
            
            <div class="page-container">
                <h1>"Home Page with Bevy Canvas"</h1>
                <p>"This page demonstrates a Bevy canvas component loaded via a lazy route."</p>
                
                <div class="canvas-section">
                    <h2>"Interactive Bevy Canvas"</h2>
                    <BevyCanvas />
                </div>
                
                <nav class="page-navigation">
                    <a href="/about">"Go to About Page"</a>
                </nav>
            </div>
        }.into_any()
    }
}