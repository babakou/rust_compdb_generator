use std::{env, fs};

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
}

fn print_usage() {
    println!("usage!");
}
