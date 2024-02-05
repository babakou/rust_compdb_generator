use std::{env, fs};
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = match args.get(1) {
        Some(filename) => filename,
        None => {
            print_usage();
            return;
        }
    };

    let input_file_contents = match fs::read_to_string(input_file) {
        Ok(contents) => {
            //println!("{contents}");
            contents
        },
        Err(_) => {
            println!("cannot open {input_file}");
            return;
        }
    };

    let json_value : Value = match serde_json::from_str(input_file_contents.as_str()) {
        Ok(value) => {
            //println!("{value}");
            value
        },
        Err(_) => {
            println!("cannot parse {input_file} as json value");
            return;
        }
    };
}

fn print_usage() {
    println!("usage!");
}
