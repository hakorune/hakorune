---
Status: Landed
Date: 2026-04-25
Scope: Pin the constructor owner shape so the remaining `birth` compatibility row can stay thin and pinned until a real carrier split exists.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-249-constructor-birth-compatibility-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-250-constructor-birth-carrier-design-card.md
  - src/mir/builder/exprs.rs
  - src/mir/builder/calls/emit.rs
  - src/mir/builder/calls/resolver.rs
  - src/mir/builder/module_lifecycle.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-251 Constructor-Birth Owner Shape Decision Card

## Goal

Choose the constructor owner shape explicitly so the current `birth`
compatibility row can remain a compatibility marker instead of becoming a
second semantics owner.

This card does not change runtime behavior. It only pins the shape that the
next thin carrier slice should implement.

## Decision

The constructor surface owns the intent.

```text
CallTarget::Constructor
  -> Callee::Constructor
  -> MirInstruction::NewBox
```

`birth` remains a compatibility initializer marker for zero-arg
`ArrayBox` / `MapBox` literals. Do not introduce a separate `birth` owner or
move the semantics back into the generic-method policy layer.

## Boundary

- Do not prune the generic-method `birth` compatibility row in this card.
- Do not change `nyash.array.birth_h` / `nyash.map.birth_h` ABI exports in
  this card.
- Do not add a new semantics owner for `birth`.
- Do not widen the constructor surface beyond `ArrayBox` / `MapBox` in this
  card.
- Do not change the existing `NewBox` lowering behavior in this card.

## Analysis

- [`src/mir/builder/calls/emit.rs`](/home/tomoaki/git/hakorune-selfhost/src/mir/builder/calls/emit.rs) already resolves `CallTarget::Constructor` to `MirInstruction::NewBox`.
- [`src/mir/builder/calls/resolver.rs`](/home/tomoaki/git/hakorune-selfhost/src/mir/builder/calls/resolver.rs) already keeps `Callee::Constructor` as the constructor call surface.
- [`src/mir/builder/exprs.rs`](/home/tomoaki/git/hakorune-selfhost/src/mir/builder/exprs.rs) still emits the explicit `birth()` compatibility marker after `NewBox` for array and map literals.
- [`lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc`](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc) already owns the concrete `nyash.*.birth_h` exports.
- [`lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc`](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc) still classifies `birth` as a transitional compatibility row.

The clean reading is therefore:

1. constructor intent lives on the constructor surface
2. `birth` is compatibility-only glue
3. any future carrier split should thin the existing constructor path, not add a new owner

## Result

The owner shape is now fixed:

```text
constructor owner = canonical
birth owner = compatibility-only
```

This keeps the remaining transition small and prevents another by-name owner
from appearing.

## Next Work

The next implementable slice should be the thinnest constructor carrier that
can absorb the explicit `birth()` marker without changing semantics.

That slice should answer:

- whether the explicit post-`NewBox` `birth()` emission can be wrapped behind a
  dedicated helper
- whether the compatibility marker can be emitted from one constructor-owned
  seam instead of `exprs.rs` inline sites
- what fixture or smoke will prove the new seam did not widen semantics

## Acceptance

- The card makes the constructor owner shape explicit before any code change.
- The card keeps `birth` pinned as compatibility-only glue.
- The card stays docs-only and does not overlap the existing prune or review
  cards.
