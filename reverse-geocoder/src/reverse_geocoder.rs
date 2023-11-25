//! A library for fast, offline reverse geocoding. The location data are from [GeoNames](http://www.geonames.org/).
//!
//! # Usage
//! ```
//! use reverse_geocoder::{Locations, ReverseGeocoder, SearchResult};
//!
//! fn main() {
//!     let geocoder = ReverseGeocoder::new();
//!     let coords = (40.7831, -73.9712);
//!     let search_result = geocoder.search(coords).expect("Nothing found.");
//!     println!("Distance {}", search_result.distance);
//!     println!("Record {}", search_result.record);
//! }
//!```

use kiddo::float::{distance::SquaredEuclidean, kdtree::KdTree};
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

impl Record {
    pub fn as_xyz(&self) -> [f64; 3] {
        degrees_lat_lng_to_unit_sphere(self.lat, self.lon)
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

/// converts Earth surface co-ordinates in degrees of latitude and longitude to 3D cartesian coordinates on a unit sphere
pub fn degrees_lat_lng_to_unit_sphere(lat: f64, lng: f64) -> [f64; 3] {
    // convert from degrees to radians
    let lat = lat.to_radians();
    let lng = lng.to_radians();

    // convert from ra/dec to xyz coords on unit sphere
    [lat.cos() * lng.cos(), lat.cos() * lng.sin(), lat.sin()]
}

/// Search result from querying a lat/long.
#[derive(Debug, Serialize, Clone)]
pub struct SearchResult<'a> {
    /// Distance away from given lat/long.
    pub distance: f64,
    /// Closest place information.
    pub record: &'a Record,
}

/// A reverse geocoder.
pub struct ReverseGeocoder {
    locations: Vec<([f64; 2], Record)>,
    tree: KdTree<f64, usize, 3, 32, u16>,
}

impl ReverseGeocoder {
    /// Create a new reverse geocoder from the built-in cities.csv.
    pub fn new() -> ReverseGeocoder {
        let mut records = Vec::new();
        let cities = include_str!("../cities.csv");

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(cities.as_bytes());

        for record in reader.deserialize() {
            let record: Record = record.unwrap();
            records.push(([record.lat, record.lon], record));
        }

        let mut tree = KdTree::new();
        records.iter().enumerate().for_each(|(idx, city)| {
            tree.add(&city.1.as_xyz(), idx);
        });
        ReverseGeocoder {
            locations: records,
            tree,
        }
    }

    pub fn from_path<P: AsRef<Path>>(
        file_path: P,
    ) -> Result<ReverseGeocoder, Box<dyn error::Error>> {
        // let start_load = Instant::now();
        let mut records = Vec::new();

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;

        for record in reader.deserialize() {
            let record: Record = record?;
            records.push(([record.lat, record.lon], record));
        }

        let mut tree = KdTree::new();
        records.iter().enumerate().for_each(|(idx, city)| {
            tree.add(&city.1.as_xyz(), idx);
        });
        Ok(ReverseGeocoder {
            locations: records,
            tree,
        })
    }

    /// Search for the closest record to a given (latitude, longitude).
    pub fn search(&self, loc: (f64, f64)) -> Option<SearchResult> {
        let query = degrees_lat_lng_to_unit_sphere(loc.0, loc.1);
        let nearest_neighbor = self.tree.nearest_one::<SquaredEuclidean>(&query);

        if nearest_neighbor.item >= self.locations.len() {
            return None;
        }

        let nearest = &self.locations[nearest_neighbor.item as usize];

        Some(SearchResult {
            distance: nearest_neighbor.distance,
            record: &nearest.1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_4_places() {
        let geocoder = ReverseGeocoder::new();

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
        let geocoder = ReverseGeocoder::from_path("./cities.csv")?;
        let search_result = geocoder.search((45.0, 54.0));
        assert!(search_result.is_some());
        Ok(())
    }

    #[test]
    fn it_handles_a_nearly_blank_file() -> Result<(), Box<dyn error::Error>> {
        let geocoder = ReverseGeocoder::from_path("./nearly-blank.csv")?;
        let search_result = geocoder.search((45.0, 54.0));
        assert!(search_result.is_none());
        Ok(())
    }

    #[test]
    fn it_handles_a_blank_file() -> Result<(), Box<dyn error::Error>> {
        let geocoder = ReverseGeocoder::from_path("./blank.csv")?;
        let search_result = geocoder.search((45.0, 54.0));
        assert!(search_result.is_none());
        Ok(())
    }

    #[test]
    fn it_handles_an_infinite_coordinate() {
        let geocoder = ReverseGeocoder::new();
        let search_result = geocoder.search((std::f64::INFINITY, 54.0));
        assert!(search_result.is_some());
    }
}
