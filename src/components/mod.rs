use leptos::*;
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <button
            class=("text-secondary", move || count.get() % 2 == 1)
            class="btn btn-primary m-4"
            on:click=move |_| { set_count.update(|n| *n += 1 ); }
        >
            "Click me:" {count}
        </button>
    }
}
