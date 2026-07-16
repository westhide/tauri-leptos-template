pub mod client;
pub mod error;
#[cfg(client)]
pub mod invoke;
pub mod logger;

pub type Null = ();
pub const NULL: Null = ();
