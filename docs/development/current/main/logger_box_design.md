# Phase 105: Logger Box Design & Framework

## 1. Overview

Logger Box provides a structured, level-based logging interface for Nyash applications, built on top of ConsoleBox (Phase 104). It enables developers to:
- Use DEBUG/INFO/WARN/ERROR log levels
- Filter logs by level at runtime
- Compose logger boxes for specific use cases
- Prepare for future output redirection (FileBox/NetworkBox in Phase 106+)

## 2. Design Principles

1. **Single Responsibility**: Each Logger Box has one logging purpose
2. **ConsoleBox Centered**: Phase 105 scope: all output via ConsoleBox
3. **Future-Proof**: Interface designed to support FileBox/NetworkBox in Phase 106+
4. **Composable**: Logger boxes can be combined or wrapped
5. **No Duplication**: Single ConsoleBox instance per logger

## 3. Log Levels

```
Priority (High → Low):
  ERROR    - Critical errors requiring immediate attention
  WARN     - Warnings that might indicate problems
  INFO     - General informational messages
  DEBUG    - Detailed debugging information
```

Level numeric mapping:
- DEBUG = 0
- INFO = 1
- WARN = 2
- ERROR = 3

## 4. Core Logger Box Interface (Reference Design)

This is a **reference design** showing the conceptual interface. Implementations may vary based on specific needs.

### Pseudo-Code Example

```nyash
box LoggerBox {
    // State
    console: ConsoleBox
    min_level: IntegerBox   // Minimum level to output (0=DEBUG, 1=INFO, 2=WARN, 3=ERROR)

    // Constants (reference levels)
    // DEBUG=0, INFO=1, WARN=2, ERROR=3

    birth(min_level: IntegerBox) {
        me.console = new ConsoleBox()
        me.min_level = min_level
    }

    // Helper: check if message should be logged
    fn enabled?(level: IntegerBox) -> BoolBox {
        return level >= me.min_level
    }

    // Public methods: log at each level
    debug(msg: StringBox) {
        if me.enabled?(0) {  // 0 = DEBUG level
            me.console.println("[DEBUG] " + msg)
        }
    }

    info(msg: StringBox) {
        if me.enabled?(1) {  // 1 = INFO level
            me.console.println("[INFO] " + msg)
        }
    }

    warn(msg: StringBox) {
        if me.enabled?(2) {  // 2 = WARN level
            me.console.println("[WARN] " + msg)
        }
    }

    error(msg: StringBox) {
        if me.enabled?(3) {  // 3 = ERROR level
            me.console.println("[ERROR] " + msg)
        }
    }
}
```

### Design Notes

- **Level comparison**: Use integers (0-3) for simplicity in Nyash
- **Optional optimization**: Environments could add bitwise filtering or enum-based levels in Phase 106+
- **ConsoleBox fixed**: Phase 105 always outputs to ConsoleBox
- **Future extensibility**: Interface design allows swapping ConsoleBox for FileBox/NetworkBox in Phase 106+

## 5. Three Design Patterns

### Pattern 1: Lightweight Logger Box

**Purpose**: Minimal logging for simple tools

**Conceptual design**:
```nyash
box SimpleLoggerBox {
    console: ConsoleBox

    birth() {
        me.console = new ConsoleBox()
    }

    log(msg: StringBox) {
        me.console.println(msg)
    }
}
```

**Use case**: Progress tracking, simple tools
**Key feature**: No levels, just output

### Pattern 2: Structured Logger Box

**Purpose**: Level-based logging with standard prefixes

**Conceptual design**:
```nyash
box StructuredLoggerBox {
    console: ConsoleBox
    min_level: IntegerBox

    birth(min_level: IntegerBox) {
        me.console = new ConsoleBox()
        me.min_level = min_level
    }

    debug(msg: StringBox) {
        if me.min_level <= 0 {
            me.console.println("[DEBUG] " + msg)
        }
    }

    info(msg: StringBox) {
        if me.min_level <= 1 {
            me.console.println("[INFO] " + msg)
        }
    }

    warn(msg: StringBox) {
        if me.min_level <= 2 {
            me.console.println("[WARN] " + msg)
        }
    }

    error(msg: StringBox) {
        if me.min_level <= 3 {
            me.console.println("[ERROR] " + msg)
        }
    }
}
```

**Use case**: General applications
**Key feature**: ERROR/WARN/INFO/DEBUG methods

### Pattern 3: Contextual Logger Box

**Purpose**: Logging with context information (request ID, operation name, etc.)

**Conceptual design**:
```nyash
box ContextualLoggerBox {
    console: ConsoleBox
    context: StringBox        // Request ID, operation name, etc.
    min_level: IntegerBox

    birth(context: StringBox, min_level: IntegerBox) {
        me.console = new ConsoleBox()
        me.context = context
        me.min_level = min_level
    }

    debug(msg: StringBox) {
        if me.min_level <= 0 {
            me.console.println("[" + me.context + "][DEBUG] " + msg)
        }
    }

    info(msg: StringBox) {
        if me.min_level <= 1 {
            me.console.println("[" + me.context + "][INFO] " + msg)
        }
    }

    warn(msg: StringBox) {
        if me.min_level <= 2 {
            me.console.println("[" + me.context + "][WARN] " + msg)
        }
    }

    error(msg: StringBox) {
        if me.min_level <= 3 {
            me.console.println("[" + me.context + "][ERROR] " + msg)
        }
    }
}
```

**Use case**: Multi-tenant systems, request handlers
**Key feature**: Context prefix in all messages

## 6. Implementation Guidance

### For Users Implementing Logger Box

When creating your own Logger Box implementation:

1. **Initialize once**: Create logger box once, reuse for all logging
2. **Level filtering**: Implement early exit for disabled levels
3. **Message formatting**: Consider prefix convention ([LEVEL])
4. **ConsoleBox encapsulation**: Keep ConsoleBox as internal detail

### Recommended Base Implementation

Start with Pattern 2 (StructuredLoggerBox) as it covers most use cases. Extend with Pattern 3 when context becomes important.

## 7. Integration with Phase 99-104 Logging Policy

### Mapping to Logging Categories

| Category | Logger Box Role | Phase 105 Support |
|----------|-----------------|-------------------|
| user-facing | Could use SimpleLogger + ConsoleBox directly (no Logger Box needed) | Simple messages only |
| dev-debug | StructuredLogger + DEBUG level | Supported |
| monitoring | StructuredLogger + INFO level | Supported |
| internal Rust | Ring0.log (not Logger Box) | N/A |

### Notes

- **User-facing messages**: Often don't need Logger Box (direct ConsoleBox sufficient)
- **Dev-debug**: Use StructuredLogger with DEBUG level enabled
- **Monitoring**: Use StructuredLogger with INFO level enabled
- **Rust internal**: Ring0.log handles this (separate from Logger Box)

## 8. Reference Examples

These are **reference examples** for understanding Logger Box design. They are NOT intended to be run or tested in Phase 105. Execution and testing will be part of Phase 106+.

### Example 1: Simple Logger Box

```nyash
// Reference: Simple logging without levels
box SimpleLoggerBox {
    console: ConsoleBox

    birth() {
        me.console = new ConsoleBox()
    }

    log(msg) {
        me.console.println(msg)
    }
}

// Usage example
static box SimpleLoggingApp {
    main() {
        local logger = new SimpleLoggerBox()
        logger.log("Starting application...")
        logger.log("✅ Success!")
    }
}
```

### Example 2: Structured Logger Box

```nyash
// Reference: Level-based structured logging
box StructuredLoggerBox {
    console: ConsoleBox
    min_level: IntegerBox  // 0=DEBUG, 1=INFO, 2=WARN, 3=ERROR

    birth(min_level) {
        me.console = new ConsoleBox()
        me.min_level = min_level
    }

    debug(msg) {
        if me.min_level <= 0 {
            me.console.println("[DEBUG] " + msg)
        }
    }

    info(msg) {
        if me.min_level <= 1 {
            me.console.println("[INFO] " + msg)
        }
    }

    warn(msg) {
        if me.min_level <= 2 {
            me.console.println("[WARN] " + msg)
        }
    }

    error(msg) {
        if me.min_level <= 3 {
            me.console.println("[ERROR] " + msg)
        }
    }
}

// Usage example
static box DataProcessor {
    logger: StructuredLoggerBox

    main() {
        me.logger = new StructuredLoggerBox(1)  // INFO level minimum

        me.logger.debug("Debug info (won't show)")
        me.logger.info("Processing started...")
        me.logger.warn("Low memory warning")
        me.logger.error("❌ Critical error!")
    }
}
```

### Example 3: Contextual Logger Box

```nyash
// Reference: Context-aware logging
box ContextualLoggerBox {
    console: ConsoleBox
    context: StringBox
    min_level: IntegerBox

    birth(context, min_level) {
        me.console = new ConsoleBox()
        me.context = context
        me.min_level = min_level
    }

    debug(msg) {
        if me.min_level <= 0 {
            me.console.println("[" + me.context + "][DEBUG] " + msg)
        }
    }

    info(msg) {
        if me.min_level <= 1 {
            me.console.println("[" + me.context + "][INFO] " + msg)
        }
    }

    warn(msg) {
        if me.min_level <= 2 {
            me.console.println("[" + me.context + "][WARN] " + msg)
        }
    }

    error(msg) {
        if me.min_level <= 3 {
            me.console.println("[" + me.context + "][ERROR] " + msg)
        }
    }
}

// Usage example
box RequestHandler {
    logger: ContextualLoggerBox

    birth(request_id) {
        me.logger = new ContextualLoggerBox(request_id, 1)
    }

    process(data) {
        me.logger.info("Request received")
        // ... processing ...
        me.logger.info("Request completed")
    }
}

static box RequestProcessingApp {
    main() {
        local handler1 = new RequestHandler("req-001")
        local handler2 = new RequestHandler("req-002")

        handler1.process(10)
        handler2.process(20)
    }
}
```

### Important Notes on Examples

These are **reference examples** for understanding Logger Box design. Users should:
1. Adapt these to their specific needs
2. Implement their own versions based on their application requirements
3. Consider Phase 106+ extensions for file/network output if needed

## 9. Anti-Patterns to Avoid

❌ **DON'T: Multiple ConsoleBox instances**
```nyash
box BadLogger {
    main() {
        local c1 = new ConsoleBox()
        local c2 = new ConsoleBox()  // Wrong: duplicate
        c1.println("msg1")
        c2.println("msg2")
    }
}
```

❌ **DON'T: Mix logging into business logic**
```nyash
process(data) {
    print("[DEBUG] processing...")  // Wrong: mixed concerns
    return data * 2
}
```

❌ **DON'T: Ignore level filtering**
```nyash
debug(msg) {
    me.console.println("[DEBUG] " + msg)  // Wrong: always outputs
}
```

✅ **DO: Centralize logging**
```nyash
box DataProcessor {
    logger: StructuredLoggerBox

    birth() {
        me.logger = new StructuredLoggerBox(1)  // INFO level
    }

    process(data) {
        me.logger.info("Starting process...")
        return data * 2
    }
}
```

## 10. Future Extensions (Phase 106+)

These are planned for future phases:

### Phase 106: Output Redirection
- Logger Box could redirect to FileBox for file logging
- Or redirect to NetworkBox for remote logging
- Interface stays compatible

### Phase 107: Application Migration
- Migrate hako_check, selfhost-compiler to use Logger Box
- Standardize logging across all Nyash tools

### Phase 108+: Advanced Features
- Structured logging (JSON format)
- Log aggregation
- Performance metrics

## 11. Related Documentation

- [Phase 99: logging_policy.md](logging_policy.md) - Overall logging framework
- [Phase 104: hako_logging_design.md](hako_logging_design.md) - User application logging patterns
- [Phase 104: ring0-inventory.md](ring0-inventory.md) - Logging infrastructure inventory
