# Yalc - Yet Another Log Cleaner
Simple Cli rust tool to clean up local log files regularly

## Prerequisites
These tools are required to build yalc:

* OS: Linux
* Rust-Lang Tools (Easy install via [rustup](https://www.rust-lang.org/tools/install))
* Visual Studio Code (Or some other IDE for developing/viewing rust code)
* Additional packages may be: `build-essential`, `pkg-config`, `libssl-dev`
* Yalc is a zero-dependencies project, it only uses the rust standard library

## Building
Compile yalc by using the default cargo commands:
```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Build release with current system cpu optimizations
RUSTFLAGS="-C target-cpu=native" cargo build --release -v

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
For system-wide installation usually `/usr/local/bin` is useful.

### Config
The config file location is: `/etc/yalc.toml`

* [Yalc Config Documentation](https://github.com/Henrik-Peters/Yalc/wiki/Config)

### Example usage
```bash
# Check if yalc is installed properly by showing the installed version
yalc version

# Show available commands and options
yalc help

# Create a new config file with default values
yalc config init

# Check if the config file is valid
yalc config check

# Only Simulate cleanup tasks (dry-mode)
yalc -d

# Execute all configured cleanup tasks
yalc
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

# Show all installed targets
rustup target list

# List all supported cpu architectures
rustc --print target-cpus

# Display current cpu architecture features
rustc -C target-cpu=native --print cfg

# Cross-Compilation ARM 64-Bit-Target
rustup target add aarch64-unknown-linux-gnu
sudo apt-get install -y gcc-aarch64-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

Configure cross-compiler linker at `~/.cargo/config.toml`:
```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```
