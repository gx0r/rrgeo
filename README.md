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

Benchmarked on Intel Core i7 4790K at 4.00GHz.

The core library ([measured with criterion](https://github.com/japaric/criterion.rs)):

```
> cargo bench
search                  time:   [2.0303 ms 2.0399 ms 2.0502 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

Served via [Actix Web](https://actix.rs/):

```
> cargo run --release --bin rrgeo-actix
> oha http://localhost:3000/\?lat\=40\&long\=\-73 -z 5sec
Summary:
  Success rate: 1.0000
  Total:        5.0209 secs
  Slowest:      0.0744 secs
  Fastest:      0.0025 secs
  Average:      0.0195 secs
  Requests/sec: 2541.7942

  Total data:   1.40 MiB
  Size/request: 115.00 B
  Size/sec:     285.46 KiB

Response time histogram:
  0.005 [465]  |■■■
  0.010 [1390] |■■■■■■■■■
  0.015 [4609] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.020 [3029] |■■■■■■■■■■■■■■■■■■■■■
  0.025 [1483] |■■■■■■■■■■
  0.031 [900]  |■■■■■■
  0.036 [467]  |■■■
  0.041 [190]  |■
  0.046 [104]  |
  0.051 [79]   |
  0.056 [46]   |

Latency distribution:
  10% in 0.0107 secs
  25% in 0.0148 secs
  50% in 0.0177 secs
  75% in 0.0230 secs
  90% in 0.0306 secs
  95% in 0.0355 secs
  99% in 0.0478 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0055 secs, 0.0010 secs, 0.0125 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [200] 12762 responses
```

Served via [Warp](https://github.com/seanmonstar/warp):

```
> cargo run --release --bin rrgeo-warp
Summary:
  Success rate: 1.0000
  Total:        5.0042 secs
  Slowest:      0.1389 secs
  Fastest:      0.0023 secs
  Average:      0.0202 secs
  Requests/sec: 2464.1392

  Total data:   1.82 MiB
  Size/request: 155.00 B
  Size/sec:     372.99 KiB

Response time histogram:
  0.005 [383]  |■■
  0.011 [995]  |■■■■■■■
  0.016 [4299] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.021 [3705] |■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.027 [1707] |■■■■■■■■■■■■
  0.032 [780]  |■■■■■
  0.037 [274]  |■■
  0.042 [81]   |
  0.048 [50]   |
  0.053 [7]    |
  0.058 [50]   |

Latency distribution:
  10% in 0.0125 secs
  25% in 0.0160 secs
  50% in 0.0187 secs
  75% in 0.0233 secs
  90% in 0.0288 secs
  95% in 0.0329 secs
  99% in 0.0432 secs

Details (average, fastest, slowest):
  DNS+dialup:   0.0052 secs, 0.0008 secs, 0.0116 secs
  DNS-lookup:   0.0000 secs, 0.0000 secs, 0.0001 secs

Status code distribution:
  [200] 12331 responses
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


