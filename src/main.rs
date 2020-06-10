mod rshio;

// TODO: check for SIGINT

use std::io;

pub struct CLIInput {
    is_verbose: bool,
    exit: bool
}

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
            std::process::exit(0);
        }
        
        if input.trim().is_empty()
        {
            continue;
        }
        break;
    }
    Ok(())
}