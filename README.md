# TODO-er

TODO-er is a cli tool for macos that can generate the "todo.md" file in the root of your project to keep track of all the todos that you have mentioned in the comments of your code.

The pattern it searches for is `// Todo : ` (with any number of spaces and tabs between the elements of the pattern). This is a very specific use case beacuse I have the habit of writing todos as such.


## How it works?
If a file in your project is as such

```rust
// src/utils.rs

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
```
and if you run the todo-cli in the root of the project, then it creates a file called `TODO.md` as such:
```markdown
# TODO List

- ./src/main.rs
	- see if we can improve the performance of this function, Line: 18

- ./src/utils.rs
	- improve the performance of this function, Line: 33
```

## Installation and Usage.
> Need to add installation steps.

