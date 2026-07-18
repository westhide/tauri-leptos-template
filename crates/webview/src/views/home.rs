use leptos::{ev::SubmitEvent, prelude::*};

use crate::shared::logger::info;

#[component]
pub fn Home() -> impl IntoView {
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
        <>
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div>
                <a href="https://tauri.app" target="_blank">
                    <img src="/assets/tauri.svg" />
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="/assets/leptos.svg" />
                </a>
            </div>
            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <form on:submit=submit>
                <input id="greet-input" placeholder="Enter a name..." on:input=update_name />
                <button type="submit">"Greet"</button>
            </form>
            <p>Version: {version}</p>
        </>
    }
}
