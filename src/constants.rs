//! Module for global constant values
//!

///Default path for the config file
pub const YALC_VERSION: &'static str = "0.1.0";

///Default path for the config file
pub const DEFAULT_CONFIG_PATH: &'static str = "/etc/yalc.toml";

///Default toml config file content
pub const DEFAULT_CONFIG_CONTENT: &'static str = r#"# Yalc log rotation config
dry_run = false
mode = "FileSize"

keep_rotate = 4

file_list = [
    "apple.log",
    "banana.log", 
    "cherry.log"
]

[retention]
file_size_mb = 10
last_write_h = 5
"#;
