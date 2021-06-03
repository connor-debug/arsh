use std::str::{SplitWhitespace};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{self, BufRead, prelude::*};


pub fn new_alias(command: String, mut args: SplitWhitespace){
    let mut file = OpenOptions::new()
        .write(true).append(true).open("/home/red/.arshrc").unwrap();

    if let Err(_e) = write!(file, "{} ", command){
        eprint!("Error writing new alias.");
    }

    if let Some(arg) = args.next(){
        if let Err(_e) = write!(file, "# {} # ", arg){
            eprint!("Error writing new alias.");
        }
        for arg in args{
            if let Err(_e) = write!(file, "{} ", arg){
            eprint!("Error writing new alias.")
            }
        }
    }
    if let Err(_e) = writeln!(file, "",){
        eprint!("Couldn't update .arshellrc.");
    }
}

/*pub fn check_next(borrow: &SplitWhitespace) -> bool{
    ///////
    return true;

}*/

pub fn show_alias(command: String, _args: SplitWhitespace){
    println!("command: {}",command);

    //open up .arshellrc and read off those aliases

    if let Ok(lines) = read_lines("/home/red/.arshrc"){
        for line in lines{
            let sline = line.unwrap();
            let mut splines = sline.split(" # ");
            if splines.next().unwrap_or("") == "alias"{
                println!("{}", splines.next().unwrap_or(""));
            }
        }
    }
}

pub fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

