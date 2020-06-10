use crate::rshio;

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
}