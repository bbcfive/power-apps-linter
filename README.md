# power-apps-linter
A pre-lint tools for power apps. 

## Tech stack

- [Rust](https://www.rust-lang.org/)
  
## Available Scripts
##### `cargo build`
Build the app locally.

##### `cargo run`
Run the lint command for yaml file under this folder.

##### `cargo run xxx`
Run other command if needed. (not finish yet)

##### `cargo run lint xxx`
Only run the lint command for xxx.yaml file. (not finish yet)

## About file path
>for now need to config it manually in `analysis_yaml` function

##### Windows
```
    let contents = fs::read_to_string("..\\Screen1.fx.yaml")?;
```

##### Mac
```
    let contents = fs::read_to_string("Screen1.fx.yaml")?;
```

## To do list
- [ ] Support multiple file path. (for now the yaml path is hardcoded in `analysis_yaml` function)
- [ ] Support command `cargo run xxx` and `cargo run lint xxx`
- [ ] add tests
- [ ] etc...