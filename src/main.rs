use std::fmt::Display;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use tokio::time::Instant;

mod server;

pub static START_TIME: OnceCell<Instant> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// dotenv::dotenv().ok();

	START_TIME.set(Instant::now()).unwrap();

	server::server().await?;
	Ok(())
}

pub trait RespError {
	fn resp_error(self) -> Response;
}

impl<T, E> RespError for Result<T, E>
where
	T: IntoResponse,
	E: Display,
{
	fn resp_error(self) -> Response {
		match self {
			| Ok(ok) => ok.into_response(),
			| Err(err) => _error_to_response(err),
		}
	}
}

fn _error_to_response<E: Display>(error: E) -> Response {
	let resp = ErrorResponse {
		_success: false,
		msg: error.to_string(),
	};

	(StatusCode::INTERNAL_SERVER_ERROR, Json(resp)).into_response()
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
	/// for consistency, always false
	pub _success: bool,
	/// Error message
	pub msg: String,
}
