use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchOnce {
  pub user_id: String,
  pub start_date: Option<String>, // ISO 8601
  pub end_date: Option<String>,   // ISO 8601
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartPeriodicFetch {
  pub user_id: String,
  pub interval_seconds: u64,
  pub duration_seconds: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopFetch {
  pub user_id: String,
}
