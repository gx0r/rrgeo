# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [4.1.0] - 2023-11-26
- `from_path` returns `Result<ReverseGeocoder, std::io::Error>` now instead of `Result<ReverseGeocoder, Box<dyn error::Error>>`

## [4.0.0] - 2023-11-25
- Upgrade kiddo 0.2.5 -> 3.0.0
- Simplify APIs

## [3.0.1] - 2022-7-11
- Performance: switch to kiddo kd tree library for ~800x improvement on benchmark 😲  [#8](https://github.com/gx0r/rrgeo/pull/8)

## [3.0.0] - 2022-3-1
- Replace quick-csv with csv
- Remove rustc-serialize in favor of serde
- Record.`admin3` became `cc` (country code)
- Update Actix, Warp and benchmark on M1
- Update to Actix v4
- Update to Warp 0.3.2

## [2.0.0] - 2020-5-2
### Changed
- No changes from alpha

## [2.0.0-alpha.0] - 2020-4-4
### Changed
#### Update and improve error handling and function signatures.

* Simplify the `search` API function signature
* Use standard 2018 edition error handling idioms

## [1.0.1] - 2020-3-29
### Added
- Rustdoc documentation
### Changed
- Removed an unnecessary `unwrap` in `Locations::from_path`

## [1.0.0] - 2020-3-28

- First release
