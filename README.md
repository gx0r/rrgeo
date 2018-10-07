[![Build Status](https://travis-ci.org/llambda/rust-reverse-geocoder.svg?branch=master)](https://travis-ci.org/llambda/rust-reverse-geocoder)

# Rust Reverse Geocoder
A fast, offline reverse geocoder in Rust, inspired by [one written in Python](https://github.com/thampiman/reverse-geocoder).
Given a latitude and longitude, this program returns the geographically closest city (using the data from the included `cities.csv` file).

This program is implemented as a library, an [Actix](https://actix.rs/) REST API, an [Iron](https://github.com/iron/iron) REST API, and as a command-line utility, thanks to [Cargo workspaces](https://doc.rust-lang.org/book/second-edition/ch14-03-cargo-workspaces.html).

(Previously implemented on [Shio](https://github.com/mehcode/shio-rs) and [Hyper](https://github.com/hyperium/hyper) HTTP libraries. Needs work to run on those libraries again.)

# Usage

## Command line search

Example usage:

```
> cargo run -p reverse-geocoder-cmd --release 40 -73
71 ms to load cities.csv
3 ms to build the KdTree
(40.72788, -73.09761): West Sayville New York Suffolk County US
```

## Web Server

Example usage:

```
cargo run -p reverse-geocoder-actix --release
```

Navigate to [the local web server](http://localhost:3000/?lat=40&long=-73).

## Benchmarks 

Actix:

```
> wrk --latency -t12 -c300 -d10s http://localhost:3000/\?lat\=45\&long\=\66
Running 10s test @ http://localhost:3000/?lat=45&long=66
  12 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    72.26ms    5.13ms 147.29ms   97.72%
    Req/Sec   346.16     41.61   696.00     71.05%
  Latency Distribution
     50%   73.14ms
     75%   73.30ms
     90%   73.65ms
     99%   77.40ms
  41492 requests in 10.10s, 7.99MB read
Requests/sec:   4108.98
Transfer/sec:    810.56KB

```

Iron:

```
> wrk --latency -t12 -c300 -d10s http://localhost:3000/\?lat\=45\&long\=\66
Running 10s test @ http://localhost:3000/?lat=45&long=66
  12 threads and 300 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    18.31ms    9.55ms  96.31ms   68.82%
    Req/Sec   501.05    329.71     1.17k    53.82%
  Latency Distribution
     50%   16.89ms
     75%   22.91ms
     90%   31.18ms
     99%   45.23ms
  35232 requests in 10.08s, 6.59MB read
Requests/sec:   3493.90
Transfer/sec:    668.75KB
```


# Performance

Below we have comparisons between the Rust, Python and Node.js versions.

|              | Rust | Node   |
|--------------|------|--------|
| Load CSV     | 61ms | 1221ms |
| Build KdTree | 4ms  | 805ms  |
| Search       | 1.5ms  | 0.5ms |

Most of the performance differences appear to be in time taken to load the CSV file and create the k-d tree, but not searching the tree. Searching time resembles algorithmic complexity of [k-d tree](https://en.wikipedia.org/wiki/K-d_tree). Python version is partly implemented in C++ meaning it is not a purely Python implementation. (It might be interesting to see how a pure Python version performs.) The Node.js version is pure JavaScript, as in, not using C add-ons.

## Rust --release build

```
     Running `target/release/web`
PT0.062677465S seconds to load cities.csv
PT0.003835230S seconds to build the KdTree
PT0.068904911S seconds to search
PT0.002596743S seconds to search
PT0.002887542S seconds to search

```

## Rust --debug build

```
     Running `target/debug/web`
PT1.198010357S seconds to load cities.csv
PT0.124435778S seconds to build the KdTree
PT1.401588031S seconds to search
PT0.077837996S seconds to search
PT0.078178297S seconds to search

```

## Python mode 1 (single threaded K-D tree)

```
➜  reverse-geocoder git:(master) ✗ time python mode1.py
Loading formatted geocoded file...
[{'name': 'Saint Louis Park', 'cc': 'US', 'lon': '-93.34801', 'admin1': 'Minnesota', 'admin2': 'Hennepin County', 'lat': '44.9483'}]

python mode1.py  1.60s user 0.22s system 98% cpu 1.847 total
```

## Python mode 2 (multi threaded K-D tree)

```
➜  reverse-geocoder git:(master) ✗ time python mode2.py
Loading formatted geocoded file...
[{'name': 'Saint Louis Park', 'cc': 'US', 'lon': '-93.34801', 'admin1': 'Minnesota', 'admin2': 'Hennepin County', 'lat': '44.9483'}]

python mode2.py  2.82s user 0.34s system 142% cpu 2.221 total
```

## [nreverse](https://github.com/llambda/nreverse) (Node.js version)

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


