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

    let os = rshio::get_user_data();
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
        let tokens: Vec<String> = input.split(" ").map(str::to_string).collect();
        rshexec::run(&tokens, &mut config);
        if config.exit {
            break;
        }        
    }
    Ok(())
}