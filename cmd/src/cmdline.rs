use reverse_geocoder::ReverseGeocoder;
use std::error::Error;
use std::time::Instant;
use std::{env, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        eprint!("Usage: rreverse lat long\n e.g., rreverse 40.7831 -73.9712\n\n");
        exit(1);
    }

    let lat = args[1].parse::<f64>()?;
    let long = args[2].parse::<f64>()?;

    let geocoder = ReverseGeocoder::new();

    let start = Instant::now();
    let search_result = geocoder.search((lat, long));
    eprintln!("{} ms to search", start.elapsed().as_millis());

    println!("Location: {}", search_result.record.name);
    println!("Distance: {}", search_result.distance);

    Ok(())
}
