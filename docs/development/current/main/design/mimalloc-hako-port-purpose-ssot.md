---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: current mimalloc port purpose, ownership, and allocator-provider stop line.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/reference/language/low-level-capabilities.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md
  - lang/src/hako_alloc/
---

# Mimalloc Hako Port Purpose (SSOT)

## Decision

The current mimalloc work exists to raise Hakorune completeness by implementing
allocator algorithms in `.hako`, primarily under the `hako_alloc` library
layer and the existing capability substrate.

This is not a mandate to replace the Hakorune process allocator now. A
Rust-style host allocator replacement feature may remain a future optional
runtime capability, but it is not the current acceptance target for the
mimalloc port.

## Terminology: Port vs Replacement

Use these terms precisely:

| Term | Current meaning | Current status |
| --- | --- | --- |
| mimalloc port | Re-express mimalloc-derived allocator algorithms in `.hako` under `hako_alloc`, using Hakorune capability substrate rows. | active completeness lane |
| `hako_alloc` production facade | A `.hako` policy seam and proof surface for allocator behavior. It can model allocation, release, page-source, remote-free, and stats policies. | active proof/algorithm surface |
| allocator provider option | A future explicit runtime option that may choose a `hako_alloc` / mimalloc-style provider the way Rust can explicitly choose a global allocator. | optional future ladder only |
| process allocator replacement | Replacing Hakorune's ordinary host/process malloc/free path or installing a global allocator hook by default. | inactive / forbidden in this lane |

The current default runtime allocation path remains the ordinary host/process
allocator path. Completing the mimalloc port makes `hako_alloc` more complete
and may create a future provider candidate, but it must not silently install
that candidate as the process allocator.

Avoid ambiguous wording such as "integrate mimalloc into Hakorune" unless the
sentence names the layer:

```text
allowed:
  port mimalloc-style algorithms into `.hako` / `hako_alloc`
  prepare a future explicit allocator-provider option

not allowed:
  imply default malloc/free replacement
  imply provider activation
  imply hook installation
  imply `#[global_allocator]`-style activation without an explicit future row
```

## Ownership

```text
.hako / hako_alloc
  allocator algorithm ownership
  size classes, pages, free lists, remote-free policy, policy state
  proof apps and production-facing allocator facade

capability substrate
  hako.mem / hako.buf / hako.ptr / hako.atomic / hako.tls / hako.osvm
  narrow native leaves and route facts required by allocator algorithms

Rust runtime / Hakorune core
  bootstrap, diagnostics, native leaf exports, verifier surfaces
  no implicit process allocator swap in the current lane

allocator-provider activation ladder
  optional future explicit provider/replacement support
  inactive after M103 unless explicitly reopened
```

The provider ladder may keep proving explicit-input diagnostics and fail-fast
preconditions, but it does not gate `.hako` mimalloc implementation progress.

## Current Implementation Order

1. Keep capability substrate rows narrow and proof-backed.
2. Move allocator logic into `.hako` / `lang/src/hako_alloc/` when each slice is
   ready.
3. Keep native leaves thin and named by capability family.
4. Use proof apps and focused gates to fix behavior before widening production
   facade ownership.
5. Reopen allocator-provider M104+ only when host allocator replacement support
   is explicitly requested as an optional provider/replacement row.

The post-analysis implementation ladder is fixed in
`docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md`.
The first concrete code row is the pure `.hako` size-class policy owner; page
mutation, page queues, OSVM page sourcing, local free, and remote free follow as
separate rows.

## Current Post-M176 Boundary

The active allocator algorithm lane is complete through `M176`:

- page-map identity (`M171`)
- page-map-backed release ordering (`M172`)
- pre-realloc release observation (`M173`)
- same-class/no-move realloc (`M174`)
- alloc-copy-release grow fallback (`M175`)
- realloc negative matrix / failure contract (`M176`)
- standalone alignment policy (`M177`)
- aligned small-path execution (`M178`)

The next row is `M179 huge threshold and routing`. Normal aligned small-path
execution now exists, but huge routing still must fail fast instead of silently
widening the small path or reopening allocator-provider activation.

## Stop Line

Until a later explicit host-replacement row reopens this lane, the mimalloc
port must not add:

- `#[global_allocator]`;
- `GlobalAlloc`;
- implicit environment selection such as `NYASH_ALLOCATOR_PROVIDER` or
  `HAKO_ALLOCATOR_PROVIDER`;
- provider selection or active registry construction;
- proof token consumption as activation permission;
- rollback preparation;
- activation gate opening;
- hook install or native activation;
- process allocator replacement;
- `.inc` provider, mimalloc, hook, facade, or policy name matching.

## M104 Reading

Within the optional allocator-provider ladder, M104 remains the next token row
after M103. It is not the default next implementation task for the repository.

If M104 is resumed, the token is proof custody/readiness only:

```text
proof_bundle_consumed=true
activation_allowed=false
rollback_prepared=false
gate_open=false
hook_installed=false
process_allocator_replaced=false
```

The default current implementation direction is `.hako` mimalloc / `hako_alloc`
completeness, not host allocator replacement.
