# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-04-18
Scope: current lane / next lane / restart order only.

## Purpose

- root から active lane / next lane に最短で戻る
- landed history と rejected history は phase docs / investigations を正本にする
- `CURRENT_TASK.md` 自体は ledger にしない

## Quick Restart Pointer

1. `docs/development/current/main/05-Restart-Quick-Resume.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
4. `docs/development/current/main/phases/phase-137x/README.md`
5. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
6. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
7. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
8. `docs/development/current/main/design/string-birth-sink-ssot.md`
9. `docs/development/current/main/15-Workstream-Map.md`
10. `git status -sb`
11. `tools/checks/dev_gate.sh quick`
12. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

## Current Lane

- expected worktree:
  - clean is expected right now
  - rejected slot-store boundary probe is parked separately in `stash@{0}` as `wip/concat-slot-store-window-probe`
- active lane:
  - `phase-137x publication/source-capture reopen after compiler-known-length keeper`
- background lanes:
  - `phase-29bq loop owner seam cleanup landing`
  - `phase-163x primitive-family / user-box fast-path landing`
- current blocker:
  - `none`

## Current Snapshot

- keeper front is still closed:
  - `kilo_micro_substring_concat`
    - `C: 2 ms`
    - `Ny AOT: 3 ms`
  - `kilo_micro_substring_only`
    - `C: 3 ms`
    - `Ny AOT: 3 ms`
- current broad gap is no longer substring:
  - `kilo_micro_array_string_store`
    - `C: 10 ms`
    - `Ny AOT: 126 ms`
  - `kilo_kernel_small_hk`
    - `C: 80 ms`
    - `Ny AOT: 724 ms`
- current reading:
  - current main owner family is `array/string-store`, not `substring`
  - trusted direct MIR no longer duplicates the `text + "xy"` producer across `set(...)` and trailing `substring(...)`
  - runtime gap stayed open after the compiler-side placement fix, so duplicated birth is no longer the live owner
  - latest keeper slice is compiler-side known string-length propagation across const / substring-window / same-length string `phi`
  - active AOT entry IR on this front no longer emits `nyash.string.len_h` inside `ny_main`
  - `Stage A` narrow owner slice is landed on the VM/reference lane:
    - `.hako` `ArrayCoreBox` now routes proven string-handle `set(...)` through `nyash.array.set_his`
    - cold tail stays in Rust
  - `Stage A` exact reread is now closed and parked on the active AOT front:
    - active AOT already reaches the current concrete `store.array.str` lowering without that VM/reference pilot
  - latest locked exact `perf-observe` counters on `kilo_micro_array_string_store` show:
    - `store.array.str total=800000`
    - `cache_hit=800000`
    - `plan.action_retarget_alias=800000`
    - `plan.action_store_from_source=0`
    - `plan.action_need_stable_object=0`
    - `carrier_kind.source_keep=0`
    - `carrier_kind.owned_bytes=1600000`
    - `carrier_kind.stable_box=1600000`
    - `carrier_kind.handle=1600000`
    - `publish_reason.generic_fallback=1600000`
  - trusted direct MIR on the same benchmark still carries generic `RuntimeDataBox.set(...)` / `substring(...)` calls
  - active AOT lowering is now confirmed separately:
    - direct MIR stays generic
    - entry LLVM IR still concretizes the array string-store call to `nyash.array.set_his`
    - guard: `tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_array_store_string_contract.sh`
  - therefore the landed `.hako` owner pilot is still VM/reference-lane only; active AOT already reaches the current concrete `store.array.str` lowering without that pilot
  - slot-store boundary delayed-publication probes were tried and rejected:
    - active slot route v1:
      - `kilo_micro_array_string_store = 252 ms`
      - `kilo_kernel_small_hk = 765 ms`
    - active slot route v2:
      - `kilo_micro_array_string_store = 211 ms`
      - `kilo_kernel_small_hk = 1807 ms`
    - the bad cut was the array-store boundary itself; it bypassed the existing `set_his` fast path / alias-retarget behavior
  - helper-only keeper from that probe is landed:
    - `b35382cf9 feat: add kernel text slot store helpers`
  - latest `perf-observe` reread on the active array-store front no longer ranks `string_len_export_slow_path`; top samples stay on:
    - `issue_fresh_handle`
    - `freeze_owned_bytes`
    - `capture_store_array_str_source`
    - `StringBox::perf_observe_from_owned`
  - current live owner remains publication/source-capture around the string births, not array-set route selection
  - next comparison must split:
    - implementation language cost
    - protocol / seam cost
  - compiler-known-length keeper is landed; next slice is no longer `len_h` removal but publication/source-capture reopen while keeping that lane fixed
  - `indexOf` stays a side diagnostic lane and is not the current keeper card

## Next

1. keep `Stage A` parked as VM/reference-only and stop spending exact-front time on owner-route widening
2. keep the compiler-known-length lane fixed and guarded on `kilo_micro_array_string_store`
3. reopen `kilo_micro_array_string_store` on producer-side publication/source-capture before `nyash.array.set_his`
4. keep the existing `set_his` fast path intact while testing any unpublished generic concat outcome cut
5. use the existing `carrier_kind` / `publish_reason` counters to measure delayed-publication cuts before any `const_suffix` reopen or `Stage B` widening

## Guardrails

- MIR/lowering still owns legality, `proof_region`, and `publication_boundary`
- keep carrier/publication split physically narrow
- do not widen this card into a generic slot API or helper substrate
- keep public ABI stable
- compare `Rust vs .hako` only under:
  - same protocol
  - same public ABI with different internal seam

## Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```

## Detail Pointers

- current evidence snapshot:
  - `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
- history / rejects / longer ledger:
  - `docs/development/current/main/phases/phase-137x/README.md`
- design anchors:
  - `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
  - `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
  - `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
  - `docs/development/current/main/design/string-birth-sink-ssot.md`
