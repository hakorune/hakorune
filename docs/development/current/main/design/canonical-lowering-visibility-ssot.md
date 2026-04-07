---
Status: SSOT
Decision: current
Date: 2026-04-06
Scope: `.hako owner -> MIR canonical reading -> current concrete lowering -> Rust executor` の visibility lock を固定する
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/semantic-optimization-authority-ssot.md
  - docs/development/current/main/phases/phase-151x/README.md
  - lang/src/runtime/kernel/string/chain_policy.hako
  - lang/src/runtime/collections/method_policy_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_string_chain_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc
  - crates/nyash_kernel/src/exports/string_helpers.rs
  - crates/nyash_kernel/src/plugin/array_runtime_facade.rs
  - crates/nyash_kernel/src/plugin/array_slot_store.rs
  - crates/nyash_kernel/src/plugin/array_string_slot.rs
  - crates/nyash_kernel/src/plugin/map_slot_store.rs
---

# Canonical Lowering Visibility SSOT

## Goal

canonical MIR reading が docs の中だけに閉じず、current concrete lowering と Rust executor に対しても追える状態を固定する。

この文書の目的は 2 つだけ。

1. `phase-137x` へ早戻りしない。
2. concrete helper / extern symbol を semantic source of truth に見せない。

## Locked Rows

### const_suffix

- `.hako owner route`
  - `StringChainPolicyBox.concat_pair_route(...)->"const_suffix"`
- canonical MIR reading
  - `thaw.str + lit.str + str.concat2 + freeze.str`
- current concrete lowering
  - `lang/c-abi/shims/hako_llvmc_ffi_string_chain_policy.inc`
  - `try_emit_string_concat_const_suffix_call(...)`
  - `nyash.string.concat_hs`
- Rust executor
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `execute_const_suffix_contract(...)`
  - `concat_const_suffix_fallback(...)`

### ArrayStoreString

- `.hako owner route`
  - `CollectionMethodPolicyBox.route_array_store_string() -> "ArrayStoreString"`
- canonical MIR reading
  - `store.array.str`
- lifecycle visibility carried above Rust
  - `source_preserve = eligible | ineligible`
  - `identity_demand = none | stable_object`
  - `publication_demand = none | publish_handle`
- current visibility carrier
  - `lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc`
    - `GenericMethodRouteState`
  - `lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc`
    - `classify_generic_method_emit_plan(...)`
  - `lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc`
    - `GenericMethodEmitPlan`
    - `classify_generic_method_set_route(...)`
- current concrete lowering
  - `lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc`
  - `nyash.array.set_his`
- Rust executor
  - `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
    - `array_runtime_store_array_string(...)`
  - `crates/nyash_kernel/src/plugin/array_slot_store.rs`
    - `array_slot_store_string_handle(...)`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
    - `execute_store_array_str_contract(...)`
    - `array_string_store_handle_at(...)`
    - backend-private split:
      - `SourceKindCheck`
      - `SourceLifetimeKeep`
      - `AliasUpdate`
      - `NeedStableObject`

### ArrayStoreString Visibility Rule

`ArrayStoreString` is allowed to stay public as one canonical row.
Do not promote `RetargetAlias` into a public MIR op.

What must be visible above Rust is only:

1. `source_preserve`
2. `identity_demand`
3. `publication_demand`

What stays Rust-backend private is:

1. `SourceKindCheck`
2. `SourceLifetimeKeep`
3. `AliasUpdate`
4. `NeedStableObject`

Only `NeedStableObject` may justify generic object-world entry.

### MapStoreAny

- `.hako owner route`
  - `CollectionMethodPolicyBox.route_map_store_any() -> "MapStoreAny"`
- canonical MIR reading
  - `store.map.value`
- current concrete lowering
  - `lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc`
  - `nyash.map.slot_store_hhh`
- Rust executor
  - `crates/nyash_kernel/src/plugin/map_slot_store.rs`
    - `map_slot_store_any(...)`

## Reopen Gate

`phase-137x` を reopen してよいのは、上の 3 rows が source-backed に読めるときだけ。

必要なのは次の 4 点。

1. `.hako owner route`
2. canonical MIR reading
3. current concrete lowering
4. Rust executor

cleaner helper naming や narrower Rust shape だけでは reopen しない。
