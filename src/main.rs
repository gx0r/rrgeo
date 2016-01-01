#![feature(float_extras)]
extern crate kdtree;
extern crate csv;
extern crate rustc_serialize;

use kdtree::KdTree;
use kdtree::ErrorKind;
use kdtree::distance::squared_euclidean;

#[derive(Clone, RustcDecodable)]
struct Record {
    lat: f64,
    lon: f64,
    name: String,
    admin1: String,
    admin2: String,
    admin3: String
}

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

fn print_record(r: &Record) {
    println!("({}, {}): {} {} {} {}", r.lat, r.lon, r.name, r.admin1, r.admin2, r.admin3);
}

fn search(my_kdtree: KdTree<&Record>, loc: &[f64; 2]) -> Option<Record> {
    let y = my_kdtree.nearest(loc, 1, &squared_euclidean).unwrap();

    if y.len() > 0 {
        return Some((*y[0].1).clone());
    } else {
        return None;
    }
}

fn main() {
    let mut coords = Vec::new();
    let mut records = Vec::new();
    let mut kdtree = KdTree::new(2);

    let mut rdr = csv::Reader::from_file("cities.csv").unwrap();
    for record in rdr.decode() {
        let r: Record = record.unwrap();
        // printRecord(&r);
        coords.push([r.lat, r.lon]);
        records.push(r);
    }

    for i in 0..coords.len() {
        kdtree.add(&coords[i], &records[i]);
    }

    println!("Loading complete.");

    // let y = kdtree.nearest(&[44.962786, -93.344722], 100, &squared_euclidean).unwrap();
    let y = search(kdtree, &[44.962786, -93.344722]).unwrap();

    print_record(&y);
}
