pub mod fallback;
pub mod home;
pub mod pingpong;
pub mod version;

use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::views::{fallback::Fallback, home::Home, pingpong::PingPong, version::Version};

#[component]
pub fn Main() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <main class="container">
                <Routes fallback=Fallback>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/version") view=Version />
                    <Route path=path!("/pingpong") view=PingPong />
                </Routes>
            </main>
        </Router>
    }
}
