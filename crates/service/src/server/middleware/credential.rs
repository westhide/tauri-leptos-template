use std::{
    future::Future,
    mem::ManuallyDrop,
    ops::Deref,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use axum::{
    body::Body,
    http::{Request, Response},
};
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

use crate::{config::server::Credential as Config, server::extensions::database::Client};

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

impl From<AuthInner> for Auth {
    fn from(inner: AuthInner) -> Self {
        Self { inner: Arc::new(inner) }
    }
}

impl Auth {
    pub fn layer(db: Client, config: Config) -> AsyncRequireAuthorizationLayer<Self> {
        let inner = AuthInner { db, bypass_paths: config.bypass_paths };
        AsyncRequireAuthorizationLayer::new(inner.into())
    }
}

impl AsyncAuthorizeRequest<Body> for Auth {
    type Future = AuthFuture;
    type RequestBody = Body;
    type ResponseBody = Body;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        AuthFuture { request: ManuallyDrop::new(request) }
    }
}

// TODO: AuthFuture
#[derive(Debug)]
pub struct AuthFuture {
    // state: Auth,
    request: ManuallyDrop<Request<Body>>,
}

impl Future for AuthFuture {
    type Output = Result<Request<Body>, Response<Body>>;

    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        let request = &mut self.request;

        Poll::Ready(Ok(unsafe { ManuallyDrop::take(request) }))

        // TODO
        // let AuthInner { db, bypass_paths } = &*self.state;
        // let path = self.request.uri().path();
        // if bypass_paths.iter().any(|prefix| path.starts_with(prefix)) {
        //     return Poll::Ready(Ok(self.request);
        // }
    }
}
