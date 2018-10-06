#![feature(rustc_private)]
#[macro_use]
extern crate failure;
extern crate reversegeocoder;
#[macro_use]
extern crate lazy_static;
extern crate actix_web;
extern crate queryst;
extern crate rustc_serialize;
extern crate serde;
extern crate time;
#[macro_use]
extern crate serde_derive;

use actix_web::{error, http, server, App, HttpResponse, Json, Query, Result};

use reversegeocoder::{Locations, Record, ReverseGeocoder};

#[derive(Fail, Debug)]
enum MyError {
    #[fail(display = "bad request")]
    BadClientData,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::BadClientData => HttpResponse::new(http::StatusCode::BAD_REQUEST),
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
        Ok(res) => Ok(Json((**((*res.get(0).unwrap()).1)).clone())),
        Err(_e) => Err(MyError::BadClientData),
    }
}

fn main() {
    server::new(|| App::new().route("/", http::Method::GET, index))
        .bind("127.0.0.1:3000")
        .expect("Can not bind to 127.0.0.1:3000")
        .run();
}
