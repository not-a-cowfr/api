use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;

use crate::START_TIME;

#[derive(Serialize)]
struct Resp {
	success: bool,
	uptime: String,
}

pub async fn get() -> impl IntoResponse {
	let secs = START_TIME.get().unwrap().elapsed().as_secs();
	let resp = Resp {
		success: true,
		uptime: format!("{}h {}m {}s", secs / 3600, (secs % 3600) / 60, secs % 60),
	};

	Json(resp)
}
