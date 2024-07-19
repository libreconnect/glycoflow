use std::sync::Arc;

use ::lapin::{auth::Credentials, Channel};
use actix_web::{App, HttpServer, web::Data};
use clap::Parser;
use dotenv::dotenv;
use sqlx::PgPool;
use tracing::info;

use database::Database;
use encryptor::Encryptor;
use env::Env;
use handler::listen_queue;
use health::{health_controller, health_service::HealthService};
use lapin::LapinClient;

mod database;
mod encryptor;
mod env;
mod handler;
mod handlers;
mod health;
mod lapin;
mod messages;

async fn spawn_listener(
  queue_name: String,
  channel: Arc<Channel>,
  pool: Arc<PgPool>,
  encryptor: Arc<Encryptor>,
) -> Result<(), Box<dyn std::error::Error>> {
  tokio::spawn(async move {
    let _ = listen_queue(&queue_name, channel.clone(), pool.clone(), encryptor).await;
  });

  Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt::init();
  dotenv().ok();
  let env = Env::parse();

  let encryptor = Encryptor::new(env.app_key)?;
  let encryptor = Arc::new(encryptor);
  let addr_in = format!("{}:{}", env.host, env.port);

  let creds = Credentials::new(env.rabbitmq_user, env.rabbitmq_password);
  let lapin = LapinClient::new(env.rabbitmq_url, env.rabbitmq_port, creds).await?;
  let database = Database::new(
    env.postgres_user,
    env.postgres_password,
    env.postgres_url,
    env.postgres_port,
    env.postgres_db,
  )
      .await;

  sqlx::migrate!("./migrations").run(&database.pool).await?;
  lapin.configure_service().await?;

  let pool = Arc::new(database.pool);
  let health_service = Arc::new(HealthService::new(pool.clone()));

  info!("Starting server at: {}", addr_in);

  spawn_listener(
    "glycoflow_register_access_token".to_string(),
    lapin.channel.clone(),
    pool.clone(),
    encryptor.clone(),
  )
      .await?;

  spawn_listener(
    "glycoflow_command_fetch_once".to_string(),
    lapin.channel.clone(),
    pool.clone(),
    encryptor.clone(),
  )
      .await?;

  // tokio::spawn(async move {
  //   let _ = listen_queue("glycoflow_register_access_token", lapin.channel.clone(), pool.clone(), encryptor).await;
  // });

  HttpServer::new(move || {
    App::new()
        .app_data(Data::new(health_service.clone()))
        .service(health_controller::live)
        .service(health_controller::readiness)
  })
      .bind(addr_in)?
      .workers(1)
      .run()
      .await?;
  Ok(())
}
