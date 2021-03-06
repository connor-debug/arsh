use std::str::{SplitWhitespace, Split};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{self, BufRead, prelude::*};


pub fn init_alias(an: &mut Vec<String>, tn: &mut Vec<String>, h_path: String){
    if let Ok(lines) = read_lines(h_path){
        for line in lines{
            let sline = line.unwrap();
            let mut splines = sline.split(" # ");
            if splines.next().unwrap_or("") == "alias"{
                an.push(splines.next().unwrap_or("").trim().to_string());
                tn.push(splines.next().unwrap_or("").trim().to_string());
            }
        }
    }

}

pub fn exchange_alias(input: Split<&str>, vector: &Vec<String>, res_vector: &Vec<String>, count: usize) -> String{
    let mut st = String::from("");
    let mut it;
    for i in input{
        it = 0;
        for (j, k) in vector.iter().zip(res_vector.iter()){
            if i.to_string() == *j{
                println!("Found match: {:?}", *j);
                println!("Resolved to {:?}", *k);
                let k_slice: &str = &k[..];
                st.push_str(k_slice);
                st.push_str(" ");
                break;
            }
            else if i.to_string() != *j{
                if it == count-1 {
                    st.push_str(i);
                    st.push_str(" ");
                }
            }
            it = it + 1;
        }
    }
    return st;
}

pub fn new_alias(command: String, mut args: SplitWhitespace){
    let mut h_path = String::new();
    let homestring = dirs::home_dir().unwrap();
    let homestr = homestring.to_str().unwrap();
    h_path.push_str(homestr);
    h_path.push_str("/.arshrc");
    let mut file = OpenOptions::new()
        .write(true).append(true).open(h_path).unwrap();

    if let Err(_e) = write!(file, "{} ", command){
        println!("Error writing new alias.");
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
        eprint!("Couldn't update .arshrc.");
    }
}

pub fn show_alias(var_n: &mut Vec<String>, var_v: &mut Vec<String>){
    println!("{:?} -> {:?}", var_n, var_v);
}

/*pub fn show_alias(command: String, _args: SplitWhitespace){
    println!("command: {}",command);
    
    if let Ok(lines) = read_lines("/home/red/.arshrc"){
        for line in lines{
            let sline = line.unwrap();
            let mut splines = sline.split(" # ");
            if splines.next().unwrap_or("") == "alias"{
                println!("{}", splines.next().unwrap_or(""));
            }
        }
    }
}*/

pub fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

