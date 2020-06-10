use crate::rshio;
use std::{process::Command, panic};

// TODO: support for commands with tokens.len() > 2

fn simple_command(tokens: &Vec<String>)
{
    panic::set_hook(Box::new(|_info| {
        // Don't panic. It's all good.
    }));

    let cmd = String::from("/bin/") + &tokens[0];

    let result = panic::catch_unwind(|| 
    {
        if tokens.len() > 1 
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

pub fn run(tokens: &Vec<String>, config: &mut rshio::CLIInput)
{
    let first = tokens[0].trim();

    if first == "cd"
    {
        println!("Change dir!");
    }

    if first == "exit" || first == "quit"
    {
        config.exit = true;
        return;
    }
    else {
        simple_command(tokens);
    }
}
