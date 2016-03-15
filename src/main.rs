#![feature(test)]
extern crate kdtree;
extern crate csv;
extern crate rustc_serialize;
extern crate time;

use kdtree::KdTree;
use time::PreciseTime;

#[derive(Clone, RustcDecodable)]
struct Record {
    lat: f64,
    lon: f64,
    name: String,
    admin1: String,
    admin2: String,
    admin3: String,
}

pub struct Locations {
    records: Vec<([f64; 2], Record)>,
}

impl Locations {
    fn from_file() -> Locations {
        let start = PreciseTime::now();
        let mut records = Vec::new();

        let mut rdr = csv::Reader::from_file("cities.csv").unwrap();

        for record in rdr.decode() {
            let r: Record = record.unwrap();
            records.push(([r.lat, r.lon], r));
        }

        let end = PreciseTime::now();

        println!("{} seconds to load cities.csv", start.to(end));

        Locations {
            records: records,
        }
    }
}

pub struct ReverseGeocoder<'a> {
    tree: KdTree<'a, &'a Record>,
}

impl<'a> ReverseGeocoder<'a> {
    fn new(loc: &'a Locations) -> ReverseGeocoder<'a> {
        let mut r = ReverseGeocoder::<'a> {
            tree: KdTree::new(2),
        };
        r.initialize(loc);
        r
    }

    fn initialize(&mut self, loc: &'a Locations) {
        for record in &loc.records {
            self.tree.add(&record.0, &record.1).unwrap();
        }
        println!("Loading complete.");
    }

    fn search(&self, loc: &[f64; 2]) -> Option<Record> {
        use kdtree::distance::squared_euclidean;

        let y = self.tree.nearest(loc, 1, &squared_euclidean).unwrap();

        if y.len() > 0 {
            return Some((*y[0].1).clone());
        } else {
            return None;
        }
    }

}

fn print_record(r: &Record) {
    println!("({}, {}): {} {} {} {}", r.lat, r.lon, r.name, r.admin1, r.admin2, r.admin3);
}

fn main() {
    let loc = Locations::from_file();
    let geocoder = ReverseGeocoder::new(&loc);

    let y = geocoder.search(&[44.962786, -93.344722]).unwrap();

    print_record(&y);
}

extern crate test;

mod tests {

    #[test]
    fn it_works() {
        let loc = super::Locations::from_file();

        let geocoder = super::ReverseGeocoder::new(&loc);

        let y = geocoder.search(&[44.962786, -93.344722]);
        assert_eq!(y.is_some(), true);
        let x = y.unwrap();

    }

    // #[bench]
    // fn bench_lookup(b: &mut Bencher) {
    //
    // }
}


/*
fn geodetic_in_ecef(geo_coords: (f32, f32)) -> (f32, f32, f32) {
    let a = 6378.137; // major axis in kms
    let e2 = 0.00669437999014;

    let lat = geo_coords.0;
    let lon = geo_coords.1;

    let lat_r = lat.to_radians();
    let lon_r = lon.to_radians();
    let normal = a / (1f32 - e2 * lat_r.sin().powi(2));

    let x = normal * lat_r.cos() * lon_r.cos();
    let y = normal * lat_r.cos() * lon_r.sin();
    let z = normal * (1f32 - e2) * lat.sin();

    (x, y, z)
}
*/
