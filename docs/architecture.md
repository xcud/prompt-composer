# Architecture Overview

## Core Philosophy

Prompt-composer is a **knowledge repository** of accumulated LLM interaction wisdom. Rather than complex algorithms, the intelligence lies in systematically encoding proven patterns that improve AI assistant effectiveness.

## Four Layers of Intelligence

### 1. Tool Behavior Layer
**Purpose**: Optimize LLM interactions with different tool categories

**Examples**:
- File system tools → "Always read before analyzing"
- Desktop-commander → "Use absolute paths, prefer read_file over shell commands"  
- API tools → "Request missing parameters, handle rate limits gracefully"

**Recognition**: Analyzes `available_tools` list to identify patterns and tool categories

### 2. Task Management Layer  
**Purpose**: Provide scaffolding for different complexity levels and task types

**Examples**:
- Complex tasks → Planning, decomposition, progress tracking
- Multi-step workflows → Context persistence, resumability patterns
- Long-running tasks → Self-monitoring, user check-ins

**Recognition**: Analyzes user prompt complexity, task indicators, session context

### 3. Domain Knowledge Layer
**Purpose**: Apply field-specific methodologies and best practices

**Examples**:
- Programming → Read code first, surgical edits, test incrementally
- Data analysis → Examine structure, document methodology, validate results
- Writing → Outline first, consider audience, support claims with evidence

**Recognition**: Keyword analysis, domain hints, tool combinations

### 4. Meta-Cognitive Layer
**Purpose**: Teach superior reasoning and self-monitoring patterns

**Examples**:
- Error handling → Investigate rather than assume
- Progress assessment → Evaluate after each major step  
- Communication → Discuss approach before major changes

**Recognition**: Applied universally with intensity based on task complexity

## System Flow

```
User Request + Available Tools + Context
           ↓
    Pattern Recognition Engine
           ↓
    Module Selection Logic
           ↓
   Layer-by-Layer Composition
           ↓
    Optimized System Prompt
```

## Intelligence Distribution

**90% Prompt Engineering Knowledge**: Carefully crafted behavioral modules encoding proven patterns

**10% Composition Logic**: Simple rule-based selection and assembly of modules

## Key Design Principles

### Accumulated Wisdom Over Algorithms
The value is in the systematized knowledge of what works, not in complex selection logic.

### Tool-Aware, Not Tool-Specific  
Recognizes categories and patterns rather than hardcoding specific tool names.

### Progressive Enhancement
Applies appropriate level of guidance based on task complexity and available context.

### Community-Driven Knowledge Base
Designed for easy contribution of new behavioral patterns and domain expertise.

### Context Separation
LLM guidance logic separated from application-specific concerns (paths, credentials, etc.).

## Module Interaction Patterns

### Complementary Modules
- Tool behaviors + Domain knowledge (e.g., file tools + programming practices)
- Task management + Meta-cognitive (e.g., planning + progress assessment)

### Conflict Resolution
- More specific modules override general ones
- Domain knowledge takes precedence over generic patterns
- Session context can modify base behaviors

### Composition Order
1. **Foundation**: Meta-cognitive reasoning patterns
2. **Structure**: Task management scaffolding
3. **Expertise**: Domain-specific knowledge  
4. **Tools**: Tool usage optimization
5. **Context**: Session-specific adaptations

This layered approach ensures coherent, effective system prompts that maximize LLM success probability across diverse scenarios.
