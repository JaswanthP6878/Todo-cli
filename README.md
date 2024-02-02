# TODO-er

TODO-er is a cli tool for macos that can generate the "todo.md" file in the root of your project to keep track of all the todos that you have mentioned in the comments of your code.

The pattern it searches for is `// Todo : ` (with any number of spaces and tabs between the elements of the pattern). This is a very specific use case beacuse I have the habit of writing todos as such.


## How it works?
If a file in your project is as such

```rust
// src/utils.rs

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

### Macos
1. Download the binary from the releases in the repo

2. copy of move the executable to `/usr/local/bin` and make it executable. Make sure that `/usr/local/bin` is in the `$PATH`
```bash
cp /location/of/todo-core /usr/local/bin
chmod +x /usr/local/bin/todo-core
```
3. **Remove the todo-core from quarantine so that macos can run the executable **
```bash
 xattr -d com.apple.quarantine /usr/local/bin/todo-core
```
4. close and open a new shell

4. Now you can use the binary, Go to the directory or code base where you want to create the todo.md and run:
```bash
todo-core 
```
5. you can see the todo.md populated with the todos (that match the pattern mentioned in the previous section)




