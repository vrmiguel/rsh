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
use std::io::{Error, ErrorKind, Write};
use std::fs::File;

fn save_output(output: &str, out_file: &str) -> std::io::Result<()>
{
    let mut file = File::create(out_file)?;
    file.write_all(output.trim().as_bytes())?;
    Ok(())
}

fn simple_command(command: &str, args: Vec<&str>, config: &rshio::CLIInput, out_file: &str)
{
    if config.is_verbose {
        println!("Running rshexec::simple_command");
    }
    panic::set_hook(Box::new(|_info| {
        // Don't panic. It's all good.
    }));

    let cmd = String::from("/bin/") + command;

    let result = panic::catch_unwind(|| {   // I really need to refactor this
        if !args.is_empty()
        {
            if out_file.is_empty() {
                // In case there's no output file.
                // I'm doing this instead of just printing the _output variable on the clause below because ..
                // .. doing this has prettier outputs, in commands such as `ls`.
                let mut child = Command::new(cmd).args(args).spawn().expect("Problem running command.");
                let _ecode = child.wait().expect("Problem waiting for child.");
            } else 
            {
                // Save output to a file
                let child_proc = Command::new(cmd).args(args).output().expect("Problem running command.");
                let output = std::str::from_utf8(&child_proc.stdout).unwrap();
                if save_output(output.trim(), out_file.trim()).is_err() {
                    println!("rsh: failed to save the output of \"{} ..\" on {}", command, out_file);
                }
                else {
                    println!("rsh: saved output to {:?}", out_file.trim());
                }
            }
        }
        else 
        {
            // There are no arguments to the command.
            // This is in a different clause because in this case, we can't have the call to .args()
            if out_file.is_empty()
            {
                let mut child = Command::new(cmd).spawn().expect("Problem running command.");
                let _ecode = child.wait().expect("Problem waiting for child.");
            }
            else {
                let child_proc = Command::new(cmd).output().expect("Problem running command.");
                let output = std::str::from_utf8(&child_proc.stdout).unwrap();
                if save_output(output.trim(), out_file.trim()).is_err() {
                    println!("rsh: failed to save the output of {} on {}", command, out_file);
                } else {
                    println!("rsh: saved output to {:?}", out_file.trim());
                }
            }
        }
    });

    match result {
        Ok(res) => res,
        Err(_) => println!("rsh: problem running command {}", command),
    }

    //println!("{}", _output.trim());
}

fn change_dir(new_dir: String, config: &mut rshio::CLIInput, os: &mut rshio::OS)
{
    if config.is_verbose {
        println!("Running rshexec::change_dir");
    }
    if new_dir.is_empty() || new_dir == "~" || new_dir == "$HOME"
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
            println!("rsh: changing directory to {} failed.", os.hmd);
        }
    }
    else {
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
            println!("rsh: changing directory to {} failed.", new_dir);
        }
    }
}

pub fn piped_command(input: &str, config:& rshio::CLIInput, c: usize, output_file: &str) -> std::io::Result<()>
{
    if c > 1 {
        eprintln!("rsh: usoing more than one pipe is currently unimplemented.");
    }

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

    if output_file.trim().is_empty() {
        // No output file supplied: print to stdout
        println!("{}", &String::from_utf8(proc_2.stdout).unwrap().trim());
    } else {
        if save_output(&String::from_utf8(proc_2.stdout).unwrap().trim(), output_file).is_err()
        {
            println!("rsh: failed to save the output of \"{} ..\" on {}", input, output_file.trim());
        }
        else {
            println!("rsh: saved output to {:?}", output_file.trim());
        }
    }
    Ok(())
}


pub fn run(input: &String, config: &mut rshio::CLIInput, os: &mut rshio::OS)
{
    if config.is_verbose {
        println!("Running rshexec::run");
    }

    let mut tokens : std::str::SplitWhitespace;
    let mut output_file = "";


    let pipe_count  = input.matches("|").count();
    let redir_count = input.matches(">").count();
    if redir_count > 0 {
        // If there's been a '>' character
        if redir_count > 1 {
            println!("rsh: user supplied more '>' characters than supported.");
            return;
        }

        let mut temp_tokens = input.split(">");

        if pipe_count > 0
        {
            let status = piped_command(temp_tokens.next().unwrap(), &config, pipe_count, temp_tokens.next().unwrap());
            if status.is_err() {  // if rshexec::piped_command failed
                println!("rsh: problem running {:?}", input);
            }
            return;
        }

        tokens      = temp_tokens.next().unwrap().split_whitespace();
        output_file = temp_tokens.next().unwrap();
    } else {
        if pipe_count > 0 {
            let status = piped_command(input, &config, pipe_count, "");
            if status.is_err() {  // if rshexec::piped_command failed
                println!("rsh: problem running {:?}", input);
            }
            return;
        }
        tokens = input.split_whitespace();
    }
    let command    = tokens.next().unwrap();         // The name of the `/bin` command ..
    let args       = tokens.collect::<Vec<&str>>();  // .. and its arguments (optional)

    if command == "cd"
    {
        if !args.is_empty() {
            change_dir(args[0].to_string(), config, os);
        }
        else {
            change_dir("".to_string(), config, os);
        }
        return;
    }

    if command == "exit" || command == "quit"
    {
        // Doing this instead of an exit() because this guarantees destructors will be called.
        config.exit = true;
        return;
    }
    else {
        simple_command(command, args, config, output_file);
    }
}
