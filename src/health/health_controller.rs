use std::sync::Arc;

use actix_web::{get, HttpResponse, Responder, web};

use crate::health::{
  health_service::HealthService,
  model::{Health, HealthCheckResponse},
};

#[get("/health/live")]
async fn live() -> impl Responder {
  HttpResponse::Ok().body("I'm alive!")
}

#[get("/health/readiness")]
async fn readiness(health_service: web::Data<Arc<HealthService>>) -> impl Responder {
  let db_health = match health_service.check_db().await {
    Ok(_) => Health {
      healthy: true,
      message: Some("All connections are healthy".to_string()),
    },
    Err(_) => Health {
      healthy: false,
      message: Some("Database connection failed".to_string()),
    },
  };

  let response = HealthCheckResponse {
    database: db_health.clone(),
  };

  // If db_health is healthy, return 200 OK, otherwise return 500 Internal Server Error
  if db_health.healthy {
    HttpResponse::Ok().json(response)
  } else {
    HttpResponse::InternalServerError().json(response)
  }
}
