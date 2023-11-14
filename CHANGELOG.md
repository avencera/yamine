# Changelog

## [Unreleased]

## [v0.4.0] - 2023-11-13

- One yaml file output json without nesting in an array
- Works with files ending in `.yml`
- Updated dependencies

### Bug Fixes

- Multiple files output a yaml file seperated by `---`
- Message if 0 files found

## [v0.3.0] - 2023-09-13

- One yaml file output json without nesting in an array

## [v0.2.0] - 2022-08-28

### Features

- Can now read and convert from `STDIN`

### Breaking

- `--std-out` is now `--stdout`

### Internal

- Switched from structopt to stdin
- Upgraded dependencies

## [v0.1.2] - 2021-09-29

- Upgraded dependencies

## [v0.1.1] - 2021-09-29

- Fixed help prompt for outputting json
- Output now accepts `json` and `json-array`

## [v0.1.0] - 2021-07-05

- Initial release
