use std::{io::Split, str::SplitWhitespace};
use evalexpr::*;
use std::io::{self, BufRead};
use std::fs::{File};
use std::path::Path;

extern crate dirs;

pub fn exchange_var(input: Split<&str>, vector: &Vec<String>, res_vector: &Vec<String>, count: usize) -> String{
    return "Hello".to_string();
}

pub fn init_var(var_n: &mut Vec<String>,var_v: &mut Vec<String>){
    var_n.push("$HOME".to_string());
    var_v.push(dirs::home_dir().unwrap().to_str().unwrap().to_string());
    var_n.push("$USER".to_string());
    var_v.push("..".to_string());
    var_n.push("$EDITOR".to_string());
    var_v.push("vim".to_string());
}

pub fn show_env(var_n: &mut Vec<String>, var_v: &mut Vec<String>){
    println!("{:?} -> {:?}", var_n, var_v);
}

pub fn new_var(args: &mut SplitWhitespace, var_n: &mut Vec<String>, var_v: &mut Vec<String>){
    let f_args = ret_string(args);
    let mut _s ="$".to_string();
    let mut assign = f_args.split("=");
    let en = assign.next().unwrap().to_string();
    if en.chars().all(char::is_alphabetic) {
        _s.push_str(&en.to_string());
        var_n.push(_s);
    }
    else{
        println!("Use only alphabetical variable names.");
        return;
    }
    let ev = eval(assign.next().unwrap()).unwrap();
    var_v.push(ev.to_string());
}

/*pub fn add(args: &mut SplitWhitespace, var_n: &mut Vec<String>, var_v: &mut Vec<String>) -> i32{
    let f_args = ret_string(args);
    println!("ret string: {}",f_args);
    let mut assign = f_args.split("+");
    let a = assign.next().unwrap_or_default().parse::<i32>().unwrap_or_default();
    let b = assign.next().unwrap_or_default().parse::<i32>().unwrap_or_default();
    return a + b;
}*/

pub fn ret_string(wsstring: &mut SplitWhitespace) -> String {
    let mut new_string = String::from("");
    for word in wsstring{
        new_string.push_str(&word.to_string());
    }
    return new_string;
}

/*pub fn evalu_string(eq: &mut String){
    println!("the evaluation is {}", eval(&eq).unwrap());
}*/

pub fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
