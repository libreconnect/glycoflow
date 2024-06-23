use std::sync::Arc;

use ::lapin::auth::Credentials;
use actix_web::{web::Data, App, HttpServer};
use clap::Parser;
use database::Database;
use dotenv::dotenv;
use env::Env;
use health::{health_controller, health_service::HealthService};
use lapin::LapinClient;
use tracing::info;

mod database;
mod env;
mod health;
mod lapin;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let env = Env::parse();

    let addr_in = format!("{}:{}", env.host, env.port);

    let creds = Credentials::new(env.rabbitmq_user, env.rabbitmq_password);
    let _lapin = LapinClient::new(env.rabbitmq_url, env.rabbitmq_port, creds).await?;
    let database = Database::new(
        env.postgres_user,
        env.postgres_password,
        env.postgres_url,
        env.postgres_port,
        env.postgres_db,
    )
    .await;

    let pool = Arc::new(database.pool);
    let health_service = Arc::new(HealthService::new(pool.clone()));

    info!("Starting server at: {}", addr_in);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(health_service.clone()))
            .service(health_controller::live)
            .service(health_controller::readiness)
    })
    .bind(addr_in)?
    .run()
    .await?;

    Ok(())
}
