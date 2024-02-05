use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = match args.get(1) {
        Some(filename) => filename,
        None => ""
    };

    if input_file == "" {
        print_usage();
        return;
    }

    println!("input file is {input_file}");
}

fn print_usage() {
    println!("usage!");
}
