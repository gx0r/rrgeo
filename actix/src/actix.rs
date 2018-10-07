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

use actix_web::{error, http, server, App, HttpResponse, Json, Query, Result};

use reverse_geocoder::{
    Locations,
    Record,
    ReverseGeocoder,
};

#[derive(Fail, Debug)]
enum MyError {
    #[fail(display = "bad request")]
    BadClientData,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::BadClientData => HttpResponse::new(http::StatusCode::BAD_REQUEST),
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

fn index(lat_long: Query<LatLong>) -> Result<Json<Record>, MyError> {
    let res = GEOCODER.search(&[lat_long.lat, lat_long.long]);

    match res {
        Ok(res) => Ok(Json((**((*res.get(0).unwrap()).1)).clone())),
        Err(_e) => Err(MyError::BadClientData),
    }
}

fn create_app() -> App {
    App::new()
        .route("/", http::Method::GET, index)
}


fn main() {
    server::new(|| create_app())
        .bind("127.0.0.1:3000")
        .expect("Can not bind to 127.0.0.1:3000")
        .run();
}


#[cfg(test)]
mod tests {
    extern crate bytes;
    use self::bytes::Bytes;

    use actix_web::{http, HttpMessage};
    use super::create_app;

    #[test]
    fn it_serves_results_on_actix() {
        use actix_web::test::TestServer;

        let mut srv = TestServer::with_factory(create_app);

        let request = srv.client(http::Method::GET, "/?lat=44.962786&long=-93.344722").finish().unwrap();
        let response = srv.execute(request.send()).unwrap();
        assert!(response.status().is_success());

        let bytes = srv.execute(response.body()).unwrap();
        assert_eq!(bytes, Bytes::from_static(b"{\"lat\":44.9483,\"lon\":-93.34801,\"name\":\"Saint Louis Park\",\"admin1\":\"Minnesota\",\"admin2\":\"Hennepin County\",\"admin3\":\"US\"}"));
    }
}
