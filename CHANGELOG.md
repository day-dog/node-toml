# Changelog

## [0.0.1] - 2021-02-17

### Features

- `parse` function to convert toml strings to JSON synchronously

## [0.0.2] - 2021-02-18

### Added

- `parseSync` function to convert toml strings to JSON both synchronously
- All function add js error that you can catch in Node.js application
- unit test

### Changed

- `parse` function becomes asynchronously

### Refactored

- Simplified toml parsing logic