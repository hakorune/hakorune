---
Status: Accepted parked inventory; no current execution authority
Date: 2026-07-28
Decision: Inventory the exact accepted-but-inactive Ownership/View grammar without widening the language
Source semantics SSOT: ../../../../reference/language/ownership.md
Execution taskboard: hakorune-sparse-ownership-surface-task-2026-07-15.md
Current lane: follow ../CURRENT_STATE.toml
Resume checkpoint: MIRBUILDER-CANONICAL-CORE-COMPLETE0-P0
First executable row when resumed: OWN-GRAM-REJECT0-HAKO0-S0
---

# Ownership/View Missing Grammar Inventory

## Outcome

The accepted Ownership/View source surface is small and exact. The missing
grammar is limited to:

```ebnf
ownership_expr := ('move' | 'share') unary_expr

parameter := IDENT (':' TYPE_REF)?
           | 'move' IDENT ':' TYPE_REF
           | 'share' IDENT ':' TYPE_REF

result_spec := ':' TYPE_REF
             | ':' 'view' TYPE_REF view_anchor?
             | ':' 'share' TYPE_REF

view_anchor := 'from' ('me' | IDENT)
```

None of these forms is parser-live. Their target semantics are accepted by
`docs/reference/language/ownership.md`, but production activation remains zero.

The current MirBuilder lane does not move. This inventory is a parked resume
map, not authority to implement grammar, Ownership SSA, View ABI, runtime, or
backend work.

## Exact current surface

### Already live; not missing

```text
typed local / parameter / result / field declarations
implicit instance receiver `me`
ordinary assignment and existing compound-assignment sugar
weak expression
weak stored field
new/birth lifecycle surface already admitted by its own profile
```

`weak` belongs to weak-reference and object-lifecycle semantics. It is not a
BorrowView spelling and must not be reused as one.

### Accepted target; parser and AST carriers absent

| Source form | Meaning | Task owner |
| --- | --- | --- |
| `move expr` | forward one existing owner; owner count unchanged | `GRAM-MOVE0` |
| `share expr` | add one same-identity independent owner | `GRAM-SHARE0` |
| `move name: T` | consuming parameter contract | `GRAM-PARAM0` |
| `share name: T` | independent-owner parameter contract | `GRAM-PARAM0` |
| `: view T` | returned non-owning WholeObject View | `GRAM-RESULT0` |
| `: view T from me` | explicit receiver anchor | `GRAM-RESULT0`, then `PROJ-S0` |
| `: view T from name` | explicit parameter anchor | `GRAM-RESULT0`, then `PROJ-S0` |
| `: share T` | returned independent Shared owner | `GRAM-RESULT0` |

`GRAM-RESULT0` owns syntax transport only. It does not verify an anchor or
activate a callable ABI. `PROJ-S0` and the later View rows consume the passive
carrier.

### Inferred semantics; no grammar row

```text
ordinary `local alias = owner`
  -> first profile infers a whole-root ScopedAlias

ordinary parameter and receiver
  -> noescape alias by default

ordinary owned return
  -> terminal owner forwarding

scope close / overwrite / failed draft discard
  -> compiler-owned DestroyOwned planning

fresh owned rvalue entering an owning destination
  -> destination ABI and Loan Flow decide the transfer
```

These behaviors need resolved ownership facts, Loan Flow, materialization, and
backend proof. Adding keywords would not supply those authorities.

### Explicit non-surface

Do not add these spellings to the baseline:

```text
borrow expression or parameter modifier
ref / & / &mut / dereference ownership syntax
owned / own local or field modifier
clone as ownership authority
explicit destroy / drop / DestroyOwned source syntax
local View declarations
field or projection View modes
exclusive `mut view`
View PHI syntax
field-path, static, temporary, or named-domain View anchors
explicit receiver ownership syntax
region/lifetime generic annotations
```

Internal names such as `BorrowView`, `Ref`, `StringViewBox`, `TypeAbiView`, and
FastMem “raw view” are plan or representation vocabulary. They do not imply a
source-language feature.

## Parser census

The Rust parser already rejects spaced inactive result lookalikes:

```text
: view Node
: share Service
```

with:

```text
[freeze:contract][parser/ownership_syntax_inactive]
```

The focused Rust fixture is green. The remaining reject-boundary work is the
Hako parser witness and the shared closure guard:

```text
OWN-GRAM-REJECT0-HAKO0-S0
-> OWN-GRAM-REJECT0-G0
```

The activation rows must preserve all of these ordinary meanings:

```text
move(...)
share(...)
local move = ...
local share = ...
local view = ...
move: T
share: T
literal type `view`
literal type `share`
qualified and generic types rooted at `view` or `share`
```

`move`, `share`, and `view` therefore remain contextual forms rather than hard
keywords.

## Why grammar is not the production blocker

The passive substrate is incomplete as a production graph:

```text
production Ownership SSA caller                 = 0
canonical CopyOwned / DestroyOwned producers    = 0
MoveExpression / ShareExpression AST carriers   = 0
callable View result carrier and anchor proof   = 0
Verified Scoped Loan Flow production product    = 0
exact Unique Box source profile                 = 0
mainline ny-llvmc ownership lowering            = 0
```

`CopyOwned` and `DestroyOwned` MIR vocabulary, a disconnected verifier, Rust
interpreter semantic tests, and a narrow witnessed `llvmlite-obj` lane exist.
They are not evidence that source ownership is product-ready.

The first semantic ownership slice does not need to consume any newly admitted
ownership spelling:

```text
Unique NewBox owner
-> inferred whole-root local alias
-> reads and mutation
-> implicit owner Return or scope DestroyOwned
```

Calls, owning stores, capture, explicit share, and borrowed return may remain
typed fail-fast boundaries in that first slice.

## Parked execution train

When `CURRENT_STATE.toml` explicitly reopens Ownership, use this order.

### Pack A — close the inactive boundary and collect facts

```text
OWN-GRAM-REJECT0-HAKO0-S0
-> OWN-GRAM-REJECT0-G0
-> O2-P0a
-> O2-P0r
-> O2-P0b1
-> O2-P0c
```

Do not repeat the already-green Rust reject implementation.

### Pack B1 — passive grammar transport

```text
GRAM-MOVE0
-> GRAM-SHARE0
-> GRAM-PARAM0
-> GRAM-RESULT0
-> REF-GRAM0
```

Each grammar row closes together:

```text
language-v1 registry row
Rust parser witness
Hako parser witness
frontend AST/schema carrier
macro/source AST transport
contextual-name collision fixtures
unsupported-route fail-fast
```

Program JSON v0 widening is a separate decision. A grammar row must not
silently make an unsupported artifact route accept the new carrier.

### Pack B2 — meaning before effects

```text
O2-A0 resolved source intent
-> O2-L0 Verified Scoped Loan Flow
-> O2-M0 owner availability / forwarding verifier
-> O2-DIAG0 typed diagnostics
```

No parser row may infer ownership from a name, runtime tag, reference count, or
method spelling.

### Packs C-E — first production ownership and View

```text
UBOX-P0 -> UBOX-M0 -> UBOX-I0
-> ALIAS-I0 -> ALIAS-CFG0
-> ABI0
-> VIEW0 / PROJ-* exact Anchored View branch
-> SHARE-PLAN0 -> SHARE-I0
```

Before product promotion, the selected source profile also needs one exact
supported backend or an explicit pre-effect rejection for every unsupported
backend.

## Per-row acceptance

Every grammar row must prove:

```text
one contextual source shape
one orthogonal AST carrier
Rust/Hako parser correspondence
ordinary identifier/call/type meanings preserved
source reconstruction = 0
name-based semantic inference = 0
production Ownership SSA activation = 0
CopyOwned / DestroyOwned source emission = 0
fallback / retry / route reselection = 0
new unrelated grammar = 0
```

The later semantic activation rows must additionally prove:

```text
one resolved ownership intent
one owner-root and Loan Flow authority
one materialization owner
one exact backend capability
failure before effects on unsupported routes
SharedV1 fallback = 0
```

## Hard stops

Stop and return to a language Decision if implementation would require:

```text
changing the accepted move/share/view meanings
adding borrow/owned/ref/mut grammar
using weak as a View substitute
encoding ownership inside type-name strings
deriving ownership from method or variable names
using current Arc cloning as proof of explicit share
letting View escape, cross await/capture, or enter PHI in the first profile
reusing local-binding ownership planning for field/index storage
silently ignoring ownership operations on ny-llvmc/WASM/PyVM
activating Ownership while the current MirBuilder lane remains selected
```

## Non-claims

This inventory does not claim:

```text
Ownership grammar is currently usable
the first Ownership production slice needs new syntax
View is immutable or exclusive
weak upgrade has its final Ownership V2 ABI
all object backends support ownership operations
the accepted default compiler ingress is already canonical
the parked Ownership lane may replace the current MirBuilder design stop
```
