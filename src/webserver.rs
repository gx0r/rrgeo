#![cfg_attr(feature="clippy", plugin(clippy))]
// #![feature(custom_derive, plugin, custom_attribute, type_macros)]
#[macro_use]
extern crate lazy_static;
extern crate kdtree;
extern crate rustc_serialize;
extern crate time;
extern crate iron;
extern crate queryst;
mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;
use iron::prelude::*;
use iron::status;
use queryst::parse;
use rustc_serialize::json;
use time::PreciseTime;

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

fn geocoder_middleware(request: &mut Request) -> IronResult<Response> {
    match request.url.query().clone() {
        Some(query) => {
            // println!("{:?}", query);
            let data = parse(&query).unwrap();
            // println!("{:?}", data);
            // println!("{:?}", data.is_object());

            let obj = data.as_object().unwrap();
            let lat = obj.get("lat").unwrap().as_str().unwrap().parse::<f64>().unwrap();
            let long = obj.get("long").unwrap().as_str().unwrap().parse::<f64>().unwrap();

            let start = PreciseTime::now();
            let y = GEOCODER.search(&[lat, long]).unwrap();
            let end = PreciseTime::now();
            println!("{} seconds to search", start.to(end));

            Ok(Response::with((status::Ok, json::encode(y).unwrap())))
        }
        None => Ok(Response::with((status::BadRequest, "Need a lat/long"))),
    }
}

fn main() {
    Iron::new(geocoder_middleware).http("localhost:3000").unwrap();
    println!("On 3000");
}
