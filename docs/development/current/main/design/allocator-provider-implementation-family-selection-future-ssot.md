---
Status: SSOT
Decision: accepted
Date: 2026-05-22
Scope: long-lived future direction for explicit allocator-provider family selection.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md
  - CURRENT_TASK.md
---

# Allocator Provider Implementation Family Selection (Future SSOT)

## Goal

Fix one durable reading for the future allocator-provider lane:

- the current mimalloc port stays focused on `.hako` / `hako_alloc`
  completeness plus comparison evidence
- future provider work should allow explicit selection across multiple allocator
  implementation families
- `CURRENT_TASK.md` should stay a thin pointer, not regrow into the roadmap body

## Decision

If the allocator-provider lane is explicitly reopened, Hakorune should prefer an
explicit provider-family selection model instead of a single hard-wired
"make mimalloc the default allocator" path.

The future selection contract should stay provider-family neutral:

| Family | Role | Current reading |
| --- | --- | --- |
| `native_system_malloc` | safe baseline / default runtime allocator | current default |
| `.hako` allocator family | reference, selfhost, algorithm visibility, proof-friendly allocator family under `hako_alloc` | active implementation direction |
| `native_mimalloc` | native performance comparison and future explicit provider candidate | parked provider family |
| custom provider family | future user/project allocator integration under the same provider contract | parked future extension |

This means "selectable allocators" is the right long-term direction, but it is a
future explicit provider lane, not a reason to replace the process allocator by
default during the current mimalloc port.

## Current Reading

Today the repository is still in the left-hand side of that split:

```text
current:
  `.hako` / `hako_alloc` completeness
  matched-workload comparison against C mimalloc
  no default process allocator replacement

future optional lane:
  explicit provider-family selection
  optional proof custody / rollback / activation ladder
  optional process allocator replacement only after explicit reopen
```

`CURRENT_TASK.md` should therefore point at this document and the existing
provider-ladder docs, but it should not duplicate the family matrix or the
future roadmap body.

## Contract for Future Selection

When the provider lane is resumed, the selection contract should keep these
properties:

1. **Explicit selection only**
   - choose via explicit manifest / CLI / project contract
   - no hidden env toggles or implicit discovery
2. **Common allocator contract**
   - allocation/release/realloc/alignment/stats semantics must be fixed at the
     provider boundary
   - unsupported capability must fail fast, not silently fall back
3. **Comparable evidence**
   - `.hako` allocator family and native allocator families should expose the
     same evidence surface for throughput, allocation counts, requested bytes,
     and memory-use diagnostics
4. **Provider-family neutrality**
   - the lane must not hard-code "native mimalloc only"
   - a future custom provider should fit under the same selection contract
5. **Default remains conservative**
   - until a later explicit row says otherwise, `native_system_malloc` remains
     the default runtime allocator

## Naming Guidance

Current diagnostic fixtures already use reserved provider names such as
`native_system_malloc`, `native_mimalloc`, and `hako_model_allocator`.

Future rows may refine the `.hako` side into more specific provider ids when the
selection lane is actually reopened, for example:

- a generic `.hako` allocator provider
- a `.hako` mimalloc-style provider
- a project-specific custom allocator provider

That refinement must happen through explicit provider rows. Do not introduce
hidden runtime selection now.

## Placement Rule

- Put the durable roadmap body here in `main/design/`.
- Keep `CURRENT_TASK.md` as a short pointer to this SSOT.
- Keep per-row execution detail in phase-293x cards and taskboards.

## Non-Goals

- no allocator activation in the current lane
- no provider registry implementation change in this doc
- no `#[global_allocator]`
- no process allocator replacement
- no hidden provider-selection environment variables
