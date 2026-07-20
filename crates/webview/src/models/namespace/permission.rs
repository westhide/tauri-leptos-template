use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthPermissionInfo {
    pub menus: Vec<Menu>,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub always_show: Option<bool>,
    pub component: Option<String>,
    pub component_name: Option<String>,
    pub icon: Option<String>,
    pub id: i64,
    pub keep_alive: bool,
    pub name: String,
    pub parent_id: i64,
    pub path: Option<String>,
    pub visible: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub avatar: String,
    pub dept_id: i64,
    pub email: Option<String>,
    pub id: i64,
    pub nickname: String,
    pub username: String,
}
