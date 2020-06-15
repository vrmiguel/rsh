/*
 * rsh
 * https://github.com/vrmiguel/rsh
 *
 * Copyright (c) 2020 Vinícius R. Miguel <vinicius.miguel at unifesp.br>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

mod rshio;
mod rshexec;

// TODO: check for SIGINT

use crate::rshio::CLIInput;
use std::{io, thread, sync::Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook::{iterator::Signals};


fn main() -> io::Result<()>
{
    let mut config = CLIInput { is_verbose: false, exit: false };
    rshio::cli(&mut config);        // Reads command-line arguments.
    if config.exit {
        std::process::exit(0);
    }

    let signals = Signals::new(&[signal_hook::SIGINT, signal_hook::SIGHUP])?;

    let exit_signal = Arc::new(AtomicBool::new(false));   // Bool to be shared between threads: tells main if there's ..
                                                          // .. been a termination signal.
   
    let exit_signal1 = Arc::clone(&exit_signal);          // Clone exit_signal as to not move a value inside of closure.

    thread::spawn(move || {
        // Signal handler thread.
        for sig in signals.forever() {
            if sig == 2 {
                eprint!("\nSIGINT received: exiting. Press backspace. ");
                exit_signal1.swap(true, Ordering::Relaxed); // The loop below will see this is true and exit.
            }
        }
    });

    let mut os = rshio::get_user_data();
    println!("rsh - github.com/vrmiguel/rsh");

    loop
    {
        if exit_signal.load(Ordering::Relaxed) {
            // Checks if there's been a signal for interruption.
            break;
        }

        rshio::prompt(&os); // Prints the prompt
        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            println!("EOF found. Exiting.");
            break;
        }
        input = input.clone().trim().to_string();
        if input.is_empty()
        {
            continue;
        }
        rshexec::run(&input, &mut config, &mut os);
        if config.exit {
            break;
        }
    }
    Ok(())
}