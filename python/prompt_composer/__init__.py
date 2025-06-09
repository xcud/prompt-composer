"""
Prompt Composer - A modular system prompt composition framework

This package provides intelligent system prompt generation based on available MCP tools,
task complexity, and session state. It helps create more effective AI assistants by
applying proven prompt engineering patterns automatically.

All prompt guidance is loaded from transparent markdown files that developers can
read, understand, and customize.

Example usage:
    import json
    from prompt_composer import compose_system_prompt
    
    # Prepare request
    request = {
        "user_prompt": "Look at config.json and fix any issues",
        "mcp_config": {
            "mcpServers": {
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
    
    # Generate optimized system prompt with default prompts
    response = compose_system_prompt(json.dumps(request))
    result = json.loads(response)
    system_prompt = result["system_prompt"]
    
    # Or use custom prompts directory
    response = compose_system_prompt_with_prompts_dir(
        json.dumps(request), 
        "/path/to/custom/prompts"
    )
"""

from ._prompt_composer import (
    compose_system_prompt, 
    compose_system_prompt_with_prompts_dir,
    compose_system_prompt_cached, 
    compose_system_prompt_cached_with_prompts_dir,
    refresh_server_tools,
    list_available_domains,
    list_available_behaviors,
    list_available_domains_in_dir,
    list_available_behaviors_in_dir
)

__version__ = "0.1.0"
__all__ = [
    "compose_system_prompt", 
    "compose_system_prompt_with_prompts_dir",
    "compose_system_prompt_cached", 
    "compose_system_prompt_cached_with_prompts_dir",
    "refresh_server_tools",
    "list_available_domains",
    "list_available_behaviors", 
    "list_available_domains_in_dir",
    "list_available_behaviors_in_dir"
]
