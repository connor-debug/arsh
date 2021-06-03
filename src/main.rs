use std::{env, process::exit, str::Split};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

mod alias;


pub fn find_alias(ali: Split<&str>, av: &Vec<String>){
    for i in ali{
        for j in av{
            if i.to_string() == *j{
                println!("Found match");
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut pln: bool = true;
    //init vector of aliases and load aliases into vector
    let mut av: Vec<String> = Vec::new();
    if let Ok(lines) = alias::read_lines("/home/red/.arshrc"){
        for line in lines{
            let sline = line.unwrap();
            let mut splines = sline.split(" # ");
            if splines.next().unwrap_or("") == "alias"{
                av.push(splines.next().unwrap_or("").to_string());
            }
        }
    }

    println!("{:?}", av);

    loop {
        let path = env::current_dir()?;
        // >> as prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("@@@{}@@@",path.display());
        if pln == true{
            println!();
        }
        print!(">>");
        
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // read_line leaves a trailing newline, which trim removes
        // this needs to be peekable so we can determine when we are on the last command
        let mut ali = input.trim().split(" ");

        find_alias( ali, &av);
        
        //println!("{}", ali);
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next()  {

            // everything after the first whitespace character is interpreted as args to the command
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            if command == "help"{
                //print some stuff that shows 
                break;
            }

            match command {
                "show_alias" => {
                    //print help for options of show: alias, envoirment variables, information
                    alias::show_alias(command.to_string(), args);
                }
                "alias" => {
                    alias::new_alias(command.to_string(), args);
                }
                "#pln" => {
                    print!("switching...");
                    if pln == true{
                        pln = false;
                    }
                    else if pln == false{
                        pln = true;
                    }
                }
                "cd" => {
                    // default to '/' as new directory if one was not provided
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                },
                "exit" => exit(0),
                command => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap()));

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            final_command.wait().unwrap();
        }

    }
}
