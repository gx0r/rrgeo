# Rust Reverse Geocoder

A fast reverse geocoder in Rust. Inspired by Python [reverse-geocoder](https://github.com/thampiman/reverse-geocoder).

## Links

- [Crate](https://crates.io/crates/reverse_geocoder)
- [Changelog](CHANGELOG.md)
- [Docs](https://docs.rs/reverse_geocoder/)

## Description

`rrgeo` takes a latitude and longitude as input and returns the closest city, country, latitude, and longitude, using a k-d tree to efficiently find the nearest neighbour based on a known list of locations. This can be useful if you need to reverse geocode a large number of coordinates quickly, or just need the rough location of coordinates but don't want the expense or complication of an online reverse geocoder.

This project contains (via [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)) a [library](https://crates.io/crates/reverse_geocoder), a [Axum](https://github.com/tokio-rs/axum) REST API, an [Actix](https://actix.rs/) REST API, a [Warp](https://seanmonstar.com/post/176530511587/warp) REST API, and a command-line utility.

## Usage

### Command line search

Example usage:

```
> cargo run -p rrgeo-cmd --release 40 -73
0 ms to search
Location: (40.72788, -73.09761): West Sayville, New York, Suffolk County, US
Distance: 0.539337006499999
```

### Web Servers

Example usage:

```bash
cargo run -p rrgeo-axum --release
cargo run -p rrgeo-actix --release
cargo run -p rrgeo-warp --release
```

## Benchmarks

Benchmarked on Apple M2.

- Core library benchmarked with `cargo bench` and [criterion](https://github.com/japaric/criterion.rs)
- Web servers benchmarked with [oha](https://github.com/hatoo/oha)

Core library:

```bash
> cargo bench
time:   [154.16 ns 155.34 ns 157.00 ns]
```

<details>
<summary>Served via Axum</summary>

```bash
> cargo run -p rrgeo-axum --release
>  oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	100.00%
  Total:	5.0004 secs
  Slowest:	0.0099 secs
  Fastest:	0.0000 secs
  Average:	0.0002 secs
  Requests/sec:	221767.0303

  Total data:	162.86 MiB
  Size/request:	154 B
  Size/sec:	32.57 MiB

Response time histogram:
  0.000 [1]       |
  0.001 [1108827] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.002 [21]      |
  0.003 [49]      |
  0.004 [9]       |
  0.005 [4]       |
  0.006 [1]       |
  0.007 [1]       |
  0.008 [2]       |
  0.009 [1]       |
  0.010 [2]       |

Response time distribution:
  10.00% in 0.0001 secs
  25.00% in 0.0002 secs
  50.00% in 0.0002 secs
  75.00% in 0.0003 secs
  90.00% in 0.0003 secs
  95.00% in 0.0004 secs
  99.00% in 0.0005 secs
  99.90% in 0.0006 secs
  99.99% in 0.0009 secs


Details (average, fastest, slowest):
  DNS+dialup:	0.0016 secs, 0.0011 secs, 0.0019 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0002 secs

Status code distribution:
  [200] 1108918 responses
```

</details>

<details>
<summary>Served via Actix Web</summary>

```bash
> cargo run --release --bin rrgeo-actix
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	100.00%
  Total:	5.0007 secs
  Slowest:	0.2502 secs
  Fastest:	0.0000 secs
  Average:	0.0002 secs
  Requests/sec:	204563.3764

  Total data:	106.34 MiB
  Size/request:	109 B
  Size/sec:	21.26 MiB

Response time histogram:
  0.000 [1]       |
  0.025 [1021753] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.050 [432]     |
  0.075 [165]     |
  0.100 [101]     |
  0.125 [218]     |
  0.150 [266]     |
  0.175 [7]       |
  0.200 [1]       |
  0.225 [7]       |
  0.250 [12]      |

Response time distribution:
  10.00% in 0.0000 secs
  25.00% in 0.0001 secs
  50.00% in 0.0001 secs
  75.00% in 0.0001 secs
  90.00% in 0.0002 secs
  95.00% in 0.0003 secs
  99.00% in 0.0011 secs
  99.90% in 0.0323 secs
  99.99% in 0.1263 secs


Details (average, fastest, slowest):
  DNS+dialup:	0.0015 secs, 0.0012 secs, 0.0019 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0002 secs

Status code distribution:
  [200] 1022963 responses

```
</details>

<details>
<summary>Served via Warp</summary>

```bash
> cargo run --release --bin rrgeo-warp
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
SuSummary:
  Success rate:	100.00%
  Total:	5.0003 secs
  Slowest:	0.0111 secs
  Fastest:	0.0000 secs
  Average:	0.0002 secs
  Requests/sec:	232498.2550

  Total data:	170.74 MiB
  Size/request:	154 B
  Size/sec:	34.15 MiB

Response time histogram:
  0.000 [1]       |
  0.001 [1162216] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.002 [167]     |
  0.003 [52]      |
  0.004 [56]      |
  0.006 [56]      |
  0.007 [4]       |
  0.008 [3]       |
  0.009 [1]       |
  0.010 [1]       |
  0.011 [2]       |

Response time distribution:
  10.00% in 0.0001 secs
  25.00% in 0.0002 secs
  50.00% in 0.0002 secs
  75.00% in 0.0003 secs
  90.00% in 0.0003 secs
  95.00% in 0.0004 secs
  99.00% in 0.0004 secs
  99.90% in 0.0007 secs
  99.99% in 0.0038 secs


Details (average, fastest, slowest):
  DNS+dialup:	0.0018 secs, 0.0013 secs, 0.0025 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0005 secs

Status code distribution:
  [200] 1162559 responses
```

</details>

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
