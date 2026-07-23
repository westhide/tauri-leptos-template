use leptos::prelude::*;
use service::routes::version::version;

use crate::pages::Loading;

#[component]
pub fn Version() -> impl IntoView {
    let version = LocalResource::new(version);

    view! {
        <Suspense fallback=Loading>
            <div>Version: {move || Suspend::new(version)}</div>
        </Suspense>
    }
}
