# rsh

Unix shell written in Rust. This is an experiment in learning Rust.

## Features

* Run any program in `/bin`.
* Implements `cd`.
* Signal handler
* Piped commands, currently limited to a single pipe.

## Build

Run `cargo build`.

## Dependencies

* signal-hook - 0.1.15
* whoami      - 0.8.1
* dirs        - 2.0.2
* termion     - 1.5.5
