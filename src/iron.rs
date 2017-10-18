#[macro_use] extern crate lazy_static;
extern crate kdtree;
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
        Some(query) => {
            // println!("{:?}", query);
            let data = match parse(&query) {
                Ok(t) => t,
                Err(e) => return Ok(Response::with((status::BadRequest, e.message))),
            };

            // println!("{:?}", data);
            // println!("{:?}", data.is_object());

            let obj = match data.as_object() {
                None => return Ok(Response::with((status::BadRequest, "No object"))),
                Some(t) => t,
            };

            let lat = match obj.get("lat") {
                None => return Ok(Response::with((status::BadRequest, "Missing lat"))),
                Some(t) => t.as_str().unwrap().parse::<f64>().unwrap(),
            };
            
            let long = match obj.get("long") {
                None => return Ok(Response::with((status::BadRequest, "Missing long"))),
                Some(t) => t.as_str().unwrap().parse::<f64>().unwrap(),
            };

            let start = PreciseTime::now();
            let y = match GEOCODER.search(&[lat, long]) {
                Some(t) => t,
                None => return Ok(Response::with((status::BadRequest, "Geocoder Search Failed"))),
            };
            let end = PreciseTime::now();
            println!("{} ms to search", start.to(end).num_milliseconds());

            Ok(Response::with((status::Ok, json::encode(y).unwrap())))
        }
        None => Ok(Response::with((status::BadRequest, "Need a lat/long"))),
    }
}

fn main() {
    Iron::new(geocoder_middleware).http("localhost:3000").unwrap();
    println!("On 3000");
}
