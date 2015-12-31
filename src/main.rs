extern crate kdtree;
extern crate csv;
extern crate rustc_serialize;

use kdtree::KdTree;
use kdtree::ErrorKind;
use kdtree::distance::squared_euclidean;

#[derive(RustcDecodable)]
struct Record {
    lat: f32,
    lon: f32,
    name: String,
    admin1: String,
    admin2: String
}

fn main() {


    let mut rdr = csv::Reader::from_file("cities.csv").unwrap();
    for record in rdr.decode() {
        let r: Record = record.unwrap();
        println!("({}, {}): {}", r.lat, r.lon, r.name);
        // match record.unwrap() {
        //     None => println!("Butts!"),
        //     Some(item: Record) => {
        //         println!("{}", item)
        //     }
        // }

        // let (s1, s2, dist): (String, String, usize) = record.unwrap();
    }

    //
    // let a: ([f64; 2], usize) = ([0f64, 0f64], 0);
    // let b: ([f64; 2], usize) = ([1f64, 1f64], 1);
    // let c: ([f64; 2], usize) = ([2f64, 2f64], 2);
    // let d: ([f64; 2], usize) = ([3f64, 3f64], 3);
    //
    // let dimensions = 2;
    // let mut kdtree = KdTree::new(dimensions);
    //
    // kdtree.add(&a.0, a.1).unwrap();
    // kdtree.add(&b.0, b.1).unwrap();
    // kdtree.add(&c.0, c.1).unwrap();
    // kdtree.add(&d.0, d.1).unwrap();
    //
    // assert_eq!(kdtree.size(), 4);
    // assert_eq!(
    //     kdtree.nearest(&a.0, 0, &squared_euclidean).unwrap(),
    //     vec![]
    // );
    // assert_eq!(
    //     kdtree.nearest(&a.0, 1, &squared_euclidean).unwrap(),
    //     vec![(0f64, &0)]
    // );
    // assert_eq!(
    //     kdtree.nearest(&a.0, 2, &squared_euclidean).unwrap(),
    //     vec![(0f64, &0), (2f64, &1)]
    // );
    // assert_eq!(
    //     kdtree.nearest(&a.0, 3, &squared_euclidean).unwrap(),
    //     vec![(0f64, &0), (2f64, &1), (8f64, &2)]
    // );
    // assert_eq!(
    //     kdtree.nearest(&a.0, 4, &squared_euclidean).unwrap(),
    //     vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
    // );
    // assert_eq!(
    //     kdtree.nearest(&a.0, 5, &squared_euclidean).unwrap(),
    //     vec![(0f64, &0), (2f64, &1), (8f64, &2), (18f64, &3)]
    // );
    // assert_eq!(
    //     kdtree.nearest(&b.0, 4, &squared_euclidean).unwrap(),
    //     vec![(0f64, &1), (2f64, &0), (2f64, &2), (8f64, &3)]
    // );

}
