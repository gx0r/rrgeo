extern crate kdtree;
extern crate csv;
extern crate rustc_serialize;

use kdtree::KdTree;

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
        let mut records = Vec::new();

        let mut rdr = csv::Reader::from_file("cities.csv").unwrap();

        for record in rdr.decode() {
            let r: Record = record.unwrap();
            records.push(([r.lat, r.lon], r));
        }

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


/* old code









extern crate test;
extern crate kdtree;
extern crate csv;
extern crate rustc_serialize;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use kdtree::KdTree;
// use kdtree::ErrorKind;

#[derive(Clone, RustcDecodable)]
struct Record {
    lat: f64,
    lon: f64,
    name: String,
    admin1: String,
    admin2: String,
    admin3: String,
}

fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            std::intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}

pub struct ReverseGeocoder<'a> {
    tree: KdTree<'a, Box<Record>>,
    // rows,
    // coords: Vec<[f64; 2]>,
    // records: Vec<Record>,
}

impl<'a> ReverseGeocoder<'a> {
    fn new() -> ReverseGeocoder<'a> {
        let mut rg = ReverseGeocoder::<'a> {
            tree: KdTree::new(2),
            // coords: Vec::new(),
            // records: Vec::new(),
        };

        let mut rdr = csv::Reader::from_file("cities.csv").unwrap();

        let rows = rdr.decode().collect::<csv::Result<Vec<Record>>>().unwrap();
        print_type_of(&rows);

        // let coords = Vec::new();
        //
        // for record in rdr.decode() {
        //     let r: Record = record.unwrap();
        //     coords.push([r.lat, r.lon]);
        // }
        //
        // let coords: [ [f64; 2]; coords.len()] = [];

        // let coords2 = rg.coords.as_slice();

        // rg.tree.add(rg.coords.get(rg.coords.len() - 1).unwrap(), Box::new(r));
            // self.records.push(r);
            // self.tree.add(&self.coords[i], Box::new(r));

        rg
    }

    // fn initialize(&'a mut self) -> &'a ReverseGeocoder {
    //     let mut rdr = csv::Reader::from_file("cities.csv").unwrap();
    //
    //     for record in rdr.decode() {
    //         let r: Record = record.unwrap();
    //         // self.tree.add(&[r.lat, r.lon], Box::new(r));
    //         self.coords.push([r.lat, r.lon]);
    //         self.records.push(r);
    //         // self.tree.add(&self.coords[i], Box::new(r));
    //
    //     }
    //     //
    //     // for i in 0..self.coords.len() {
    //     //     self.tree.add(&self.coords[i], Box::new(self.records[i]));
    //     // }
    //     //
    //     // println!("Loading complete.");
    //
    //     self
    // }

    // fn search(&self, loc: &[f64; 2]) -> Option<Record> {
    //     use kdtree::distance::squared_euclidean;
    //
    //     let y = self.tree.nearest(loc, 1, &squared_euclidean).unwrap();
    //
    //     if y.len() > 0 {
    //         return Some((*y[0].1).clone());
    //     } else {
    //         return None;
    //     }
    // }

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

fn main() {
    // let coder: Rc<RefCell<ReverseGeocoder>> = Rc::new(RefCell::new(ReverseGeocoder::new()));
    let coder = ReverseGeocoder::new();
    // coder.borrow_mut().initialize();

    // let mut coder = ReverseGeocoder::new();
    // // let i = &mut coder;
    // coder.initialize();
    // let y = coder.search(&[44.962786, -93.344722]).unwrap();
    //
    // print_record(&y);

}


#[cfg(test)]
extern crate test;

mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
    }

    #[bench]
    fn bench_lookup(b: &mut Bencher) {
    }
}


*/
