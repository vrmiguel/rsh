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
use std::io;

fn main() -> io::Result<()>
{
    let mut config = CLIInput { is_verbose: false, exit: false };
    rshio::cli(&mut config);
    if config.exit {
        std::process::exit(0);
    }

    let mut os = rshio::get_user_data();
    println!("rsh - github.com/vrmiguel/rsh");
    loop 
    {
        rshio::prompt(&os);
        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            println!("EOF found. Exiting.");
            break;
        }
        if input.trim().is_empty()
        {
            continue;
        }
        let mut tokens: Vec<String> = input.split(" ").map(str::to_string).collect();
        let mut command: Vec<String> = Vec::new();
        command.push(tokens[0].clone());
        tokens.remove(0);
        command.push(tokens.join(" "));

        if config.is_verbose
        {
            println!("bin: {}", command[0].trim());
            println!("args: {}", command[1].trim());
        }
        rshexec::run(&command, &mut config, &mut os);
        if config.exit {
            break;
        }        
    }
    Ok(())
}