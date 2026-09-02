use anyhow::{Result, bail};
use serde_json::{Value, json};
use std::process::Command;

pub fn exec_bash(args: Value) -> Result<Value> {
    if let Some(command) = args["command"].as_str() {
        let output = Command::new("bash").arg("-c").arg(command).output()?;
        let out = String::from_utf8(output.stdout)?;
        let err = String::from_utf8(output.stderr)?;
        let res = if !err.is_empty() {
            Value::String(format!("bash command execute error: {}", err))
        } else if !out.is_empty() {
            Value::String(out)
        } else {
            Value::Null
        };
        return Ok(res);
    }
    bail!("argument command is null")
}

pub fn bash_tool_config() -> Value {
    json!({
      "type": "function",
      "function": {
        "name": "Bash",
        "description": "Execute a shell command",
        "parameters": {
          "type": "object",
          "required": ["command"],
          "properties": {
            "command": {
              "type": "string",
              "description": "The command to execute"
            }
          }
        }
      }
    })
}
