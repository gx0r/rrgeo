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

Benchmarked on Apple M1.

  * Core library benchmarked with `cargo bench` and [criterion](https://github.com/japaric/criterion.rs)
  * Web servers benchmarked with [oha](https://github.com/hatoo/oha)

Core library:

```
> cargo bench
search                  time:   [416.30 ns 416.39 ns 416.49 ns]
```

<details>
<summary>Served via Axum</summary>

```
> cargo run -p rrgeo-axum --release
>  oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	1.0000
  Total:	5.0009 secs
  Slowest:	0.0115 secs
  Fastest:	0.0000 secs
  Average:	0.0003 secs
  Requests/sec:	171372.8673

  Total data:	123.42 MiB
  Size/request:	151 B
  Size/sec:	24.68 MiB

Response time histogram:
  0.000 [1]      |
  0.001 [856908] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.002 [22]     |
  0.003 [54]     |
  0.005 [7]      |
  0.006 [7]      |
  0.007 [8]      |
  0.008 [4]      |
  0.009 [4]      |
  0.010 [2]      |
  0.012 [4]      |

Latency distribution:
  10% in 0.0002 secs
  25% in 0.0002 secs
  50% in 0.0003 secs
  75% in 0.0004 secs
  90% in 0.0004 secs
  95% in 0.0005 secs
  99% in 0.0006 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0014 secs, 0.0010 secs, 0.0019 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0003 secs

Status code distribution:
  [200] 857021 responses
```

</details>

<details>
<summary>Served via Actix Web</summary>

```
> cargo run --release --bin rrgeo-actix
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	1.0000
  Total:	5.0007 secs
  Slowest:	0.2903 secs
  Fastest:	0.0000 secs
  Average:	0.0003 secs
  Requests/sec:	161291.4437

  Total data:	85.38 MiB
  Size/request:	111 B
  Size/sec:	17.07 MiB

Response time histogram:
  0.000 [1]      |
  0.029 [805611] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.058 [379]    |
  0.087 [192]    |
  0.116 [111]    |
  0.145 [231]    |
  0.174 [15]     |
  0.203 [6]      |
  0.232 [6]      |
  0.261 [11]     |
  0.290 [1]      |

Latency distribution:
  10% in 0.0001 secs
  25% in 0.0001 secs
  50% in 0.0001 secs
  75% in 0.0002 secs
  90% in 0.0004 secs
  95% in 0.0005 secs
  99% in 0.0017 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0015 secs, 0.0013 secs, 0.0017 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0003 secs

Status code distribution:
  [200] 806564 responses

```
</details>

<details>
<summary>Served via Warp</summary>

```
> cargo run --release --bin rrgeo-warp
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate:	1.0000
  Total:	5.0009 secs
  Slowest:	0.0134 secs
  Fastest:	0.0000 secs
  Average:	0.0003 secs
  Requests/sec:	180231.7215

  Total data:	129.79 MiB
  Size/request:	151 B
  Size/sec:	25.95 MiB

Response time histogram:
  0.000 [1]      |
  0.001 [901133] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.003 [77]     |
  0.004 [24]     |
  0.005 [58]     |
  0.007 [3]      |
  0.008 [7]      |
  0.009 [8]      |
  0.011 [3]      |
  0.012 [2]      |
  0.013 [2]      |

Latency distribution:
  10% in 0.0002 secs
  25% in 0.0002 secs
  50% in 0.0003 secs
  75% in 0.0003 secs
  90% in 0.0004 secs
  95% in 0.0005 secs
  99% in 0.0006 secs

Details (average, fastest, slowest):
  DNS+dialup:	0.0014 secs, 0.0010 secs, 0.0019 secs
  DNS-lookup:	0.0000 secs, 0.0000 secs, 0.0003 secs

Status code distribution:
  [200] 901318 responses
```

</details>


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


