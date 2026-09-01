pub mod read;
pub mod write;

use anyhow::{Result, bail};
pub use read::{exec_read, read_tool_config};
use serde_json::Value;
pub use write::{exec_write, write_tool_config};

pub fn execute_tool(name: &str, args: Value) -> Result<Value> {
    match name {
        "Read" => exec_read(args),
        "Write" => exec_write(args),
        _ => bail!("unsported tool: {}", name),
    }
}
