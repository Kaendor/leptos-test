use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            class:red=move || count % 2 == 1
            on:click=move |_| { set_count.update(|n| *n += 1 ); }
        >
            "Click me:" {count}
        </button>
    }
}
