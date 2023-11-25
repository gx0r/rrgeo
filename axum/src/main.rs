use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use lazy_static::lazy_static;
use reverse_geocoder::ReverseGeocoder;
use serde::Deserialize;
use std::net::SocketAddr;

lazy_static! {
    static ref GEOCODER: ReverseGeocoder = ReverseGeocoder::new();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(query));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct LatLong {
    lat: f64,
    long: f64,
}

async fn query(Query(params): Query<LatLong>) -> impl IntoResponse {
    let loc = GEOCODER.search((params.lat, params.long));
    (StatusCode::OK, Json(loc))
}
