#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate time;
extern crate iron;
extern crate queryst;
mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;
use iron::prelude::Request;
use iron::prelude::Response;
use iron::prelude::IronResult;
use iron::prelude::Iron;
use iron::status;
use queryst::parse;
use rustc_serialize::json;
use time::PreciseTime;

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

fn geocoder_middleware(request: &mut Request) -> IronResult<Response> {
    match request.url.query() {
        None => Ok(Response::with((status::BadRequest, "Need a lat/long"))),
        Some(query) => {
            // println!("{:?}", query);
            let data = match parse(&query) {
                Err(e) => return Ok(Response::with((status::BadRequest, e.message))),
                Ok(t) => t,
            };

            // println!("{:?}", data);
            // println!("{:?}", data.is_object());

            let obj = match data.as_object() {
                Some(t) => t,
                None => return Ok(Response::with((status::BadRequest, "No object"))),
            };

            let lat = match obj.get("lat") {
                Some(t) => t.as_str().unwrap().parse::<f64>().unwrap(),
                None => return Ok(Response::with((status::BadRequest, "Missing lat"))),
            };
            
            let long = match obj.get("long") {
                Some(t) => t.as_str().unwrap().parse::<f64>().unwrap(),
                None => return Ok(Response::with((status::BadRequest, "Missing long"))),
            };

            // let start = PreciseTime::now();
            let y = match GEOCODER.search(&[lat, long]) {
                None => return Ok(Response::with((status::BadRequest, "Geocoder Search Failed"))),
                Some(t) => t,
            };
            // let end = PreciseTime::now();
            // println!("{} ms to search", start.to(end).num_milliseconds());

            Ok(Response::with((status::Ok, json::encode(y).unwrap())))
        },
    }
}

fn main() {
    Iron::new(geocoder_middleware).http("localhost:3000").unwrap();
    println!("On 3000");
}
