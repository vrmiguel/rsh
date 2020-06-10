// Reads the input from the command-line
use crate::CLIInput;
use {std::{env}, whoami, dirs, termion::color};

pub struct OS { // User data
    username: String,   // Name of the user that started rsh
    hostname: String,   // Hostname of the user that started rsh
    cwd:      String,   // Current working directory
    hmd:      String,   // User's home directory
    cwd_pp:   String    // cwd but for prompt pretty-printing
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
}