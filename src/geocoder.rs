use kdtree::KdTree;
use kdtree::distance::squared_euclidean;
use time::PreciseTime;
extern crate quick_csv;

#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct Record {
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
    pub fn from_file() -> Locations {
        let start = PreciseTime::now();
        let mut records = Vec::new();

        let rdr = quick_csv::Csv::from_file("cities.csv").unwrap().has_header(true);

        for record in rdr {
            let r: Record = record.unwrap().decode().unwrap();
            records.push(([r.lat, r.lon], r));
        }

        let end = PreciseTime::now();

        println!("{} seconds to load cities.csv", start.to(end));

        Locations { records: records }
    }
}

pub struct ReverseGeocoder<'a> {
    tree: KdTree<'a, &'a Record>,
}

impl<'a> ReverseGeocoder<'a> {
    pub fn new(loc: &'a Locations) -> ReverseGeocoder<'a> {
        let mut r = ReverseGeocoder::<'a> { tree: KdTree::new_with_capacity(2, loc.records.len()) };
        r.initialize(loc);
        r
    }

    fn initialize(&mut self, loc: &'a Locations) {
        let start = PreciseTime::now();
        for record in &loc.records {
            self.tree.add(&record.0, &record.1).unwrap();
        }
        let end = PreciseTime::now();
        println!("{} seconds to build the KdTree", start.to(end));
    }

    pub fn search(&'a self, loc: &[f64; 2]) -> Option<&'a Record> {
        let y = self.tree.nearest(loc, 1, &squared_euclidean).unwrap();
        if y.is_empty() {
            None
        } else {
            Some(&y[0].1)
        }
    }
}

pub fn print_record(r: &Record) {
    println!("({}, {}): {} {} {} {}",
             r.lat,
             r.lon,
             r.name,
             r.admin1,
             r.admin2,
             r.admin3);
}

mod tests {
    #[test]
    fn it_works() {
        let loc = super::Locations::from_file();
        let geocoder = super::ReverseGeocoder::new(&loc);
        let y = geocoder.search(&[44.962786, -93.344722]);
        assert_eq!(y.is_some(), true);
        let slp = y.unwrap();

        assert_eq!(slp.name, "Saint Louis Park");

        // [44.894519, -93.308702] is 60 St W @ Penn Ave S, Minneapolis, Minnesota; however, this is physically closer to Richfield
        let mpls = geocoder.search(&[44.894519, -93.308702]).unwrap();
        assert_eq!(mpls.name, "Richfield");

        // [44.887055, -93.334204] is HWY 62 and Valley View Road, whish is in Edina
        let edina = geocoder.search(&[44.887055, -93.334204]).unwrap();
        assert_eq!(edina.name, "Edina");
    }
}