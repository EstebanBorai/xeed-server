use crate::database::DbConn;
use std::sync::Arc;
use warp::Filter;

mod auth;
mod user;

pub use auth::*;
pub use user::*;

#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<user::UserService>,
    pub auth_service: Arc<auth::AuthService>,
}

pub type InjectedServices = Arc<Services>;

impl Services {
    pub fn init(db_conn: DbConn) -> Arc<Self> {
        let db_conn = Arc::new(db_conn);
        let user_service = user::UserService::new(db_conn.clone());
        let auth_service = auth::AuthService::new(db_conn.clone(), user_service.clone());

        Arc::new(Services {
            user_service: Arc::new(user_service),
            auth_service: Arc::new(auth_service),
        })
    }
}

pub fn with_service<S>(
    service: S,
) -> impl Filter<Extract = (S,), Error = std::convert::Infallible> + Clone
where
    S: Clone + std::marker::Send + std::marker::Sync,
{
    warp::any().map(move || service.clone())
}