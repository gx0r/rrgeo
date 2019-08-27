extern crate rustc_serialize;
extern crate time;
extern crate quick_csv;
extern crate kdtree;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

use self::kdtree::{
    KdTree,
    ErrorKind,
    distance::squared_euclidean,
};
use time::PreciseTime;
use std::path::PathBuf;

use failure::Error;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable, Serialize, Deserialize)]
pub struct Record {
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub admin1: String,
    pub admin2: String,
    pub admin3: String,
}

pub struct Locations {
    records: Vec<([f64; 2], Record)>,
}

impl Locations {
    pub fn from_memory() -> Locations {
        let mut records = Vec::new();
        let my_str = include_str!("../../cities.csv");
        let reader = quick_csv::Csv::from_string(my_str).has_header(true);

        for read_record in reader {
            let record: Record = read_record.unwrap().decode().unwrap();
            records.push(([record.lat, record.lon], record));
        }
        Locations { records: records }
    }

    pub fn from_path(path: Option<PathBuf>) -> Result<Locations, Error> {
        let start = PreciseTime::now();
        let mut records = Vec::new();

        let path = match path {
            Some(path) => path,
            None => PathBuf::from("cities.csv"),
        };

        let reader = quick_csv::Csv::from_file(path).unwrap().has_header(true);

        for read_record in reader {
            let record: Record = read_record?.decode()?;
            records.push(([record.lat, record.lon], record));
        }

        let end = PreciseTime::now();

        println!("{} ms to load cities.csv", start.to(end).num_milliseconds());

        Ok(Locations { records: records })
    }
}

pub struct ReverseGeocoder<'a> {
    tree: KdTree<f64, &'a Record, &'a [f64; 2]>,
}

impl<'a> ReverseGeocoder<'a> {
    pub fn new(loc: &'a Locations) -> ReverseGeocoder<'a> {
        let mut reverse_geocoder =
            ReverseGeocoder::<'a> { tree: KdTree::new_with_capacity(2, loc.records.len()) };
        reverse_geocoder.initialize(loc);
        reverse_geocoder
    }

    fn initialize(&mut self, loc: &'a Locations) {
        let start = PreciseTime::now();
        for record in &loc.records {
            self.tree.add(&record.0, &record.1).unwrap();
        }
        let end = PreciseTime::now();
        println!("{} ms to build the KdTree", start.to(end).num_milliseconds());
    }

    pub fn search(&self, loc: &[f64; 2]) -> Result<Vec<(f64, &&Record)>, ErrorKind> {
        self.tree.nearest(loc, 1, &squared_euclidean)
    }
}

#[allow(dead_code)]
pub fn print_record(record: &Record) {
    println!("({}, {}): {}, {}, {}, {}",
             record.lat,
             record.lon,
             record.name,
             record.admin1,
             record.admin2,
             record.admin3);
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn it_finds_3_places() {
        use super::*;
        let loc = Locations::from_memory();
        let geocoder = ReverseGeocoder::new(&loc);
        let y = geocoder.search(&[44.962786, -93.344722]);
        assert_eq!(y.is_ok(), true);
        let slp = y.unwrap();

        assert_eq!(slp.get(0).unwrap().1.name, "Saint Louis Park");

        // [44.894519, -93.308702] is 60 St W @ Penn Ave S, Minneapolis, Minnesota; however, this is physically closer to Richfield
        let mpls = geocoder.search(&[44.894519, -93.308702]).unwrap();
        assert_eq!(mpls.get(0).unwrap().1.name, "Richfield");

        // [44.887055, -93.334204] is HWY 62 and Valley View Road, whish is in Edina
        let edina = geocoder.search(&[44.887055, -93.334204]).unwrap();
        assert_eq!(edina.get(0).unwrap().1.name, "Edina");
    }
}
