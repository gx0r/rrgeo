use lazy_static::lazy_static;
use reverse_geocoder::ReverseGeocoder;
use serde_derive::Deserialize;
use warp::Filter;

#[derive(Deserialize)]
struct LatLong {
    lat: f64,
    long: f64,
}

lazy_static! {
    static ref GEOCODER: ReverseGeocoder = ReverseGeocoder::new();
}

#[tokio::main]
async fn main() {
    let hello = warp::any()
        .and(warp::query::<LatLong>())
        .map(|lat_long: LatLong| {
            warp::reply::json(&GEOCODER.search((lat_long.lat, lat_long.long)))
        });

    warp::serve(hello).run(([127, 0, 0, 1], 3000)).await;
}
