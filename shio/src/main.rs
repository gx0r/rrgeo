#![feature(rustc_private)]

#[macro_use]
extern crate lazy_static;
extern crate shio;
extern crate rustc_serialize;
extern crate time;
extern crate queryst;
extern crate reversegeocoder;

use reversegeocoder::{
    Locations,
    ReverseGeocoder,
};
use self::rustc_serialize::json;
use time::PreciseTime;
use queryst::parse;
use shio::prelude::*;
use shio::response;

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

fn main() {
    Shio::default()
        .route((Method::Get, "/", geocoder))
        .run(":3000").unwrap();
}

fn geocoder(ctx: Context) -> Response {
    match ctx.uri().query() {
        None => response::Builder::new().status(StatusCode::BadRequest).body("Missing lat/long query").into(),
        Some(query) => {
            let data = match parse(&query) {
                Err(_) => return response::Builder::new().status(StatusCode::BadRequest).body("Bad querystring").into(),
                Ok(t) => t,
            };

            let obj = match data.as_object() {
                None => return response::Builder::new().status(StatusCode::BadRequest).body("No data").into(),
                Some(t) => t,
            };

            let lat = match obj.get("lat") {
                None => return response::Builder::new().status(StatusCode::BadRequest).body("Missing \"lat\" parameter").into(),
                Some(t) => {
                    if let Some(t) = t.as_str() {
                        if let Ok(t) = t.parse::<f64>() {
                            t
                        } else {
                            return response::Builder::new().status(StatusCode::BadRequest).body("lat didn't parse").into()
                        }
                    } else {
                        return response::Builder::new().status(StatusCode::BadRequest).body("lat wasn't string").into()
                    }
                }
            };
            
            let long = match obj.get("long") {
                None => return response::Builder::new().status(StatusCode::BadRequest).body("Missing \"long\" parameter").into(),
                Some(t) => {
                    if let Some(t) = t.as_str() {
                        if let Ok(t) = t.parse::<f64>() {
                            t
                        } else {
                            return response::Builder::new().status(StatusCode::BadRequest).body("long didn't parse").into()
                        }
                    } else {
                        return response::Builder::new().status(StatusCode::BadRequest).body("long wasn't string").into()
                    }
                }
            };

            // let start = PreciseTime::now();
            let y = match GEOCODER.search(&[lat, long]) {
                None => return response::Builder::new().status(StatusCode::InternalServerError).body("Search failure").into(),
                Some(t) => t,
            };
            // let end = PreciseTime::now();
            // println!("{} ms to search", start.to(end).num_milliseconds());

            response::Builder::new().body( match json::encode(y) {
                Err(_) => return response::Builder::new().status(StatusCode::InternalServerError).body("JSON encode failure").into(),
                Ok(t) => t,
            })
        },
    }
}
