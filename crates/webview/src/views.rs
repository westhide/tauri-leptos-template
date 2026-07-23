pub mod dashboard;
pub mod failure;
pub mod fallback;
pub mod login;
pub mod pingpong;
pub mod register;
pub mod version;

use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::{
    SsrMode,
    components::{Route, Router, Routes, RoutingProgress},
    path,
    static_routes::StaticRoute,
};

#[cfg(client)]
use crate::bootstrap::hydrate::hydrate_hook;
use crate::{
    shared::consts::MAX_ROUTING_TIME,
    views::{
        dashboard::Dashboard,
        failure::error_fallback,
        fallback::{Loading, NotFound},
        login::Login,
        pingpong::PingPong,
        register::Register,
        version::Version,
    },
};

#[component]
pub fn Main() -> impl IntoView {
    provide_meta_context();

    let (is_routing, set_is_routing) = signal(false);
    let static_route = SsrMode::Static(StaticRoute::new());

    view! {
        <ErrorBoundary fallback=error_fallback>
            <Router set_is_routing>
                <div>
                    <RoutingProgress is_routing max_time=MAX_ROUTING_TIME />
                </div>
                <main class="h-full">
                    <Routes transition=true fallback=NotFound>
                        <Route path=path!("/") view=Dashboard />
                        <Route path=path!("/login") view=Login />
                        <Route path=path!("/version") view=Version />
                        <Route path=path!("/register") view=Register />
                        <Route path=path!("/pingpong") view=PingPong />
                        <Route path=path!("/loading") view=Loading ssr=static_route.clone() />
                        <Route path=path!("/404") view=NotFound ssr=static_route />
                    </Routes>
                </main>
            </Router>
            <HydrationHook />
        </ErrorBoundary>
    }
}

#[component]
pub fn HydrationHook() -> impl IntoView {
    #[cfg(client)]
    Suspend::new(hydrate_hook())
}
