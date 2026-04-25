---
Status: Active
Date: 2026-04-25
Scope: Prune the generic-method `birth` emit-kind classifier after the deletion criteria and compatibility smokes are pinned.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-253-birth-compat-deletion-criteria-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_lowering.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-254 Birth Emit-Kind Prune Card

## Goal

Remove the remaining generic-method `birth` string classifier from `.inc`
policy now that the constructor marker emission sites and compatibility smokes
are explicit.

This is a BoxShape cleanup. It removes a mirror row; it does not add a new
accepted language shape.

## Change

Removed from
[`hako_llvmc_ffi_generic_method_policy.inc`](/home/tomoaki/git/hakorune-selfhost/lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc):

```text
method birth
box ArrayBox
box MapBox
```

inside `classify_generic_method_emit_kind(...)`.

Because that was the only producer of `HAKO_LLVMC_GENERIC_METHOD_EMIT_BIRTH_ZERO`,
the dead enum variant and lowering case were removed as well.

The no-growth allowlist now tracks the smaller baseline:

```text
classifiers=14
rows=14
```

## Boundary

- `NewBox -> birth` MIR marker emission remains unchanged.
- `emit_constructor_birth_marker(...)` remains the Rust-side owner for explicit
  transitional marker emission.
- llvmlite harness compatibility for explicit `ArrayBox.birth()` remains pinned
  by the harness smoke.
- `main(args)` bootstrap compatibility remains pinned by the archived
  vm-hako capability smoke.
- General constructor fallback in `builder_build.rs` is unchanged.

## Result

The generic-method emit-kind policy no longer classifies constructor `birth` by
method/box name. Constructor compatibility is now represented by MIR marker
emission and the dedicated constructor/harness paths, not by this generic
method mirror.

## Next Work

The next cleanup should inspect the remaining constructor-related compat rows
outside generic-method emit-kind selection:

- `classify_mir_call_receiver_surface(... ArrayBox ...)`
- any dead comments/docs that still describe `birth` as an active generic-method
  emit-kind owner

Do not mix this with `set` / `push` / `len` mirror pruning; those have separate
metadata-absent fallback contracts.

## Acceptance

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_harness_arraybox_birth_ternary_basic_vm.sh
bash tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh
tools/checks/dev_gate.sh quick
git diff --check
```
