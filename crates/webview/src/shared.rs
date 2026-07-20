pub mod client;
pub mod consts;
pub mod error;
pub mod http_client;
#[cfg(client)]
pub mod invoke;
pub mod logger;

pub type Null = ();
pub const NULL: Null = ();
