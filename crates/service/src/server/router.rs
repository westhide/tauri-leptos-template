use axum::{
    extract::FromRef,
    http::StatusCode,
    routing::{Router, get},
};
use leptos::prelude::*;
use leptos_axum::{
    ErrorHandler, LeptosRoutes, generate_route_list, site_pkg_dir_service,
    site_pkg_dir_service_route_path,
};

use crate::{routes::version::version, server::context::Context, shared::error::Result};

pub async fn router<F, IV, S, S2>(context: Context<S>, shell: F) -> Result<Router<S2>>
where
    S: Clone + Send + Sync + 'static,
    F: Fn(LeptosOptions) -> IV + 'static + Clone + Sync + Send,
    IV: IntoView + 'static,
    LeptosOptions: FromRef<Context<S>>,
{
    let options = LeptosOptions::from_ref(&context);

    let routes = {
        let shell = shell.clone();
        let options = options.clone();
        generate_route_list(move || shell(options.clone()))
    };

    let serve_dir = site_pkg_dir_service(&options);
    let pkg_dir_route = site_pkg_dir_service_route_path(&options);

    let fallback = ErrorHandler::new(shell.clone(), options.clone());

    let router = Router::new()
        .route("/api/health", get(StatusCode::OK))
        .route("/api/version", get(version))
        .leptos_routes(&context, routes, move || shell(options.clone()))
        .route_service("/assets/{*path}", serve_dir.clone())
        .route_service(&pkg_dir_route, serve_dir)
        .fallback_service(fallback)
        .with_state(context);

    Ok(router)
}

impl<S> FromRef<Context<S>> for LeptosOptions
where
    LeptosOptions: FromRef<S>,
{
    fn from_ref(ctx: &Context<S>) -> Self {
        Self::from_ref(&ctx.state)
    }
}
