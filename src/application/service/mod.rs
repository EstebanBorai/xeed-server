use std::sync::Arc;

use crate::infrastructure::database::DbPool;

mod auth;
mod file;
mod secret;
mod user;

pub use auth::*;
pub use file::*;
pub use secret::*;
pub use user::*;

#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<user::UserService>,
    pub secret_service: Arc<secret::SecretService>,
    pub auth_service: Arc<auth::AuthService>,
    pub file_service: Arc<file::FileService>,
}

impl Services {
    pub fn init(db_pool: &'static DbPool) -> Self {
        let user_service = Arc::new(user::make_user_service(db_pool));
        let secret_service = Arc::new(secret::make_secret_service(db_pool));
        let file_service = Arc::new(file::make_file_service(db_pool));
        let auth_service = Arc::new(auth::make_auth_service(secret_service.clone()));

        Self {
            user_service,
            secret_service,
            auth_service,
            file_service,
        }
    }
}
