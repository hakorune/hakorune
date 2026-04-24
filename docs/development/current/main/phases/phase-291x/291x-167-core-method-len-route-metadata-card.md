---
Status: Landed
Date: 2026-04-24
Scope: Add MIR CoreMethod metadata carriers for arity-0 `len`/`length`/`size` generic method routes.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-166-metadata-absent-get-fallback-contract-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-167 CoreMethod Len Route Metadata Card

## Goal

Prepare the `len`/`length`/`size` mirror cleanup without changing backend
lowering:

```text
MethodCall(ArrayBox|MapBox|StringBox, len/length/size, [])
  -> generic_method_routes[].core_method.op = ArrayLen | MapLen | StringLen
  -> arity = 0, key metadata = null
  -> .inc still falls back to the legacy method-name classifier
```

This is a carrier card. It makes MIR own the length-family CoreMethod identity,
but does not make `.inc` consume it yet.

## Boundary

- Do not remove `len`/`length`/`size` allowlist rows.
- Do not change helper symbols or lowering.
- Do not add hot inline lowering.
- Do not classify length aliases in `.inc` from the new metadata in this card.
- Keep key-based `get`/`has` fusion behavior unchanged.

## Implementation

- Extend `GenericMethodRoute` so key metadata is optional. This lets one route
  family represent both key-based `get`/`has` and arity-0 length calls without
  fake key values.
- Add MIR route kinds for `MapEntryCount`, `ArraySlotLen`, and `StringLen`.
- Add `LenSurfacePolicy` and `scalar_i64` return-shape metadata.
- Emit JSON with `arity=0`, `key_route=null`, and `key_value=null` for length
  routes.
- Keep MapLookup fusion strict by requiring key metadata to be present before
  pairing `MapGet` and `MapHas`.

## Result

MIR JSON can now carry length-family CoreMethod metadata:

```text
generic_method.len + core_method.op=MapLen
generic_method.len + core_method.op=ArrayLen
generic_method.len + core_method.op=StringLen
```

This creates the seam for the next card to make generic emit-kind `LEN`
selection metadata-first while preserving the legacy alias fallback.

## Acceptance

```bash
cargo fmt --check
cargo test -q generic_method_route
cargo test -q map_lookup_fusion
env NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
