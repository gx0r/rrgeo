extern crate shio;
#[macro_use] extern crate lazy_static;
extern crate kdtree;
extern crate rustc_serialize;
extern crate time;
extern crate queryst;
mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;
use rustc_serialize::json;
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

fn hello(ctx: Context) -> Response {
    Response::with(format!("Hello, {}!\n", &ctx.get::<Parameters>()["name"]))
}

fn geocoder(ctx: Context) -> Response {
    match ctx.uri().query() {
        Some(query) => {
            let data = match parse(&query) {
                Ok(t) => t,
                Err(e) => return response::Builder::new().status(StatusCode::BadRequest).body("Bad querystring").into(),
            };

            let obj = match data.as_object() {
                Some(t) => t,
                None => return response::Builder::new().status(StatusCode::BadRequest).body("No data").into(),
            };

            let lat = match obj.get("lat") {
                Some(t) => t.as_str().unwrap().parse::<f64>().unwrap(),
                None => return response::Builder::new().status(StatusCode::BadRequest).body("Missing \"lat\" parameter").into(),
            };
            
            let long = match obj.get("long") {
                Some(t) => t.as_str().unwrap().parse::<f64>().unwrap(),
                None => return response::Builder::new().status(StatusCode::BadRequest).body("Missing \"lat\" parameter").into(),
            };

            // let start = PreciseTime::now();
            let y = match GEOCODER.search(&[lat, long]) {
                Some(t) => t,
                None => return response::Builder::new().status(StatusCode::InternalServerError).body("Search failure").into(),
            };
            // let end = PreciseTime::now();
            // println!("{} ms to search", start.to(end).num_milliseconds());

            response::Builder::new().body(json::encode(y).unwrap())
        },
        None => response::Builder::new().status(StatusCode::BadRequest).body("Missing lat/long query\n").into()
    }
}
