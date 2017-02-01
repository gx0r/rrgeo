# Rust Reverse Geocoder
A fast, offline reverse geocoder in Rust, inspired by https://github.com/thampiman/reverse-geocoder

# Usage

## Built in web server

```
cargo run --bin web
http://localhost:3000/?lat=55&long=66
```

## Command line search

```
cargo run --bin cmd
cargo run --bin cmd 55 66
```

```
➜  rreverse git:(master) ✗ cargo run --bin cmd 55 66
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/cmd 55 66`
1208 ms to load cities.csv
116 ms to build the KdTree
(54.79139, 65.98639): Polovinnoye Kurgan  RU
```

# Performance

Below we have comparisons between the Rust, Python and Node.js versions.

|              | Rust | Node   |
|--------------|------|--------|
| Load CSV     | 61ms | 1221ms |
| Build KdTree | 4ms  | 805ms  |
| Search       | 2ms  | 0.10ms |

Most of the performance differences appear to be in time taken to load the CSV file and create the k-d tree, but not searching the tree. Searching time resembles algorithmic complexity of [k-d tree](https://en.wikipedia.org/wiki/K-d_tree). Python version is partly implemented in C++ meaning it is not a purely Python implementation. (It might be interesting to see how a pure Python version performs.) The Node.js version is pure JavaScript, as in, not using C add-ons.

Rust --release web server performance:

```
➜  wrk git:(master) wrk --latency -t12 -c100 -d10s http://localhost:3000/\?lat\=45\&long\=\66
Running 10s test @ http://localhost:3000/?lat=45&long=66
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    25.76ms   19.55ms 198.42ms   67.61%
    Req/Sec   284.78     92.28   580.00     77.30%
  Latency Distribution
     50%   23.06ms
     75%   31.66ms
     90%   50.47ms
     99%   92.36ms
  25645 requests in 10.10s, 4.79MB read
Requests/sec:   2539.13
Transfer/sec:    486.01KB
```

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
