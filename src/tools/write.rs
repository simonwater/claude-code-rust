use anyhow::{Result, bail};
use serde_json::{Value, json};
use std::fs;

pub fn exec_write(args: Value) -> Result<Value> {
    if let Some(file_path) = args["file_path"].as_str() {
        if let Some(content) = args["content"].as_str() {
            fs::write(file_path, content.as_bytes())?;
        } else {
            bail!("argument content is null")
        }
        Ok(Value::Null)
    } else {
        bail!("argument file_path is null")
    }
}

pub fn write_tool_config() -> Value {
    json!({
      "type": "function",
      "function": {
        "name": "Write",
        "description": "Write content to a file",
        "parameters": {
          "type": "object",
          "required": ["file_path", "content"],
          "properties": {
            "file_path": {
              "type": "string",
              "description": "The path of the file to write to"
            },
            "content": {
              "type": "string",
              "description": "The content to write to the file"
            }
          }
        }
      }
    })
}
