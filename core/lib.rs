pub mod types;
pub mod modules;
pub mod discovery;
pub mod loader;
pub mod composition;

#[cfg(feature = "python")]
pub mod python;

use types::*;
use composition::PromptComposer;
use discovery::ToolDiscovery;
use std::sync::Mutex;

/// Global tool discovery instance with caching
static TOOL_DISCOVERY: Mutex<Option<ToolDiscovery>> = Mutex::new(None);

/// Main function to compose system prompts based on MCP configuration and session state
pub async fn compose_system_prompt(request: PromptRequest) -> Result<PromptResponse, PromptError> {
    compose_system_prompt_with_prompts_dir(request, None).await
}

/// Compose system prompt with custom prompts directory
pub async fn compose_system_prompt_with_prompts_dir(
    request: PromptRequest, 
    prompts_dir: Option<String>
) -> Result<PromptResponse, PromptError> {
    let start_time = std::time::Instant::now();
    
    // Initialize or get tool discovery instance
    let mut discovery = {
        let mut guard = TOOL_DISCOVERY.lock().unwrap();
        if guard.is_none() {
            *guard = Some(match &prompts_dir {
                Some(dir) => ToolDiscovery::with_prompts_dir(dir.clone()),
                None => ToolDiscovery::new(),
            });
        }
        guard.take().unwrap()
    };

    // Discover available tools
    let tools = discovery.discover_tools(&request.mcp_config).await?;
    
    // Put discovery back
    {
        let mut guard = TOOL_DISCOVERY.lock().unwrap();
        *guard = Some(discovery);
    }

    // Create composer with appropriate prompts directory
    let mut composer = match prompts_dir {
        Some(dir) => PromptComposer::with_prompts_dir(dir),
        None => PromptComposer::new(),
    };

    // Compose the prompt
    let response = composer.compose(&request, &tools)?;
    
    // Track performance
    let elapsed = start_time.elapsed();
    if elapsed.as_millis() > 50 {
        eprintln!("Warning: Prompt composition took {}ms", elapsed.as_millis());
    }

    Ok(response)
}

/// Synchronous version that uses cached tools or infers from config
pub fn compose_system_prompt_cached(request: PromptRequest) -> Result<PromptResponse, PromptError> {
    compose_system_prompt_cached_with_prompts_dir(request, None)
}

/// Cached version with custom prompts directory
pub fn compose_system_prompt_cached_with_prompts_dir(
    request: PromptRequest, 
    prompts_dir: Option<String>
) -> Result<PromptResponse, PromptError> {
    let start_time = std::time::Instant::now();
    
    // Get cached tools, but if cache is empty, infer from config
    let tools = {
        let mut guard = TOOL_DISCOVERY.lock().unwrap();
        if let Some(ref mut discovery) = *guard {
            let cached = discovery.get_cached_tools();
            if cached.is_empty() {
                // Cache is empty, infer tools from MCP config
                discovery.infer_tools_from_config_immediate(&request.mcp_config)
            } else {
                cached
            }
        } else {
            // No discovery instance, create temporary one to infer from config
            let mut discovery = ToolDiscovery::new();
            discovery.infer_tools_from_config_immediate(&request.mcp_config)
        }
    };

    // Create composer with appropriate prompts directory
    let mut composer = match prompts_dir {
        Some(dir) => PromptComposer::with_prompts_dir(dir),
        None => PromptComposer::new(),
    };

    // Compose the prompt
    let response = composer.compose(&request, &tools)?;
    
    // Track performance
    let elapsed = start_time.elapsed();
    if elapsed.as_millis() > 10 {
        eprintln!("Warning: Cached prompt composition took {}ms", elapsed.as_millis());
    }

    Ok(response)
}

/// Force refresh tools for a specific server
pub async fn refresh_server_tools(server_name: &str, mcp_config: &McpConfig) -> Result<Vec<Tool>, PromptError> {
    let mut discovery = {
        let mut guard = TOOL_DISCOVERY.lock().unwrap();
        guard.take().unwrap_or_else(ToolDiscovery::new)
    };

    let tools = discovery.refresh_server(server_name, mcp_config).await?;
    
    // Put discovery back
    {
        let mut guard = TOOL_DISCOVERY.lock().unwrap();
        *guard = Some(discovery);
    }

    Ok(tools)
}

/// List available domain modules in default prompts directory
pub fn list_available_domains() -> Result<Vec<String>, PromptError> {
    let composer = PromptComposer::new();
    composer.list_domains()
}

/// List available behavior modules in default prompts directory
pub fn list_available_behaviors() -> Result<Vec<String>, PromptError> {
    let composer = PromptComposer::new();
    composer.list_behaviors()
}

/// List available domain modules in custom prompts directory
pub fn list_available_domains_in_dir(prompts_dir: String) -> Result<Vec<String>, PromptError> {
    let composer = PromptComposer::with_prompts_dir(prompts_dir);
    composer.list_domains()
}

/// List available behavior modules in custom prompts directory
pub fn list_available_behaviors_in_dir(prompts_dir: String) -> Result<Vec<String>, PromptError> {
    let composer = PromptComposer::with_prompts_dir(prompts_dir);
    composer.list_behaviors()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_basic_prompt_composition() {
        let mut mcp_servers = HashMap::new();
        mcp_servers.insert("test-server".to_string(), McpServer {
            name: "test-server".to_string(),
            command: "test-command".to_string(),
            args: vec![],
            env: None,
        });

        let request = PromptRequest {
            user_prompt: "Read the README.md file".to_string(),
            mcp_config: McpConfig { mcp_servers },
            session_state: Some(SessionState {
                tool_call_count: Some(0),
                ..Default::default()
            }),
            domain_hints: None,
            task_complexity: Some(TaskComplexity::Simple),
        };

        let response = compose_system_prompt(request).await.unwrap();
        assert!(!response.system_prompt.is_empty());
        // Note: tool recognition depends on server pattern matching
    }

    #[test]
    fn test_cached_prompt_composition() {
        let mut mcp_servers = HashMap::new();
        mcp_servers.insert("test-server".to_string(), McpServer {
            name: "test-server".to_string(),
            command: "test-command".to_string(),
            args: vec![],
            env: None,
        });

        let request = PromptRequest {
            user_prompt: "Read a file".to_string(),
            mcp_config: McpConfig { mcp_servers },
            session_state: None,
            domain_hints: None,
            task_complexity: None,
        };

        let response = compose_system_prompt_cached(request).unwrap();
        assert!(!response.system_prompt.is_empty());
        // Note: tool recognition depends on server pattern matching
    }

    #[test]
    fn test_list_modules() {
        // These will only work if prompts directory exists
        if let Ok(domains) = list_available_domains() {
            println!("Available domains: {:?}", domains);
        }
        
        if let Ok(behaviors) = list_available_behaviors() {
            println!("Available behaviors: {:?}", behaviors);
        }
    }
}
