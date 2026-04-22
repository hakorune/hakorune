---
Status: Closed
Date: 2026-04-23
Scope: Triage Hako LL / provider stack overflow before deleting `pure_compile_minimal_paths` path #1/#2.
Related:
  - docs/development/current/main/phases/phase-292x/292x-112-pure-compile-minimal-ret-branch-deletion-card.md
  - docs/development/current/main/phases/phase-292x/292x-113-mapbox-duplicate-receiver-unified-dispatch-card.md
  - tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_ret_const_min.sh
  - tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/run_llvmlite_monitor_keep.sh
---

# 292x-114: Hako LL Stack Overflow Predelete

## Resolution

The stack overflow and follow-up `String + Void` failure are fixed.

Root causes:

- `ArrayBox::clone` / `MapBox::clone` deep-cloned nested identity collections,
  which could recurse forever on self/cyclic collection graphs.
- `execute_method_callee` normalized duplicate receiver aliases for the general
  method path, but the ArrayBox fast bridge invoked the surface before that
  normalization. `rows.get(0)` in Hako LL could therefore pass the receiver
  alias as the index and return `Error: get() requires integer index`.

Landed fix:

- `ArrayBox` visible reads / clones share nested `InstanceBox`, `ArrayBox`,
  `MapBox`, and borrowed-handle identity boxes instead of deep-cloning them.
- `MapBox::clone` reuses the visible-read clone/share policy.
- CoreBox method callee duplicate-receiver stripping is now arity-aware:
  it strips only when the arg list is exactly one item longer than the
  surface arity and the first arg is the receiver alias.
- Regression coverage pins ArrayBox duplicate receiver aliases, legitimate
  `array.push(array)` self arguments, and collection clone recursion.

Verification:

```bash
cargo test -q method_callee_arraybox_get_strips_duplicate_receiver_alias_arg --lib
cargo test -q method_callee_arraybox_push_keeps_legitimate_self_argument --lib
cargo test -q method_callee_mapbox_set_get_strips_duplicate_receiver_arg --lib
cargo test -q clone_recursion --lib
cargo build --release --bin hakorune
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_ret_const_min.sh
bash tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/run_llvmlite_monitor_keep.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

Current delete-readiness:

- `phase29x_backend_owner_daily_ret_const_min.sh`: PASS.
- `compat/llvmlite-monitor-keep`: PASS 3/3.
- `.inc` guard remains unchanged at 5 files / 47 analysis-debt lines.

## Original Blocker

`pure_compile_minimal_paths` path #1/#2 cannot be deleted yet.

Current evidence after `292x-113`:

- `BackendRecipeBox` profile validation no longer fails with missing
  `route_profile`.
- `phase29x_backend_owner_daily_ret_const_min.sh` aborts with stack overflow.
- `compat/llvmlite-monitor-keep` aborts with stack overflow in all three
  canaries.
- Read-only worker inventory with `NYASH_DEBUG_STACK_OVERFLOW=1` saw repeated
  `ArrayBox::clone` / `ArrayBox::clone_box` recursion before allocator noise.
- `archive/pure-historical` passes after the runner root fix.
- `compat/pure-keep` passes.

## Working Hypothesis

The deletion target is now blocked by a Hako LL / provider recursion path, not
by C minimal-path recognizer behavior. The next slice should find the recursive
owner and either:

- repair the Hako LL/provider route so ret-const and compare-branch canaries
  pass, or
- demote/archive the stale monitor canaries with an explicit replacement owner.

First inspection targets:

- `src/boxes/array/traits.rs`
- `src/boxes/array/storage.rs`
- `src/boxes/map_box.rs`

If the owner is an ArrayBox self/cyclic clone shape, add a focused repro/unit
before changing provider routing.

`compat/llvmlite-monitor-keep` also needs owner validation. Its current Hako
path appears to exercise `LlvmBackendBox.compile_obj_root(...)` through Hako LL
adapter files, not a direct Rust `provider_keep.rs` proof. Rewrite or archive
that monitor only after a replacement owner is explicit.

## Non-Goals

- Do not delete `pure_compile_minimal_paths` path #1/#2 in this card.
- Do not widen `.inc` raw MIR analysis.
- Do not paper over stack overflow with a fallback route.

## Acceptance

At minimum:

```bash
cargo fmt --check
cargo test -q method_callee_mapbox_set_get_strips_duplicate_receiver_arg --lib
bash tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Delete-readiness for `292x-112` additionally requires:

```bash
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_ret_const_min.sh
bash tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/run_llvmlite_monitor_keep.sh
```

Both delete-readiness commands are now green; retry `292x-112` next.
