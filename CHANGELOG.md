# Changelog

All notable changes to xsv2 will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### About xsv2

xsv2 is a modernized fork of the original [xsv](https://github.com/BurntSushi/xsv) project, which was retired. This fork aims to modernize the codebase while maintaining the simplicity and performance that made xsv great, without the extensive feature expansion seen in other projects like [qsv](https://github.com/dathere/qsv).

### Added

- **Compression Support**: Added comprehensive compression support for input and output

  - Gzip compression/decompression support
  - Zstd compression/decompression support
  - New `--compress` flag available on all commands that write output: `cat`, `fixlengths`, `fmt`, `frequency`, `input`, `join`, `partition`, `reverse`, `sample`, `search`, `select`, `slice`, `sort`, `split`, `stats`
  - Automatic detection of compressed input files based on file extension

- **Flexible CSV Parsing**: Added `--flexible` flag to allow CSV files with varying number of fields per record.

  - Available on all commands that read CSV input
  - Useful for working with malformed or inconsistent CSV files

- **GitHub Actions CI/CD**: Added automated workflows
  - CI workflow for testing on Linux, macOS, and Windows
  - Release workflow for building and publishing binaries

### Changed

- **Binary Name**: Renamed binary from `xsv` to `xsv2` (not considered a breaking change - just the executable name)
- **Rust Edition**: Upgraded from Rust 2018 to Rust 2021
- **Code Modernization**: Applied extensive clippy lints and code improvements
  - Modernized error handling patterns
  - Improved type inference and explicit type annotations
  - Enhanced code clarity and idiomatic Rust patterns
  - Refactored test suite for better maintainability
- **Dependency Updates**: Updated all dependencies to their latest compatible versions

---

## Historical Context

xsv2 builds upon the excellent foundation of [xsv](https://github.com/BurntSushi/xsv) by Andrew Gallant (@BurntSushi). The original xsv was a fast CSV command line toolkit written in Rust that pioneered many efficient CSV processing techniques.
