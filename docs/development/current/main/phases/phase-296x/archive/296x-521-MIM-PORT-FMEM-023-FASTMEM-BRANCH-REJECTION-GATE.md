---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-023.
Related:
  - docs/development/current/main/phases/phase-296x/296x-520-MIM-PORT-FMEM-022-BRANCH-PROOF-PREFLIGHT-SELECTION.md
  - src/mir/builder/fastmem.rs
  - tools/hako_check/fastmem_source_syntax_smoke.sh
---

# 296x-521 MIM-PORT-FMEM-023 Fastmem Branch Rejection Gate

## Purpose

Fail fast on source `if` blocks inside `fastmem Contract { ... }` until a
dedicated branch proof row opens real CFG semantics.

MIM-022 selected this gate because current fastmem `if` lowering is
observation-only and linearized:

```text
condition
then body
else body
```

That shape is not a branch execution model. It can make proof facts from a
non-taken branch appear available to later MemOps.

## Decision

```text
fastmem source `if`:
  rejected by MIRBuilder

accepted fastmem source shape:
  straight-line local/assignment/expression MemOps only
```

The rejection reason must be stable and explicit:

```text
[freeze:contract][fastmem/branch_cfg_closed]
```

## Acceptance

```text
source fastmem `if` fails before MIR metadata claims are produced
existing straight-line hako_alloc fastmem pilots still pass
page_local_alloc_route_branch_claim remains 0
page_local_alloc_route_cfg_lowering_enabled remains 0
no backend redecision or ABI hot lookup appears
```

## Landed Evidence

```text
MIRBuilder:
  source fastmem `if` returns
  [freeze:contract][fastmem/branch_cfg_closed]

Unit:
  cargo test -q --lib fastmem_source_rejects_branch_cfg_until_proof_envelope_exists
  cargo test -q --lib fastmem

Smoke:
  bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Still Closed

```text
fastmem branch CFG lowering
route-exclusive proof envelopes
path-sensitive dominance
LayoutRef join / phi rules
multi-block refill transfer
remote owner routing
AtomicRemoteHead
TLS backing transfer
provider activation
process allocator replacement
hook installation
global allocator claim
winner claim
```

## Next Row

```text
MIM-PORT-FMEM-024:
  Page-local same-owner free body pilot

Goal:
  add a straight-line `.hako hako_alloc` free body that composes same-owner
  proof, block-next proof, LocalFreePush, and PageMeta.used decrement through
  existing fastmem MemOps.

Non-goals:
  branch route selection
  remote-owner free
  AtomicRemoteHead
  TLS backing transfer
```
