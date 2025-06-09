# Testing Prompt Composer Interactive Guide

## Setup Instructions

1. **Activate the virtual environment**:
```bash
cd /home/ben/lit-platform/prompt-composer
source venv/bin/activate
```

2. **Install ipython for interactive testing**:
```bash
pip install ipython
```

3. **Start ipython**:
```bash
ipython
```

## Interactive Testing Examples

### Basic Test - Simple File Operation

```python
import json
import prompt_composer

# Test a simple file reading request
request = {
    "user_prompt": "Read the README.md file and summarize it",
    "mcp_config": {
        "mcpServers": {
            "desktop-commander": {
                "name": "desktop-commander",
                "command": "desktop-commander", 
                "args": []
            }
        }
    },
    "session_state": {
        "tool_call_count": 0
    }
}

# Generate prompt (using cached version for speed)
response_json = prompt_composer.compose_system_prompt_cached(json.dumps(request))
response = json.loads(response_json)

print("Applied modules:", response['applied_modules'])
print("Recognized tools:", response['recognized_tools'])
print("Complexity:", response['complexity_assessment'])
print("\n--- Generated System Prompt ---")
print(response['system_prompt'])
```

### Test Complex Task Detection

```python
# Test complex task with planning guidance
complex_request = {
    "user_prompt": "Refactor this entire Python codebase to implement async/await throughout and add comprehensive error handling with logging",
    "mcp_config": {
        "mcpServers": {
            "desktop-commander": {
                "name": "desktop-commander",
                "command": "desktop-commander",
                "args": []
            }
        }
    },
    "session_state": {
        "tool_call_count": 0,
        "has_plan": False
    }
}

response_json = prompt_composer.compose_system_prompt_cached(json.dumps(complex_request))
response = json.loads(response_json)

print("Complexity detected:", response['complexity_assessment'])
print("Applied modules:", response['applied_modules'])
print("\n--- Planning Guidance ---")
print(response['system_prompt'])
```

### Test Progress Monitoring

```python
# Test progress monitoring after many tool calls
progress_request = {
    "user_prompt": "Continue working on the data analysis",
    "mcp_config": {
        "mcpServers": {
            "desktop-commander": {
                "name": "desktop-commander",
                "command": "desktop-commander",
                "args": []
            }
        }
    },
    "session_state": {
        "tool_call_count": 8,  # Triggers progress monitoring
        "original_task": "Analyze Q4 sales data and identify trends",
        "has_plan": True
    }
}

response_json = prompt_composer.compose_system_prompt_cached(json.dumps(response_json))
response = json.loads(response_json)

print("Modules applied:", response['applied_modules'])
print("\n--- Progress Monitoring Guidance ---")
print(response['system_prompt'])
```

### Test Multiple MCP Servers

```python
# Test with multiple servers (weather + filesystem)
multi_server_request = {
    "user_prompt": "Check the weather and then save a report to a file",
    "mcp_config": {
        "mcpServers": {
            "desktop-commander": {
                "name": "desktop-commander",
                "command": "desktop-commander",
                "args": []
            },
            "weather": {
                "name": "weather",
                "command": "weather-mcp-server",
                "args": ["--api-key", "test"]
            }
        }
    },
    "session_state": {
        "tool_call_count": 0
    }
}

response_json = prompt_composer.compose_system_prompt_cached(json.dumps(multi_server_request))
response = json.loads(response_json)

print("Recognized tools:", response['recognized_tools'])
print("Applied modules:", response['applied_modules'])
print("\n--- Multi-Server Guidance ---")
print(response['system_prompt'])
```

### Test LIT Platform Dynamic Tools

```python
# Test with lit-platform dynamic MCP server
lit_request = {
    "user_prompt": "Create a new tool for processing CSV files",
    "mcp_config": {
        "mcpServers": {
            "lit-platform": {
                "name": "lit-platform",
                "command": "python",
                "args": ["/home/ben/lit-platform/lit-lib/src/lit/bin/mcp_server.py"]
            }
        }
    },
    "session_state": {
        "tool_call_count": 0
    }
}

response_json = prompt_composer.compose_system_prompt_cached(json.dumps(lit_request))
response = json.loads(response_json)

print("Dynamic tools recognized:", response['recognized_tools'])
print("Applied modules:", response['applied_modules'])
print("\n--- Dynamic Tool Guidance ---")
print(response['system_prompt'])
```

### Performance Testing

```python
import time

# Test performance
start_time = time.time()
for i in range(100):
    response_json = prompt_composer.compose_system_prompt_cached(json.dumps(request))
end_time = time.time()

avg_time = (end_time - start_time) / 100 * 1000  # Convert to milliseconds
print(f"Average prompt composition time: {avg_time:.2f}ms")
print(f"Target was <10ms: {'✅ PASS' if avg_time < 10 else '❌ FAIL'}")
```

## What to Look For

1. **Module Selection**: Check which modules are applied for different request types
2. **Tool Recognition**: Verify tools are correctly identified from MCP config
3. **Complexity Detection**: See how the system categorizes simple vs complex tasks
4. **Performance**: Composition should be very fast (<10ms typically)
5. **Content Quality**: Generated prompts should be helpful and relevant

## Debugging Tips

If something doesn't work:

```python
# Check if import works
try:
    import prompt_composer
    print("✅ Import successful")
except ImportError as e:
    print(f"❌ Import failed: {e}")

# Check version
print("Version:", prompt_composer.__version__)

# Test with minimal request
minimal_request = {
    "user_prompt": "Hello",
    "mcp_config": {"mcpServers": {}}
}

try:
    response = prompt_composer.compose_system_prompt_cached(json.dumps(minimal_request))
    print("✅ Basic functionality works")
except Exception as e:
    print(f"❌ Error: {e}")
```
