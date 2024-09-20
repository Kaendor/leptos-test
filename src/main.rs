use leptos::*;

mod components;

use components::App;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
