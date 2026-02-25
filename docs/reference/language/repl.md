# REPL Mode (Read–Eval–Print Loop) — Specification (SSOT)

**Status**: Phase 288.1 complete (2025-12-25, Session Persistence + Auto-Display)

## Philosophy: Two Execution Contexts, One Language

Nyash provides **two execution contexts** for the same language:

1. **File Mode** - Strict, box-oriented, production-ready
   - Forces explicit structure (`static box Main { main() { ... } }`)
   - Prevents mutable globals at top-level
   - Optimized for large, maintainable codebases

2. **REPL Mode** - Interactive, exploratory, development-friendly
   - Implicit local variables for rapid iteration
   - Persistent session scope across inputs
   - Optimized for learning, debugging, and experimentation

**Key Principle**: Same parser, same language semantics. The difference is **binding rules** and **execution context**.

This document defines Nyash REPL mode semantics. The primary design goal is:

- **File mode** stays strict and box-oriented (no mutable globals at top-level).
- **REPL mode** is convenience-oriented (interactive, persistent session scope, implicit locals).
- **Parser stays the same**; the difference is primarily **binding (name resolution)** rules.

## Terms

- **Top-level**: Code that is not inside a function/method body.
- **File mode**: Normal execution of a `.hako`/Nyash file via the runner.
- **REPL mode**: Interactive execution session (future CLI: `--repl`).
- **Global variable (in this project)**: A mutable binding created at file top-level (e.g., `x = 1` or `local x = 1` at top-level).

## 1) File Mode vs REPL Mode (high-level contract)

### Starting REPL Mode

```bash
hakorune --repl   # Full flag
hakorune -i       # Short alias (interactive)
```

### File Mode (strict) - Code Example

```nyash
// File: program.hako

// ❌ ERROR: Top-level executable statements forbidden
x = 1
print("Hello")
local temp = 42

// ✅ ALLOWED: Declarations only
static box Main {
    main() {
        local x  // ✅ Explicit declaration REQUIRED
        x = 1
        print(x)
        return 0
    }
}
```

**File Mode Rules**:
- Top-level allows **declarations only** (`box`, `static box`, `function`, `static function`, `using`)
- Top-level **statements are rejected** (Fail-Fast):
  - Assignment (`x = 1`)
  - `local` declarations (`local x = 1`)
  - Expression statements (`f()`), `print(...)`, control flow, etc.
- **Rationale**: Prevents mutable globals; maintains "state lives in boxes" discipline

### REPL Mode (convenient) - Interactive Example

```nyash
>>> x = 1           // ✅ Implicitly creates session-level local
>>>

>>> print(x)        // ⏳ Phase 288.1: session→eval bridge needed
Error: Undefined variable 'x'

>>> y               // ❌ ERROR: Undefined variable (Fail-Fast)
Error: Undefined variable 'y'
Hint: Variable not defined. Assign a value first.

>>> local z = 3     // ✅ Explicit declaration also works
>>>

>>> print(z)
3

>>> 1 + 1           // ⏳ Phase 288.1: expression auto-display

>>> _               // ⏳ Phase 288.1: last value binding

>>> .reset          // ✅ Clear session
Session reset

>>> print(x)        // ❌ ERROR: Session cleared
Error: Undefined variable 'x'
```

**REPL Mode Rules**:
- The REPL has exactly one **persistent session scope**
- Session scope is conceptually a **lexical frame that persists across inputs**
- Assignments can create bindings implicitly (see §2)
- Reads of undefined names are errors (Fail-Fast; no silent `void`)
- **Implementation**: Session stores runtime values (`VMValue`), not MIR IDs

## Implementation Status (MVP vs planned)

Implemented in Phase 288 MVP:
- `hakorune --repl` / `-i`
- `.help`, `.exit/.quit`, `.reset`
- REPL-only session object exists (runtime-value based)
- File mode regression gate remains green

Planned for Phase 288.1:
- Session variables visible to subsequent compiled lines (`x = 1` then `print(x)`)
- Expression auto-display and `_` last-value binding

## 2) Binding Rules in REPL Mode

### 2.1 Implicit local on assignment (key feature)

When executing `name = expr` in REPL mode:

- If `name` already exists in the session scope, update it.
- If `name` does not exist, **create a new session binding** and assign to it.

This applies to compound assignments as well (if supported): `name += expr`, etc.

### 2.2 Reads are strict

When evaluating `name` in REPL mode:

- If `name` exists in the session scope, return its value.
- Otherwise, raise **NameError / undeclared variable** (Fail-Fast).

### 2.3 `local` is accepted but not required

REPL accepts `local name = expr` / `local name` as explicit declarations.

- Semantics: declare/update `name` in the session scope (same end result as implicit assignment).
- Guidance: `local` remains useful for clarity, but REPL users are not forced to write it.

## 3) Output Rules (REPL UX contract)

REPL output distinguishes expressions vs statements:

- If the input is an **expression**, print its value (pretty display) unless it is `void`.
- If the input is a **statement**, do not auto-print.

### 3.1 Convenience binding `_`

- `_` is bound to the **last auto-printed value** (expressions only).
- `_` is not updated when the value is `void`.

### 3.2 Output suppression (planned)

- A trailing `;` may suppress auto-print for expressions (planned; should be implemented without changing the core parser).

## 4) REPL Meta Commands

REPL supports dot-commands (not part of the language grammar):

- `.help` — show help
- `.exit` — exit the REPL
- `.reset` — clear the session scope (remove all bindings and definitions created in the session)

Additional commands may be added for debugging (e.g., `.ast`, `.mir`), but they must remain REPL-only and default-off for CI.

## 5) Implementation Contract (Phase 288 SSOT)

### 5.1 Architecture: Box-First Modularization

```
┌─────────────────────────────────────┐
│  Runner (src/runner/mod.rs)         │
│  ┌───────────────────────────────┐  │
│  │ ReplSessionBox                │  │  ← REPL専用state隔離
│  │ - variables: BTreeMap<        │  │
│  │     String, VMValue>          │  │  ← 実行時の値を永続化
│  │ - last_value: Option<VMValue> │  │
│  │ - eval_count: usize           │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│  MirBuilder (src/mir/builder.rs)    │
│  - repl_mode: bool                  │  ← mode 分岐フラグのみ
│  (assignment_resolver.rs は不変)   │  ← file mode 専用に保つ
└─────────────────────────────────────┘
```

### 5.2 Mode Flags

| Flag | File Mode | REPL Mode | Purpose |
|------|-----------|-----------|---------|
| `builder.repl_mode` | `false` | `true` | Implicit local 許可フラグ |
| `top_level_exec_allowed` | `false` | N/A | File mode: top-level 実行禁止 |

### 5.3 Session Persistence Contract

**File Mode**:
- Each file execution is independent
- No state persists across runs
- All variables are scoped to functions/boxes

**REPL Mode**:
- Session persists across line inputs
- Variables stored as `VMValue` (runtime values)
- **Critical**: NOT `ValueId` (MIR-compilation-specific, invalidated per line)

**Why VMValue?**
```rust
// ❌ WRONG: ValueId is per-compilation
session.variables: BTreeMap<String, ValueId>  // Invalid across lines!

// ✅ CORRECT: VMValue is runtime value
session.variables: BTreeMap<String, VMValue>  // Persists across compilations
```

### 5.4 Evaluation Pipeline (REPL)

```
User Input (line)
  ↓
Parse → AST
  ↓
MirCompiler (repl_mode=true)
  ↓
MIR Module
  ↓
MirInterpreter::execute_module()
  ↓
VMValue (return value)
  ↓
session.set_last_value(vm_value)  // Store in session
  ↓
Output (Phase 288.1: expression auto-display)
```

### 5.5 Fail-Fast Guarantee

**Both Modes**:
- Undefined variable reads → Immediate error
- No silent `void` or fallback values
- Clear error messages with hints

**File Mode Hint**:
```
Error: Undefined variable 'x'
Hint: Nyash requires explicit local declaration. Use `local <name>` before assignment.
```

**REPL Mode Hint**:
```
Error: Undefined variable 'x'
Hint: Variable not defined. Assign a value first.
```

## 6) Compatibility and `strip_local_decl` policy

Historical compatibility code exists that can strip top-level `local` from certain inputs.

SSOT policy:

- **File mode must not "strip and accept" top-level `local`** (would violate the "no globals" rule).
- If compatibility behavior is kept, it must be **REPL-only** and/or explicitly gated (e.g., `--compat`), with a stable warning tag.

## 7) Error Messages (Fail-Fast wording)

Recommended file-mode errors:

- `Error: top-level statements are not allowed in file mode. Put code inside Main.main() or run with --repl.`
- `Error: 'local' is not allowed at top-level in file mode. Use Main.main() or REPL mode.`

## 8) Minimal Conformance Tests (spec lock)

### File mode

1. `x = 1` at top-level → error (top-level statements not allowed)
2. `local x = 1` at top-level → error (local not allowed at top-level)
3. `print("hi")` at top-level → error
4. Declarations at top-level → OK
5. Statements inside `Main.main()` or `main()` → OK

### REPL mode

1. `x = 1` then `x` → prints `1`
2. `y` (undefined) → NameError
3. `x = 1; x = 2; x` → prints `2`
4. `local z = 3; z` → prints `3`
5. `x = 1; .reset; x` → NameError

---

## 9) Phase 288 MVP Implementation Status

### Completed (Phase 288.1) - 2025-12-25

✅ **CLI Entry** (`--repl` / `-i` flags)
✅ **REPL Loop** (`.help`, `.exit`, `.reset` commands)
✅ **ReplSessionBox** (VMValue-based session state with `Rc<RefCell<>>`)
✅ **Implicit Local Binding** (`x = 1` compiles without error)
✅ **print() Output** (ExternCall output displays correctly)
✅ **Variable Persistence** (session variables accessible across lines via AST rewrite)
✅ **Expression Auto-Display** (`1+1` → `2` automatic output)
✅ **`_` Variable** (last displayed value accessible, Void not stored)
✅ **Session Reset** (`.reset` command clears state)
✅ **Fail-Fast Undefined Variables** (clear error messages with hints)

### Implementation Architecture (Phase 288.1)

**AST Rewrite Approach**:
```
User Input: x = 42
  ↓
Parse → AST (static box Main { main() { x = 42 } })
  ↓
AST Rewriter (REPL-specific):
  - Undeclared Variable: x → __repl.get("x")
  - Undeclared Assignment: x = 42 → __repl.set("x", 42)
  - Respects: local declarations, reserved names (me/true/false/null)
  - Skips: nested scopes (function/method bodies), Main wrapper
  ↓
Rewritten AST → MIR Compiler
  ↓
ExternCall("__repl", "get"/"set", args)
  ↓
VM Execution → ReplSessionBox.get/set
```

**Key Components**:
- **`src/runner/repl/ast_rewriter.rs`** (~430 lines): AST transformation
- **`src/mir/builder/calls/build.rs`**: `__repl.get/set` → ExternCall MIR lowering
- **`src/backend/mir_interpreter/handlers/externals.rs`**: ExternCall handlers
- **`Rc<RefCell<ReplSessionBox>>`**: Session sharing between REPL runner and VM

**Expression Detection**:
- Checks wrapper AST (`Main.main` body) for single expression node
- Pure expressions: Literal, BinaryOp, MethodCall, New, etc.
- Statements: Assignment, Local, Print, If, Loop, etc.
- Void values not displayed or stored in `_`

### Current Behavior (Phase 288.1)

```nyash
>>> .help
Commands:
  .exit / .quit - Exit REPL
  .reset - Clear session
  .help - Show this help

>>> x = 42
>>>                    # Silent (implicit local creation via __repl.set)

>>> print(x)
42                     # ✅ Variable persistence works!

>>> 1 + 1
2                      # ✅ Expression auto-display works!

>>> _
2                      # ✅ Last displayed value accessible

>>> print("Hello")
Hello                  # Statements don't auto-display

>>> .reset
Session reset

>>> print(x)
Error: Runtime error: Invalid instruction: Undefined variable: 'x'
Hint: Variable not defined. Assign a value first.
>>>                    # ✅ Fail-Fast with clear error
```

**Design Principles**:
- **AST Rewrite**: REPL-specific transformation, parser unchanged
- **ExternCall Bridge**: Clean VM/REPL separation
- **Fail-Fast**: No silent void, clear error messages
- **Box-First**: Complete isolation in `src/runner/repl/`

---

**Document Version**: Phase 288.1 (2025-12-25)
**Implementation**: Complete (CLI + Session + Persistence + Auto-Display)
**Next**: Further UX improvements (multiline input, syntax highlighting, etc.)
