use anyhow::{Result, bail};
use serde_json::Value;
use std::fs;

pub fn execute_tool(name: &str, args: Value) -> Result<Value> {
    match name {
        "Read" => read(args),
        _ => bail!("unsported tool: {}", name),
    }
}

pub fn read(args: Value) -> Result<Value> {
    if let Some(file_path) = args["file_path"].as_str() {
        let s = fs::read_to_string(file_path)?;
        return Ok(Value::String(s));
    }
    bail!("argument file_path is null")
}
