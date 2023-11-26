use actix_web::{http::KeepAlive, middleware, web, App, HttpServer, Result};
use lazy_static::lazy_static;
use reverse_geocoder::{Record, ReverseGeocoder};
use serde_derive::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
struct LatLong {
    lat: f64,
    long: f64,
}

async fn index(lat_long: web::Query<LatLong>) -> Result<web::Json<Record>> {
    lazy_static! {
        static ref GEOCODER: ReverseGeocoder = ReverseGeocoder::new();
    }

    let search_result = GEOCODER.search((lat_long.lat, lat_long.long));
    Ok(web::Json(search_result.record.clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
    })
    .keep_alive(KeepAlive::Timeout(Duration::from_secs(10)))
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate bytes;

    use actix_web::body::MessageBody;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App};

    #[actix_web::test]
    async fn it_serves_results_on_actix() {
        let app = test::init_service(App::new().route("/", web::get().to(index))).await;

        let req = test::TestRequest::get()
            .uri("/?lat=44.962786&long=-93.344722")
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body().try_into_bytes().unwrap();

        assert_eq!(
            response_body,
            r##"{"lat":44.9483,"lon":-93.34801,"name":"Saint Louis Park","admin1":"Minnesota","admin2":"Hennepin County","cc":"US"}"##
        );
    }
}
