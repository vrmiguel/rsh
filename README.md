# rsh

Unix shell written in Rust. This is an experiment in learning Rust.

![Screenshot](https://user-images.githubusercontent.com/36349314/84598912-82947d00-ae44-11ea-8673-6e76574304e9.png)


## Features

* Runs any program in `/bin` that does not require `sudo`;
* Implements `cd`;
* Save to file with `>`.
* Signal handler;
* Prompt similar to `bash`'s default.
* Piped commands, currently limited to a single pipe.

## Build

Run `cargo build`.

## Dependencies

* signal-hook - 0.1.15
* whoami      - 0.8.1
* dirs        - 2.0.2
* termion     - 1.5.5
