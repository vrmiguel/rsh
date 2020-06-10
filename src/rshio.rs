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

pub struct CLIInput {
    pub is_verbose: bool,
    pub exit: bool
}

use {std::{env, io, io::Write}, whoami, dirs, termion::color};

pub struct OS {           // User data
    username:   String,   // Name of the user that started rsh
    hostname:   String,   // Hostname of the user that started rsh
    pub cwd:    String,   // Current working directory
    pub hmd:    String,   // User's home directory
    pub cwd_pp: String    // cwd but for prompt pretty-printing
}

pub fn cli(config: &mut CLIInput) 
{
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args
    {
        if arg.trim() == "-h" || arg.trim() == "--help"
        {
            println!("Haha help here");
            config.exit = true;
        }

        else if arg.trim() == "-v" || arg.trim() == "--verbose"
        {
            println!("Verbose mode on");
            config.is_verbose = true;
        }

        else 
        {
            println!("{} is not a valid argument.", arg);
        }
    }
}

pub fn get_user_data() -> OS
{
    let cwd = env::current_dir().unwrap();
    let cwd_str: String = cwd.as_os_str().to_str().unwrap().to_string();
    drop(cwd);

    let hmd = dirs::home_dir().unwrap();
    let hmd_str: String = hmd.as_os_str().to_str().unwrap().to_string();
    drop(hmd);

    let pp_path = cwd_str.replace(&hmd_str, "~");

    let os = OS 
    {
        cwd: cwd_str,
        cwd_pp: pp_path,
        hmd: hmd_str,
        username: whoami::username(),
        hostname: whoami::hostname()
    };

    return os;
}

pub fn prompt(os: &OS)
{
    print!("{}{}@{}{}:{}{}{}$ ", color::Fg(color::Green), os.username, os.hostname, 
        color::Fg(color::Reset), color::Fg(color::Blue), os.cwd_pp, color::Fg(color::Reset));
    io::stdout().flush().unwrap();
}