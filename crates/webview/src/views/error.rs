use leptos::prelude::*;

use crate::shared::{
    Null,
    error::{Error, Result},
};

#[component]
pub fn ErrorPage() -> impl IntoView {
    Result::<Null>::Err(Error::Error("Error Page".into()))
}
