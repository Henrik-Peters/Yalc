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

keep_rotate = 3

missing_files_ok = true
copy_truncate = true

file_list = [
    "/var/log/test.log",
    "/opt/app/logs/server.log"
]

[retention]
file_size_mib = 10
last_write_h = 5
"#;
