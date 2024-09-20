use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="navbar bg-accent">
            <div class="flex-1">
                <a class="btn btn-ghost text-xl">"Leptos Tests"</a>
            </div>
            <div class="flex-none">
                <ul class="menu menu-horizontal px-1">
                    <li>
                        <a href="/">Acceuil</a>
                    </li>
                    <li>
                        <details>
                            <summary>Contenu</summary>
                            <ul class="bg-base-100 rounded-t-none p-2">
                                <li>
                                    <a href="/about">"A propos"</a>
                                </li>
                            </ul>
                        </details>
                    </li>
                </ul>
            </div>
        </div>
    }
}
