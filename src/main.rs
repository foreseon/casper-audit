extern crate walkdir;
use std::fs::File;
use std::io::prelude::*;
use std::{ffi::OsStr, path::Path};
use walkdir::WalkDir;
use colored::Colorize;
use std::env;

fn main() {
    if let Some(project_path) = env::args().nth(1) {
        list_entry_points(project_path);
    }
    else {
        list_entry_points(env::current_dir().unwrap().as_path().to_str().unwrap().to_string());
    }
    
}

//Recursively scans the given project directory, finds .rs files and scans for entry_points inside them.
fn list_entry_points(project_path: String) {
    let mut project_path = project_path;
    if project_path == "" {
        project_path = "./".to_owned()
    }

    //Add .rs file paths to vector
    let mut rust_files_vec: Vec<String> = vec![];
    for file in WalkDir::new(project_path)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        let filepath_string = file.path().to_str().unwrap().to_owned();
        let file_ext = Path::new(&filepath_string)
            .extension()
            .and_then(OsStr::to_str);
        match file_ext {
            Some("rs") => {
                rust_files_vec.push(filepath_string);
            }
            Some(_) => (),
            None => (),
        }
    }
    //Read found rust files, search for entry_points
    for rust_file in rust_files_vec {
        let filepath = rust_file.clone();
        let mut file = File::open(rust_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut entry_point_names: Vec<String> = vec![];

        let mut entry_point_location = contents.find("EntryPoint::new(");
        if entry_point_location != None { 
            println!();
            println!("{}", filepath.red());
        }
        while entry_point_location != None {
            contents = contents[entry_point_location.unwrap()..contents.len()].to_string();
            let mut read_location = 0;
            let mut entry_point_name = "".to_string();

            while contents.chars().nth(read_location) != Some('"') {
                read_location += 1;
            }

            read_location += 1;

            while contents.chars().nth(read_location) != Some('"') {
                entry_point_name.push(contents.chars().nth(read_location).unwrap());
                read_location += 1;
            }

            println!("   {}", entry_point_name.blue());

            entry_point_names.push(entry_point_name);
            contents = contents[read_location..contents.len()].to_string();

            entry_point_location = contents.find("EntryPoint::new(");
        }
    }
}
