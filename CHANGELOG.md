# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Internal Changes

All:

- Replace remaining uses of `chrono::Duration` type alias with the actual type
  (`chrono::TimeDelta`)
  - This shouldn't be a breaking change even though `cargo-public-api` says
    so; I can't imagine any way existing calling code would be broken. The only
    changed public api is `FileBackend::create_upload` where it is used as a
    parameter.

bobashare:

- Merge many separated module files into inline `pub mod ... {` definitions
- Reduce amount of submodules in `bobashare::serde::tests` to simplify structure

bobashare-web:

- Ignore `clippy::result_large_err` in `views::render_template`

## [v0.2.14] - 2025-01-26

### Bugfixes

- Fix crash on startup due to path matching syntax changing in axum 0.8

### Internal Changes

- Rewrite `bobashare_web::str_to_duration` to not use regex
  - This removes regex as a dependency entirely, since that was the only place
    it was used.
- Create `generate_delete_key` function in public api instead of
  generating it in a hidden closure in bobashare::storage::file

### Major Dependency Upgrades

- rand 0.8 -> 0.9

## [v0.2.13] - 2025-01-17

### Features

- Add configurable, custom about page
  - configured by setting `about_page` to the path of a markdown file to render

### Internal Changes

- Deduplicate the link preview meta tags in `<head>` set in template files; move
  to `base.html.jinja` instead
- Throw compile error if main block is not overridden in a template inheriting
  `base.html.jinja`
- Move markdown rendering code out of `bobashare-web/src/views/display.rs` and
  into main module, so it can be reused in about page rendering
- Replace usage of `Arc<AppState>` with `&'static AppState` which is leaked
  before starting web server
  - This feels a bit nicer because there's no need to reference count a type
    which must live for the entire program anyways.
- Don't potentially try to create a string with `usize::MAX` capacity when
  displaying a file
  - Pretty sure that since all relevant (>= 32 bits) platforms can fit 1048576
    in a usize, that this bug would never actually happen. But it shouldn't
    exist in the code.

## [v0.2.12] - 2024-11-04

### Bugfixes

- Fix seconds not being a supported duration unit in `str_to_duration`
  (originally introduced [8eeb4db]).

[8eeb4db]: https://github.com/BBaoVanC/bobashare/commit/8eeb4dbcf3df2bd5d92c73e3174eeb341740400d

### Internal Changes

- Entirely rewrite `TemplateState` to use references instead of cloning
  - Also remove askama_axum as it is easier to reimplement locally by rendering
    template to String manually
  - Also includes a major rewrite of how ErrorResponse works
- Make `CurrentNavigation` type used in `TemplateState` be `Copy` since it is a
  simple value enum
- Remove unnecessary lifetime parameter from ExpiryUnit since they are always
  `'static`

## [v0.2.11] - 2024-09-22

### Bugfixes

- Fix uncaught error if network connection drops in failed upload
  - This was also causing the popup error dialog not to be shown

### API (minor, non-breaking) Changes

- Use plaintext response to invalid routes on /api
  - Instead of default HTML template response, send a plaintext error message
    which is easier to deal with or read, i.e. when using curl directly.
  - **This message is not considered part of the stable API! Please do not match
    it directly in API clients; read the error code (404) in the response
    instead.**

### Visual Tweaks

- Add extra info bar to display page to show upload ID and view raw button
  (fixes [#4](https://github.com/BBaoVanC/bobashare/issues/4))
  - Browsers do not display the URL in an unambiguous font, so it can be hard to
    read the upload id by eye. Add a second bar so the id is displayed in a
    monospace font in the page body. Also make the filename monospace font so it
    is easy to read too. Two bars gives more space, so add a "View raw" button
    which is good for copy-pasting the entire file.
- Make current navbar selection white instead of blue and add margin below top
  bar on main upload page
  - This makes it easier to distinguish the "Upload" navbar item from the file
    upload form, so users are less likely to accidentally click it when trying
    to upload the current selected file.
- Fix filename not being vertically aligned on display page
- Set color-scheme: dark to make native elements use dark mode (partial fix for
  [#7](https://github.com/BBaoVanC/bobashare/issues/7))
  - This fixes the arrows looking light on the number input for expiry, but not
    the weird-looking pseudo-skeumorphic dropdown select box for the expiry unit
    on Safari.

### Internal Changes

- Destructure State directly in URL handler function parameters to clean up
  state.0 usage
- Remove redundant struct construction in `From<Arc<AppState> for TemplateState`
  - Exactly identical code was used in both the `From<Arc<AppState>>` and
    `From<&AppState>` impl's to construct `TemplateState`; instead, just call
    the `From<&AppState>` impl from the `From<Arc<AppState>>` to remove the
    redundancy.
- Remove obsolete leftover TemplateState::icon method
- Remove unnecessary allow(unused_variables) in `views::display::string_is_true`
  - Replace it with an underscore expression

#### Testing

- Add `cargo-public-api` test that fails if public API changes

## [v0.2.10] - 2024-08-07

- Upgrade dependencies
- Replace [duration_str] with simple regex implementation
  - This means that you can no longer use sums of durations (ie. `1d 5h 20m`).
    However, this feature may return in the future.

[duration_str]: https://docs.rs/duration-str/latest/duration_str/

### Documentation

- Fix indentation in `bobashare_web::Cli` list
- Fix headline for static_routes module

## [v0.2.9] - 2024-03-09

- Add rustdoc via GitHub Pages
- Add basic documentation
- Upgrade dependencies

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

[unreleased]: https://github.com/BBaoVanC/bobashare/compare/v0.2.14..HEAD
[v0.2.14]: https://github.com/BBaoVanC/bobashare/compare/v0.2.13..v0.2.14
[v0.2.13]: https://github.com/BBaoVanC/bobashare/compare/v0.2.12..v0.2.13
[v0.2.12]: https://github.com/BBaoVanC/bobashare/compare/v0.2.11..v0.2.12
[v0.2.11]: https://github.com/BBaoVanC/bobashare/compare/v0.2.10..v0.2.11
[v0.2.10]: https://github.com/BBaoVanC/bobashare/compare/v0.2.9..v0.2.10
[v0.2.9]: https://github.com/BBaoVanC/bobashare/compare/v0.2.8..v0.2.9
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
