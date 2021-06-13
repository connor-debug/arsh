use std::{env, process::exit, str::Split};
use std::path::Path;
use std::process::{Child, Command, Stdio};

mod alias;

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() -> std::io::Result<()> {
    println!("Reading script...");
    let args: Vec<String> = env::args().collect();
    let script = &args[1];
    println!("The argument is {}", script);

    //init vector for and load aliases into vector from config file
    let mut an: Vec<String> = Vec::new();
    let mut tn: Vec<String> = Vec::new();
    if let Ok(lines) = alias::read_lines("/home/red/.arshrc"){
        for line in lines{
            let sline = line.unwrap();
            let mut splines = sline.split(" # ");
            if splines.next().unwrap_or("") == "alias"{
                an.push(splines.next().unwrap_or("").trim().to_string());
                tn.push(splines.next().unwrap_or("").trim().to_string());
            }
        }
    }

    println!("Initialized aliases: {:?} and the true resolved values: {:?}", an, tn);

    Ok(if let Ok(lines) = alias::read_lines(script) {
        for line in lines{
            let sline = line.unwrap();

            let input = sline;

            let mut commands = input.trim().split(" | ").peekable();
            let mut previous_command = None;

            while let Some(command) = commands.next()  {

                // everything after the first whitespace character is interpreted as args to the command
                let mut parts = command.trim().split_whitespace();
                let command = parts.next().unwrap_or("");
                let args = parts;

                if command == "help"{
                    //print some stuff that shows help/tipd
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
                    continue;
                }
                continue;
            }
        })
    }
