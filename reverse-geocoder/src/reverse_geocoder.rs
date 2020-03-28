#![deny(missing_docs)]
//! A library for fast, offline reverse geocoding. The location data are from [GeoNames](http://www.geonames.org/).
//!
//! # Usage
//! First, add this to your Cargo.toml
//!
//! ```toml
//! [dependencies]
//! reverse_geocoder = "^1.0.0"
//! ```
//!
//! Next:
//!
//! ```
//! use reverse_geocoder::{Locations, ReverseGeocoder};
//!
//! fn main() {
//!     let loc = Locations::from_memory();
//!     let geocoder = ReverseGeocoder::new(&loc);
//!     let y = geocoder.search(&[45.0, 54.0]).expect("Search error.");
//!     let pair = y.get(0).expect("No results.");
//!     println!("Distance {}", pair.0);
//!     println!("Record {}", pair.1);
//! }
//!```

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

use kdtree::{distance::squared_euclidean, ErrorKind, KdTree};
// use time::Instant;
use failure::Error;
use std::fmt;
use std::path::PathBuf;

/// A parsed location.
#[derive(Debug, Clone, RustcDecodable, RustcEncodable, Serialize, Deserialize)]
pub struct Record {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Location name
    pub name: String,
    /// Administrative district 1
    pub admin1: String,
    /// Administrative district 2
    pub admin2: String,
    /// Administrative district 3
    pub admin3: String,
}

/// A set of location records.
pub struct Locations {
    records: Vec<([f64; 2], Record)>,
}

impl Locations {
    /// Use the built-in cities.csv.
    pub fn from_memory() -> Locations {
        let mut records = Vec::new();
        let my_str = include_str!("../cities.csv");
        let reader = quick_csv::Csv::from_string(my_str).has_header(true);

        for read_record in reader {
            let record: Record = read_record.unwrap().decode().unwrap();
            records.push(([record.lat, record.lon], record));
        }
        Locations { records: records }
    }

    /// Supply your own path to a CSV file.
    pub fn from_path(path: Option<PathBuf>) -> Result<Locations, Error> {
        // let start = Instant::now();
        let mut records = Vec::new();

        let path = match path {
            Some(path) => path,
            None => PathBuf::from("cities.csv"),
        };

        let reader = quick_csv::Csv::from_file(path)?.has_header(true);

        for read_record in reader {
            let record: Record = read_record?.decode()?;
            records.push(([record.lat, record.lon], record));
        }

        // let end = Instant::now();

        // println!("{} ms to load cities.csv", (end - start).whole_milliseconds());

        Ok(Locations { records: records })
    }
}

/// A reverse geocoder.
pub struct ReverseGeocoder<'a> {
    tree: KdTree<f64, &'a Record, &'a [f64; 2]>,
}

impl<'a> ReverseGeocoder<'a> {
    /// Create a new reverse geocoder from a set of locations.
    pub fn new(loc: &'a Locations) -> ReverseGeocoder<'a> {
        let mut reverse_geocoder = ReverseGeocoder::<'a> {
            tree: KdTree::with_capacity(2, loc.records.len()),
        };
        reverse_geocoder.initialize(loc);
        reverse_geocoder
    }

    fn initialize(&mut self, loc: &'a Locations) {
        // let start = Instant::now();
        for record in &loc.records {
            self.tree.add(&record.0, &record.1).unwrap();
        }
        // let end = Instant::now();
        // println!("{} ms to build the KdTree", (end - start).whole_milliseconds());
    }

    /// Search for the closest record to a given lat/long. Returns Result<Vec<(distance, record)>>.
    pub fn search(&self, loc: &[f64; 2]) -> Result<Vec<(f64, &&Record)>, ErrorKind> {
        self.tree.nearest(loc, 1, &squared_euclidean)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}): {}, {}, {}, {}",
            self.lat, self.lon, self.name, self.admin1, self.admin2, self.admin3
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use failure::Error;

    #[test]
    fn it_finds_3_places() {
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

    #[test]
    fn locations_from_path() -> Result<(), Error> {
        let loc = Locations::from_path(Some("./cities.csv".into()))?;
        ReverseGeocoder::new(&loc);

        Ok(())
    }
}
