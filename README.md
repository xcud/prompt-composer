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

#### `listAvailableDomains()`
Returns array of available domain modules: `["programming", "analysis", "filesystem", "system"]`

#### `listAvailableBehaviors()` 
Returns array of available behavior modules: `["planning", "progress", "reasoning", "tools"]`

#### `getStatus()`
Returns system status and configuration information.

#### `isAvailable()`
Always returns `true` for native bindings.

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
