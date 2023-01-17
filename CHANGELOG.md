# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.2.0] - 2022-01-17

### Changed

- **BREAKING**: Rename v1 upload format to v0
  - **You will need to update all existing uploads by changing `"version"` to `0` instead of `1` in `metadata.json`!**
- Return `204 No Content` in `/api/v1/delete/:id` endpoint
- `UploadMetadata::into_migrated_upload` is now a method (rather than associated function)

### Removed

- Unfinished API documentation page that was accidentally left in from development

## [v0.1.0] - 2022-01-17

- The first release.

[unreleased]: https://github.com/BBaoVanC/bobashare/compare/v0.2.0..HEAD
[v0.2.0]: https://github.com/BBaoVanC/bobashare/compare/v0.1.0..v0.2.0
[v0.1.0]: https://github.com/BBaoVanC/bobashare/releases/tag/v0.1.0
