# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1] - 2021-08-29
### Changed
- Fixed shield.io badge paths

## [0.1.0] - 2021-08-29
### Added
- Add first version of the `esp32c-rt` crate
- Include build script for regular and for `direct-boot` mode (configurable
  via the feature `direct-boot`, which is enabled by default)
- Include memory layout for ESP32-C3 with feature `esp32-c3` which is enabled
  by default
- Compared to `riscv-rt`, removed support for all architectures other than
  `rv32imc` as these are not necessary in this context
