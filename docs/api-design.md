# API Design

## Core API

### Primary Function
```python
def compose_system_prompt(
    user_prompt: str,
    available_tools: List[Dict[str, str]] = None,
    domain_hints: List[str] = None,
    session_context: Dict[str, Any] = None,
    task_complexity: str = "simple"  # simple, complex, multi-step
) -> str:
    """
    Compose optimal system prompt by recognizing patterns and applying proven techniques.
    
    Args:
        user_prompt: The user's natural language request
        available_tools: List of available tools with name/description
        domain_hints: Optional hints about expected domains (programming, analysis, etc.)
        session_context: Optional context (tool_call_count, has_plan, original_task, etc.)
        task_complexity: Estimated complexity level for appropriate scaffolding
        
    Returns:
        Optimized system prompt string ready for LLM
    """
```

## Usage Examples

### Basic Tool Recognition
```python
from prompt_composer import compose_system_prompt

available_tools = [
    {"name": "desktop-commander.read_file", "description": "Read file contents"},
    {"name": "desktop-commander.write_file", "description": "Write to file"}
]

system_prompt = compose_system_prompt(
    user_prompt="Look at config.json and fix any issues",
    available_tools=available_tools
)
# Returns prompt with file system best practices + desktop-commander specifics
```

### Complex Task with Domain Knowledge
```python
system_prompt = compose_system_prompt(
    user_prompt="Refactor this Python codebase to use async/await",
    available_tools=file_system_tools,
    domain_hints=["programming"],
    task_complexity="complex"
)
# Returns prompt with:
# - File system usage patterns
# - Programming best practices  
# - Complex task planning guidance
# - Progress tracking instructions
```

### Session-Aware Progress Monitoring
```python
system_prompt = compose_system_prompt(
    user_prompt="Continue the refactoring",
    available_tools=file_system_tools,
    session_context={
        "tool_call_count": 5,
        "original_task": "Refactor Python codebase",
        "has_plan": True,
        "current_step": "implementing async patterns"
    }
)
# Returns prompt with escalated guidance based on tool usage
```

## Internal Architecture

### Recognition Engine
1. **Tool Pattern Recognition**: Identifies tool categories and their optimal usage patterns
2. **Task Analysis**: Analyzes request complexity and decomposition needs  
3. **Domain Detection**: Recognizes field-specific requirements
4. **Context Assessment**: Evaluates session state and progress

### Module Selection
1. **Tool Behaviors**: Select relevant tool usage modules
2. **Task Patterns**: Choose appropriate task management scaffolding
3. **Domain Knowledge**: Apply field-specific best practices
4. **Meta-Cognitive**: Add reasoning and self-monitoring guidance

### Composition Process
1. **Layer Assembly**: Combine selected modules in priority order
2. **Context Injection**: Integrate session-specific information
3. **Optimization**: Remove redundancy, ensure clarity and coherence
4. **Validation**: Ensure prompt supports the user's specific request

## Design Principles

### Intelligence Over Configuration
- Automatic pattern recognition rather than manual specification
- Smart defaults with minimal required parameters
- Progressive enhancement based on context

### Accumulated Wisdom
- Repository of proven prompt engineering techniques
- Patterns learned from successful LLM interactions
- Community-driven knowledge base of best practices

### Composable Architecture
- Modular components that combine cleanly
- Clear separation between tool, task, domain, and cognitive layers
- Easy to extend with new patterns and knowledge
