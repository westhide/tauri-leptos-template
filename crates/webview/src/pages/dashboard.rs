use leptos::prelude::*;

use crate::shared::logger::info;

#[component]
pub fn Dashboard() -> impl IntoView {
    info!("Dashboard");

    view! { <h1>"Dashboard"</h1> }
}
