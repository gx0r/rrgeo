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
search                  time:   [489.30 us 490.03 us 490.94 us]
```

Served via [Actix Web](https://actix.rs/):

```
> cargo run --release --bin rrgeo-actix
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	1.0000
  Total:	5.0069 secs
  Slowest:	0.1105 secs
  Fastest:	0.0012 secs
  Average:	0.0079 secs
  Requests/sec:	5773.4290

  Total data:	3.17 MiB
  Size/request:	115 B
  Size/sec:	648.38 KiB

Response time histogram:
  0.002 [451]  |■
  0.004 [4146] |■■■■■■■■■■■■■
  0.006 [9914] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.008 [7200] |■■■■■■■■■■■■■■■■■■■■■■■
  0.010 [4414] |■■■■■■■■■■■■■■
  0.012 [1915] |■■■■■■
  0.014 [557]  |■
  0.016 [160]  |
  0.018 [59]   |
  0.020 [18]   |
  0.023 [73]   |

Latency distribution:
  10% in 0.0048 secs
  25% in 0.0059 secs
  50% in 0.0073 secs
  75% in 0.0093 secs
  90% in 0.0113 secs
  95% in 0.0126 secs
  99% in 0.0157 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0018 secs, 0.0013 secs, 0.0029 secs
  DNS-lookup:	0.0002 secs, 0.0000 secs, 0.0013 secs

Status code distribution:
  [200] 28907 responses

```

Served via [Warp](https://github.com/seanmonstar/warp):

```
> cargo run --release --bin rrgeo-warp
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	1.0000
  Total:	5.0040 secs
  Slowest:	0.0812 secs
  Fastest:	0.0008 secs
  Average:	0.0078 secs
  Requests/sec:	5883.9353

  Total data:	4.35 MiB
  Size/request:	155 B
  Size/sec:	890.63 KiB

Response time histogram:
  0.002 [1237] |■■■■
  0.004 [3309] |■■■■■■■■■■■
  0.006 [7022] |■■■■■■■■■■■■■■■■■■■■■■■■■
  0.008 [8985] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.010 [5565] |■■■■■■■■■■■■■■■■■■■
  0.012 [2402] |■■■■■■■■
  0.014 [648]  |■■
  0.016 [173]  |
  0.019 [35]   |
  0.021 [7]    |
  0.023 [60]   |

Latency distribution:
  10% in 0.0041 secs
  25% in 0.0058 secs
  50% in 0.0077 secs
  75% in 0.0095 secs
  90% in 0.0113 secs
  95% in 0.0123 secs
  99% in 0.0151 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0015 secs, 0.0013 secs, 0.0019 secs
  DNS-lookup:	0.0001 secs, 0.0000 secs, 0.0004 secs

Status code distribution:
  [200] 29443 responses
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


