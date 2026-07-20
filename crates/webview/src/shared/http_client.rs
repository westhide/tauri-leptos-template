#[cfg(client)]
use gloo_net::http::Request;

use crate::models::namespace::{
    Response,
    register::{RegisterData, RegisterParams},
};

#[cfg(client)]
const BASE_URL: &str = "http://192.168.100.64:8102";
#[cfg(client)]
const TENANT_ID: &str = "172";

#[derive(Debug, Clone)]
pub struct HttpClient;

impl HttpClient {
    /// POST /admin-api/system/auth/register
    #[cfg(client)]
    pub async fn register(params: &RegisterParams) -> Result<Response<RegisterData>, String> {
        let url = format!("{BASE_URL}/admin-api/system/auth/register");

        let resp = Request::post(&url)
            .header("tenant-id", TENANT_ID)
            .header("Content-Type", "application/json")
            .json(params)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        resp.json::<Response<RegisterData>>().await.map_err(|e| e.to_string())
    }
}
