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

fn simple_command(tokens: &Vec<String>, config: &rshio::CLIInput)
{
    if config.is_verbose {
        println!("Running rshexec::simple_command");
    }
    panic::set_hook(Box::new(|_info| {
        // Don't panic. It's all good.
    }));

    let cmd = String::from("/bin/") + &tokens[0];

    let result = panic::catch_unwind(|| 
    {
        if !tokens[1].trim().is_empty()
        {
            let args1: Vec<&str> = tokens.iter().skip(1).map(|s| &**s).collect(); // Optimize this
            let mut child = Command::new(cmd.trim()).args(args1).spawn().expect("Problem running command.");
            let _ecode = child.wait().expect("Problem waiting for child.");
        }
        else {
            let mut child = Command::new(cmd.trim()).spawn().expect("Problem running command.");
            let _ecode = child.wait().expect("Problem waiting for child.");
        }
    });

    match result {
        Ok(res) => res,
        Err(_) => println!("rsh: problem running command {}", tokens[0].trim()),
    }
}

fn change_dir(tokens: &Vec<String>, config: &mut rshio::CLIInput, os: &mut rshio::OS)
{
    if config.is_verbose {
        println!("Running rshexec::change_dir");
    }
    if tokens[1].trim().is_empty() || tokens[1].trim() == "~" || tokens[1].trim() == "$HOME"
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
        let tok_trim = tokens[1].trim();
        let next_dir = Path::new(&tok_trim);
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
            println!("Changing directory to {} failed.", tok_trim);
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
    let command1_str = commands[0].clone();
    let command2_str = commands[1].clone();
    drop(commands);

    // Setting up command 1.
    let mut command1_tokens: Vec<String> = command1_str.trim().split(" ").map(str::to_string).collect();
    drop(command1_str);
    let mut command1: Vec<String> = Vec::new();
    command1.push(command1_tokens[0].clone());
    command1_tokens.remove(0);
    command1.push(command1_tokens.join(" "));
    drop(command1_tokens);

    // Setting up command 2.
    let mut command2_tokens: Vec<String> = command2_str.trim().split(" ").map(str::to_string).collect();
    drop(command2_str);
    let mut command2: Vec<String> = Vec::new();
    command2.push(command2_tokens[0].clone());
    command2_tokens.remove(0);
    command2.push(command2_tokens.join(" "));
    drop(command2_tokens);

    let args1: Vec<&str> = command1.iter().skip(1).map(|s| &**s).collect(); // Optimize this
    let proc_1 = Command::new(&command1[0]).args(args1).stdout(Stdio::piped()).spawn();

    let stdout = match proc_1 {
            Ok(proc_1) => match proc_1.stdout {
                Some(stdout) => stdout,
                None => return Err(Error::new(ErrorKind::Other, "no command to run")),
            },
            Err(e) => return Err(e),
        };

    let stdio  = unsafe{Stdio::from_raw_fd(stdout.as_raw_fd())};
    
    let args2: Vec<&str> = command2.iter().skip(1).map(|s| &**s).collect(); // Optimize this
    let proc_2 = Command::new(&command2[0]).args(args2).stdout(Stdio::piped()).stdin(stdio).spawn()
                .expect("Commands did not pipe")
                .wait_with_output()
                .expect("failed to wait on child");

    println!("{}", &String::from_utf8(proc_2.stdout).unwrap().trim());
    Ok(())
}


pub fn run(tokens: &Vec<String>, config: &mut rshio::CLIInput, os: &mut rshio::OS)
{
    if config.is_verbose {
        println!("Running rshexec::run");
    }

    let first = &tokens[0];

    if first == "cd"
    {
        change_dir(tokens, config, os);
        return;
    }

    if first == "exit" || first == "quit"
    {
        config.exit = true;
        return;
    }
    else {
        simple_command(tokens, config);
    }
}
