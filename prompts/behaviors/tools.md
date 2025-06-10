# Tool Usage Patterns

General guidance for effective tool utilization and interaction patterns.

## Tool Selection Principles

- **Use the most appropriate tool for each task**
- **Understand that MCP tools provide direct function calls** - not shell commands
- **Prefer direct tool functions over generic shell commands when available**
- **Consider the context and requirements of the specific situation**

## Tool Call Format

### Direct Function Calls
Most MCP tools provide direct function calls. Use them directly:
```
read_file("/path/to/file.txt")
list_directory("/home/user")
search_code("/project", "function_name")
```

### NOT shell commands through execute_command:
```
execute_command("cat /path/to/file.txt", 5000)  // ❌ Inefficient, avoid this
execute_command("ls /home/user", 5000)          // ❌ Use list_directory instead
```

### When to use execute_command:
Only use execute_command for operations that don't have direct tool functions:
- Running build tools (npm, cargo, make)
- Starting/stopping services
- Complex shell operations with pipes/redirects
- System administration commands

## Tool Decision Tree

1. **Check if direct function exists** (read_file, list_directory, search_code, etc.)
   → Use the direct function
2. **If no direct function available** 
   → Use execute_command with appropriate timeout

## Error Handling
- **Handle tool errors gracefully and informatively**
- **Provide fallback options when primary tools fail**
- **Explain tool limitations clearly to users**
- **Retry with different parameters when appropriate**

## Efficiency Guidelines

### Batch Operations
- **Group related operations when possible**
- **Minimize redundant tool calls**
- **Cache results when appropriate for reuse**

### Tool Sequencing
- **Plan tool usage sequences for optimal workflow**
- **Validate prerequisites before complex operations**
- **Use tool outputs effectively as inputs to subsequent operations**

## Quality Assurance

### Verification
- **Verify tool results before proceeding with dependent operations**
- **Cross-check important results using multiple approaches**
- **Validate assumptions about tool behavior and outputs**

### User Communication
- **Explain tool usage and reasoning to users when helpful**
- **Provide progress updates during long-running operations**
- **Clarify when tool limitations may affect outcomes**

## Common Anti-Patterns to Avoid

❌ **Using execute_command for file operations**:
```
execute_command("cat file.txt", 5000)  // Wrong
```
✅ **Use direct function instead**:
```
read_file("file.txt")  // Correct
```

❌ **Using execute_command for directory listing**:
```
execute_command("ls /directory", 5000)  // Wrong
```
✅ **Use direct function instead**:
```
list_directory("/directory")  // Correct
```

❌ **Using execute_command for code search**:
```
execute_command("grep -r 'pattern' /code", 10000)  // Wrong
```
✅ **Use direct function instead**:
```
search_code("/code", "pattern")  // Correct
```
