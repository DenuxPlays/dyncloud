# Dyncloud

A programm that handles updating DNS records with your current public IP address for Cloudflare.

## Features

- multiple domains and records support

## Usage

The [config.toml.dist](config.toml.dist) is filled with example values. Copy it to `config.toml` and fill in your values.
**Remove anything you don't need.**

1. create an A (or AAAA) record in Cloudflare
2. create a config file `config.toml` in the same directory as the executable. (see [config.toml.dist](config.toml.dist))
3. update the values in the config file
4. run the executable
