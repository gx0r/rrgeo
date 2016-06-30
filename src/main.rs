#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(custom_derive, plugin, custom_attribute, type_macros)]
extern crate kdtree;
extern crate rustc_serialize;
extern crate time;
mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;
// use geocoder::print_record;
//
// use std::env;
// use std::process::exit;

// fn main() {
//     let args: Vec<_> = env::args().collect();
//
//     if args.len() < 3 {
//         print!("Usage: rreverse lat long\n e.g., rreverse 44.962786 -93.344722\n\n");
//         exit(1);
//     }
//
//     // println!("{:?}", args);
//
//     let lat = args[1].parse::<f64>().unwrap();
//     let long = args[2].parse::<f64>().unwrap();
//
//     let loc = Locations::from_file();
//     let geocoder = ReverseGeocoder::new(&loc);
//
//     let y = geocoder.search(&[lat, long]).unwrap();
//
//     print_record(y);
// }

extern crate iron;
extern crate queryst;

use iron::prelude::*;
use iron::status;
use queryst::parse;
use rustc_serialize::json::{self};

fn main() {

    fn hello_world(request: &mut Request) -> IronResult<Response> {
        match request.url.query.clone() {
            Some(query) => {
                println!("{:?}", query);
                let data = parse(&query).unwrap();
                println!("{:?}", data );
                println!("{:?}", data.is_object() );

                let obj = data.as_object().unwrap();
                let lat = obj.get("lat").unwrap().as_string().unwrap().parse::<f64>().unwrap();
                let long = obj.get("long").unwrap().as_string().unwrap().parse::<f64>().unwrap();

                let loc = Locations::from_file();
                let geocoder = ReverseGeocoder::new(&loc);

                let y = geocoder.search(&[lat, long]).unwrap();
                Ok(Response::with((status::Ok, json::encode(y).unwrap())))
            },
            None => Ok(Response::with((status::BadRequest, "Need a lat/long")))
        }
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
