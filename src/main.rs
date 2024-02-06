use std::{env, fmt::Display, fs};
use serde_json::Value;

#[derive(Debug, Default)]
struct WorkspaceSetting {
    c_compiler_path: String,
    cpp_compiler_path: String,
    root_folder_path: String,
    src_pattern: Vec<String>,
    exclude_pattern: Vec<String>,
    include_folders: Vec<String>,
    compile_flags: Vec<String>,
}

#[derive(Debug, Default)]
struct FolderSetting {
    folder_path: String,
    src_pattern: Vec<String>,
    exclude_pattern: Vec<String>,
    include_folders: Vec<String>,
    compile_flags: Vec<String>
}

impl Display for WorkspaceSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("================workspace settings===================");
        println!("c_compiler_path : {}", self.c_compiler_path);
        println!("cpp_compiler_path : {}", self.cpp_compiler_path);
        println!("root_folder : {}", self.root_folder_path);
        println!("src_pattern:");
        for src_pattern in &self.src_pattern {
            println!("  {}", src_pattern);
        }
        println!("exclude_pattern:");
        for exclude_pattern in &self.exclude_pattern {
            println!("  {}", exclude_pattern);
        }
        println!("include_folders:");
        for include_folder in &self.include_folders {
            println!("  {}", include_folder);
        }
        println!("compile_flags:");
        for compile_flag in &self.compile_flags {
            println!("  {}", compile_flag);
        }
        Ok(())
    }
}

impl Display for FolderSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("================folder settings===================");
        println!("folder_path : {}", self.folder_path);
        println!("src_pattern:");
        for src_pattern in &self.src_pattern {
            println!("  {}", src_pattern);
        }
        println!("exclude_pattern:");
        for exclude_pattern in &self.exclude_pattern {
            println!("  {}", exclude_pattern);
        }
        println!("include_folders:");
        for include_folder in &self.include_folders {
            println!("  {}", include_folder);
        }
        println!("compile_flags:");
        for compile_flag in &self.compile_flags {
            println!("  {}", compile_flag);
        }
        Ok(())
    }
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
    ws_setting.root_folder_path = json_value["workspace_root_folder"].to_string();
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
    ws_setting.include_folders =
        json_value["workspace_include_folders"]
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

    //println!("{}", ws_setting);
    //println!("{:?}", json_value["folders"]);
    for folder_setting_value in json_value["folders"].as_array().unwrap() {
        let mut folder_setting = FolderSetting::default();
        folder_setting.folder_path = folder_setting_value["folder"].to_string();
        folder_setting.src_pattern = match folder_setting_value["src_pattern"].as_array() {
            Some(src_patterns) => src_patterns.iter().map(|v| v.to_string()).collect(),
            None => vec![]
        };
        folder_setting.exclude_pattern = match folder_setting_value["exclude_pattern"].as_array() {
            Some(exclude_patterns) => exclude_patterns.iter().map(|v| v.to_string()).collect(),
            None => vec![]
        };
        folder_setting.include_folders = match folder_setting_value["include_folders"].as_array() {
            Some(include_folders) => include_folders.iter().map(|v| v.to_string()).collect(),
            None => vec![]
        };
        folder_setting.compile_flags = match folder_setting_value["compile_flags"].as_array() {
            Some(compile_flags) => compile_flags.iter().map(|v| v.to_string()).collect(),
            None => vec![]
        };
        //println!("{}", folder_setting);
    }
}

fn print_usage() {
    println!("usage!");
}
