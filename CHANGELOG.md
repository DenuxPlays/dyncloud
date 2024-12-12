# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

We use an updated version of the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format.
<br>
What we've changed:

- We use a nicer title format (Version x.x.x (DD.MM.YYYY)) that plays well with `cargo-dist`.
- We add `**[Breaking]**` to breaking changes to make them stand out more.
  <br>

We do this to make it easier to read when using `cargo-dist` to generate the release notes from the changelog.

## unreleased

### Added

- added build for "aarch64-unknown-linux-gnu"
- added build for "aarch64-pc-windows-msvc"

### Changed

- updated dependencies
- updated to latest `cargo-dist`
- updated docker image to use alpine 3.21

## Version 1.2.3 (04.12.2024)

### Changed

- updated dependencies
- fixed CVE [GHSA-wwq9-3cpr-mm53](https://github.com/DenuxPlays/dyncloud/security/dependabot/5)

## Version 1.2.2 (04.11.2024)

### Changed

- updated dependencies

## Version 1.2.1 (22.08.2024)

### Fixed

- fixed cloudflare api

### Changed

- updated dependencies
- updated to latest `cargo-dist`
- updated rust

## Version 1.2.0 (12.07.2024)

### Added

- docker image

## Version 1.1.0 (12.07.2024)

### Added

- introduced cargo-dist
- enhanced installers
- added updater

## Version 1.0.4 (05.03.2024)

### Changed

- updated dependencies

## Version 1.0.3 (05.02.2024)

### Changed

- updated dependencies

## Version 1.0.2 (04.12.2023)

### Changed

- only log when updated -> prevent log spam

## Version 1.0.1 (03.12.2023)

### Fixed

- windows build

## Version 1.0.0 (01.12.2023)

- initial release
