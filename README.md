# power-apps-linter
A pre-lint tools for power apps. 

## Tech stack

- [Rust](https://www.rust-lang.org/)
  
## Get started
##### `cargo run`
Run the lint command for `all` the yaml file under this folder.

##### `cargo build`
Build the app locally.

##### `cargo run xxx`
Run other command if needed(default command is lint).

##### `cargo run lint xxx`
Only run the lint command for xxx.yaml file. 

## Disable checking for next line
Add this comments in yaml file, lint checking will skip the next line
```
# disable checking for next line
here some keys  // this line will be ignored
```

## About file path
>for now need to config it manually in `lint_checking` function
##### Windows
```
    let standards = json_from_file(".\\standard.json");
```

##### Mac
```
    let standards = json_from_file("standard.json");
```

## To contribute
- [x] Support multiple file path. (for now the yaml path is hardcoded in `analysis_yaml` function)
- [x] Support command `cargo run xxx` and `cargo run lint xxx`
- [x] Do key duplication checking for yaml file
- [x] Tips for line, file of the warnings 
- [x] Add `Disable lint checking for next line`
- [x] Add upperCamelCase
- [ ] Variable naming grammer checking
- [ ] Code implementation verification
- [ ] Add tests
- [ ] etc...