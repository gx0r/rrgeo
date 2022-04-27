#![deny(missing_docs)]
//! A library for fast, offline reverse geocoding. The location data are from [GeoNames](http://www.geonames.org/).
//!
//! # Usage
//! ```
//! use reverse_geocoder::{Locations, ReverseGeocoder, SearchResult};
//!
//! fn main() {
//!     let loc = Locations::from_memory();
//!     let geocoder = ReverseGeocoder::new(&loc);
//!     let coords = (40.7831, -73.9712);
//!     let search_result = geocoder.search(coords).unwrap();
//!     println!("Distance {}", search_result.distance);
//!     println!("Record {}", search_result.record);
//! }
//!```

use kiddo::{distance::squared_euclidean, KdTree, ErrorKind};
// use time::Instant;
use csv::ReaderBuilder;
use serde_derive::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::path::Path;

/// A parsed location.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    /// Country Code
    pub cc: String,
}

/// Search result from querying a lat/long.
#[derive(Debug, Serialize, Clone)]
pub struct SearchResult<'a> {
    /// Distance away from given lat/long.
    pub distance: f64,
    /// Closest place information.
    pub record: &'a Record,
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

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(my_str.as_bytes());

        for record in reader.deserialize() {
            let record: Record = record.unwrap();
            records.push(([record.lat, record.lon], record));
        }

        Locations { records }
    }

    /// Supply your own path to a CSV file.
    pub fn from_path<P: AsRef<Path>>(file_path: P) -> Result<Locations, Box<dyn error::Error>> {
        // let start_load = Instant::now();
        let mut records = Vec::new();

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;

        for record in reader.deserialize() {
            let record: Record = record?;
            records.push(([record.lat, record.lon], record));
        }

        // eprintln!("{} ms to load csv", start_load.elapsed().whole_milliseconds());
        Ok(Locations { records })
    }
}

/// A reverse geocoder.
pub struct ReverseGeocoder<'a> {
    tree: KdTree<f64, &'a Record, 2>,
}

impl<'a> ReverseGeocoder<'a> {
    /// Create a new reverse geocoder from a set of locations.
    pub fn new(loc: &'a Locations) -> ReverseGeocoder<'a> {
        let mut reverse_geocoder = ReverseGeocoder::<'a> {
            tree: KdTree::new()
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

    /// Search for the closest record to a given (latitude, longitude). Non-finite numbers will always return None.
    pub fn search(&self, loc: (f64, f64)) -> Option<SearchResult> {
        let nearest = match self.tree.nearest_one(&[loc.0, loc.1], &squared_euclidean) {
            Ok(nearest) => nearest,
            Err(error) => match error {
                ErrorKind::Empty => return None,
                ErrorKind::NonFiniteCoordinate => return None,
                ErrorKind::ZeroCapacity => {
                    panic!("Internal error, kdtree::ErrorKind::ZeroCapacity should never occur")
                }
            },
        };
        Some(SearchResult {
            distance: nearest.0,
            record: nearest.1,
        })
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}): {}, {}, {}, {}",
            self.lat, self.lon, self.name, self.admin1, self.admin2, self.cc
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_4_places() {
        let loc = Locations::from_memory();
        let geocoder = ReverseGeocoder::new(&loc);

        let slp = geocoder.search((40.7831, -73.9712)).unwrap();
        assert_eq!(slp.record.name, "Manhattan");

        let slp = geocoder.search((44.962786, -93.344722)).unwrap();

        assert_eq!(slp.record.name, "Saint Louis Park");

        // [44.894519, -93.308702] is 60 St W @ Penn Ave S, Minneapolis, Minnesota; however, this is physically closer to Richfield
        let mpls = geocoder.search((44.894519, -93.308702)).unwrap();
        assert_eq!(mpls.record.name, "Richfield");

        // [44.887055, -93.334204] is HWY 62 and Valley View Road, whish is in Edina
        let edina = geocoder.search((44.887055, -93.334204)).unwrap();
        assert_eq!(edina.record.name, "Edina");
    }

    #[test]
    fn it_loads_locations_from_a_path() -> Result<(), Box<dyn error::Error>> {
        let loc = Locations::from_path("./cities.csv")?;
        let geocoder = ReverseGeocoder::new(&loc);
        let search_result = geocoder.search((45.0, 54.0));
        assert!(search_result.is_some());
        Ok(())
    }

    #[test]
    fn it_loads_locations_from_a_nearly_blank_file() -> Result<(), Box<dyn error::Error>> {
        let loc = Locations::from_path("./nearly-blank.csv")?;
        let geocoder = ReverseGeocoder::new(&loc);
        let search_result = geocoder.search((45.0, 54.0));
        assert!(search_result.is_none());
        Ok(())
    }

    #[test]
    fn it_loads_locations_from_a_blank_file() -> Result<(), Box<dyn error::Error>> {
        let loc = Locations::from_path("./blank.csv")?;
        let geocoder = ReverseGeocoder::new(&loc);
        let search_result = geocoder.search((45.0, 54.0));
        assert!(search_result.is_none());
        Ok(())
    }

    #[test]
    fn it_returns_none_given_an_infinite_coordinate() {
        let loc = Locations::from_memory();
        let geocoder = ReverseGeocoder::new(&loc);
        let search_result = geocoder.search((std::f64::INFINITY, 54.0));
        assert!(search_result.is_none());
    }
}
