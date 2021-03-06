use async_trait::async_trait;
use sqlx::postgres::Postgres;
use sqlx::Transaction;
use uuid::Uuid;

use crate::error::Result;

use super::Profile;

#[async_trait]
pub trait ProfileRepository {
    async fn create_tx<'a>(
        &'a self,
        tx: &mut Transaction<'static, Postgres>,
        user_id: &Uuid,
        email: &str,
    ) -> Result<Profile>;
    async fn find_by_user_id(&self, user_id: &Uuid) -> Result<Profile>;
}
