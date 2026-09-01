use anyhow::Result;
use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use codecrafters_claude_code::tools;
use serde_json::{self, Value, json};
use std::{env, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let (base_url, api_key, model) = get_cfg();
    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);
    let read_tool = get_read_tool();
    let request = json!({
        "messages": [
            {
                "role": "user",
                "content": args.prompt
            }
        ],
        "model": model,
        "tools": [read_tool],
    });
    let response: Value = client.chat().create_byot(request).await?;

    if let Some(func) = response["choices"][0]["message"]["tool_calls"][0].get("function") {
        let fn_name = func["name"].as_str().ok_or("function tool missing name")?;
        let arg_str = func["arguments"]
            .as_str()
            .ok_or("function tool missing arguments")?;
        let args: Value = serde_json::from_str(arg_str)?;
        let out = tools::execute_tool(fn_name, args)?;
        if let Some(content) = out.as_str() {
            print!("{}", content);
        }
    }

    if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
        println!("{}", content);
    }

    Ok(())
}

fn get_read_tool() -> Value {
    let v = json!({
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
    });
    v
}

fn get_cfg() -> (String, String, String) {
    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let model =
        env::var("OPENROUTER_MODEL").unwrap_or_else(|_| "anthropic/claude-haiku-4.5".to_string());
    (base_url, api_key, model)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn value_test() {
        let value = Value::Null;
        if let Some(t) = value[11]["a"]["b"]["c"].as_array() {
            println!("ttt");
        } else {
            println!("None");
        }
    }
}
