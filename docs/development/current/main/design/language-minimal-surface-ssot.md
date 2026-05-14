---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: Minimal canonical Hakorune language surface for low-level and selfhost work.
Related:
  - docs/development/current/main/design/delegation-no-inheritance-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
  - docs/reference/boxes-system/delegation-system.md
  - docs/reference/core-language/override-delegation-syntax.md
  - docs/reference/language/using.md
---

# Language Minimal Surface SSOT

## Decision

Hakorune keeps one canonical spelling for each core idea.
Before adding a keyword, prefer extending an existing header shape if it does not create ambiguous meaning.

Small surface, strong semantics:

```text
surface syntax:
  keep small

semantic metadata:
  may grow when it makes compiler facts explicit

verifier / CorePlan / backend capability:
  may grow behind fail-fast gates

Stage0:
  read / transport / trivial desugar only

Stage1:
  owns language meaning
```

This document fixes the current simplification rule:

```text
repetition:
  loop only

concrete composition:
  delegate field exposes method list

cleanup:
  fini

capability:
  uses first

identity object:
  box

identity-free aggregate:
  record

state value:
  enum

lifecycle relation:
  transition
```

## Surface admission checklist

Use this checklist before accepting any new language surface.

| Question | Required decision |
| --- | --- |
| Can an existing canonical family express it? | Do not add a new keyword; fold into the existing family. |
| Is it only an alias for an existing spelling? | Keep it non-canonical or reject it. |
| Would Stage0 need to own semantic meaning? | Reject the shape or move the meaning to a Stage1 row. |
| Can the compiler learn it as type metadata / verifier facts / CorePlan facts? | Prefer metadata/facts over syntax. |
| Does unsupported backend behavior exist? | Define fail-fast behavior before implementation. |
| Does it require inference? | Split inference into its own Stage1 row; do not hide it in parser work. |
| Does it add sugar for convenience only? | Defer until a real code owner needs it. |

Allowed growth:

```text
type metadata
brand facts
record layout facts
verifier facts
CorePlan lowering recipes
fail-fast capability gates
standard protocol contracts after concrete delegation is insufficient
```

Default-forbidden growth:

```text
new keywords
parallel aliases
duplicate sugar families
implicit inference
silent fallback
Stage0 semantic checkers
```

## Fold-first rule

When a feature request appears, fold it into the existing family first.

| Requested family | Canonical fold |
| --- | --- |
| `while` / `for` / `repeat` / `until` / `do` | `loop` header shapes |
| `try` / `throw` / `?` | `Result<T,E>` + `guard` + `match` |
| `class` / `extends` / `super` | `box` + `delegate`; protocol later only if needed |
| `struct` / `data` / `valuebox` | `record` |
| `Vec<T>` / `List<T>` / canonical `T[]` | `Array<T>` |
| broad `unsafe` | `uses` / capability facts |
| event keyword syntax | box protocol / future interface contract, not syntax |
| `state` keyword | `enum` + `transition` |

This is the main guardrail: Hakorune can gain richer meaning without gaining a
large surface.

## Canonical keyword families

| Family | Canonical surface |
| --- | --- |
| identity object | `box` |
| concrete composition | `delegate` |
| identity-free aggregate | `record` |
| sum/state value | `enum` |
| scalar meaning | `brand` |
| repetition | `loop` |
| branch | `if`, `guard`, `match` |
| control exit | `break`, `continue`, `return` |
| local binding | `local` |
| scope cleanup | `fini` |
| proof list | `check` |
| contract | `assert`, `invariant`, `requires`, `ensures` |
| lifecycle relation | `transition` |
| capability declaration | `uses` |
| current import | `using` |
| current static table | `static const` |

Reserved protocol surface:

```text
interface:
  deferred until concrete delegate/exposes cannot express the method-set need

impl / trait-object / default methods / associated types / blanket impl:
  not MVP
```

## Not canonical for MVP

These spellings must not be added as MVP syntax.
If a later card wants one, it must explain why the canonical family above cannot express the need.

```text
while
for
repeat
until
do
defer
struct
class
extends
super
origin
let
var
unsafe
try
throw
state
cap block syntax
all
assert_all
valuebox
data
```

## Loop-only repetition

Canonical repetition surface:

```hako
loop condition {
    ...
}

loop i in start..end {
    ...
}

loop {
    ...
}
```

Meaning split:

| Shape | Meaning | Owner |
| --- | --- | --- |
| `loop cond { ... }` | pre-condition loop, same family as existing `loop(expr)` | existing parser / Stage1 loop facts |
| `loop i in start..end { ... }` | compiler-managed end-exclusive range loop | Stage0 parser capsule, Stage1 lowering |
| `loop { ... }` | infinite loop | existing parser / Stage1 loop facts |

`while` and `for` are not canonical keywords.
`repeat` / `until` / `do while` are not canonical keywords.

Range-loop MVP rules:

```text
start and end are evaluated once at loop entry
index is block-local
index is read-only
continue goes to the range step
break goes to loop exit
range is end-exclusive
step is 1
reverse and negative-step ranges are deferred
array iteration is deferred
```

Stage0 must not desugar range loops to `local i; loop i < end; i += 1`, because `continue` semantics would be wrong.

## Delegation before interface

Concrete composition is defined by:

```text
docs/development/current/main/design/delegation-no-inheritance-ssot.md
```

Canonical MVP composition uses field delegation:

```hako
box Child {
    parent: Parent = new Parent()

    delegate parent exposes {
        method
    }

    localWrapper() {
        return me.parent.method()
    }
}
```

Delegation covers:

```text
concrete Box behavior reuse
explicit forwarding
override-based specialization
known provider composition
```

Delegation does not cover:

```text
static interface conformance
abstract method-set checking
trait-like static dispatch guarantees
```

Therefore:

```text
MVP:
  prefer box + delegate exposes + uses

Later:
  interface / impl only if a real abstract method-set contract is needed
```

Legacy `box Child from Parent`, `override`, and `from Parent.method(...)` are treated as legacy delegation surface, not canonical new spelling.
They need a separate migration row before retirement.

## State without `state`

State values are ordinary enum values:

```hako
enum PageState {
    Active
    Retired
    Decommitted
    Recommitted
}
```

Lifecycle relations use `transition`:

```hako
transition PageState::Active -> PageState::Retired by retire
```

`state` is not canonical MVP syntax.

## Capability without broad `unsafe`

Canonical MVP capability surface:

```hako
freshPage(size: Bytes): Result<Page, Error>
    uses osvm
{
    ...
}
```

`unsafe` is not a Hakorune family.
`cap osvm { ... }` block syntax is deferred until method-level `uses` proves insufficient.

## Import and module surface

`using` remains the current import surface.
`module` / `use` / `export` / `private` stay later-stage package and visibility work.

Do not keep long-term duplicate canonical spellings for import.
If `module` / `use` lands, it must include a migration plan for `using`.

## Sugar rule

Compatibility forms may be accepted by a parser capsule only when a card says so, but docs and formatters should emit one canonical spelling.

```text
canonical first
compat second
silent fallback never
```
