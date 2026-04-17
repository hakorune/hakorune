---
Status: SSOT
Date: 2026-04-18
Scope: current lane / blocker / next pointer only.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md
  - docs/development/current/main/phases/phase-137x/README.md
---

# Self Current Task — Now (main)

## Current

- current optimization lane:
  - `phase-137x publication/source-capture reopen after compiler-known-length keeper`
- background compiler lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- blocker:
  - `none`

## Snapshot

- keeper front stays closed:
  - `kilo_micro_substring_concat = C 2 ms / Ny AOT 3 ms`
  - `kilo_micro_substring_only = C 3 ms / Ny AOT 3 ms`
- current broad gap:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 132 ms`
  - `kilo_kernel_small_hk = C 80 ms / Ny AOT 731 ms`
- `indexOf` separation:
  - keep as side diagnosis; reread only when the main card reopens it
- current owner reading:
  - current main owner family is `array/string-store`
  - duplicated `text + "xy"` producer is already removed in trusted direct MIR
  - compiler-side known string-length propagation is now landed for const / substring-window / same-length string `phi`
  - active AOT entry IR on this front no longer emits `nyash.string.len_h` in `ny_main`
  - current exact owner is still publication/source-capture
  - `Stage A` narrow owner slice is now landed on the VM/reference lane:
    - `.hako` `ArrayCoreBox` routes proven string-handle `set(...)` through `nyash.array.set_his`
    - same protocol, same cold Rust tail
  - `Stage A` exact reread is now closed on the active AOT front:
    - `store.array.str total=800000`
    - `plan.action_retarget_alias=800000`
    - `plan.action_store_from_source=0`
    - `carrier_kind.source_keep=0`
    - `publish_reason.generic_fallback=1600000`
  - trusted direct MIR still carries generic `RuntimeDataBox.set(...)` / `substring(...)`
  - active AOT lowering is separately locked:
    - direct MIR stays generic
    - entry LLVM IR still calls `nyash.array.set_his`
    - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
  - active AOT exact is therefore not the `.hako` owner pilot itself; the live owner stays publication/source-capture
  - slot-store boundary probes are now a rejected card:
    - v1 exact/whole: `252 ms / 765 ms`
    - v2 exact/whole: `211 ms / 1807 ms`
    - they cut at the wrong seam and broke the existing `set_his` fast path
  - a producer-side unpublished-outcome active probe is also rejected:
    - exact/whole: `236 ms / 2173 ms`
    - it regressed both fronts while changing the active boundary shape
  - helper-side keepers from these rejected cards are:
    - `b35382cf9`
    - runtime-side alias-retarget repair for kernel-slot store into existing string slots
  - latest `perf-observe` reread no longer ranks `string_len_export_slow_path`; the live top stays on `issue_fresh_handle` / `freeze_owned_bytes` / `capture_store_array_str_source` / `StringBox::perf_observe_from_owned`
  - latest observability split made `lookup_array_store_str_source_obj` visible as its own hot symbol; source-capture is now split enough to compare lookup vs proof shaping vs verified-source shaping
  - latest runtime-fix-only reread stays on the same owner family:
    - `kilo_micro_array_string_store = C 10 ms / Ny AOT 132 ms`
    - `kilo_kernel_small_hk = C 80 ms / Ny AOT 731 ms`
  - next first slice is no longer `len_h` removal; it is publication/source-capture reopen with the compiler-known-length lane fixed
  - latest design consult is accepted in narrowed form:
    - no syntax expansion
    - no public raw string / mutable bytes
    - `const_suffix` stays a future narrow probe, not the immediate active widening
    - if publication timing wins, reuse existing runtime-private `TextPlan` / `OwnedBytes` seams first

## Next

1. keep `Stage A` parked as VM/reference-only
2. keep the compiler-known-length lane fixed and guarded on this front
3. keep exact rereads pinned on producer-side publication/source-capture before `nyash.array.set_his`
4. preserve the existing `set_his` fast path while testing a narrow producer-side unpublished-outcome A/B probe
5. split `lookup_array_store_str_source_obj` vs proof shaping vs verified-source shaping before any route widening
6. compare `issue_fresh_handle` against the rest of the publish tail with the existing `carrier_kind` / `publish_reason` counters
7. only then try a narrow `const_suffix -> TextPlan::Pieces2` exact-front A/B
8. keep `Stage B` narrow and data-driven through `carrier_kind` / `publish_reason`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
3. `docs/development/current/main/phases/phase-137x/README.md`
4. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
5. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
6. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
7. `docs/development/current/main/design/string-birth-sink-ssot.md`
8. `docs/development/current/main/15-Workstream-Map.md`

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
