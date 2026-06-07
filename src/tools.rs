use serde_json::{Value, json};
use std::process;

pub fn handle_read_file(
    arguments: &Value,
    tool_call_id: &str,
    name: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let file_path = arguments["file_path"]
        .as_str()
        .ok_or("Missing 'file_path' in arguments")?;
    let contents = std::fs::read_to_string(file_path)?;
    Ok(json!({
        "role": "tool",
        "tool_call_id": tool_call_id,
        "name": name,
        "content": contents
    }))
}

pub fn handle_write_file(
    arguments: &Value,
    tool_call_id: &str,
    name: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let file_path = arguments["file_path"]
        .as_str()
        .ok_or("Missing 'file_path' in arguments")?;
    let content = arguments["content"]
        .as_str()
        .ok_or("Missing 'content' in arguments")?;
    std::fs::write(file_path, content)?;
    Ok(json!({
        "role": "tool",
        "tool_call_id": tool_call_id,
        "name": name,
        "content": format!("Wrote to file {}", file_path)
    }))
}

pub fn handle_bash(
    arguments: &Value,
    tool_call_id: &str,
    name: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let command = arguments["command"]
        .as_str()
        .ok_or("Missing 'command' in arguments")?;
    let output = process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    Ok(json!({
        "role": "tool",
        "tool_call_id": tool_call_id,
        "name": name,
        "content": format!("stdout: {}\nstderr: {}", stdout, stderr)
    }))
}
