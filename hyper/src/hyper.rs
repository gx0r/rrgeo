#![feature(rustc_private)]

#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate time;
extern crate queryst;
extern crate hyper;
extern crate futures;
extern crate reverse_geocoder;

use reverse_geocoder::{
    Locations,
    ReverseGeocoder,
};
use queryst::parse;
use rustc_serialize::json;
use time::PreciseTime;
use hyper::server::{Http, Request, Response, Service};

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

struct ReverseGeocoderServer;

impl Service for ReverseGeocoderServer {
    type Request =  hyper::server::Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let query = req.query();

        if !query.is_some() {
            return futures::future::ok(
                Response::new()
                    .with_body("missing  query ")
            )
        }

        let data = match parse(&query.unwrap()) {
            Err(_) => panic!("couldn't parse"),
            Ok(t) => t,
        };

        let obj = match data.as_object() {
            None => panic!("not object"),
            Some(t) => t,
        };

        let lat = match obj.get("lat") {
            None => panic!("missing lat"),
            Some(t) => t.as_str().expect("a").parse::<f64>().expect("b"),
        };
        
        let long = match obj.get("long") {
            None => panic!("missing long"),
            Some(t) => t.as_str().expect("c").parse::<f64>().expect("d"),
        };

        // let start = PreciseTime::now();
        let y = match GEOCODER.search(&[lat, long]) {
            None => panic!("couldn't find"),
            Some(t) => t,
        };
        // let end = PreciseTime::now();

        futures::future::ok(
            Response::new()
                .with_body(json::encode(y).expect("e"))
        )
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().expect("bind fail");
    let server = Http::new().bind(&addr, || Ok(ReverseGeocoderServer)).expect("server fail");

    server.run().expect("run fail");
}