# Spec
- Needs to read a directory and create a TODO.md page with the listed TODOS that are to be made
- a transient list where the developer can have like a checklist of todo items in there

- List should be like
```markdown
- /relative/path/from/root/FileName
    [ ] TODO: {Content} - Line No;
```


## TODO :
- create a HashMap of "fileName" : Vec{TODO_statements};
- read each file and add populate that hashmap
- finally construct the TODO.md file in the root folder 
