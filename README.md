# Rust Reverse Geocoder
A fast reverse geocoder in Rust. Inspired by Python [reverse-geocoder](https://github.com/thampiman/reverse-geocoder).

## Links

- [Crate](https://crates.io/crates/reverse_geocoder)
- [Changelog](CHANGELOG.md)
- [Latest Docs](https://docs.rs/reverse_geocoder/)
- [v2.0 Docs](https://docs.rs/reverse_geocoder/2.0.0/reverse_geocoder/index.html)
- [v1.0 Docs](https://docs.rs/reverse_geocoder/1.0.1/reverse_geocoder/)

## Description

`rrgeo` takes a latitude and longitude as input and returns the closest city, country, latitude, and longitude, using a k-d tree to efficiently find the nearest neighbour based on a known list of locations. This can be useful if you need to reverse geocode a large number of coordinates quickly, or just need the rough location of coordinates but don't want the expense or complication of an online reverse geocoder.

This crate is implemented as a [library](https://crates.io/crates/reverse_geocoder), an [Actix](https://actix.rs/) REST API, a [Warp](https://seanmonstar.com/post/176530511587/warp) REST API, and as a command-line utility, thanks to [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

## Usage

### Command line search

Example usage:

```
> cargo run -p rrgeo-cmd --release 40 -73
0 ms to search
Location: (40.72788, -73.09761): West Sayville, New York, Suffolk County, US
Distance: 0.539337006499999
```

### Actix Web Server

Example usage:

```
cargo run -p rrgeo-actix --release
```

Navigate to [the local web server](http://localhost:3000/?lat=40&long=-73).

### Warp Web Server

Example usage:

```
cargo run -p rrgeo-warp --release
```

Navigate to [the local web server](http://localhost:3000/?lat=40&long=-73).

## Benchmarks

Benchmarked on Apple M1.

  * Core library benchmarked with `cargo bench` and [criterion](https://github.com/japaric/criterion.rs)
  * Web servers benchmarked with [oha](https://github.com/hatoo/oha)

Core library:

```
> cargo bench
search                  time:   [416.30 ns 416.39 ns 416.49 ns]
```

Served via [Actix Web](https://actix.rs/):

```
> cargo run --release --bin rrgeo-actix
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	1.0000
  Total:	5.0008 secs
  Slowest:	0.2505 secs
  Fastest:	0.0000 secs
  Average:	0.0003 secs
  Requests/sec:	164159.8709

  Total data:	86.90 MiB
  Size/request:	111 B
  Size/sec:	17.38 MiB

Response time histogram:
  0.000 [395294] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.000 [209631] |■■■■■■■■■■■■■■■■
  0.000 [88554]  |■■■■■■■
  0.000 [53553]  |■■■■
  0.000 [30225]  |■■
  0.000 [13830]  |■
  0.001 [7100]   |
  0.001 [4594]   |
  0.001 [3040]   |
  0.001 [1890]   |
  0.001 [13218]  |■

Latency distribution:
  10% in 0.0000 secs
  25% in 0.0001 secs
  50% in 0.0001 secs
  75% in 0.0002 secs
  90% in 0.0003 secs
  95% in 0.0004 secs
  99% in 0.0012 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0017 secs, 0.0013 secs, 0.0020 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0003 secs

Status code distribution:
  [200] 820929 responses

```

Served via [Warp](https://github.com/seanmonstar/warp):

```
> cargo run --release --bin rrgeo-warp
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	1.0000
  Total:	5.0022 secs
  Slowest:	0.0158 secs
  Fastest:	0.0000 secs
  Average:	0.0003 secs
  Requests/sec:	183796.4070

  Total data:	132.40 MiB
  Size/request:	151 B
  Size/sec:	26.47 MiB

Response time histogram:
  0.000 [20392]  |■■
  0.000 [102885] |■■■■■■■■■■■■
  0.000 [269371] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.000 [242570] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.000 [159505] |■■■■■■■■■■■■■■■■■■
  0.000 [71758]  |■■■■■■■■
  0.001 [27128]  |■■■
  0.001 [11721]  |■
  0.001 [5919]   |
  0.001 [3213]   |
  0.001 [4918]   |

Latency distribution:
  10% in 0.0002 secs
  25% in 0.0002 secs
  50% in 0.0003 secs
  75% in 0.0003 secs
  90% in 0.0004 secs
  95% in 0.0005 secs
  99% in 0.0007 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0017 secs, 0.0010 secs, 0.0019 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0002 secs

Status code distribution:
  [200] 919380 responses
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


