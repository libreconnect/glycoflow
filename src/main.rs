use std::sync::Arc;

use ::lapin::auth::Credentials;
use actix_web::{web::Data, App, HttpServer};
use clap::Parser;
use dotenv::dotenv;
use env::Env;
use health::{health_controller, health_service::HealthService};
use lapin::LapinClient;
use sqlx::postgres::PgPoolOptions;

mod env;
mod health;
mod lapin;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let env = Env::parse();

    let addr_in = format!("{}:{}", "localhost", 3333);

    let creds = Credentials::new(env.rabbitmq_user, env.rabbitmq_password);
    let _lapin = LapinClient::new(env.rabbitmq_url, env.rabbitmq_port, creds).await?;

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}?connect_timeout=5",
        "postgres", "postgres", "localhost", 5432, "glycoflow"
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let pool = Arc::new(pool);

    let health_service = Arc::new(HealthService::new(pool.clone()));

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
