pub mod error;
pub mod failure;
pub mod fallback;
pub mod home;
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
    error::ErrorPage, failure::Failure, fallback::Fallback, home::Home, pingpong::PingPong,
    version::Version,
};

#[component]
pub fn HydrationHook() -> impl IntoView {
    #[cfg(client)]
    Suspend::new(hydrate_hook())
}

#[component]
pub fn Main() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <nav class="nav">
                <A href="/">"Home"</A>
                <A href="/version">"Version"</A>
                <A href="/pingpong">"PingPong"</A>
            </nav>
            <ErrorBoundary fallback=|errors| {
                view! { <Failure errors=errors /> }
            }>
                <main class="container">
                    <Routes fallback=Fallback>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/version") view=Version />
                        <Route path=path!("/pingpong") view=PingPong />
                        <Route path=path!("/error") view=ErrorPage />
                        <Route
                            path=path!("/fallback")
                            view=Fallback
                            ssr=SsrMode::Static(StaticRoute::default())
                        />
                    </Routes>
                </main>
            </ErrorBoundary>
        </Router>
        <HydrationHook />
    }
}
