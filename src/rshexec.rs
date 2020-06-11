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
use pipers::Pipe;
use std::{process::{Command}, panic, env, path::Path};

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
            let mut child = Command::new(cmd.trim()).arg(&(tokens[1].trim())).spawn().expect("Problem running command.");
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

pub fn piped_command(input: &String, config:& rshio::CLIInput, c: usize)
{
    // TODO: redo this entire function, please

    if config.is_verbose {
        println!("Running rshexec::run");
    }

    // Getting the strings for both commands
    let _commands: Vec<String> = input.split("|").map(str::to_string).collect();  // We know by know that there's a single pipe character here

    if c==1 {
        let out = Pipe::new(&(_commands[0].trim()))
              .then(&(_commands[1].trim()))  
              .finally()         
              .expect("Commands did not pipe")
              .wait_with_output()
              .expect("failed to wait on child");
        println!("{}", String::from_utf8(out.stdout).unwrap().trim());
    }

    else if c==2 {
        let out = Pipe::new(&(_commands[0].trim()))
              .then(&(_commands[1].trim()))
              .then(&(_commands[2].trim()))
              .finally()         
              .expect("Commands did not pipe")
              .wait_with_output()
              .expect("failed to wait on child");
        println!("{}", String::from_utf8(out.stdout).unwrap().trim());
    }
    else {
        println!("Not implemented yet.");
    }
}

pub fn run(tokens: &Vec<String>, config: &mut rshio::CLIInput, os: &mut rshio::OS)
{
    if config.is_verbose {
        println!("Running rshexec::run");
    }

    let first = tokens[0].trim();

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
