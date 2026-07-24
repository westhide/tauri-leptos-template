use std::{
    future::Future,
    ops::Deref,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use axum::{
    body::Body,
    http::{HeaderValue, Request, Response, StatusCode, header},
};
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer as AuthLayer};

use crate::{
    config::server::Credential as Config, server::extensions::database::Client,
    shared::error::Result,
};

#[derive(Debug)]
pub struct AuthInner {
    pub db: Client,
    pub bypass_paths: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Auth {
    inner: Arc<AuthInner>,
}

impl Deref for Auth {
    type Target = AuthInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Auth {
    pub fn new(db: Client, config: Config) -> Self {
        let inner = AuthInner { db, bypass_paths: config.bypass_paths };

        Self { inner: Arc::new(inner) }
    }

    pub fn layer(db: Client, config: Config) -> AuthLayer<Self> {
        AuthLayer::new(Self::new(db, config))
    }
}

impl AsyncAuthorizeRequest<Body> for Auth {
    type Future = AuthFuture;
    type RequestBody = Body;
    type ResponseBody = Body;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        AuthFuture { inner: self.inner.clone(), request: Some(request) }
    }
}

#[derive(Debug)]
pub struct AuthFuture {
    inner: Arc<AuthInner>,
    request: Option<Request<Body>>,
}

fn redirect<B>() -> Response<B>
where
    B: Default,
{
    const LOGIN: HeaderValue = HeaderValue::from_static("/login");

    let (mut parts, data) = Response::default().into_parts();
    parts.status = StatusCode::FOUND;
    parts.headers.insert(header::LOCATION, LOGIN);
    Response::from_parts(parts, data)
}

impl Future for AuthFuture {
    type Output = Result<Request<Body>, Response<Body>>;

    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(req) = self.request.take() {
            Poll::Ready(Ok(req))
        } else {
            Poll::Ready(Err(redirect()))
        }

        // TODO
        // let AuthInner { db, bypass_paths } = &*self.state;
        // let path = self.request.uri().path();
        // if bypass_paths.iter().any(|prefix| path.starts_with(prefix)) {
        //     return Poll::Ready(Ok(self.request);
        // }
    }
}
