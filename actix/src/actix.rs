#![feature(rustc_private)]

extern crate reversegeocoder;
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate time;
extern crate queryst;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use actix_web::{http, server, App, Query, Responder, Json, Result, HttpRequest, HttpResponse, error};

use reversegeocoder::{
    Locations,
    ReverseGeocoder,
    Record,
};

#[macro_use] extern crate failure;

#[derive(Fail, Debug)]
enum MyError {
   #[fail(display="internal error")]
   InternalError,
   #[fail(display="bad request")]
   BadClientData,
   #[fail(display="timeout")]
   Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
       match *self {
          MyError::InternalError => HttpResponse::new(
              http::StatusCode::INTERNAL_SERVER_ERROR),
          MyError::BadClientData => HttpResponse::new(
              http::StatusCode::BAD_REQUEST),
          MyError::Timeout => HttpResponse::new(
              http::StatusCode::GATEWAY_TIMEOUT),
       }
    }
}

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

#[derive(Deserialize)]
struct LatLong {
    lat: f64,
    long: f64,
}

fn index(lat_long: Query<LatLong>) -> Result<Json<Record>, MyError> {
    let res = GEOCODER.search(&[lat_long.lat, lat_long.long]);

    match res {
        Ok(res) => {
            Ok(Json( (**((*res.get(0).unwrap()).1)).clone() ))
        },
        Error => Err(MyError::BadClientData),
    }
}

fn main() {
    server::new(
        || App::new()
        .route("/", http::Method::GET, index))
        .bind("127.0.0.1:3000").expect("Can not bind to 127.0.0.1:3000")
        .run();
}