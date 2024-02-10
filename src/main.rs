use std::{env, fmt::Display, fs, io::Write};
use glob::glob;
use serde_json::{Value, json};
use std::fs::File;

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
    ws_setting.c_compiler_path = json_value["c_compiler_path"].to_string().replace("\"", "");
    ws_setting.cpp_compiler_path = json_value["cpp_compiler_path"].to_string().replace("\"", "");
    ws_setting.root_folder_path = json_value["workspace_root_folder"].to_string().replace("\"", "");
    ws_setting.src_pattern = match json_value["workspace_src_pattern"].as_array() {
        Some(src_pattern) => src_pattern.iter().map(|v| v.to_string().replace("\"", "")).collect(),
        None => vec!()
    };
    ws_setting.exclude_pattern = match json_value["workspace_exclude_pattern"].as_array() {
        Some(exclude_pattern) => exclude_pattern.iter().map(|v| v.to_string().replace("\"", "")).collect(),
        None => vec!()
    };
    ws_setting.include_folders = match json_value["workspace_include_folders"].as_array() {
        Some(include_folder) => include_folder.iter().map(|v| v.to_string().replace("\"", "")).collect(),
        None => vec!()
    };
    ws_setting.compile_flags = match json_value["workspace_compile_flags"].as_array() {
        Some(compile_flags) => compile_flags.iter().map(|v| v.to_string().replace("\"", "")).collect(),
        None => vec!()
    };

    let mut compile_commands: Vec<Value> = Vec::new();
    //println!("{}", ws_setting);
    //println!("{:?}", json_value["folders"]);
    for folder_setting_value in json_value["folders"].as_array().unwrap() {
        let mut folder_setting = FolderSetting::default();
        folder_setting.folder_path = folder_setting_value["folder"].to_string().replace("\"", "");
        folder_setting.src_pattern = match folder_setting_value["src_pattern"].as_array() {
            Some(src_patterns) => src_patterns.iter().map(|v| v.to_string().replace("\"", "")).collect(),
            None => vec![]
        };
        folder_setting.exclude_pattern = match folder_setting_value["exclude_pattern"].as_array() {
            Some(exclude_patterns) => exclude_patterns.iter().map(|v| v.to_string().replace("\"", "")).collect(),
            None => vec![]
        };
        folder_setting.include_folders = match folder_setting_value["include_folders"].as_array() {
            Some(include_folders) => include_folders.iter().map(|v| v.to_string().replace("\"", "")).collect(),
            None => vec![]
        };
        folder_setting.compile_flags = match folder_setting_value["compile_flags"].as_array() {
            Some(compile_flags) => compile_flags.iter().map(|v| v.to_string().replace("\"", "")).collect(),
            None => vec![]
        };
        //println!("{}", folder_setting);

        let mut folder_all_exclude_pattern = ws_setting.exclude_pattern.clone();
        folder_all_exclude_pattern.append(folder_setting.exclude_pattern.as_mut());

        let mut excluded_src : Vec<String> = Vec::new();
        for exclude_pattern in folder_all_exclude_pattern {
            //println!("exclude glob pattern: {}", format!("{}/{}/{}", ws_setting.root_folder_path, folder_setting.folder_path, exclude_pattern).as_str());
            if let Ok(paths) = glob(format!("{}/{}/{}",
                                    ws_setting.root_folder_path,
                                    folder_setting.folder_path,
                                    exclude_pattern).as_str()) {
                for path in paths {
                    let tmp = path.unwrap().to_string_lossy().replace("\\", "/");
                    if !excluded_src.contains(&tmp) {
                        //println!("{tmp}");
                        excluded_src.push(tmp);
                    }
                }
            }
        }

        let mut folder_all_src_pattern = ws_setting.src_pattern.clone();
        folder_all_src_pattern.append(folder_setting.src_pattern.as_mut());

        let mut src : Vec<String> = Vec::new();
        for src_pattern in folder_all_src_pattern {
            //println!("src glob pattern: {}", format!("{}/{}/{}", ws_setting.root_folder_path, folder_setting.folder_path, src_pattern).as_str());
            if let Ok(paths) = glob(format!("{}/{}/{}", 
                                            ws_setting.root_folder_path, 
                                            folder_setting.folder_path,
                                            src_pattern).as_str()) {
                for path in paths {
                    //println!("{}", path.unwrap().to_string_lossy().replace("\\", "/"));
                    let tmp = path.unwrap().to_string_lossy().replace("\\", "/");
                    if !src.contains(&tmp) && !excluded_src.contains(&tmp) {
                        //println!("{tmp}");
                        src.push(tmp);
                    }
                }
            }
        }

        let mut compile_arguments : Vec<String> = Vec::new();
        for ws_include in &ws_setting.include_folders {
            compile_arguments.push(format!("-include{}", ws_include));
        }
        for folder_include in &folder_setting.include_folders {
            compile_arguments.push(format!("-include{}", folder_include));
        }

        compile_arguments.extend(ws_setting.compile_flags.clone());
        compile_arguments.extend(folder_setting.compile_flags);
        //println!("{:?}", compile_arguments);

        for s in src {
            let mut compile_command = json!({
                "directory": "",
                "arguments": [],
                "file": ""
            });

            compile_command["directory"] = Value::from(ws_setting.root_folder_path.as_str());
            compile_command["file"] = Value::from(s.as_str());
            let mut arguments_with_compiler = compile_arguments.clone();
            if s.contains(".cpp") || s.contains(".cxx") || s.contains(".cc") {
                arguments_with_compiler.insert(0, ws_setting.cpp_compiler_path.clone());
            } else {
                arguments_with_compiler.insert(0, ws_setting.c_compiler_path.clone());
            }
            compile_command["arguments"] = Value::from(arguments_with_compiler);

            compile_commands.push(compile_command);
        }
    }

    let mut compile_commands_file = File::create("compile_commands.json").unwrap();
    compile_commands_file.write_all(
        serde_json::to_string_pretty(&compile_commands).unwrap().as_bytes()
    ).unwrap();
}

fn print_usage() {
    println!("usage!");
}
