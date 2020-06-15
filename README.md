# rsh

Unix shell written in Rust. This is an experiment in learning Rust.

![Screenshot](https://user-images.githubusercontent.com/36349314/84598912-82947d00-ae44-11ea-8673-6e76574304e9.png)


## Features

* Runs any program in `/bin` that does not require `sudo`;
* Piped commands, currently limited to a single pipe.
* Save the output of simple and piped commands to a file with `>`.
    * e.g. `ls -li | tr s x > output`
* Signal handler;
* Implements `cd`;
* Unwinds before exit, interrupted or not;
* Prompt similar to `bash`'s default.

## Build

Run `cargo build -- release` (or `cargo build` for a debug build) inside the `rsh-master` folder.

## Dependencies

* signal-hook - 0.1.15
* whoami      - 0.8.1
* dirs        - 2.0.2
* termion     - 1.5.5
