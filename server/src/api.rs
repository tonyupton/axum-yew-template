#![allow(unused_imports)]

use axum::{
	Json,
	routing::{Router, get, post}, 
	response::IntoResponse, 
	http::StatusCode, 
	extract::{Path, Query}
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn build_api_router() -> Router {
	Router::new()
		.route("/hello", post(post_hello))
		.route("/hello/", get(get_hello_query))
		.route("/hello/:name", get(get_hello_name))
}

async fn post_hello() -> impl IntoResponse {
	(StatusCode::ACCEPTED, String::from("Hello, World!"))
}

async fn get_hello_query(Query(query) : Query<HelloQuery>) -> impl IntoResponse {
	let response = Json(json!({
		"message": format!("Hello, {}!", match query.name {
			Some(name) => name,
			None => String::from("World"),
		}),
	}));
	(StatusCode::OK, response)
}

async fn get_hello_name(Path(name): Path<String>) -> impl IntoResponse {
	(StatusCode::OK, format!("Hello, {name}!"))
}

#[derive(Debug, Serialize, Deserialize)]
struct HelloQuery {
	name: Option<String>,
}
