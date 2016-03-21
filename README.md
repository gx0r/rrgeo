# rreverse
Rust port of https://github.com/thampiman/reverse-geocoder

# Performance

Note: most of the performance differences are going to be in time taken loading CSV file and creating k-d tree, not searching the tree. Searching time resembles algorithmic complexity of [k-d tree](https://en.wikipedia.org/wiki/K-d_tree). Python version is partly implemented in C++ meaning it is not a purely Python implementation. (It might be interesting to see how a pure Python version performs.) Node.js version is pure JavaScript e.g. not using C add ons.

## Rust --release build

```
➜  rreverse git:(master) time target/release/rreverse
PT0.122817855S seconds to load cities.csv
PT0.007221280S seconds to build the KdTree
(44.9483, -93.34801): Saint Louis Park Minnesota Hennepin County US

target/release/rreverse  0.12s user 0.03s system 95% cpu 0.158 total
```

## Rust --debug build

```
➜  rreverse git:(master) time target/debug/rreverse
PT2.896100517S seconds to load cities.csv
PT0.147031331S seconds to build the KdTree
(44.9483, -93.34801): Saint Louis Park Minnesota Hennepin County US

target/debug/rreverse  3.05s user 0.04s system 99% cpu 3.126 total
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
