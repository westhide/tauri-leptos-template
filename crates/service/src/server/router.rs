use axum::{
    extract::FromRef,
    http::StatusCode,
    routing::{Router, get},
};
use leptos::prelude::*;
use leptos_axum::{
    ErrorHandler, LeptosRoutes,
    generate_route_list_with_exclusions_and_ssg_and_context as generate_route_list,
    site_pkg_dir_service, site_pkg_dir_service_route_path,
};

use crate::{
    config::Config, routes::version::version, server::context::Context, shared::error::Result,
};

pub async fn router<F, IV, S, S2>(ctx: Context<S>, shell: F) -> Result<Router<S2>>
where
    S: Clone + Send + Sync + 'static,
    F: Fn(LeptosOptions) -> IV + 'static + Clone + Sync + Send,
    IV: IntoView + 'static,
    Config: FromRef<S>,
    LeptosOptions: FromRef<Context<S>>,
{
    let options = LeptosOptions::from_ref(&ctx);
    let ctx_hook = ctx.provide_context_hook();

    let render_fn = {
        let shell = shell.clone();
        let options = options.clone();
        move || shell(options.clone())
    };

    let routes = { generate_route_list(render_fn.clone(), None, ctx_hook.clone()).0 };

    let serve_dir = site_pkg_dir_service(&options);
    let pkg_dir_route = site_pkg_dir_service_route_path(&options);

    let fallback = ErrorHandler::new_with_context(ctx_hook.clone(), shell, options);

    let router = Router::new()
        .route("/api/health", get(StatusCode::OK))
        .route("/api/version", get(version))
        .leptos_routes_with_context(&ctx, routes, ctx_hook, render_fn)
        .route_service("/assets/{*path}", serve_dir.clone())
        .route_service(&pkg_dir_route, serve_dir)
        .fallback_service(fallback)
        .with_state(ctx);

    Ok(router)
}
