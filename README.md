# Dyncloud

... is a program that checks if you public IP address has changed and updates the DNS records accordingly.

## Features

- multiple domains and records support
- IPv4 & IPv6 support
- multiple ways to get your public IP address
- automatic creation of DNS records if they don't exist

## Usage

The [config.toml.dist](config.toml.dist) is filled with example values. Copy it to `config.toml` and fill in your
values.  
**Remove anything you don't need.**

1. create a config file `config.toml` in the same directory as the executable. (
   see [config.toml.dist](config.toml.dist))
2. update the values in the config file
3. run the executable
    - all necessary DNS records will be created if they don't exist
