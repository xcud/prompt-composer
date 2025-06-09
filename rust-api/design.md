# Rust API Design for Prompt Composer

## Core API Structure

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    pub mcp_servers: HashMap<String, McpServer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub server: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub tool_call_count: Option<u32>,
    pub original_task: Option<String>,
    pub has_plan: Option<bool>,
    pub last_action: Option<String>,
    pub current_step: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRequest {
    pub user_prompt: String,
    pub mcp_config: McpConfig,
    pub session_state: Option<SessionState>,
    pub domain_hints: Option<Vec<String>>,
    pub task_complexity: Option<TaskComplexity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskComplexity {
    Auto,
    Simple,
    Complex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptResponse {
    pub system_prompt: String,
    pub applied_modules: Vec<String>,
    pub recognized_tools: Vec<String>,
    pub complexity_assessment: TaskComplexity,
}

// Main API
pub fn compose_system_prompt(request: PromptRequest) -> Result<PromptResponse, PromptError> {
    // Implementation here
}

// Async version for when we need to query MCP servers
pub async fn compose_system_prompt_async(request: PromptRequest) -> Result<PromptResponse, PromptError> {
    // Implementation here
}

#[derive(Debug, thiserror::Error)]
pub enum PromptError {
    #[error("MCP server connection failed: {0}")]
    McpConnectionFailed(String),
    #[error("Tool discovery failed: {0}")]
    ToolDiscoveryFailed(String),
    #[error("Module loading failed: {0}")]
    ModuleLoadingFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
```

## Language Bindings

### Python Binding
```rust
use pyo3::prelude::*;

#[pyfunction]
fn compose_system_prompt(request_json: &str) -> PyResult<String> {
    let request: PromptRequest = serde_json::from_str(request_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
    
    let response = crate::compose_system_prompt(request)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    serde_json::to_string(&response)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pymodule]
fn prompt_composer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compose_system_prompt, m)?)?;
    Ok(())
}
```

### Node.js Binding (via napi-rs)
```rust
use napi_derive::napi;

#[napi]
pub fn compose_system_prompt(request_json: String) -> napi::Result<String> {
    let request: PromptRequest = serde_json::from_str(&request_json)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    
    let response = crate::compose_system_prompt(request)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    
    serde_json::to_string(&response)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}
```

## Usage Examples

### Python Usage
```python
import json
import prompt_composer

request = {
    "user_prompt": "Look at config.json and fix any issues",
    "mcp_config": {
        "mcp_servers": {
            "desktop-commander": {
                "name": "desktop-commander",
                "command": "npx",
                "args": ["@modelcontextprotocol/server-filesystem"]
            }
        }
    },
    "session_state": {
        "tool_call_count": 0
    }
}

response_json = prompt_composer.compose_system_prompt(json.dumps(request))
response = json.loads(response_json)
system_prompt = response["system_prompt"]
```

### Node.js Usage
```javascript
const promptComposer = require('prompt-composer');

const request = {
    userPrompt: "Look at config.json and fix any issues",
    mcpConfig: {
        mcpServers: {
            "desktop-commander": {
                name: "desktop-commander",
                command: "npx",
                args: ["@modelcontextprotocol/server-filesystem"]
            }
        }
    },
    sessionState: {
        toolCallCount: 0
    }
};

const responseJson = promptComposer.composeSystemPrompt(JSON.stringify(request));
const response = JSON.parse(responseJson);
const systemPrompt = response.systemPrompt;
```

## Implementation Strategy

1. **Core Logic in Rust**: All prompt composition, module selection, tool recognition
2. **Embedded Modules**: Prompt modules compiled into the binary as embedded resources
3. **MCP Client**: Rust MCP client to query available tools from servers
4. **Stateless Design**: No persistence, caller manages session state
5. **Fast Startup**: Optimized for quick prompt generation (< 10ms typical)

## Benefits of Rust Implementation

- **Performance**: Sub-10ms prompt composition
- **Memory Safety**: No crashes from bad input
- **Single Binary**: Easy distribution and deployment  
- **Language Agnostic**: Clean bindings for Python, Node.js, others
- **Concurrent Safe**: Multiple prompt requests handled safely
- **Small Footprint**: Minimal resource usage
