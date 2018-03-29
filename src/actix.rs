#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate time;
extern crate queryst;
extern crate actix_web;
use actix_web::*;

mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;
use rustc_serialize::json;
use time::PreciseTime;
use queryst::parse;


lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_file();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

fn index(req: HttpRequest) -> String  {

    let params = req.query();
    let lat = params.get("lat").unwrap_or("0");
    let long = params.get("long").unwrap_or("0");

    let latN = lat.parse::<f64>().unwrap_or(0.0);
    let longN = long.parse::<f64>().unwrap_or(0.0);

    let y = match GEOCODER.search(&[latN, longN]) {
        None => return "Search failure".to_string(),
        Some(t) => t,
    };

    json::encode(y).unwrap_or("{}".to_string())
}


fn main() {
    HttpServer::new(
        || Application::new()
            .resource("/", |r| r.f(index)))
        .bind("127.0.0.1:3000").expect("Can not bind to 127.0.0.1:3000")
        .run();
}