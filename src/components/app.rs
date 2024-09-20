use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::navbar::Navbar;

use super::about::About;
use super::home::Home;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Stylesheet id="leptos" href="/style/output.css" />
        <Navbar />
        <Router>
            <Routes>
                <Route path="" view=move || view! { <Home /> } />
                <Route path="about" view=move || view! { <About /> } />
            </Routes>
        </Router>
    }
}
