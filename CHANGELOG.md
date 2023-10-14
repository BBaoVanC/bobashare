# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Add rustdoc via GitHub Pages
- Add basic documentation

## [v0.2.8] - 2023-09-18

- Fix Docker image by upgrading Debian

## [v0.2.7] - 2023-09-18

- Upgrade dependencies

## [v0.2.6] - 2023-05-05

- Upgrade dependencies

## [v0.2.5] - 2023-04-11

- Upgrade dependencies :: see axum 0.6.13 to 0.6.14

## [v0.2.4] - 2023-04-04

- Upgrade all dependencies

## [v0.2.3] - 2023-03-06

### Fixed

- Return 404 instead of 500 on `/raw/:id` when upload can't be found

## [v0.2.2] - 2023-01-31

### Added

- Display version of bobashare (from `Cargo.toml`) on footer ([9d646d1])
- Hover tooltip for limits in footer (file size in bytes, max expiry in seconds) ([24af218])

### Fixed

- Better keyboard support on JS-powered links (delete/cancel upload buttons) ([cfc2818])
  - rather than make the `keypress` event trigger the `click` event, just set an `href` and `click` magically works
- Properly pluralize human durations (if 1 of the unit) ([113e257])
- Return an error when cleaning up an upload with unknown metadata version instead of deleting it ([f29996c])

[9d646d1]: https://github.com/BBaoVanC/bobashare/commit/9d646d1d0c1ceccd341c92fbc2fd04359710efc2
[24af218]: https://github.com/BBaoVanC/bobashare/commit/24af218e0f26be758a0ca917a1d9d71f436f9373
[cfc2818]: https://github.com/BBaoVanC/bobashare/commit/cfc2818747bccfcc4462d8805f9c8760abfa3338
[113e257]: https://github.com/BBaoVanC/bobashare/commit/113e257d479b750ca605075ba08759dbc7fb6e9e
[f29996c]: https://github.com/BBaoVanC/bobashare/commit/f29996cd69fc98401afd40e924a253fc3ce6fd7f

## [v0.2.1] - 2023-01-21

### Added

- Log a message when ErrorResponse is return from a view (24424bd5acf564de36c07e17839552f3c171b1fb)

### Changed

- Override `Content-Type` header if the file is valid UTF-8 (plaintext) (5f382f692cc63989ab0ccfc437251315b589f7cb and 900fae603c35129521a4f5ec60bacca61675f32d)
- Replace `chrono-humanize` dependency with simple logic for English durations (04b90eb2069d16bbbd84a35b615bf5942409e3c9 and 4d6f3dc43978082093771d724a92ec41f52428ca)
  - See `bobashare_web::views::filters::humanduration`

### Fixed

- Change upload page text from "Click or drop files here" to "Click to select files" because you cannot actually drop files currently. (7cc7197ff1c042a369d88836d2c04f3c85e5b5db)

## [v0.2.0] - 2023-01-17

### Changed

- **BREAKING**: Rename v1 upload format to v0
  - **You will need to update all existing uploads by changing `"version"` to `0` instead of `1` in `metadata.json`!**
- Return `204 No Content` in `/api/v1/delete/:id` endpoint
- `UploadMetadata::into_migrated_upload` is now a method (rather than associated function)

### Removed

- Unfinished API documentation page that was accidentally left in from development

## [v0.1.0] - 2023-01-17

- The first release.

[unreleased]: https://github.com/BBaoVanC/bobashare/compare/v0.2.8..HEAD
[v0.2.8]: https://github.com/BBaoVanC/bobashare/compare/v0.2.7..v0.2.8
[v0.2.7]: https://github.com/BBaoVanC/bobashare/compare/v0.2.6..v0.2.7
[v0.2.6]: https://github.com/BBaoVanC/bobashare/compare/v0.2.5..v0.2.6
[v0.2.5]: https://github.com/BBaoVanC/bobashare/compare/v0.2.4..v0.2.5
[v0.2.4]: https://github.com/BBaoVanC/bobashare/compare/v0.2.3..v0.2.4
[v0.2.3]: https://github.com/BBaoVanC/bobashare/compare/v0.2.2..v0.2.3
[v0.2.2]: https://github.com/BBaoVanC/bobashare/compare/v0.2.1..v0.2.2
[v0.2.1]: https://github.com/BBaoVanC/bobashare/compare/v0.2.0..v0.2.1
[v0.2.0]: https://github.com/BBaoVanC/bobashare/compare/v0.1.0..v0.2.0
[v0.1.0]: https://github.com/BBaoVanC/bobashare/releases/tag/v0.1.0
