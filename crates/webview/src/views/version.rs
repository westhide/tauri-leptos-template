use leptos::prelude::*;

use crate::routes::version::version;

#[component]
pub fn Version() -> impl IntoView {
    let version = LocalResource::new(version);

    view! {
        <div>
            <p>
                Version:
                {move || {
                    version
                        .map(|res| match res {
                            Ok(v) => v.clone(),
                            Err(e) => format!("Error loading version: {e}"),
                        })
                }}
            </p>
        </div>
    }
}
