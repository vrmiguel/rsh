use crate::rshio;
use std::process::Command;

// TODO: support for commands with tokens.len() > 2

fn simple_command(tokens: &Vec<String>)
{
    let cmd = String::from("/bin/") + &tokens[0];
    println!("{:?}", cmd);
    let err = String::from("Running '") + &tokens[0] + &String::from("' failed.");   // TODO: this is horrible
    if tokens.len() > 1 {
        let mut child = Command::new(cmd.trim()).arg(&(tokens[1].trim())).spawn().expect("failed to execute child");
        let _ecode = child.wait().expect(&err);
    }
    else {
        let mut child = Command::new(cmd.trim()).spawn().expect("failed to execute child");
        let _ecode = child.wait().expect(&err);
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
