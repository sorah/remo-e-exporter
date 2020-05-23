use std::env;

use remo_e_exporter::nature_client;
use remo_e_exporter::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

	let token = read_token()?;
	let bind = read_bind()?;
	let cache_duration = read_cache_duration()?;

	let client = nature_client::Client::new("https://api.nature.global".to_string(), token).unwrap();
	let server = std::sync::Arc::new(server::Server::new(client, cache_duration));

	let make_service = hyper::service::make_service_fn(move |_conn| {
		let server = server.clone();
		async move {
			Ok::<_, std::convert::Infallible>(hyper::service::service_fn(move |req: hyper::Request<hyper::Body>| {
				let server = server.clone();
				async move {
					server.serve(req).await
				}
			}))
		}
	});

	if let Err(err) = hyper::Server::bind(&bind).serve(make_service).await {
		return Err(anyhow::Error::new(err));
	}

	Ok(())
}

fn read_token() -> anyhow::Result<String> {
	if let Ok(token) = env::var("OAUTH_TOKEN") {
		return Ok(token);
	}
	if let Ok(token_file) = env::var("OAUTH_TOKEN_FILE") {
		return Ok(String::from_utf8(std::fs::read(token_file)?)?);
	}
	Err(anyhow::anyhow!("Missing API token"))
}

fn read_bind() -> Result<std::net::SocketAddr, std::net::AddrParseError> {
	let bind_str = env::var("BIND").unwrap_or("[::]:9742".to_string());
	let bind: Result<std::net::SocketAddr, std::net::AddrParseError> = bind_str.parse();
	bind
}

fn read_cache_duration() -> anyhow::Result<std::time::Duration> {
	let dur_str = env::var("CACHE_INVALIDATION_SECONDS").unwrap_or("30".to_string());
	let dur_seconds: u64 = dur_str.parse()?;
	Ok(std::time::Duration::from_secs(dur_seconds))
}
