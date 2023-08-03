# Jsonx

A JSON transformer, written in Rust.

Provides fast pretty-printing and minimizing of JSON-encoded strings
and streams, at the command line or within Rust programs.

## Instalation

if cloned from repo

`cargo build`

it will generate compiled file named as `jsonx` under `target/debug/` folder

## Command-line Examples

Minify a file of json into one liner:

`jsonx -m <foo.json> foo-min.json`

Run `jsonx -h` to see all configuration options.

## Run test

`cargo test`

## Authorship and License

Copyright 2023, Tian Adi.

Jsonx is released under the MIT License.

