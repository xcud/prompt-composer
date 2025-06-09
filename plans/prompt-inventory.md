# LIT Platform Prompt Inventory

## Active Prompts (Non-deprecated)

### 1. SimpleChatService (`lit-lib/src/lit/tools/simple_chat.py`)

**Current Prompts:**
- **Tool Instructions** (lines 311-344): Dynamic system prompt with:
  - Tool usage format instructions
  - MCP tool creation instructions  
  - Basic behavioral guidelines for tool usage

**Key Patterns:**
- Dynamic tool description injection
- Team-specific path templating
- Tool usage enforcement

### 2. ToolCallProcessor (`lit-lib/src/lit/tools/chat_tools.py`)

**Current Prompts:**
- **Self-Monitoring Prompt** (`_create_monitoring_prompt`, lines 367-397):
  - Progress assessment based on tool call count
  - Original task reminders
  - Escalating guidance from 1-10+ tool calls

**Key Patterns:**
- Context-aware guidance
- Progress tracking
- Task completion detection

## Chat Session Context Requirements

### SimpleChatService Session Context:
- `team` - Team identifier for tool paths
- `username` - User context for MCP tools  
- `available_tools` - Dynamic tool descriptions
- `project_context` - Optional project information

### Workflow Context:
- `team` - Team for prompt file paths
- `strategy_name` - Strategy context
- `market_df`, `model_df` - Data context for prompt injection

## Migration Candidates

### High Priority (Core Chat Functionality):
1. **SimpleChatService tool instructions** → `behaviors/tool-usage.md`
2. **MCP tool creation instructions** → `behaviors/mcp-tools.md` 
3. **Self-monitoring prompts** → `behaviors/progress-tracking.md`

### Medium Priority (Workflow System):
4. **System/Final prompt loading** → Prompt file management system
5. **Data context injection** → Context injection patterns

### Lower Priority (Specialized Features):
7. **Agent system prompts** → Domain-specific modules

## Key Architectural Requirements

### Dynamic Content Support:
- Tool descriptions injection
- Team-specific path templating
- Data context variables
- Session state awareness

### File-Based Prompt Management:
- Team-specific prompt directories
- Prompt file loading and caching
- Override system support

### Modular Composition:
- Behavioral layer (tool usage, progress tracking)
- Domain layer (platform-specific knowledge)
- Context layer (team, session, data)
