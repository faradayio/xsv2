# Changelog

All notable changes to xsv2 will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [0.14.1] - 2026-01-06

### Added

- **Automatic Encoding Detection**: Added support for automatic detection and conversion of different text encodings
  - UTF-16 LE (Little Endian) and UTF-16 BE (Big Endian) are now automatically detected and converted to UTF-8
  - Other encodings supported by the `encoding_rs` library are also handled
  - All commands now work consistently with UTF-16 encoded CSV files
  - Fixes the issue where `headers` and `select` commands had inconsistent behavior with UTF-16 files
  - Streaming implementation ensures minimal memory usage regardless of file size (tested with 2GB+ files)
  - New dependency: `encoding_rs_io` for streaming encoding detection and conversion

### Fixed

- Fixed inconsistency where `headers` command could display UTF-16 files but `select` command would fail with "Selector name does not exist" error
- **Index and Flexible Mode Incompatibility**: Indexes are now incompatible with flexible mode to prevent data corruption
  - The `index` command no longer accepts the `--flexible` flag
  - When `--flexible` is used on any command, any existing index will be automatically ignored
  - This prevents data corruption that could occur when using flexible parsing with byte-offset indexes

## [0.14.0] - 2025-12-26

### About xsv2

xsv2 is a modernized fork of the original [xsv](https://github.com/BurntSushi/xsv) project, which was retired. This fork aims to modernize the codebase while maintaining the simplicity and performance that made xsv great, without the extensive feature expansion seen in other projects like [qsv](https://github.com/dathere/qsv).

### Added

- **Compression Support**: Added comprehensive compression support for input and output

  - Gzip compression/decompression support
  - Zstd compression/decompression support
  - New `--compress` flag available on all commands that write output: `cat`, `fixlengths`, `fmt`, `frequency`, `input`, `join`, `partition`, `reverse`, `sample`, `search`, `select`, `slice`, `sort`, `split`, `stats`
  - Automatic detection of compressed input files based on file extension

- **Flexible CSV Parsing**: Added `--flexible` flag to allow CSV files with varying number of fields per record (jagged rows).

  - Available on all commands that read CSV input
  - It's the equivalent of passing things through `fixlengths`, except that it also returns jagged rows.

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
