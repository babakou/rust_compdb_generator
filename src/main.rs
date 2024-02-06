use std::{env, fs};
use serde_json::Value;

#[derive(Debug, Default)]
struct WorkspaceSetting {
    c_compiler_path: String,
    cpp_compiler_path: String,
    root_folder: String,
    src_pattern: Vec<String>,
    exclude_pattern: Vec<String>,
    compile_flags: Vec<String>,
}

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

    let mut ws_setting = WorkspaceSetting::default();
    ws_setting.c_compiler_path = json_value["c_compiler_path"].to_string();
    ws_setting.cpp_compiler_path = json_value["cpp_compiler_path"].to_string();
    ws_setting.root_folder = json_value["workspace_root_folder"].to_string();
    ws_setting.src_pattern = 
        json_value["workspace_src_pattern"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.to_string())
        .collect();
    ws_setting.exclude_pattern = 
        json_value["workspace_exclude_pattern"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.to_string())
        .collect();
    ws_setting.compile_flags =
        json_value["workspace_compile_flags"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.to_string())
        .collect();

    //println!("{:?}", ws_setting);
}

fn print_usage() {
    println!("usage!");
}
