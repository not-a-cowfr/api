use axum::Router;
use axum_folder_router::folder_router;
use tracing::info;

#[derive(Clone, Debug)]
struct AppState {
	_foo: String,
}

#[folder_router("src/endpoints/", AppState)]
struct MyFolderRouter();

pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
	let app_state = AppState {
		_foo: String::new(),
	};

	let folder_router: Router<AppState> = MyFolderRouter::into_router();

	let app: Router<()> = folder_router.with_state(app_state);

	let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await?;
	info!("Listening on http://{}", listener.local_addr()?);
	axum::serve(listener, app).await?;
	Ok(())
}
