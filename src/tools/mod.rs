pub mod bash;
pub mod read;
pub mod write;

use anyhow::{Result, bail};
pub use bash::{bash_tool_config, exec_bash};
pub use read::{exec_read, read_tool_config};
use serde_json::Value;
pub use write::{exec_write, write_tool_config};

pub fn execute_tool(name: &str, args: Value) -> Result<Value> {
    match name {
        "Read" => exec_read(args),
        "Write" => exec_write(args),
        "Bash" => exec_bash(args),
        _ => bail!("unsported tool: {}", name),
    }
}

pub fn get_all_tools() -> Vec<Value> {
    vec![bash_tool_config(), read_tool_config(), write_tool_config()]
}
