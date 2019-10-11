# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2019-10-10
### Added
- Add a build script to build releases for each target
- Add installation instructions for Debian

### Removed
- Remove the `fstrings` crate so we can target Linux musl

## [0.2.0] - 2019-10-08
### Added
- Add a Homebrew installation option

### Changed
- Revise error handling
- Update binary to show the help output if no args are passed

### Fixed
- Fix typo in the crate description

### Removed
- Remove unit tests since they didn't actually test anything

## [0.1.0] - 2019-10-07
### Added
- Initial implementation of `clparse` library and binary

[Unreleased]: https://github.com/marcaddeo/clparse/compare/0.3.0...HEAD
[0.3.0]: https://github.com/marcaddeo/clparse/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/marcaddeo/clparse/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/marcadde/clparse/releases/tag/0.1.0
