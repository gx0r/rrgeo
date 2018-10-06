#![feature(rustc_private)]

extern crate rustc_serialize;
extern crate time;
extern crate reversegeocoder;
use reversegeocoder::{
    Locations,
    ReverseGeocoder,
    print_record,
};

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        print!("Usage: rreverse lat long\n e.g., rreverse 44.962786 -93.344722\n\n");
        exit(1);
    }

    let lat = args[1].parse::<f64>().expect("Couldn't parse latitude");
    let long = args[2].parse::<f64>().expect("Couldn't parse longitude");

    let loc = Locations::from_file();
    let geocoder = ReverseGeocoder::new(&loc);

    let y = geocoder.search(&[lat, long]).expect("Nothing found.");

    print_record(y);
}
