# Module System

## Module Categories

### Tool Behavior Modules
Encode optimal usage patterns for different categories of tools.

**File System Tools** (`behaviors/tools/file-system.md`):
```
When file system tools are available:
- ALWAYS read files before analyzing or modifying them
- Use absolute paths for reliability  
- Prefer specialized file tools over shell commands
- Handle potential file access errors gracefully
```

**Desktop Commander** (`behaviors/tools/desktop-commander.md`):
```
When desktop-commander tools are detected:
- Use read_file instead of execute_command for viewing files
- Use write_file with chunking for large files (25-30 lines max)
- Use edit_block for surgical changes rather than full rewrites
- Always use absolute paths starting with '/' or drive letters
```

**API Tools** (`behaviors/tools/api-tools.md`):
```
When API tools are available:
- Request required parameters if not provided by user
- Handle rate limiting and error responses gracefully
- Cache results when appropriate to avoid redundant calls
```

### Task Management Modules
Proven patterns for handling different types of tasks.

**Complex Task Planning** (`behaviors/tasks/complex-planning.md`):
```
For multi-step tasks:
1. Create a detailed plan in markdown format
2. Break down into concrete, actionable steps
3. Save the plan to a file for persistence
4. Check off completed steps as you progress
5. Update the plan if requirements change
```

**Context Window Management** (`behaviors/tasks/context-management.md`):
```
For tasks requiring multiple interactions:
- Save progress to files rather than relying on conversation memory
- Use structured formats (markdown, JSON) for persistent state
- Include enough context in saved files to resume work later
- Reference previous work explicitly when continuing
```

**Progress Monitoring** (`behaviors/tasks/progress-monitoring.md`):
```
Self-assessment patterns based on progress:
- After 1-3 actions: "Am I making progress toward the goal?"
- After 4-6 actions: "Should I check with the user about my approach?"  
- After 7+ actions: "Time to summarize progress and confirm direction"
```

### Domain Knowledge Modules
Field-specific best practices and methodologies.

**Programming** (`domains/programming.md`):
```
When working with code:
- Read and understand existing code before making changes
- Make surgical, targeted edits rather than large rewrites
- Test changes incrementally when possible
- Follow project conventions and existing patterns
- Consider backwards compatibility and testing implications
```

**Data Analysis** (`domains/analysis.md`):
```
When analyzing data:
- Examine data structure and quality first
- Document methodology and assumptions
- Show work step-by-step for transparency
- Validate results and check for edge cases
- Summarize findings clearly with supporting evidence
```

**Writing & Communication** (`domains/writing.md`):
```
When creating written content:
- Consider audience and purpose before writing
- Create an outline or structure first
- Use clear, concise language appropriate to context
- Support claims with evidence or examples
- Review and revise for clarity and completeness
```

### Meta-Cognitive Modules
Superior reasoning and self-monitoring patterns.

**Error Handling** (`behaviors/meta/error-handling.md`):
```
When encountering uncertainty:
- Investigate rather than making assumptions
- Ask clarifying questions when requirements are ambiguous
- Acknowledge limitations and suggest alternatives
- Document assumptions you're making explicitly
```

**Step-by-Step Reasoning** (`behaviors/meta/reasoning.md`):
```
For complex reasoning:
- Break problems into smaller, manageable pieces
- Show your thinking process explicitly
- Validate each step before proceeding
- Consider alternative approaches and tradeoffs
```

## Module Selection Logic

### Automatic Recognition Patterns
```python
# Tool category recognition
if any("read_file" in tool["name"] for tool in available_tools):
    modules.append("behaviors/tools/file-system")
    
if any("desktop-commander" in tool["name"] for tool in available_tools):
    modules.append("behaviors/tools/desktop-commander")

# Task complexity recognition  
if task_complexity == "complex" or len(user_prompt.split()) > 50:
    modules.append("behaviors/tasks/complex-planning")
    
# Domain recognition
if any(keyword in user_prompt.lower() for keyword in ["code", "function", "class", "refactor"]):
    modules.append("domains/programming")
```

### Session Context Integration
```python
# Progress monitoring based on session state
if session_context.get("tool_call_count", 0) > 3:
    modules.append("behaviors/tasks/progress-monitoring")
    
# Planning status
if not session_context.get("has_plan", False) and task_complexity == "complex":
    modules.append("behaviors/tasks/complex-planning")
```

## Module Format

Each module contains:
1. **Trigger Conditions**: When this module should be activated
2. **Core Instructions**: The actual prompt text to include
3. **Examples**: Demonstrations of the desired behavior
4. **Integration Notes**: How this module works with others

## Composition Strategy

Modules are layered in priority order:
1. **Meta-Cognitive**: Fundamental reasoning patterns
2. **Task Management**: Task-specific scaffolding  
3. **Domain Knowledge**: Field-specific expertise
4. **Tool Behaviors**: Tool-specific usage patterns
5. **Context Integration**: Session and user-specific information

The result is a coherent system prompt that maximizes success probability.
