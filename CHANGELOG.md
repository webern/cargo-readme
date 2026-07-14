# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.3.3] - 2026-07-14

### Changed

- Update dependencies, including toml v0.8 to v1, and replace `lazy_lock` with `std::sync::LazyLock` and deprecated `assert_cli` with `assert_cmd`. [#113]
- Update clap from v4.6.0 to v4.6.1. [#127]
- Update regex from v1.12.3 to v1.13.0. [#134]

### Added

- Support `compile_fail` code blocks in generated README output. [#116]

### Fixed

- Hide indented lines beginning with `#` in Rust code blocks. [#130]

[#113]: https://github.com/webern/cargo-readme/pull/113
[#127]: https://github.com/webern/cargo-readme/pull/127
[#134]: https://github.com/webern/cargo-readme/pull/134
[#116]: https://github.com/webern/cargo-readme/pull/116
[#130]: https://github.com/webern/cargo-readme/pull/130

## [3.3.2] - 2026-04-09

### Changed

- Update dependencies including a major version change (toml v0.8 to v1). [#124]
- Update GitHub Actions workflow to use main branch and checkout v3. [#122]

### Fixed

- Fix explicit lifetime elision warning in percent_encode function. [#122]

[#124]: https://github.com/webern/cargo-readme/pull/124
[#122]: https://github.com/webern/cargo-readme/pull/122

## [3.3.1] - 2023-11-06

### Changed

- Update dependencies including a major version change (toml v0.7 to v0.8). [#86]

### Fixed

- Circle CI badge was incorrect [#28]

[#86]: https://github.com/webern/cargo-readme/pull/86
[#28]: https://github.com/webern/cargo-readme/pull/28

## [3.3.0] - 2013-09-05

## Changed

- Update dependencies including major version changes: [#84]
  - clap v2 to v4
  - toml v0.5 tp v0.7
- Update documentation to reflect that the maintainer of this repository has changed/ [#84]

[#84]: https://github.com/webern/cargo-readme/pull/84

[unreleased]: https://github.com/webern/cargo-readme/compare/v3.3.3...HEAD
[3.3.3]: https://github.com/webern/cargo-readme/compare/v3.3.2...v3.3.3
[3.3.2]: https://github.com/webern/cargo-readme/compare/v3.3.1...v3.3.2
[3.3.1]: https://github.com/webern/cargo-readme/compare/v3.3.0...v3.3.1
[3.3.0]: https://github.com/webern/cargo-readme/compare/v3.2.0...v3.3.0
