pub mod credential;

use leptos::prelude::*;
use service::{impl_from_ctx, models::database::user::User};

use crate::{shared::logger::debug, state::credential::Credential};

#[derive(Debug, Clone, Default)]
pub struct State {
    pub has_login: RwSignal<bool>,
    pub credential: RwSignal<Credential>,
}

impl State {
    pub fn login(&self, user: User) {
        debug!("user login");
        self.has_login.set(true);
        self.credential.set(user.into());
    }

    pub fn logout(&self) {
        debug!("user logout");
        self.has_login.set(false);
        self.credential.set(Credential::default());
    }
}

// Unsafe: must call provide_context()
impl_from_ctx!(State);
