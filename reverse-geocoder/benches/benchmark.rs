#[macro_use]
extern crate criterion;
#[macro_use]
extern crate lazy_static;
extern crate reverse_geocoder;

use reverse_geocoder::ReverseGeocoder;

use criterion::Criterion;

lazy_static! {
    static ref GEOCODER: ReverseGeocoder = ReverseGeocoder::new();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search", |b| b.iter(|| GEOCODER.search((45.0, 54.0))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
