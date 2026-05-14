---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: USES-001 Stage0 method-level capability metadata capsule.
Related:
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/stage0-stage1-feature-responsibility-split-ssot.md
  - docs/development/current/main/design/language-feature-implementation-order-ssot.md
---

# USES-001 Method-Level Uses Metadata Capsule SSOT

## Decision

`uses capability` is accepted as Stage0 metadata-only syntax on function,
method, and constructor headers.

`uses` is a contextual identifier, not a global keyword.

## Canonical syntax

```hako
freshPage(size: Bytes): Result<Page, Error>
    uses osvm
{
    return OsVm.reserve(size)
}

copyRaw(dst: RawBuf, src: RawBuf, len: Bytes): i64
    uses rawbuf, atomic
{
    return len
}
```

## Owner split

Stage0 owns:

```text
parse uses clauses before function/method/constructor bodies
preserve ordered capability names as metadata
transport metadata through AST, AST JSON, and Program JSON v0 helper defs
```

Stage0 does not own:

```text
capability policy
host route permission
backend capability gates
runtime lowering
verifier facts
cap block syntax
provider activation or allocator hook selection
```

Stage1 owns:

```text
capability checking
allowed host route validation
unsupported backend fail-fast
integration with verifier/CorePlan facts
```

## Metadata shape

Function declarations carry ordered capability names:

```text
uses: [osvm, rawbuf]
```

## Stop lines

```text
no unsafe keyword
no cap block in USES-001
no Stage0 capability checker
no backend route selection
no provider activation / hook / replacement coupling
```

## Retire condition

Retire this capsule when the Stage1/selfhost parser and metadata transport emit
the same uses shape without relying on Rust parser ownership.
