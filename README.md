# Prompt Composer

A modular system prompt composition framework that encodes accumulated LLM interaction wisdom to maximize AI assistant effectiveness across different tools, tasks, and domains.

## Why Prompt Composer?

Most AI applications struggle with consistent tool usage and task completion. Prompt Composer solves this by systematically applying proven prompt engineering patterns based on available tools, task complexity, and domain requirements.

**Stop paying token taxes for basic functionality.** If you're running local models on your own hardware, you shouldn't need to pay rent-seeking middlemen for prompt optimization. This is community-owned knowledge infrastructure.

## Democratizing AI Through Transparent Knowledge

### No Hidden Prompts - Everything is Readable

Prompt Composer embodies a radical transparency philosophy: **no hidden knowledge, no black boxes**. All prompt guidance is stored in plain text markdown files that developers can read, understand, and modify:

```
prompts/
├── domains/        # Domain-specific guidance (filesystem, programming, analysis, system admin)
├── behaviors/      # Universal patterns (planning, progress monitoring, tool usage, reasoning)
```

### Community-Owned Wisdom

This isn't just a library - it's a **democratized knowledge repository** where:

- **Domain experts contribute proven patterns** (medical professionals → clinical reasoning, financial analysts → market analysis)
- **Tool developers optimize usage patterns** for their MCP servers
- **Practitioners share real-world methodologies** that actually work
- **Everyone benefits** from collective intelligence

### Customizable for Your Needs

```python
# Use default community prompts
response = prompt_composer.compose_system_prompt(request)

# Use your organization's custom prompts
response = prompt_composer.compose_system_prompt(
    request, 
    prompts_dir="/path/to/your/company/prompts"
)
```

Organizations can:
- Override default prompts with company-specific best practices
- Add proprietary domain knowledge while keeping core logic
- Maintain competitive advantages through specialized guidance
- Contribute improvements back to the community

## The Problem We Solve

Transform this basic prompt:
```
You have access to these tools: read_file, write_file, search_code...
```

Into intelligent, context-aware guidance that actually works:
```
COMPLEX TASK PLANNING: Create a detailed plan first and save it to a file...
PROGRAMMING BEST PRACTICES: Read code before changing it, make surgical edits...
PROGRESS MONITORING: You've made 6 tool calls, assess your progress...
```

## Stateless Design

Prompt-composer is completely stateless - it doesn't store or track any session information. Instead, your application passes in the current session state, making the library:

- **Simple**: No databases, persistence, or session management complexity
- **Flexible**: Works with any application architecture (microservices, monoliths, serverless)
- **Reliable**: No state corruption or concurrency issues
- **Portable**: Same behavior across different deployment environments

Your application manages sessions however it wants (files, databases, memory) and just passes the relevant state to get optimized prompts.

## Complete Examples

### Example 1: Simple File Reading Task

**Input:**
```python
compose_system_prompt(
    user_prompt="Look at config.json and tell me what's in it",
    mcp_config={
        "mcpServers": {
            "desktop-commander": {"command": "npx", "args": ["@modelcontextprotocol/server-filesystem"]}
        }
    },
    session_state={
        "tool_call_count": 0
    }
)
```

**Output:**
```
You have access to the following tools:
- desktop-commander.read_file: Read file contents
- desktop-commander.list_directory: List directory contents

To use a tool, respond with JSON in the following format:
{
  "tool": "desktop-commander.read_file",
  "arguments": {
    "path": "/absolute/path/to/file"
  }
}

ALWAYS read files before analyzing them. When working with desktop-commander tools:
- Use absolute paths for reliability (starting with '/' or drive letters like 'C:\')
- Prefer read_file over execute_command for viewing file contents
- Handle potential file access errors gracefully

When a user asks you to examine a file, your first step should be to read the file using the available tools.
```

### Example 2: Complex Programming Task

**Input:**
```python
compose_system_prompt(
    user_prompt="Refactor this Python codebase to use async/await throughout",
    mcp_config=filesystem_mcp_config,
    domain_hints=["programming"],
    task_complexity="complex",
    session_state={
        "tool_call_count": 0,
        "has_plan": False
    }
)
```

**Output:**
```
You have access to the following tools:
- desktop-commander.read_file: Read file contents
- desktop-commander.write_file: Write file contents
- desktop-commander.edit_block: Make surgical edits
- desktop-commander.search_code: Search within files

COMPLEX TASK PLANNING:
This appears to be a substantial refactoring task. Before beginning:
1. Create a detailed plan breaking down the work into concrete steps
2. Save this plan to a file so progress can be tracked
3. Examine the current codebase structure thoroughly
4. Identify all areas that need async conversion
5. Plan the order of changes to minimize breaking dependencies

PROGRAMMING BEST PRACTICES:
- Read and understand existing code before making changes
- Make surgical, targeted edits rather than large rewrites  
- Use edit_block for small changes, write_file only for new files
- Test changes incrementally when possible
- Follow existing code style and patterns
- Consider backwards compatibility implications

FILE SYSTEM GUIDANCE:
- Always read files before analyzing or modifying them
- Use absolute paths for reliability
- Use chunked writing for large files (25-30 lines max per write_file call)

PROGRESS MONITORING:
- After completing each major step, assess your progress
- Update your plan as you learn more about the codebase
- If you've made many tool calls, consider checking with the user about your approach
```

### Example 3: Session-Aware Progress Monitoring

**Input:**
```python
compose_system_prompt(
    user_prompt="Continue analyzing the sales data",
    mcp_config=data_tools_config,
    domain_hints=["analysis"],
    session_state={
        "tool_call_count": 6,
        "original_task": "Analyze Q4 sales data and identify trends",
        "has_plan": True
    }
)
```

**Output:**
```
You have access to the following tools:
- desktop-commander.read_file: Read file contents  
- data-analysis.process_csv: Process CSV data

CONTINUING PREVIOUS WORK:
Your original task was: "Analyze Q4 sales data and identify trends"

You've executed 6 tool calls so far. Take a moment to assess: Are you making clear progress toward completing the analysis? If you're not sure your next steps are essential, consider summarizing what you've accomplished and asking the user for guidance on priorities.

DATA ANALYSIS METHODOLOGY:
- Examine data structure and quality before drawing conclusions
- Document your methodology and any assumptions you're making
- Show your analytical work step-by-step for transparency  
- Validate results and check for edge cases or anomalies
- Summarize findings clearly with supporting evidence

TOOL USAGE:
- Always read files before processing to understand data structure
- Handle potential data quality issues gracefully
```

### Example 4: Simple API Task

**Input:**
```python
compose_system_prompt(
    user_prompt="What's the weather like today?",
    mcp_config={
        "mcpServers": {
            "weather": {"command": "weather-mcp-server", "args": ["--api-key", "..."]}
        }
    },
    session_state={
        "tool_call_count": 0
    }
)
```

**Output:**
```
You have access to the following tools:
- weather.get_current: Get current weather
- weather.get_forecast: Get weather forecast

When working with weather APIs:
- Request location information if not provided by the user
- Handle API errors gracefully (service unavailable, invalid location, etc.)
- Provide clear, user-friendly weather information
```

### Example 5: Multi-Tool Complex System Task

**Input:**
```python
compose_system_prompt(
    user_prompt="Create a comprehensive backup strategy for our development environment",
    mcp_config={
        "mcpServers": {
            "desktop-commander": filesystem_config,
            "github": github_api_config,
            "aws": aws_s3_config
        }
    },
    task_complexity="complex",
    session_state={
        "tool_call_count": 0,
        "has_plan": False
    }
)
```

**Output:**
```
You have access to the following tools:
- desktop-commander.read_file: Read file contents
- desktop-commander.write_file: Write file contents  
- desktop-commander.execute_command: Run shell commands
- github.list_repos: List GitHub repositories
- aws.list_buckets: List S3 buckets

COMPLEX TASK PLANNING:
This is a comprehensive system design task. Before implementation:
1. Create a detailed backup strategy plan covering all components
2. Assess current infrastructure and identify what needs backing up
3. Research backup best practices for development environments
4. Design backup procedures, schedules, and recovery processes
5. Document everything thoroughly for team use

SYSTEM ADMINISTRATION GUIDANCE:
- Gather comprehensive information about the current environment first
- Consider security implications of backup procedures
- Plan for both automated and manual backup scenarios
- Include testing and validation of backup integrity
- Document recovery procedures clearly

MULTI-TOOL COORDINATION:
- Use file system tools to examine local configuration
- Use GitHub tools to understand repository structure  
- Use AWS tools to assess cloud storage options
- Coordinate information from all sources before making recommendations

PROGRESS MONITORING:
Given the complexity of this task, regularly assess your progress and check with the user about your approach and findings.
```

## Architecture

### Application Stack
```
User Request
     ↓
LLM Application 
     ↓
prompt-composer (composes system prompt based on available MCP tools)
     ↓  
LLM (receives optimized system prompt + user prompt)
     ↓
MCP Tools (LLM calls these during execution)
```

### Transparent Knowledge Structure

Prompt-composer loads all guidance from readable text files, organized by purpose:

```
prompts/
├── domains/           # Domain-specific guidance
│   ├── filesystem.md     # File operations best practices
│   ├── programming.md    # Code development guidance  
│   ├── analysis.md       # Data analysis methodologies
│   └── system.md         # System administration patterns
├── behaviors/         # Universal behavioral patterns
│   ├── planning.md       # Complex task planning guidance
│   ├── progress.md       # Progress monitoring interventions
│   ├── tools.md          # General tool usage patterns
│   └── reasoning.md      # Meta-cognitive patterns
└── server_patterns.toml  # MCP server type recognition patterns
```

### Dynamic Tool Discovery

Prompt-composer uses **zero hardcoded server assumptions**. Instead, it loads server patterns from the same transparent prompts directory:

```
prompts/
└── server_patterns.toml  # Community-editable server patterns
```

This means:
- **No vendor lock-in**: Works with any MCP server, not just predefined ones
- **Community extensible**: Add new server types without touching core code
- **Transparent logic**: All pattern matching rules are visible and editable

### Dynamic Composition Process

1. **Tool Discovery**: Analyze MCP configuration using external patterns to identify available tools
2. **Context Assessment**: Evaluate user prompt, session state, and task complexity
3. **Module Selection**: Choose relevant prompt modules based on tools and context
4. **Content Loading**: Load guidance from appropriate `.md` files
5. **Intelligent Assembly**: Compose final system prompt with relevant guidance

This architecture ensures that **every piece of guidance is transparent, modifiable, and community-improvable**.

Prompt-composer is **application infrastructure** - it helps you build better prompts for your LLM, it's not a tool that your LLM calls.

### Four Layers of Intelligence

1. **Tool Behavior Layer** - Recognizes tool categories and provides optimal usage patterns
2. **Task Management Layer** - Planning, progress tracking, context management for complex work  
3. **Domain Knowledge Layer** - Field-specific methodologies (programming, analysis, system administration)
4. **Meta-Cognitive Layer** - Superior reasoning and self-monitoring patterns

**90% Prompt Engineering Knowledge + 10% Composition Logic**

## Community-Driven Knowledge Base

As this project grows, domain experts contribute proven patterns:
- **Medical professionals** → Clinical reasoning workflows (in `prompts/domains/medical.md`)
- **Financial analysts** → Market analysis methodologies (in `prompts/domains/finance.md`)
- **Legal experts** → Contract review approaches (in `prompts/domains/legal.md`)
- **Scientists** → Research methodology guidance (in `prompts/domains/research.md`)
- **Tool developers** → Optimal usage patterns for their MCP servers

### Transparent Contribution Process

Every contribution is **visible and reviewable**:
1. Submit new `.md` files or improvements to existing ones
2. Changes are reviewed by domain experts and the community
3. All guidance remains in plain text, readable by developers
4. No hidden algorithms or black-box prompt engineering

This creates a **virtuous cycle**: better prompts → better AI performance → more contributors → even better prompts.

Every contribution makes everyone's AI assistants more effective.

## Getting Started

### Python
```bash
pip install prompt-composer
```

```python
from prompt_composer import compose_system_prompt

# Compose system prompt based on your MCP configuration and session state
system_prompt = compose_system_prompt(
    user_prompt="Your user's request",
    mcp_config=your_mcp_configuration,
    session_state={
        "tool_call_count": 0,  # Track progress
        "has_plan": False      # Task planning state
    }
)

# Use with your local LLM
messages = [
    {"role": "system", "content": system_prompt},
    {"role": "user", "content": user_prompt}
]
response = your_local_llm.chat(messages)
```

### Node.js
```bash
npm install prompt-composer
```

```javascript
const { composeSystemPrompt } = require('prompt-composer');

// Compose system prompt 
const systemPrompt = await composeSystemPrompt({
    userPrompt: "Your user's request", 
    mcpConfig: yourMcpConfiguration,
    sessionState: {
        toolCallCount: 0,  // Track progress
        hasPlan: false     // Task planning state
    }
});

// Use with your local LLM
const messages = [
    { role: "system", content: systemPrompt },
    { role: "user", content: userPrompt }
];
const response = await yourLocalLLM.chat(messages);
```

### Other Languages
Language wrappers for Rust, Go, and other languages coming soon. The core engine is language-agnostic.

## Contributing

We welcome contributions of behavioral patterns, domain expertise, and tool optimization knowledge. This is community-owned infrastructure - every improvement benefits everyone using local AI.

---

**Democratizing high-functioning AI through systematized prompt engineering knowledge.**

*Your hardware, your models, your data - no token taxes.*
