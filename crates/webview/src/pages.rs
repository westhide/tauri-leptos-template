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
    components::{Outlet, ParentRoute, Redirect, Route, Router, Routes, RoutingProgress},
    path,
    static_routes::StaticRoute,
};
use service::traits::from_ctx::FromCtx;

#[cfg(client)]
use crate::bootstrap::hydrate::hydrate_hook;
use crate::{
    bootstrap::Bootstrap,
    pages::{
        dashboard::Dashboard,
        failure::error_fallback,
        fallback::{Loading, NotFound},
        login::Login,
        pingpong::PingPong,
        register::Register,
        version::Version,
    },
    shared::consts::{HOME_PAGE, LOGIN_PAGE, MAX_ROUTING_TIME},
    state::State,
};

#[component]
pub fn HydrationHook() -> impl IntoView {
    #[cfg(client)]
    Suspend::new(hydrate_hook())
}

#[component]
pub fn Home() -> impl IntoView {
    view! { <Redirect path=HOME_PAGE /> }
}

#[component]
pub fn LoginPage() -> impl IntoView {
    view! { <Redirect path=LOGIN_PAGE /> }
}

#[component]
pub fn Pages() -> impl IntoView {
    let State { has_login, .. } = State::from_ctx();

    view! {
        <Show when=has_login fallback=LoginPage>
            <Bootstrap>
                <Outlet />
            </Bootstrap>
        </Show>
    }
}

#[component]
pub fn Main() -> impl IntoView {
    provide_meta_context();

    let state = State::default();
    provide_context(state);

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
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/login") view=Login />
                        <Route path=path!("/version") view=Version />
                        <Route path=path!("/register") view=Register />
                        <Route path=path!("/pingpong") view=PingPong />
                        <Route path=path!("/loading") view=Loading ssr=static_route.clone() />
                        <Route path=path!("/404") view=NotFound ssr=static_route />
                        // pages
                        <ParentRoute path=path!("/pages") view=Pages>
                            <Route path=path!("/dashboard") view=Dashboard />
                        </ParentRoute>
                    </Routes>
                </main>
            </Router>
            <HydrationHook />
        </ErrorBoundary>
    }
}
