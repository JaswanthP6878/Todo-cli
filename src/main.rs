mod utils;
use std::env::args;
use std::process;
use std::path::{Path, PathBuf};

fn verify_path_is_valid(path_str: &str) -> bool {
    let path = Path::new(path_str);
    path.is_dir()
}

// by default ignore the target dir.
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

fn main() {
    let mut args: Vec<String> = args().collect();
    // let pwd = env::current_dir().expect("cannot get current working directory");
    // default:  if no argument is given then,  entire directory ".";
    if args.len() ==  1 {
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
    println!("{:?}", files);
}
