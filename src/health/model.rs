use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Health {
  pub healthy: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct HealthCheckResponse {
  pub database: Health,
}
