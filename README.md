# power-apps-linter
A pre-lint tools for power apps. 

## Tech stack

- [Rust](https://www.rust-lang.org/)
  
## Available Scripts
##### `cargo build`
Build the app locally.

##### `cargo run`
Run the lint command for `all` the yaml file under this folder.

##### `cargo run xxx`
Run other command if needed. (not finish yet)

##### `cargo run lint xxx`
Only run the lint command for xxx.yaml file. 

## To do list
- [x] Support multiple file path. (for now the yaml path is hardcoded in `analysis_yaml` function)
- [x] Support command `cargo run xxx` and `cargo run lint xxx`
- [ ] Do key check for yaml file
- [ ] Variable naming grammer checking
- [ ] Code implementation verification
- [ ] Add tests
- [ ] etc...