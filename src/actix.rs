#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate time;
extern crate queryst;
extern crate actix_web;
use actix_web::{http, server, App, Path, Responder, HttpRequest, HttpResponse};

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
#[derive(Deserialize)]
struct LatLong {
    lat: f64,
    long: f64,
}


fn index(req: HttpRequest) -> impl Responder {

    let params = req.query();
    let lat = match params.get("lat") {
        Some(x) => x,
        None => return HttpResponse::BadRequest().body("missing lat param"),
    };

    let long = match params.get("long") {
        Some(x) => x,
        None => return HttpResponse::BadRequest().body("missing long param"),
    };

    let lat_n = lat.parse::<f64>().unwrap_or(0.0);
    let long_n = long.parse::<f64>().unwrap_or(0.0);

    let y = match GEOCODER.search(&[lat_n, long_n]) {
        None => return HttpResponse::InternalServerError().body("Couldn't match"),
        Some(t) => t,
    };

    match json::encode(y) {
        Ok(encoded) => HttpResponse::Ok()
                .content_type("application/json")
                .body(encoded),   
        Err(_) => HttpResponse::InternalServerError().body("Couldn't encode")
    }    
}


fn main() {
    server::new(
        || App::new()
        .route("/", http::Method::GET, index))
        .bind("127.0.0.1:3000").expect("Can not bind to 127.0.0.1:3000")
        .run();
}