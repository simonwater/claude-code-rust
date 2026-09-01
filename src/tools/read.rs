use anyhow::{Result, bail};
use serde_json::{Value, json};
use std::fs;

pub fn exec_read(args: Value) -> Result<Value> {
    if let Some(file_path) = args["file_path"].as_str() {
        let s = fs::read_to_string(file_path)?;
        return Ok(Value::String(s));
    }
    bail!("argument file_path is null")
}

pub fn read_tool_config() -> Value {
    json!({
      "type": "function",
      "function": {
        "name": "Read",
        "description": "Read and return the contents of a file",
        "parameters": {
          "type": "object",
          "properties": {
            "file_path": {
              "type": "string",
              "description": "The path to the file to read"
            }
          },
          "required": ["file_path"]
        }
      }
    })
}
