# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
## [2.0.0] - 
### Changed
#### Update and improve error handling and function signatures.

* Simplify the `search` API function signature
* Use standard 2018 edition error handling idioms
* actix_web handler demonstrates exhaustive match of all errors


## [1.0.1] - 2020/3/29
### Added
- Rustdoc documentation
### Changed
- Removed an unnecessary `unwrap` in `Locations::from_path`

## [1.0.0] - 2020/3/28

- First release