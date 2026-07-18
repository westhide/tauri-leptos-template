pub mod error;
pub mod failure;
pub mod fallback;
pub mod home;
pub mod login;
pub mod pingpong;
pub mod version;

use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    SsrMode,
    components::{A, Route, Router, Routes},
    path,
    static_routes::StaticRoute,
};

#[cfg(client)]
use crate::bootstrap::hydrate::hydrate_hook;
use crate::views::{
    error::ErrorPage, failure::error_fallback, fallback::Fallback, home::Home, login::Login,
    pingpong::PingPong, version::Version,
};

#[component]
pub fn Main() -> impl IntoView {
    provide_meta_context();

    let static_route = SsrMode::Static(StaticRoute::new());

    view! {
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/version">"Version"</A>
                <A href="/pingpong">"PingPong"</A>
            </nav>
            <ErrorBoundary fallback=error_fallback>
                <main>
                    <Routes fallback=Fallback>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/version") view=Version />
                        <Route path=path!("/login") view=Login />
                        <Route path=path!("/pingpong") view=PingPong />
                        <Route path=path!("/error") view=ErrorPage />
                        <Route path=path!("/fallback") view=Fallback ssr=static_route />
                    </Routes>
                </main>
            </ErrorBoundary>
        </Router>
        <HydrationHook />
    }
}

#[component]
pub fn HydrationHook() -> impl IntoView {
    #[cfg(client)]
    Suspend::new(hydrate_hook())
}
