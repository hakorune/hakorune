---
Status: SSOT
Decision: accepted
Date: 2026-05-11
Scope: current mimalloc port purpose, ownership, and allocator-provider stop line.
Related:
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
  optional future host replacement support
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
   is explicitly requested.

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
