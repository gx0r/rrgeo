extern crate kdtree;
extern crate rustc_serialize;
extern crate time;

mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;
use geocoder::print_record;

fn main() {
    let loc = Locations::from_file();
    let geocoder = ReverseGeocoder::new(&loc);

    let y = geocoder.search(&[44.962786, -93.344722]).unwrap();

    print_record(y);
}
