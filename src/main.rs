mod utils;
use std::collections::HashMap;
use std::env::args;
use std::io::Write;
use std::process;
use std::path::{Path, PathBuf};

use std::fs::{File, OpenOptions};

use crate::utils::read_files_and_create_hashmap;

fn verify_path_is_valid(path_str: &str) -> bool {
    let path = Path::new(path_str);
    path.is_dir()
}

// by default ignore the target dir.
// TODO : see if we can improve the performance of this function
fn read_files_recursively(path: PathBuf, files: &mut Vec<PathBuf>) {
    let target = Path::new("./target").to_path_buf();
    for entry in path.read_dir().expect("cannot read dir") {
        if let Ok(entry) = entry {
            let path= entry.path();
            if path == target{
                continue;
            }
            if path.is_dir() {
                read_files_recursively(path, files);
            } else {
                if path.extension().unwrap_or_default() == "rs" {
                    files.push(path)
                }
            }
        }
    }
}

fn save_to_todo_md(data: HashMap<&PathBuf, Vec<(String, usize)>>) {
    let todo_path = Path::new("./todo.md");
    let mut file: File;
    if todo_path.exists() {
        file = OpenOptions::new().write(true).truncate(true).open(todo_path).expect("cannot create the todo file");
    } else  {
        file = File::create("todo.md").unwrap();
    }
    file.write_all(b"# TODO List\n").expect("cannot write to file");
    for (path, vec) in data.into_iter() {
        if vec.len() == 0 {
            continue;
        }
        file.write_all(b"\n").expect("cannot write to file");
        let file_name = format!("- {}\n", path.to_str().expect("cannot convert to string"));
        file.write_all(file_name.as_bytes()).expect("cannot write line");
        for (todo, line_number) in vec {
            let todo_item = format!("\t- {}, Line: {}\n", todo, line_number);
            file.write_all(todo_item.as_bytes()).expect("cannot write line");
        }
    }
    
}

fn main() {
    let mut args: Vec<String> = args().collect();
    // let pwd = env::current_dir().expect("cannot get current working directory");
    // default:  if no argument is given then,  entire directory ".";
    if args.len() ==  1{
        args.push(".".to_string());
    }
    // accept only relative paths and verify path is there;
    let path_str = &args[1];
    if !verify_path_is_valid(path_str){
        println!("usage: Is not a valid dir");
        process::exit(1);
    }
    let path = Path::new(path_str).to_path_buf(); // redundant, can be optimized;
    // read the files in the dir
    let mut files: Vec<PathBuf> = Vec::new();
    read_files_recursively(path, &mut files);
    // println!("{:?} {:?}, {:?}", args, pwd, path);
    // now we have all the valid files in "files" Vector;
    // println!("{:?}", read_files_and_create_hashmap(&files));
    // use the HashMap and write the todos to a markdown file;
    save_to_todo_md(read_files_and_create_hashmap(&files));
}
