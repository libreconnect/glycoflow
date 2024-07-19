use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct Database {
  pub pool: PgPool,
}

impl Database {
  pub async fn new(
    username: String,
    password: String,
    host: String,
    port: u16,
    database: String,
  ) -> Self {
    let database_url = format!(
      "postgres://{}:{}@{}:{}/{}",
      username, password, host, port, database
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    Database { pool }
  }
}
