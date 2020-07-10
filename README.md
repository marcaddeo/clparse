# `clparse`
`clparse` is a command line tool for parsing CHANGELOG files that use the Keep
A Changelog format. The CHANGELOG file can be Markdown, JSON, or YAML format
and can also be output into any of those formats once parsed.

## Installation

### Homebrew
```
$ brew install marcaddeo/clsuite/clparse
```

### Debian
```
$ curl -LO https://github.com/marcaddeo/clparse/releases/download/0.7.0/clparse_0.7.0_amd64.deb
$ sudo dpkg -i clparse_0.7.0_amd64.deb
```

### Linux
```
$ curl -LO https://github.com/marcaddeo/clparse/releases/download/0.7.0/clparse-0.7.0-x86_64-unknown-linux-musl.tar.gz
$ tar czvf clparse-0.7.0-x86_64-unknown-linux-musl.tar.gz
$ sudo mv clparse /usr/local/bin/clparse
```

### Cargo
```
$ cargo install clparse
```

## Usage
```
clparse 0.7.0
Marc Addeo <hi@marc.cx>
A command line tool for parsing CHANGELOG.md files that use the Keep A Changelog format.

USAGE:
    clparse [OPTIONS] <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --format <format>    Sets the output format of the parsed CHANGELOG [default: markdown] [possible values: json,
                             yaml, yml, markdown, md]

ARGS:
    <FILE>    The CHANGELOG file to parse. This should be either a Markdown, JSON, or Yaml representation of a
              changelog. Use '-' to read from stdin.
```

### Examples
By default, `clparse` will parse the input file, and output the changelog in
markdown format.

```markdown
$ clparse CHANGELOG.md
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Fix a bug that caused undefined behavior

## [1.0.0] - 2019-06-20
### Added
- Add a cool new feature

### Changed
- Change something that was notable

## [0.0.1] - 2019-05-31
### Added
- Add the initial features

[Unreleased]: https://github.com/example/example/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/example/example/compare/v0.0.1...v1.0.0
[0.0.1]: https://github.com/example/example/releases/tag/v0.0.1
```

We can transform the markdown into a format that can be easily used in scripts:

```json
$ clparse -f json CHANGELOG.md
{
  "title": "Changelog",
  "description": "All notable changes to this project will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n",
  "releases": [
    {
      "version": null,
      "link": "https://github.com/example/example/compare/v1.0.0...HEAD",
      "date": null,
      "changes": [
        {
          "fixed": "Fix a bug that caused undefined behavior"
        }
      ],
      "yanked": false
    },
    {
      "version": "1.0.0",
      "link": "https://github.com/example/example/compare/v0.0.1...v1.0.0",
      "date": "2019-06-20",
      "changes": [
        {
          "added": "Add a cool new feature"
        },
        {
          "changed": "Change something that was notable"
        }
      ],
      "yanked": false
    },
    {
      "version": "0.0.1",
      "link": "https://github.com/example/example/releases/tag/v0.0.1",
      "date": "2019-05-31",
      "changes": [
        {
          "added": "Add the initial features"
        }
      ],
      "yanked": false
    }
  ]
}
```

`clparse` can also parse JSON and YAML representations of the changelog:

```markdown
$ clparse CHANGELOG.json
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Fix a bug that caused undefined behavior

## [1.0.0] - 2019-06-20
### Added
- Add a cool new feature

### Changed
- Change something that was notable

## [0.0.1] - 2019-05-31
### Added
- Add the initial features

[Unreleased]: https://github.com/example/example/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/example/example/compare/v0.0.1...v1.0.0
[0.0.1]: https://github.com/example/example/releases/tag/v0.0.1
```

And finally, we can read from stdin by passing `-` as the `FILE` argument:

```markdown
$ clparse -f json CHANGELOG.md | clparse -
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Fixed
- Fix a bug that caused undefined behavior

## [1.0.0] - 2019-06-20
### Added
- Add a cool new feature

### Changed
- Change something that was notable

## [0.0.1] - 2019-05-31
### Added
- Add the initial features

[Unreleased]: https://github.com/example/example/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/example/example/compare/v0.0.1...v1.0.0
[0.0.1]: https://github.com/example/example/releases/tag/v0.0.1
```
