# Rust Reverse Geocoder
Rust port of https://github.com/thampiman/reverse-geocoder

# Performance

Note: most of the performance differences are going to be in time taken loading CSV file and creating k-d tree, not searching the tree. Searching time resembles algorithmic complexity of [k-d tree](https://en.wikipedia.org/wiki/K-d_tree). Python version is partly implemented in C++ meaning it is not a purely Python implementation. (It might be interesting to see how a pure Python version performs.) Node.js version is pure JavaScript e.g. not using C add ons.

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
➜  nreverse time node index
load CSV: 1447.281ms
createTree: 1060.373ms
search: 1.389ms
[ { lat: 44.9483,
    long: -93.34801,
    name: 'Saint Louis Park',
    ad1: 'Minnesota',
    ad2: 'Hennepin County',
    cc: 'US' },
  0 ]
node index  2.57s user 0.09s system 101% cpu 2.630 total
```
