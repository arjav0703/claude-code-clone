#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Role {
    user,
    assistant,
    tool,
}

// impl std::fmt::Display for Role {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Role::User => write!(f, "user"),
//             Role::Assistant => write!(f, "assistant"),
//             Role::Tool => write!(f, "tool"),
//         }
//     }
// }

pub struct Model {
    pub name: String,
}

impl Model {
    pub fn from_env() -> Self {
        let name = std::env::var("OPENROUTER_MODEL")
            .unwrap_or_else(|_| "anthropic/claude-haiku-4.5".to_string());
        Self { name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

impl ToolSpec {
    pub fn read_file() -> Self {
        Self {
            name: "ReadFile".to_string(),
            description: "Read the contents of a file".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "Path to the file to read"
                    }
                },
                "required": ["file_path"]
            }),
        }
    }

    pub fn write_file() -> Self {
        Self {
            name: "WriteFile".to_string(),
            description: "Write content to a file".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "Path to the file to write"
                    },
                    "content": {
                        "type": "string",
                        "description": "Content to write to the file"
                    }
                },
                "required": ["file_path", "content"]
            }),
        }
    }

    pub fn bash() -> Self {
        Self {
            name: "Bash".to_string(),
            description: "Execute a shell command".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The command to execute"
                    }
                },
                "required": ["command"]
            }),
        }
    }
}
