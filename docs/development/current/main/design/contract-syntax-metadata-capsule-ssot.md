---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: CONTRACT-002 Stage0 contract syntax metadata capsule.
Related:
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
---

# CONTRACT-002 Contract Syntax Metadata Capsule SSOT

## Decision

`requires`, `ensures`, and `invariant` are accepted as Stage0 metadata-only
syntax.

They are contextual tokens, not global keywords.

## Canonical syntax

```hako
releaseLocal(block_id: BlockId): Result<void, ReleaseError>
    requires block_id >= 0
    ensures block_id >= 0
{
    return Ok(void)
}

box HakoAllocPageModel {
    used: usize
    capacity: usize

    invariant used <= capacity
}

record HakoAllocAlignedSmallMeta {
    ptr: PtrId
    usable_size: Bytes

    invariant usable_size >= 0
}
```

## Owner split

Stage0 owns:

```text
parse requires / ensures before function or method bodies
parse invariant in box and record member lists
transport metadata through AST, AST JSON, and Program JSON v0
keep the function body unchanged
```

Stage0 does not own:

```text
runtime precondition insertion
runtime postcondition insertion
invariant boundary selection
static contract discharge
verifier diagnostics
assert runtime-check sugar
```

Stage1 owns:

```text
contract lowering
invariant checking policy
verifier facts and static discharge
runtime fail-fast insertion when explicitly carded
```

## Metadata shape

Function declarations carry ordered contract clauses:

```text
contracts: [
  { kind: requires, condition: expr },
  { kind: ensures, condition: expr },
]
```

Box and record declarations carry ordered invariant expressions:

```text
invariants: [expr]
```

## Stop lines

```text
no Stage0 invariant checker
no Stage0 requires/ensures lowering
no Stage0 verifier facts
no assert sugar in CONTRACT-002
no keyword reservation for requires/ensures/invariant outside their syntax slots
```

## Retire condition

Retire this capsule when the Stage1/selfhost parser and metadata transport emit
the same contract/invariant shape without relying on Rust parser ownership.
