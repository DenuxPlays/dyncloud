# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

We use an updated version of the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format.

What we've changed:

- We use a nicer title format (Version x.x.x (DD.MM.YYYY)) that plays well with `cargo-dist`.
- We add `**[Breaking]**` to breaking changes to make them stand out more.
  <br>

We do this to make it easier to read when using `cargo-dist` to generate the release notes from the changelog.

## unreleased

### Added

- x64 freebsd support

## Version 2.0.0 (15.10.2025)

### Breaking

- Convert dyncloud to a CLI
- Updated Config format: See migration guide for further instructions
- Replace interval with cron expression [#136](https://github.com/DenuxPlays/dyncloud/issues/136)

### Added

- Added IPv6 docker compose config example
- Added docker timezone support
- Added migration guide
- Added V2 Documentation
- `--debug`/`-d` flag to enable debug logging
- Added proper Config validation with user-friendly output
- Get Cloudflare Record by DNSType and Name [#107](https://github.com/DenuxPlays/dyncloud/issues/107)
- Added utility command to list cloudflare zones
- Added command to sync records just one time
- support for Windows arm64
- added colored output

### Changed

- added timestamp to log messages
- renamed `compose.yaml.dist` to `compose.dist.yaml`
- updated dependencies
- introduced custom logger
- Log errors instead of crashing [#108](https://github.com/DenuxPlays/dyncloud/issues/108)
- The Docker Image can now be used as a standalone CLI
- updated rust to 1.90.0

## Version 2.0.0-beta.1 (13.10.2025)

### Added

- Added IPv6 docker compose config example
- Added docker timezone support
- Added migration guide
- Added V2 Documentation

### Changed

- added timestamp to log messages
- renamed `compose.yaml.dist` to `compose.dist.yaml`
- updated dependencies
s
## Version 2.0.0-alpha.5 (10.10.2025)

### Added

- `--debug`/`-d` flag to enable debug logging

### Changed

- updated dependencies
- introduced custom logger

## Version 2.0.0-alpha.1 (09.10.2025)

### Breaking

- Convert dyncloud to a CLI
- Updated Config format: See migration guide for further instructions
- Replace interval with cron expression [#136](https://github.com/DenuxPlays/dyncloud/issues/136)

### Added

- Added proper Config validation with user-friendly output
- Get Cloudflare Record by DNSType and Name [#107](https://github.com/DenuxPlays/dyncloud/issues/107)
- Added utility command to list cloudflare zones
- Added command to sync records just one time
- support for Windows arm64

### Changed

- Log errors instead of crashing [#108](https://github.com/DenuxPlays/dyncloud/issues/108)
- The Docker Image can now be used as a standalone CLI
- updated rust to 1.90.0

## Version 1.5.4 (12.09.2025)

### Changed

- updated cargo-dist
- use mimalloc v3
- updated dependencies

## Version 1.5.3 (23.07.2025)

### Changed

- updated rust
- updated dependencies
- updated cargo-dist
- updated alpine to 3.22

## Version 1.5.2 (31.03.2025)

### Fixed

- fixed [GHSA-4p46-pwfr-66x6](https://github.com/DenuxPlays/dyncloud/security/dependabot/7)
    - We should not be affected by this, but we updated the dependency to be sure.
- Fixed cloudflare API error

### Changed

- updated cargo-dist
- updated dependencies
- updated to latest rust

## Version 1.5.1 (12.12.2024)

### Fixed

- docker workflow

### Changed

- optimized ipifiy implementation

## Version 1.5.0 (12.12.2024)

### Added

- add own implementation that uses `ipify` for getting the public ip

### Fixed

- fixed [CVE-2024-12224](https://github.com/DenuxPlays/dyncloud/security/dependabot/6)

## Version 1.4.0 (12.12.2024)

### Added

- added build for "aarch64-unknown-linux-musl"

### Removed

- removed build for "aarch64-pc-windows-msvc" (was not working so no breaking change here)

## Version 1.3.0 (12.12.2024)

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
