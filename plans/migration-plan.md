# Prompt Composer Migration Plan

## Objective
Extract existing system prompts from lit-server and lit-desktop into prompt-composer, then integrate prompt-composer back into both projects to achieve baseline functionality.

## Current State Analysis

### SimpleChatService Current Prompt Structure
- Tool-focused system prompt that's dynamically generated
- Tool descriptions from MCP integration
- Basic behavioral instructions for tool usage
- MCP tool creation instructions
- No modular structure - monolithic prompt building

### Existing Prompts in LIT Platform
- Extensive LangChain-based prompts in `lit-lib/src/lit/tools/old_tools/`
- Legacy agent prompts (supervisor, vault, build, analysis agents)
- Each has domain-specific knowledge and behavioral patterns
- Mix of system prompts and few-shot examples

## Migration Strategy

### Phase 1: Extract Current Prompts (This Phase)
1. **Extract SimpleChatService prompt logic** → `prompt-composer/modules/`
   - Tool usage behaviors → `behaviors/tool-usage.md`
   - MCP tool creation → `behaviors/mcp-tools.md`
   
2. **Extract key legacy agent prompts** → `prompt-composer/domains/`
   - Programming/code analysis patterns
   - LIT platform domain knowledge
   - Tool execution patterns

3. **Create minimal composition engine**
   - Basic Python module structure
   - Simple template-based composition
   - Module selection logic

### Phase 2: Replace Existing Integration
1. **Update SimpleChatService to use prompt-composer**
   - Replace hardcoded prompt building with composer calls
   - Maintain exact same functionality as baseline
   
2. **Update lit-desktop to use prompt-composer**
   - Identify any desktop-specific prompts
   - Migrate to prompt-composer

3. **Update lit-server to use prompt-composer**
   - Identify server-specific prompts  
   - Migrate to prompt-composer

### Phase 3: Verification
1. **Test baseline functionality**
   - Ensure chat behavior is identical to before migration
   - Verify tool usage patterns work correctly
   - Check MCP integration still functions

2. **Performance validation**
   - Compare response quality before/after
   - Validate no regression in capabilities

## Implementation Plan

### Step 1: Create Prompt Modules (Next)
- Extract tool usage patterns from SimpleChatService
- Create behavioral modules for key patterns
- Start with minimal viable structure

### Step 2: Create Composition Engine
- Basic Python module to combine prompt modules
- Simple API matching our proposed design
- Focus on reproducing current functionality exactly

### Step 3: Integration
- Replace SimpleChatService prompt logic with composer calls
- Ensure identical output for same inputs
- Gradual rollout with fallback capability

## Success Criteria
✅ **Baseline Achievement**: All current functionality works identically after migration
✅ **Clean Architecture**: Prompts are modular and reusable
✅ **Foundation Ready**: Architecture supports future enhancement and community contributions

## Risk Mitigation
- **Regression Risk**: Test extensively against current behavior
- **Complexity Risk**: Start with minimal viable implementation
- **Integration Risk**: Maintain fallback to current system during transition
