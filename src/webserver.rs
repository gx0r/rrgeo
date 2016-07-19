#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(custom_derive, plugin, custom_attribute, type_macros)]
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

fn main() {
    fn hello_world(request: &mut Request) -> IronResult<Response> {
        match request.url.query().clone() {
            Some(query) => {
                println!("{:?}", query);
                let data = parse(&query).unwrap();
                println!("{:?}", data);
                println!("{:?}", data.is_object());

                let obj = data.as_object().unwrap();
                let lat = obj.get("lat").unwrap().as_string().unwrap().parse::<f64>().unwrap();
                let long = obj.get("long").unwrap().as_string().unwrap().parse::<f64>().unwrap();

                let loc = Locations::from_file();
                let geocoder = ReverseGeocoder::new(&loc);

                let y = geocoder.search(&[lat, long]).unwrap();
                Ok(Response::with((status::Ok, json::encode(y).unwrap())))
            }
            None => Ok(Response::with((status::BadRequest, "Need a lat/long"))),
        }
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
