use axum::http::HeaderName;
use reqwest::{Client, Request, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{
    config::server::SaasPlatform as Config,
    models::namespace::{
        GetNamespaceData, GetNamespaceParams,
        login::{LoginData, LoginParams},
        register::{RegisterData, RegisterParams},
    },
    shared::error::{Result, err},
};

#[derive(Debug, Clone)]
pub struct SaasPlatform {
    ns: String,
    client: Client,
    base_url: String,
}

pub const NS_HEADER: HeaderName = HeaderName::from_static("tenant-id");

impl SaasPlatform {
    pub fn new(config: &Config) -> Self {
        Self {
            ns: config.namespace.clone(),
            client: Client::new(),
            base_url: config.base_url.clone(),
        }
    }

    pub async fn fetch<R>(&self, req: Request) -> Result<R>
    where
        R: for<'de> Deserialize<'de>,
    {
        let res = self.client.execute(req).await?;
        if res.status() != StatusCode::OK {
            return err!("request failed, status={}", res.status())
        }

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Response<T> {
            pub code: i32,
            pub data: Option<T>,
            pub msg: String,
        }

        let ret: Response<R> = res.json().await?;
        if ret.code != 0 {
            return err!("request fail: {}, code={}", ret.msg, ret.code);
        }

        match ret.data {
            Some(data) => Ok(data),
            None => err!("fetch return None"),
        }
    }

    pub fn endpoint(&self, path: &str) -> String {
        format!("{}{path}", &self.base_url)
    }

    pub async fn get_namespace(&self, params: &GetNamespaceParams) -> Result<GetNamespaceData> {
        let url = self.endpoint("/admin-api/system/tenant/get-by-website");
        let req = self.client.get(url).query(params).build()?;
        Ok(self.fetch(req).await?)
    }

    pub async fn register(&self, params: &RegisterParams) -> Result<RegisterData> {
        let url = self.endpoint("/admin-api/system/auth/register");
        let req = self.client.post(url).header(NS_HEADER, &self.ns).json(params).build()?;
        Ok(self.fetch(req).await?)
    }

    pub async fn login(&self, params: &LoginParams) -> Result<LoginData> {
        let url = self.endpoint("/admin-api/system/auth/login");
        let req = self.client.post(url).header(NS_HEADER, &self.ns).json(params).build()?;
        Ok(self.fetch(req).await?)
    }
}
