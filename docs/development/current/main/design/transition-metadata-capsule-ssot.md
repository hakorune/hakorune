---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: TRANS-001 Stage0 transition metadata capsule.
Related:
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
---

# TRANS-001 Transition Metadata Capsule SSOT

## Decision

`transition Enum.Value -> Enum.Value by method` is accepted as box-local
Stage0 metadata-only syntax.

`transition` and `by` are contextual identifiers, not global keywords.

## Canonical syntax

```hako
enum PageState {
    Active
    Retired
    Decommitted
}

box HakoAllocPageModel {
    state: PageState

    transition PageState.Active -> PageState.Retired by retire
    transition PageState.Retired -> PageState.Decommitted by decommit
}
```

State values are enum values. The `state` keyword is not part of the MVP.

## Owner split

Stage0 owns:

```text
parse transition declarations in box member lists
preserve from-state / to-state / method-name metadata
transport metadata through AST, AST JSON, and Program JSON v0
```

Stage0 does not own:

```text
enum existence checks
variant existence checks
method existence checks
transition legality checking
runtime lowering
lifecycle verifier facts
state keyword syntax
```

Stage1 owns:

```text
enum transition legality
method contract integration
lifecycle verifier facts
diagnostics and static discharge
```

## Metadata shape

Box declarations carry ordered transition declarations:

```text
transitions: [
  { from: PageState.Active, to: PageState.Retired, method: retire },
]
```

The AST struct uses explicit field names:

```text
from_state
to_state
method_name
```

## Stop lines

```text
no state keyword
no Stage0 transition checker
no Stage0 enum/method lookup
no MIR/runtime instruction lowering
no lifecycle verifier facts in TRANS-001
```

## Retire condition

Retire this capsule when the Stage1/selfhost parser and metadata transport emit
the same transition shape without relying on Rust parser ownership.
