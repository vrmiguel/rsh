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

use crate::rshio;
use std::{process::{Stdio,Command}, panic, env, path::Path};
use std::os::unix::io::{FromRawFd, AsRawFd};
use std::io::{Error, ErrorKind};

fn simple_command(command: &str, args: Vec<&str>, config: &rshio::CLIInput)
{
    if config.is_verbose {
        println!("Running rshexec::simple_command");
    }
    panic::set_hook(Box::new(|_info| {
        // Don't panic. It's all good.
    }));

    let cmd = String::from("/bin/") + command;

    let result = panic::catch_unwind(|| 
    {
        if !args.is_empty()
        {
            let mut child = Command::new(cmd).args(args).spawn().expect("Problem running command.");
            let _ecode = child.wait().expect("Problem waiting for child.");
        }
        else {
            let mut child = Command::new(cmd).spawn().expect("Problem running command.");
            let _ecode = child.wait().expect("Problem waiting for child.");
        }
    });

    match result {
        Ok(res) => res,
        Err(_) => println!("rsh: problem running command {}", command),
    }
}

fn change_dir(tokens: &Vec<&str>, config: &mut rshio::CLIInput, os: &mut rshio::OS)
{
    if config.is_verbose {
        println!("Running rshexec::change_dir");
    }
    if tokens.is_empty() || tokens[1] == "~" || tokens[1] == "$HOME"
    {
        // User wants to go to $HOME
        let home = Path::new(&os.hmd);
        if env::set_current_dir(&home).is_ok()
        {
            os.cwd.clear();
            os.cwd = os.hmd.clone();
            os.cwd_pp.clear();
            os.cwd_pp = String::from("~");
        }
        else {
            println!("Changing directory to {} failed.", os.hmd);
        }
    }
    else {
        let new_dir = tokens.join(" ");
        let next_dir = Path::new(&new_dir);
        if env::set_current_dir(&next_dir).is_ok()
        {
            os.cwd.clear();
            rshio::OS::get_cwd(os);
            os.cwd_pp.clear();
            if config.is_verbose {
                println!("New cwd: {:?}", os.cwd);
            }
            os.cwd_pp = os.cwd.replace(&os.hmd, "~");
        }
        else {
            println!("Changing directory to {} failed.", tokens[1]);
        }
    }
}



pub fn piped_command(input: &String, config:& rshio::CLIInput, _c: usize) -> std::io::Result<()>
{
    // TODO: redo this entire function, please

    if config.is_verbose {
        println!("Running rshexec::run");
    }

    // Getting the strings for both commands
    let commands: Vec<String> = input.split("|").map(str::to_string).collect();  // We know by know that there's a single pipe character here

    // Setting up command 1.
    let mut split_1 = commands[0].split_whitespace();
    let command_1 = match split_1.next() {
            Some(x) => x,
            None =>  return Err(Error::new(ErrorKind::Other, "no command to run.")),
        };
    let args_cmd_1  = split_1.collect::<Vec<&str>>();

    // Setting up command 2.
    let mut split_2 = commands[1].split_whitespace();
    let command_2 = match split_2.next() {
            Some(x) => x,
            None =>  return Err(Error::new(ErrorKind::Other, "no command to run.")),
        };
    let args_cmd_2  = split_2.collect::<Vec<&str>>();

    let proc_1: std::result::Result<std::process::Child, std::io::Error>;
    if !args_cmd_1.is_empty()
    {
        proc_1 = Command::new(command_1).args(args_cmd_1.as_slice()).stdout(Stdio::piped()).spawn();
    } else {
        proc_1 = Command::new(command_1).stdout(Stdio::piped()).spawn();
    }

    let stdout = match proc_1 {
            Ok(proc_1) => match proc_1.stdout {
                Some(stdout) => stdout,
                None => return Err(Error::new(ErrorKind::Other, "no command to run")),
            },
            Err(e) => return Err(e),
        };

    let stdio  = unsafe{Stdio::from_raw_fd(stdout.as_raw_fd())};
    
    let proc_2: std::process::Output;

    if !args_cmd_2.is_empty()
    {
        proc_2 = Command::new(command_2).args(args_cmd_2.as_slice()).stdout(Stdio::piped()).stdin(stdio).spawn()
                .expect("Commands did not pipe")
                .wait_with_output()
                .expect("failed to wait on child");
    } else {
        proc_2 = Command::new(command_2).stdout(Stdio::piped()).stdin(stdio).spawn()
                .expect("Commands did not pipe")
                .wait_with_output()
                .expect("failed to wait on child");
    }

    println!("{}", &String::from_utf8(proc_2.stdout).unwrap().trim());
    Ok(())
}


pub fn run(input: &String, config: &mut rshio::CLIInput, os: &mut rshio::OS)
{
    if config.is_verbose {
        println!("Running rshexec::run");
    }

    let mut tokens = input.split_whitespace();
    let command    = tokens.next().unwrap();
    let args       = tokens.collect::<Vec<&str>>();

    if command == "cd"
    {
        change_dir(&args, config, os);
        return;
    }

    if command == "exit" || command == "quit"
    {
        config.exit = true;
        return;
    }
    else {
        simple_command(command, args, config);
    }
}
