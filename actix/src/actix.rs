#[macro_use]
extern crate failure;
extern crate reverse_geocoder;
#[macro_use]
extern crate lazy_static;
extern crate actix_web;
extern crate queryst;
extern crate serde;
extern crate time;
#[macro_use]
extern crate serde_derive;

use actix_web::{error, http, middleware, web, App, HttpResponse, HttpServer, Result};

use reverse_geocoder::{
    Locations,
    Record,
    ReverseGeocoder,
};

use failure::Error;

#[derive(Fail, Debug)]
enum MyError {
    #[fail(display = "bad request")]
    BadClientData,
    #[fail(display = "not found")]
    NotFound,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::BadClientData => HttpResponse::new(http::StatusCode::BAD_REQUEST),
            MyError::NotFound => HttpResponse::new(http::StatusCode::BAD_REQUEST),
        }
    }
}

lazy_static! {
    static ref LOCATIONS: Locations = Locations::from_memory();
    static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCATIONS);
}

#[derive(Deserialize)]
struct LatLong {
    lat: f64,
    long: f64,
}

fn index(lat_long: web::Query<LatLong>) -> Result<web::Json<Record>, Error> {
    let res = GEOCODER.search(&[lat_long.lat, lat_long.long])?;

    match res.len() {
        0 => Err(Error::from(MyError::NotFound)),
        _ => Ok(web::Json((*((res.get(0).unwrap()).1)).clone())),
    }
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(index))
        })
        .keep_alive(10)
        .bind("127.0.0.1:3000")
        .expect("Can not bind to 127.0.0.1:3000")
        .run()
        .expect("Couldn't run");
}


#[cfg(test)]
mod tests {
    extern crate bytes;
    use self::bytes::Bytes;

    use actix_web::{test, web, App};
    use super::index;

    #[test]
    fn it_serves_results_on_actix() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index)));
        let req = test::TestRequest::get().uri("/?lat=44.962786&long=-93.344722").to_request();
        let resp = test::read_response(&mut app, req);
        assert_eq!(resp, Bytes::from_static(b"{\"lat\":44.9483,\"lon\":-93.34801,\"name\":\"Saint Louis Park\",\"admin1\":\"Minnesota\",\"admin2\":\"Hennepin County\",\"admin3\":\"US\"}"));
    }
}
