# Prompt Composer Examples

## Example 1: Simple File Reading Task

### Input
```python
compose_system_prompt(
    user_prompt="Look at config.json and tell me what's in it",
    available_tools=[
        {"name": "desktop-commander.read_file", "description": "Read file contents"},
        {"name": "desktop-commander.list_directory", "description": "List directory contents"}
    ]
)
```

### Output
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

## Example 2: Complex Programming Task

### Input
```python
compose_system_prompt(
    user_prompt="Refactor this Python codebase to use async/await throughout",
    available_tools=[
        {"name": "desktop-commander.read_file", "description": "Read file contents"},
        {"name": "desktop-commander.write_file", "description": "Write file contents"}, 
        {"name": "desktop-commander.edit_block", "description": "Make surgical edits"},
        {"name": "desktop-commander.search_code", "description": "Search within files"}
    ],
    domain_hints=["programming"],
    task_complexity="complex"
)
```

### Output
```
You have access to the following tools:
- desktop-commander.read_file: Read file contents
- desktop-commander.write_file: Write file contents
- desktop-commander.edit_block: Make surgical edits
- desktop-commander.search_code: Search within files

To use a tool, respond with JSON format as specified above.

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

## Example 3: Analysis Task with Session Context

### Input
```python
compose_system_prompt(
    user_prompt="Continue analyzing the sales data",
    available_tools=[
        {"name": "desktop-commander.read_file", "description": "Read file contents"},
        {"name": "data-analysis.process_csv", "description": "Process CSV data"}
    ],
    domain_hints=["analysis"],
    session_context={
        "tool_call_count": 6,
        "original_task": "Analyze Q4 sales data and identify trends",
        "has_plan": True
    }
)
```

### Output
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

## Example 4: Simple Task, No Special Context

### Input
```python
compose_system_prompt(
    user_prompt="What's the weather like today?",
    available_tools=[
        {"name": "weather.get_current", "description": "Get current weather"},
        {"name": "weather.get_forecast", "description": "Get weather forecast"}
    ]
)
```

### Output
```
You have access to the following tools:
- weather.get_current: Get current weather
- weather.get_forecast: Get weather forecast

To use a tool, respond with JSON format as specified above.

When working with weather APIs:
- Request location information if not provided by the user
- Handle API errors gracefully (service unavailable, invalid location, etc.)
- Provide clear, user-friendly weather information
```

## Example 5: Multi-Tool Complex Task

### Input
```python
compose_system_prompt(
    user_prompt="Create a comprehensive backup strategy for our development environment",
    available_tools=[
        {"name": "desktop-commander.read_file", "description": "Read file contents"},
        {"name": "desktop-commander.write_file", "description": "Write file contents"},
        {"name": "desktop-commander.execute_command", "description": "Run shell commands"},
        {"name": "github.list_repos", "description": "List GitHub repositories"},
        {"name": "aws.list_buckets", "description": "List S3 buckets"}
    ],
    task_complexity="complex"
)
```

### Output
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
