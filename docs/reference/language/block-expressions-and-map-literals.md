---
Status: Draft
Decision: provisional (syntax migration planned; implementation staged)
Scope: language surface syntax + AST representation
---

# Block Expressions and Map Literals (Provisional)

This document defines a *provisional* language direction to:

- Reserve `{ ... }` for **block expressions** (and statement blocks).
- Move map literals off `{ ... }` to avoid `{}` ambiguity in expression position.

This is a **spec-level** document. Implementation may be staged behind phases and migration gates.

## Selfhost compiler v1 (SSOT link)

The selfhost compiler “v1” boundary (the frozen subset used to unblock `.hako` mirbuilder migration) is defined here:

- `docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md`

## 1. Block Expression (`{ ... }`)

### Syntax (expression position)

`{ <stmt>* <tail_expr> }`

- Statements execute in source order.
- `tail_expr` is **required** and evaluated exactly once after the statements.

### Value

The block expression's value is the value of `tail_expr`.

Note: Empty blocks or blocks ending with a statement (without a trailing expression) are rejected at compile time. Use explicit `void` literal if needed.

### Exit statements (v1 rule)

In **expression position**, `return` / `break` / `continue` / `throw` **anywhere inside** a block expression are **rejected** (compile-time fail-fast). This includes exits nested inside control flow structures (e.g., `if true { return 1 }` is forbidden).

Rationale: permitting non-local exit turns block expressions into mini-CFG and expands verifier/lowering responsibility.

Statement blocks (e.g. `if ... { ... }`) continue to allow exit statements as normal.

### Parentheses recommendation

When using a block expression as a condition (e.g., in `if`), wrap it in parentheses for clarity:

```nyash
if ({ local a = calc(); a > 0 }) { ... }  // recommended
```

This avoids potential ambiguity with `if {cond}{then}` patterns.

### Condition position (planner-required)

Block expressions used in condition position (e.g. `if ({ ... }) { ... }`, `loop(({ ... })) { ... }`) execute their `prelude` statements before evaluating `tail_expr`, including under planner-required (strict/dev) compilation paths (Phase B4).

v1 constraint: the prelude statement vocabulary is restricted (and enforced) by SSOT:
- `src/mir/builder/control_flow/plan/policies/cond_prelude_vocab.rs`

### Examples

```nyash
local x = {
  local y = 10
  y + 1
}
```

```nyash
if ({
  local a = calc()
  a > 0
}) {
  print("ok")
}
```

## 2. Map Literal (`%{ ... }`)

### Syntax

`%{ <entry> (',' <entry>)* (',')? }`

`<entry> := <key> '=>' <expr>`

Key (v1):
- String key: `"k" => expr`
- (Optional future) Identifier key: `k => expr`

### Examples

```nyash
local m = %{"a" => 1, "b" => 2}
```

## 3. Backward Compatibility (provisional window)

During the migration window:

- Legacy map literal form `{ "k": v }` may remain accepted in some tools/paths.
- The long-term target is:
  - `{ ... }` is a block (expression or statement).
  - `%{ ... }` is the only map literal surface syntax.

The concrete migration schedule is tracked in design SSOT:

- `docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md`
