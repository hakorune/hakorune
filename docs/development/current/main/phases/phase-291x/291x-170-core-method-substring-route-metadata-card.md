---
Status: Landed
Date: 2026-04-24
Scope: Add MIR CoreMethod metadata carriers for `StringBox.substring` generic method routes.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-169-metadata-absent-len-fallback-contract-card.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-170 CoreMethod Substring Route Metadata Card

## Goal

Prepare the `substring` mirror cleanup without changing backend lowering:

```text
MethodCall(StringBox|RuntimeDataBox(StringBox origin), "substring", [start, end])
  -> generic_method_routes[].core_method.op = StringSubstring
  -> arity = 1 or 2, key metadata = null
  -> .inc still falls back to the legacy substring classifier
```

This is a carrier card. It gives MIR a typed `StringSubstring` identity while
leaving the existing string corridor/window lowering unchanged.

## Boundary

- Do not remove the `substring` allowlist row.
- Do not change substring helper symbols or string corridor/window policy.
- Do not add hot inline lowering.
- Do not make `.inc` consume `generic_method.substring` in this card.
- Do not represent substring arguments as fake key metadata.

## Implementation

- Add an explicit `arity` field to `GenericMethodRoute`, so non-key method
  families can be represented without deriving arity from key presence.
- Add `StringSubstring` route kind and `SubstringSurfacePolicy` proof metadata.
- Add MIR route detection for direct `StringBox.substring` and
  `RuntimeDataBox.substring` with `StringBox` receiver origin.
- Emit JSON with `route_id=generic_method.substring`, `arity=1|2`, and null
  key metadata for substring routes.

## Result

MIR JSON can now carry:

```text
generic_method.substring + core_method.op=StringSubstring
```

This creates the seam for the next card to make generic emit-kind
`SUBSTRING` selection metadata-first while preserving legacy fallback behavior.

## Acceptance

```bash
cargo fmt --check
cargo test -q generic_method_route
cargo test -q map_lookup_fusion
env NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
