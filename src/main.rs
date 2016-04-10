#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(custom_derive, plugin, custom_attribute, type_macros)]
#![plugin(docopt_macros)]
extern crate kdtree;
extern crate rustc_serialize;
extern crate time;
extern crate docopt;

mod geocoder;
use geocoder::Locations;
use geocoder::ReverseGeocoder;
use geocoder::print_record;

docopt!(Args derivce, "
rreverse -- offline reverse geocoder

Usage:
    rreverse lat long
");

fn main() {
    let loc = Locations::from_file();
    let geocoder = ReverseGeocoder::new(&loc);

    let y = geocoder.search(&[44.962786, -93.344722]).unwrap();

    print_record(y);
}
