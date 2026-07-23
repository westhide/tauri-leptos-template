use leptos::{ev::SubmitEvent, prelude::*};

use crate::{bootstrap::Bootstrap, shared::logger::info};

#[component]
pub fn Dashboard() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let version = RwSignal::new(String::new());

    let update_name = move |ev| {
        name.set(event_target_value(&ev));
    };

    let submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        info!("Greet: {}", name.get_untracked());
    };

    view! {
        <Bootstrap>
            <h1>"Welcome to Tauri + Leptos"</h1>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form on:submit=submit>
                <input id="greet-input" placeholder="Enter a name..." on:input=update_name />
                <button type="submit">"Greet"</button>
            </form>
            <p>Version: {version}</p>
        </Bootstrap>
    }
}
