# Prompt Composer

A modular system prompt composition framework that intelligently generates system prompts for AI assistants based on available tools, task complexity, and contextual information.

## Language Support

- **Python**: `pip install system-prompt-composer` 
- **Node.js**: `npm install system-prompt-composer` (native bindings - no Python required!)

## Quick Start

### Python
```python
import system_prompt_composer
import json

request = {
    "user_prompt": "Help me analyze this code",
    "mcp_config": {"mcpServers": {...}},
    "session_state": {"tool_call_count": 0}
}

response = system_prompt_composer.compose_system_prompt(json.dumps(request))
result = json.loads(response)
print(result["system_prompt"])
```

### Node.js (Native)
```javascript
const { composeSystemPrompt } = require('system-prompt-composer');

const request = {
  user_prompt: "Help me analyze this code",
  mcp_config: { mcpServers: {...} },
  session_state: { tool_call_count: 0 }
};

const response = await composeSystemPrompt(request);
console.log(response.system_prompt);
```

**📖 For detailed Node.js documentation, see [node/README.md](./node/README.md)**

## Project Structure

```
prompt-composer/
├── core/              # Rust core implementation
│   ├── lib.rs         # Main library with NAPI bindings
│   ├── types.rs       # Type definitions
│   ├── composition.rs # Prompt composition logic
│   └── ...
├── python/            # Python package (PyO3 bindings)
├── node/              # Node.js package (native NAPI-RS bindings)
│   ├── package.json
│   ├── index.js       # Native bindings wrapper
│   └── index.d.ts     # TypeScript definitions
├── prompts/           # Modular prompt library
│   ├── domains/       # Domain-specific prompts
│   └── behaviors/     # Behavioral guidance prompts
└── README.md          # This file
```

## Features

- 🧠 **Intelligent prompts** that adapt to available MCP tools
- 📋 **Automatic task planning** for complex requests  
- 🎯 **Context-aware guidance** for different domains (programming, analysis, filesystem, etc.)
- 📊 **Progress monitoring** for multi-step workflows
- 🔄 **Modular design** with composable prompt components
- ⚡ **High performance** with Rust core
- 🌍 **Multi-language support** (Python, Node.js)

## Installation

### Python Package
```bash
pip install system-prompt-composer
```

### Node.js Package (Native - No Python Required!)
```bash
npm install system-prompt-composer
```

The Node.js package now uses **native Rust bindings** via NAPI-RS, eliminating the Python dependency!

## API Reference

### Core Functions

#### `composeSystemPrompt(request)`
Generate an intelligent system prompt based on available tools and context.

**Parameters:**
- `request.user_prompt` (string): The user's request
- `request.mcp_config` (object): MCP server configuration with `mcpServers`
- `request.session_state` (object): Current session state including `tool_call_count`
- `request.domain_hints` (array, optional): Domain hints like `["programming", "analysis"]`
- `request.task_complexity` (string, optional): `"Simple"`, `"Complex"`, or `"Auto"`

**Returns:**
```javascript
{
  system_prompt: "Generated prompt text...",
  source: "native",
  version: "1.1.0",
  // ... additional metadata
}
```

#### `getStatus()`
Returns system status and configuration information including available domains and behaviors.

#### `isAvailable()`
Always returns `true` for native bindings.

## Tools Directory Feature

The system-prompt-composer supports tool-specific instruction files to improve how LLMs use MCP tools.

### How It Works

When you call `composeSystemPrompt()` with MCP server configurations, the system automatically looks for corresponding instruction files in `prompts/tools/` and includes them in the generated system prompt.

```
prompts/
├── behaviors/          # General AI behaviors 
├── domains/           # Domain-specific knowledge
├── tools/             # NEW: Tool-specific instructions
│   ├── desktop-commander.md
│   ├── weather-service.md
│   └── [your-mcp-server-name].md
└── server_patterns.toml
```

### Creating Tool Instructions

Create a markdown file named after your MCP server:

```markdown
# My Custom Tool Instructions

You have access to my-custom-tool with these capabilities:
- Function 1: description and best practices
- Function 2: common usage patterns

## Best Practices
- Specific guidance for effective tool usage
- Error handling approaches
- Performance considerations
```

### Updated API Response

Tool instructions are automatically included and tracked:

```javascript
{
  system_prompt: "...",
  applied_modules: [
    "planning",
    "tool:desktop-commander",  // Tool instructions included
    "tool:weather-service"
  ],
  recognized_tools: [...],
  complexity_assessment: "simple"
}
```

The `getStatus()` function now also returns available tools:
```javascript
{
  available: true,
  domains: ["analysis", "filesystem", "programming"],
  behaviors: ["planning", "progress", "reasoning"], 
  tools: ["desktop-commander", "weather-service"],  // NEW
  version: "1.0.3"
}
```

### Benefits
- **Better Tool Usage**: LLMs get specific guidance for each tool
- **Developer Control**: Customize instructions for your MCP tools
- **Automatic Integration**: Just add markdown files - no code changes
- **Graceful Fallback**: Missing tool files are safely ignored

## Architecture

**Native Node.js Architecture (NEW):**
```
Node.js → NAPI-RS → Rust Core
```

**Python Architecture:**
```
Python → PyO3 → Rust Core  
```

**Key Benefits of Native Bindings:**
- ✅ No Python dependency for Node.js users
- ✅ Native performance - direct Rust execution  
- ✅ Simple deployment - just `npm install`
- ✅ Better error handling
- ✅ Cross-platform binary distribution
- ✅ Smaller bundle size

## Development

### Building Native Node.js Package
```bash
cd node/
npm install
npm run build        # Build release binaries
npm run build:debug  # Build debug binaries
npm test            # Run tests
```

### Python Development
```bash
cd python/
pip install -e .
```

### Rust Core Development
```bash
cargo build --release
cargo test
cargo build --features nodejs  # For Node.js bindings
cargo build --features python  # For Python bindings
```

## Publishing

### Node.js (npm)
```bash
cd node/
npm run build       # Build native binaries
npm publish --access public
```

### Python (PyPI)
```bash
cd python/
pip install build twine
python -m build
twine upload dist/*
```

## Contributing

Contributions welcome! The project uses:
- **Rust** for the core prompt composition engine
- **NAPI-RS** for Node.js native bindings
- **PyO3** for Python bindings
- **Modular prompts** in the `prompts/` directory

## License

MIT License - see LICENSE file for details.
