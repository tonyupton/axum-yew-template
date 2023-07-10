use axum::Router;
use clap::Parser as ClapParser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod api;

// Setup the command line interface with clap.
#[derive(ClapParser, Debug)]
#[clap(name = "server", about = "A server for our wasm project!")]
struct Args {
	/// set the log level
	#[clap(short = '1', long = "log", default_value = "debug")]
	log_level: String,

	/// set the listen address
	#[clap(short = 'a', long = "addr", default_value = "127.0.0.1")]
	addr: String,

	/// set the listen port
	#[clap(short = 'p', long = "port", default_value = "8080")]
	port: u16,

	/// set the directory where static files are to be found
	#[clap(long = "static-dir", default_value = "./web")]
	static_dir: String,
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	// Setup logging & RUST_LOG from args
	if std::env::var("RUST_LOG").is_err() {
		std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", args.log_level));
	}
	// enable console logging
	tracing_subscriber::fmt::init();

	let sock_addr = SocketAddr::from((
		IpAddr::from_str(args.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
		args.port,
	));

	// This will serve files in the "assets" directory and
	// its subdirectories
	let service = ServeDir::new(args.static_dir);

	let app = Router::new()
		.nest("/api", api::build_api_router())
		.fallback_service(service)
		.layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

	log::info!("listening on http://{}", sock_addr);

	match axum::Server::bind(&sock_addr)
		.serve(app.into_make_service())
		.await {
		Ok(_) => log::info!("server exited gracefully"),
		Err(e) => log::error!("server exited with error: {}", e),
	}
}