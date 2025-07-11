# Yalc - Yet Another Log Cleaner
Simple Cli rust tool to clean up local log files regularly

## Prerequisites
These tools are required to build yalc:

* OS: Linux (But probably also runs on other operating systems)
* Rust-Lang Tools (Easy install via [rustup](https://www.rust-lang.org/tools/install))
* Visual Studio Code (Or some other IDE for developing/viewing rust code)
* Additional packages may be: `build-essential`, `pkg-config`, `libssl-dev`

## Building
Compile yalc by using the default cargo commands:
```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Execute via cargo in debug mode
cargo run

# Run tests
cargo test

# Generate rustdoc
cargo doc
```

## Usage
### Functionality
Yalc does not run permanently but only once. Yalc is started once via the
CLI. When Yalc is executed, a Yalc command is executed. The Yalc command
is selected via CLI arguments. If you want Yalc to run regularly and
automatically, you can set up a cronjob or something similar.

### Install
The Yalc executable must simply be installed in some `$PATH` included folder.

### Example usage
```bash
# Check if yalc is installed properly by showing the installed version
yalc version
```

## Notes for development
### VS Code Plugins for Rust
These plugins may be helpful for development with Rust:

* [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
* [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
* [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker)

### Other notes
```bash
# Pass shell params via cargo
cargo run -- run input.txt output.txt
cargo run -- config check

# Show default compile target and details
rustc --version --verbose
```
