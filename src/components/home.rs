use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let (countdown, set_countdown) = create_signal(10);

    let is_over = move || countdown.get() <= 0;

    // The show component avoid massive rerendering
    view! {
        <button
            class=("text-white", move || countdown.get() % 2 == 1)
            class="btn btn-primary m-4 text-primary-content"
            on:click=move |_| {
                set_countdown.update(|n| *n -= 1);
            }
        >
            "Click me"
        </button>
        <span class="countdown font-mono text-6xl">
            <span style=("--value", countdown)></span>
        </span>
        <Show when=move || { is_over() }>
            <p>Bravo</p>
        </Show>
    }
}
