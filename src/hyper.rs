#[macro_use] extern crate lazy_static;
extern crate kdtree;
extern crate rustc_serialize;
extern crate time;
extern crate iron;
extern crate queryst;

extern crate hyper;
extern crate futures;

mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;

// use iron::prelude::Request;
// use iron::prelude::Response;
// use iron::prelude::IronResult;
// use iron::prelude::Iron;
// use iron::status;
use queryst::parse;
use rustc_serialize::json;
use time::PreciseTime;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

const PHRASE: &'static str = "Hello, World!";

struct ReverseGeocoderServer;

impl Service for ReverseGeocoderServer {
    // boilerplate hooking up hyper's server types
    type Request =  hyper::server::Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.

        let query = req.query();

        if !query.is_some() {
            return futures::future::ok(
                Response::new()
                    // .with_header(ContentLength(PHRASE.len() as u64))
                    .with_body("missing da query ")
            )
        }

        let data = match parse(&query.unwrap()) {
            Ok(t) => t,
            Err(e) => panic!("doh")
        };

        // println!("{:?}", data);
        // println!("{:?}", data.is_object());

        let obj = match data.as_object() {
            None => panic!("doh"),
            Some(t) => t,
        };

        let lat = match obj.get("lat") {
            None => panic!("doh"),
            Some(t) => t.as_str().expect("a").parse::<f64>().expect("b"),
        };
        
        let long = match obj.get("long") {
            None => panic!("doh"),
            Some(t) => t.as_str().expect("c").parse::<f64>().expect("d"),
        };

        let start = PreciseTime::now();
        let y = match GEOCODER.search(&[lat, long]) {
            Some(t) => t,
            None => panic!("doh"),
        };
        let end = PreciseTime::now();
        // println!("{} ms to search", start.to(end).num_milliseconds());

        // Ok(Response::with((status::Ok, json::encode(y).unwrap())))

        // let mpls = GEOCODER.search(&[44.894519, -93.308702]).unwrap();

        futures::future::ok(
            Response::new()
                // .with_header(ContentLength(PHRASE.len() as u64))
                .with_body(json::encode(y).expect("e"))
        )
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().expect("bind fail");
    let server = Http::new().bind(&addr, || Ok(ReverseGeocoderServer)).expect("server fail");

    server.run().expect("run fail");
}