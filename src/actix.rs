#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate time;
extern crate queryst;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use actix_web::{http, server, App, Query, Responder, HttpRequest, HttpResponse};

mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;
use rustc_serialize::json;

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

#[derive(Deserialize)]
struct LatLong {
    lat: f64,
    long: f64,
}

fn index(latLong: Query<LatLong>) -> impl Responder {
    let y = match GEOCODER.search(&[latLong.lat, latLong.long]) {
        None => return HttpResponse::InternalServerError().body("Couldn't match"),
        Some(t) => t,
    };

    match json::encode(y) {
        Ok(encoded) => HttpResponse::Ok()
                .content_type("application/json")
                .body(encoded),   
        Err(_) => HttpResponse::InternalServerError().body("Couldn't encode")
    }    
}

fn main() {
    server::new(
        || App::new()
        .route("/", http::Method::GET, index))
        .bind("127.0.0.1:3000").expect("Can not bind to 127.0.0.1:3000")
        .run();
}