use std::sync::Arc;

use sqlx::PgPool;

pub struct HealthService {
  pool: Arc<PgPool>,
}

impl HealthService {
  pub fn new(pool: Arc<PgPool>) -> Self {
    Self { pool }
  }

  pub async fn check_db(&self) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(self.pool.as_ref()).await?;

    Ok(())
  }
}
