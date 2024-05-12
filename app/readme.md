## Andrii Asmolovskyi

1) cargo new app will create a project app
# install extension "Even Better TOML" - to display the "TOML" file correctly. TOML-file is a dependency file similar to NodJS like package.json
2) cd app - go to app Folder 
3) cargo run - compiling and running the project
    Finished dev [unoptimized + debuginfo] target(s)
    Running `target\debug\app.exe` (os windows)
    Running `target\debug\app` (mac os)
4) cargo build - building the developer version with debug (debug folder)
    Finished dev [unoptimized + debuginfo] target(s)
5) cargo build --release - building the release version without debugging (release folder)
    Finished release [optimized] target(s)