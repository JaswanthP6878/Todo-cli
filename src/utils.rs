use std::{collections::HashMap, path::PathBuf};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;


pub fn read_files_and_create_hashmap(files: &Vec<PathBuf>) -> HashMap<&PathBuf, Vec<(String, usize)>>  {
    let mut todo_data:HashMap<&PathBuf, Vec<(String, usize)>> = HashMap::new();
    for entry in files {
        let file = File::open(entry).expect("cannot open file");
        let mut buf_reader: BufReader<File> = BufReader::new(file);
        let list_of_todos = read_file_and_create_todos(&mut buf_reader); // contains a list of todo-text and line_number;
        todo_data.insert(entry, list_of_todos);
    }
    return todo_data;
}


fn read_file_and_create_todos(reader: &mut BufReader<File>) -> Vec<(String, usize)>  {
    let mut todos: Vec<(String, usize)> = Vec::new();
    // let re = Regex::new(re)
    for (line_number, line) in reader.lines().enumerate() {
        if let Ok(val) = line {
            if let Some(todo_val) = get_todo(&val) {
                todos.push((todo_val, line_number + 1));
            }
        }
    }
    todos
}

// checks if a line of code has "// TODO :" pattern and return that string.
// TODO : improve the performance of this function
fn get_todo(val: &String) -> Option<String> {
    let re = Regex::new(r"[ \t]*//[ \t]*TODO[ \t]*:[ \t]*(.+)").expect("cannot parse the regex");
    if let Some(val) = re.captures(val) {
        // println!("{:?}", val);
        return Some(val[1].to_string())
    }
    return None;
}