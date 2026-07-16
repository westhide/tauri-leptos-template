use serde::{Deserialize, Serialize};

/// Content Security Policy
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Csp {
    pub enable: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Client {
    pub csp: Csp,
}
