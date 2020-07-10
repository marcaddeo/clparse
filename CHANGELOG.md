# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0] - 2020-07-10
### Fixed
- Fix bug that caused wrapped lines to be missing spaces between words at the
  newline boundary

## [0.5.0] - 2020-02-07
### Added
- Add getters to the `Changelog` and `Release` structs
- Add a `unreleased_changes` method to the `Changelog` struct

### Changed
- Ensure change entries are wrapped at 80 characters

## [0.4.0] - 2019-10-11
### Added
- Add support for link references in the CHANGELOG description
- Add Linux (musl) installation instructions

### Fixed
- Fix inconsistencies in the README examples
- Fix error when piping the output of `clparse`

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

### Removed
- Remove unit tests since they didn't actually test anything

### Fixed
- Fix typo in the crate description

## [0.1.0] - 2019-10-07
### Added
- Initial implementation of `clparse` library and binary

[Unreleased]: https://github.com/marcaddeo/clparse/compare/0.6.0...HEAD
[0.6.0]: https://github.com/marcaddeo/clparse/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/marcaddeo/clparse/compare/0.4.0...0.5.0
[0.4.0]: https://github.com/marcaddeo/clparse/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/marcaddeo/clparse/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/marcaddeo/clparse/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/marcadde/clparse/releases/tag/0.1.0
