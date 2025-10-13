# Dyncloud

... is a program that checks if your public IP address has changed and updates the DNS records accordingly.

## Features

- multiple domains and records support
- IPv4 & IPv6 support
- multiple ways to get your public IP address
- automatic creation of DNS records if they don't exist

**Missing Features, resolvers or supported registries?**  
Just open an issue or a pull request.

## Examples

- [Example config](config.dist.toml)
- [Example Docker Compose file](compose.dist.yaml)

## Usage

The [config.dist.toml](config.dist.toml) is filled with example values. Copy it to `config.toml` and fill in your
values.  
**Remove anything you don't need.**

1. create a config file `config.toml` in the same directory as the executable. (
   see [config.dist.toml](config.dist.toml))
2. update the values in the config file
3. run the executable
    - all necessary DNS records will be created if they don't exist

## Cron

In V2 we migrated from a second interval to cron patterns.  
To see which patterns are supported take a look at the documentation of the library we use:
https://github.com/Hexagon/croner-rust/tree/v3.0.0?tab=readme-ov-file#pattern

If we can find the system timezone we use it otherwise we fall back to `Etc/UTC`.  
To set the timezone in docker pass the `TZ` environment variable to the container or see the
[Example Docker Compose file](compose.dist.yaml).

## Migration

With V2 we introduced a new config format to improve the usability.  
It is recommended that you rebuild your config using the [config.dist.toml](config.dist.toml) example file.
