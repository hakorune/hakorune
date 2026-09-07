---
Status: Accepted (semantic contract; implementation migration staged)
Decision: accepted — `lexical_blockexpr`
Scope: language surface syntax + AST representation
---

# Block Expressions and Map Literals

This document defines the language contract to:

- Reserve `{ ... }` for **block expressions** (and statement blocks).
- Move map literals off `{ ... }` to avoid `{}` ambiguity in expression position.

This is a **spec-level** document. Implementation may be staged behind phases
and migration gates, but every source-level `BlockExpr` has the same lexical
meaning on every route.

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

This explicit BlockExpr tail is an expression result, not an implicit ordinary
function return. Function/Main fallthrough and Script result selection are
owned by `function-exit-and-entry-result.md`.

### Lexical scope

Every source-level block expression introduces one lexical scope.

```text
scope begins:
  immediately before the first prelude statement

scope contains:
  every prelude statement and the tail expression

scope ends:
  immediately after the tail expression has been evaluated once
```

Bindings declared inside the block expression are visible to later prelude
statements and to the tail expression, but do not escape the expression. The
tail value may escape. A rebind of an already-visible outer binding also
propagates normally; shadowing it with a new local does not.

```nyash
local x = 1
local y = {
  local x = 2
  x
}
// y == 2, outer x == 1
```

```nyash
local x = 1
local y = {
  x = 2
  x
}
// y == 2, outer x == 2
```

### Exit statements (v1 rule)

In **expression position**, `return` / `break` / `continue` are rejected
anywhere inside a block expression (compile-time fail-fast). `throw` is
rejected by the surface parser in general; it is mentioned here only for legacy
compatibility clarity.

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
- `src/mir/builder/control_flow/cleanup/policies/cond_prelude_vocab.rs`

The block-expression scope ends after its tail is evaluated. It does not
extend into an enclosing `if` then/else body or a loop body. A future syntax
that intentionally exposes a condition binding to branches requires a
separate language decision and a distinct scope owner; it must not be
desugared to a plain `BlockExpr`.

### Compatibility sequencing is not source syntax

Compiler compatibility paths may temporarily need an explicitly typed,
compiler-private sequencing carrier with no lexical scope. Such a carrier is
not `BlockExpr`, is not emitted by either source parser, and is not part of the
public language. The same `BlockExpr` node must never change scope semantics
according to producer, consumer, or lowering route.

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

## 3. Backward Compatibility (migration window)

During the migration window:

- Legacy map literal form `{ "k": v }` may remain accepted in some tools/paths.
- The long-term target is:
  - `{ ... }` is a block (expression or statement).
  - `%{ ... }` is the only map literal surface syntax.

`Program(JSON v0)` is an explicitly lossy compatibility family and is not
source lexical-scope authority. Untagged legacy artifacts must not be used to
infer whether a source statement was a declaration or an assignment. No new
compatibility-expression tag is added to the v0 wire schema.

The concrete migration schedule is tracked in design SSOT:

- `docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md`

## 4. Collection literal construction identity

Decision: accepted target — collection literals have intrinsic collection identity;
implementation cutover is pending. This decision does not change literal syntax.

Array literals `[...]` and Map literals `%{...}` select the language's builtin
Array and Map semantics. A same-named user box, static singleton or plugin override
must not change their construction or introduce an extra user `birth` call.
This also applies to an accepted legacy surface already classified as MapLiteral;
it does not extend acceptance of legacy map syntax.

Intrinsic identity does not freeze a Rust class or memory layout. A backend may
choose an equivalent physical representation under the existing verified
collection contract. Unannotated arrays remain `AnyDefault`; supported explicit
`Array<T>` keeps its element contract. Unsupported `PackedArray<T> = []` still
rejects and never falls back to ordinary Array (see [EBNF](EBNF.md)).

Named construction such as `new ArrayBox()` and `new MapBox()` remains under
the existing name-resolution/provider contract. Plugin configuration may affect
that explicit named operation where supported; it cannot redefine literal meaning.
This is a construction distinction, not a second global runtime registry.

Retain existing evaluation and failure order: allocate before evaluating entries,
evaluate children exactly once in source order, and perform each existing write
after its child evaluation. Keep current Map key, duplicate-key and iteration
semantics; this decision adds no Map feature. Allocation failure and child failure
are not suppressed by removal of a redundant birth marker.

The source literal owner selects intrinsic identity before child lowering.
The existing construction product must preserve that selection through publication
and physical transport. A string-only `NewBox("ArrayBox")` cannot prove whether
the source was a literal or named construction. Do not infer identity from names,
allocation origins, JSON, provider settings or a module's unrelated typed call.
Use an explicit intrinsic-versus-named target in the construction representation;
do not add a parallel lookup table or optional receipt to repair lost identity.

Migration order: retain source selection -> preserve and verify the construction
target -> selected consumer cutover -> remove the replaced literal birth edge.
Array goes first; Map is a separate slice. Existing wire inputs without that
distinction remain explicitly legacy/named data and cannot claim intrinsic
authority. Unsupported consumers reject before artifact instead of retrying name
resolution. Until this cutover is verified, current runtime behavior is not proof
that every execution path already implements this target contract.
