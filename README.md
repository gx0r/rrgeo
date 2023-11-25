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

```
cargo run -p rrgeo-axum --release
cargo run -p rrgeo-actix --release
cargo run -p rrgeo-warp --release
```

## Benchmarks

Benchmarked on Apple M2.

  * Core library benchmarked with `cargo bench` and [criterion](https://github.com/japaric/criterion.rs)
  * Web servers benchmarked with [oha](https://github.com/hatoo/oha)

Core library:

```
> cargo bench
search                  time:   [315.15 ns 315.85 ns 316.51 ns]
```

<details>
<summary>Served via Axum</summary>

```
> cargo run -p rrgeo-axum --release
>  oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	100.00%
  Total:	5.0008 secs
  Slowest:	0.0850 secs
  Fastest:	0.0000 secs
  Average:	0.0002 secs
  Requests/sec:	212162.3166

  Total data:	152.79 MiB
  Size/request:	151 B
  Size/sec:	30.55 MiB

Response time histogram:
  0.000 [1]       |
  0.009 [1060933] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.017 [0]       |
  0.026 [0]       |
  0.034 [0]       |
  0.043 [0]       |
  0.051 [0]       |
  0.060 [0]       |
  0.068 [0]       |
  0.077 [0]       |
  0.085 [50]      |

Response time distribution:
  10.00% in 0.0001 secs
  25.00% in 0.0002 secs
  50.00% in 0.0002 secs
  75.00% in 0.0003 secs
  90.00% in 0.0003 secs
  95.00% in 0.0004 secs
  99.00% in 0.0005 secs
  99.90% in 0.0006 secs
  99.99% in 0.0008 secs


Details (average, fastest, slowest):
  DNS+dialup:	0.0014 secs, 0.0010 secs, 0.0022 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0008 secs

Status code distribution:
  [200] 1060984 responses
```

</details>

<details>
<summary>Served via Actix Web</summary>

```
> cargo run --release --bin rrgeo-actix
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	100.00%
  Total:	5.0003 secs
  Slowest:	0.2662 secs
  Fastest:	0.0000 secs
  Average:	0.0003 secs
  Requests/sec:	196005.8198

  Total data:	103.75 MiB
  Size/request:	111 B
  Size/sec:	20.75 MiB

Response time histogram:
  0.000 [1]      |
  0.027 [978934] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.053 [416]    |
  0.080 [218]    |
  0.106 [140]    |
  0.133 [301]    |
  0.160 [29]     |
  0.186 [21]     |
  0.213 [9]      |
  0.240 [3]      |
  0.266 [18]     |

Response time distribution:
  10.00% in 0.0000 secs
  25.00% in 0.0001 secs
  50.00% in 0.0001 secs
  75.00% in 0.0001 secs
  90.00% in 0.0003 secs
  95.00% in 0.0004 secs
  99.00% in 0.0010 secs
  99.90% in 0.0348 secs
  99.99% in 0.1274 secs


Details (average, fastest, slowest):
  DNS+dialup:	0.0015 secs, 0.0012 secs, 0.0029 secs
  DNS-lookup:	0.0002 secs, 0.0000 secs, 0.0013 secs

Status code distribution:
  [200] 980090 responses

```
</details>

<details>
<summary>Served via Warp</summary>

```
> cargo run --release --bin rrgeo-warp
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	100.00%
  Total:	5.0002 secs
  Slowest:	0.0030 secs
  Fastest:	0.0000 secs
  Average:	0.0002 secs
  Requests/sec:	225022.8641

  Total data:	162.03 MiB
  Size/request:	151 B
  Size/sec:	32.40 MiB

Response time histogram:
  0.000 [1]      |
  0.000 [975102] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.001 [148843] |■■■■
  0.001 [1053]   |
  0.001 [100]    |
  0.002 [4]      |
  0.002 [3]      |
  0.002 [0]      |
  0.002 [8]      |
  0.003 [29]     |
  0.003 [14]     |

Response time distribution:
  10.00% in 0.0001 secs
  25.00% in 0.0002 secs
  50.00% in 0.0002 secs
  75.00% in 0.0003 secs
  90.00% in 0.0003 secs
  95.00% in 0.0004 secs
  99.00% in 0.0005 secs
  99.90% in 0.0006 secs
  99.99% in 0.0010 secs


Details (average, fastest, slowest):
  DNS+dialup:	0.0016 secs, 0.0013 secs, 0.0019 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0002 secs

Status code distribution:
  [200] 1125157 responses
```

</details>


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


