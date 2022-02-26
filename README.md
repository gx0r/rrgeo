# Rust Reverse Geocoder
A fast reverse geocoder in Rust. Inspired by Python [reverse-geocoder](https://github.com/thampiman/reverse-geocoder).

## Links

- [Crate](https://crates.io/crates/reverse_geocoder)
- [2.0.0 Docs](https://docs.rs/reverse_geocoder/2.0.0/reverse_geocoder/index.html)
- [1.0.1 Docs](https://docs.rs/reverse_geocoder/1.0.1/reverse_geocoder/)

## Description

`rrgeo` takes a latitude and longitude as input and returns the closest city, country, latitude, and longitude, using a k-d tree to efficiently find the nearest neighbour based on a known list of locations. This can be useful if you need to reverse geocode a large number of coordinates quickly, or just need the rough location of coordinates but don't want the expense or complication of an online reverse geocoder.

This crate is implemented as a [library](https://crates.io/crates/reverse_geocoder), an [Actix](https://actix.rs/) REST API, a [Warp](https://seanmonstar.com/post/176530511587/warp) REST API, and as a command-line utility, thanks to [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

## Usage

### Command line search

Example usage:

```
> cargo run -p rrgeo-cmd --release 40 -73
71 ms to load cities.csv
3 ms to build the KdTree
(40.72788, -73.09761): West Sayville New York Suffolk County US
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

The core library ([measured with criterion](https://github.com/japaric/criterion.rs)):

```
> cargo bench
search                  time:   [518.99 us 535.48 us 553.21 us]
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


## Performance

Below we have comparisons between the Rust, Python and Node.js versions.

|              | Rust | Node   |
|--------------|------|--------|
| Load CSV     | 61ms | 1221ms |
| Build KdTree | 4ms  | 805ms  |
| Search       | 1.1ms  | 0.5ms |

Most of the performance differences appear to be in time taken to load the CSV file and create the k-d tree, but not searching the tree. Searching time resembles algorithmic complexity of [k-d tree](https://en.wikipedia.org/wiki/K-d_tree). Python version is partly implemented in C++ meaning it is not a purely Python implementation. (It might be interesting to see how a pure Python version performs.) The Node.js version is pure JavaScript, as in, not using C add-ons.

### Rust --release build

```
     Running `target/release/web`
PT0.062677465S seconds to load cities.csv
PT0.003835230S seconds to build the KdTree
PT0.068904911S seconds to search
PT0.002596743S seconds to search
PT0.002887542S seconds to search

```

### Rust --debug build

```
     Running `target/debug/web`
PT1.198010357S seconds to load cities.csv
PT0.124435778S seconds to build the KdTree
PT1.401588031S seconds to search
PT0.077837996S seconds to search
PT0.078178297S seconds to search

```

### Python mode 1 (single threaded K-D tree)

```
➜  reverse-geocoder git:(master) ✗ time python mode1.py
Loading formatted geocoded file...
[{'name': 'Saint Louis Park', 'cc': 'US', 'lon': '-93.34801', 'admin1': 'Minnesota', 'admin2': 'Hennepin County', 'lat': '44.9483'}]

python mode1.py  1.60s user 0.22s system 98% cpu 1.847 total
```

### Python mode 2 (multi threaded K-D tree)

```
➜  reverse-geocoder git:(master) ✗ time python mode2.py
Loading formatted geocoded file...
[{'name': 'Saint Louis Park', 'cc': 'US', 'lon': '-93.34801', 'admin1': 'Minnesota', 'admin2': 'Hennepin County', 'lat': '44.9483'}]

python mode2.py  2.82s user 0.34s system 142% cpu 2.221 total
```

### [nreverse](https://github.com/ggcode1/nreverse) (Node.js version)

```
load modules: 12.619ms
load cities.csv: 1221.833ms
create kdtree: 805.310ms
search tree: 0.758ms
search tree: 0.086ms
search tree: 0.198ms
search tree: 0.104ms
search tree: 0.031ms
total_heap_size 114mb
total_heap_size_executable 5mb
total_physical_size 112mb
total_available_size 1325mb
used_heap_size 83mb
heap_size_limit 1432mb
malloced_memory 0mb
peak_malloced_memory 4mb
does_zap_garbage 0mb

```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


