---
Status: Active
Date: 2026-04-21
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md
  - docs/development/current/main/design/string-value-model-phased-rollout-ssot.md
  - docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-137x/137x-91-task-board.md
  - docs/development/current/main/phases/phase-137x/137x-93-container-primitive-design-cleanout.md
  - docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
```

## Current

- lane:
  - `phase-137x-H owner-first optimization return` (active; H28 array text observer-store search/copy owner split)
  - execution mode:
    - `137x-E1 minimal TextLane / ArrayStorage::Text` is landed before further kilo tuning
    - `137x-F Value Lane bridge` is closed; `137x-F1 demand-to-lane executor bridge` and `137x-F2 producer outcome manifest split` are landed
    - `137x-G` allocator / arena pilot is rejected for now
    - `137x-D` exact route-shape keeper is landed; next owner-first optimization return is `137x-H`
    - current blocker is `137x-H28 array text observer-store search/copy owner split`
    - keeper evidence remains direct-only; exact/middle/whole gates must be recorded before accepting each implementation slice
- blocker:
  - `137x-H28 array text observer-store search/copy owner split`
- worktree:
  - clean is expected; do not resurrect `stash@{0}` unless you are explicitly reopening the rejected slot-store boundary probe
  - current snapshot:
    - `kilo_micro_substring_concat = C 2 ms / Ny AOT 3 ms`
    - `kilo_micro_array_string_store = C 11 ms / Ny AOT 4 ms`
    - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 4 ms`
    - `kilo_kernel_small = C 83 ms / Ny AOT 7 ms`
  - adopted middle bridge:
    - `substring + concat + array.set + loopcarry`
    - use it to confirm store/publication cuts without the whole-front `indexOf("line")` row-scan noise
  - current whole-front owner after H27:
    - H26 observer-store region executor remains the active owner family
    - H27 removed the outer edit `nyash.array.string_len_hi` call
    - H28.1 removed the fixed const-needle Pattern-search owner
    - H28.2 removed the accidental short-literal prefix `bcmp` compare owner
    - next inspect suffix mutation/copy and write-frame mechanics under the
      same MIR-owned observer-store contract
  - first landed 137x-D keeper:
    - same-slot piecewise concat3 subrange store originally lowered to the CStr helper `nyash.array.string_insert_mid_subrange_store_hisiii`
    - current direct lowering uses the explicit-length helper `nyash.array.string_insert_mid_subrange_store_hisiiii`
    - direct-only correctness: `Result: 2880064`, exit code `64`
  - `kilo_kernel_small_hk = C 81 ms / Ny AOT 26 ms`
- immediate next:
  - active entry: `docs/development/current/main/phases/phase-137x/137x-current.md`
  - ownership map: `docs/development/current/main/phases/phase-137x/137x-array-text-contract-map.md`
  - H24 verdict: active owner is write-lock guard mechanics, not fallback/promotion or byte-edit/memmove
  - H25a landed: metadata-only `array_text_residence_sessions`; `.inc` and runtime behavior were unchanged
  - H25b landed: MIR-owned begin/update/end placement metadata and skip indices
  - H25c.1 landed: `.inc` consumes residence-session metadata first, still behavior-preserving
  - H25c.2a landed: runtime-private session substrate
    (`ArrayTextSlotSession` + kernel-private `ArrayTextWriteTxn`)
  - H25c.2b closed: clean one-call update boundary, but non-keeper because it
    still acquires the write lock once per iteration
  - H25c.2c/H25c.3 closed: MIR-owned single-region executor metadata now
    drives one begin-site runtime call; result was `C 3 ms / Ny AOT 5 ms`
  - H25d.1/H25d.2 landed: direct text-resident region loop plus hot/cold fixed
    len-store mutation split
  - H25d result: `C 3 ms / Ny AOT 3 ms`, `ny_aot_instr=16570267`,
    `ny_aot_cycles=3471656`
  - H25d.5 closes residual memmove / mutation surgery: H25d.3/H25d.4 both
    regressed, so keep H25d.1/H25d.2 as the accepted code
  - H25e post-parity owner refresh selected the whole-front inner scan:
    `indexOf("line") >= 0` followed by same-slot suffix store
  - next slice: H26 array text observer-store region contract; keep legality in
    MIR observer metadata and keep `.inc` emit-only
  - H21 is closed: MIR now owns the loopcarry len/store route; lowered loop body is one `nyash.array.string_insert_mid_subrange_len_store_hisi` call and no standalone `nyash.array.string_len_hi`
  - H20 is closed: pure meso substring concat len now folds to arithmetic, with no loop `substring_len_hii` / `substring_hii`
  - H20 result: `kilo_meso_substring_concat_len = C 3 ms / Ny AOT 3 ms`, `ny_aot_instr=1190204`
  - H19 is closed: whole `array.get -> indexOf` source liveness now treats same-slot const suffix store as a slot-capable consumer; row-scan `array.get_hi` materialization is gone
  - H19 result: `kilo_kernel_small_hk = C 82 ms / Ny AOT 28 ms`
  - H18 is closed: exact `kilo_micro_array_string_store` is `C 10 ms / Ny AOT 4 ms`, `ny_aot_instr=9270464`, `ny_aot_cycles=2343815`, and loop-carried text now stays in an SSA vector
  - keep array slot stores unchanged unless a separate MIR-owned no-escape / consumer proof is opened
  - H17 is closed: exact `kilo_micro_array_string_store` stays `C 10 ms / Ny AOT 5 ms`, `ny_aot_instr=10870861`, `ny_aot_cycles=9526782`, and the loop-body `text+16` terminator store is gone
  - exact array-store route-shape card is closed; do not reopen it without a new failed measurement
  - kilo optimization is already active as `137x-H`; keep owner-first evidence for each slice
  - 137x-C final gate already passed: `tools/checks/dev_gate.sh quick`
  - done in this cleanout:
    - `array-typed-slot-truth-sync`
    - `map-demand-vs-typed-lane-boundary`
    - `primitive-residuals-classification`
    - `container-identity-residence-contract`
- method anchor:
  - `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
- rollout anchor:
  - `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
  - `docs/development/current/main/design/string-value-model-phased-rollout-ssot.md`
- successor planning anchor:
  - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
  - `docs/development/current/main/phases/phase-289x/README.md`
  - `docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md`
  - `docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md`
  - read as vocabulary / demand input for the constrained `137x-F` bridge, not as broad runtime rewrite permission
  - current docs focus is shared runtime vocabulary:
    - `Ref / Owned / Cell / Immediate / Stable`
    - `get / set / call` as demand verbs
  - array/map remain identity containers; only internal residence may become lane-hosted later
  - `publish` / `promote` stay boundary effects; `freeze.str` stays the only string birth sink
  - do not start broad runtime-wide implementation; `137x-F` is the only open Value Lane bridge
- taskboard:
  - `docs/development/current/main/phases/phase-137x/137x-91-task-board.md`
  - `docs/development/current/main/phases/phase-137x/137x-94-textlane-value-allocator-implementation-gate.md`
  - `docs/development/current/main/phases/phase-137x/137x-93-container-primitive-design-cleanout.md`
  - `docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md`
- immediate follow-on:
  - `137x-E1`: minimal `TextLane` / `ArrayStorage::Text` is landed
  - `137x-F`: runtime-wide Value Lane implementation bridge
  - `137x-G`: allocator / arena pilot
  - `137x-H`: kilo optimization return after F/G land or reject
- baseline 137x-D code seam:
  - phase 2.5 read-side alias lane remains landed baseline evidence:
    - `TextReadOnly`
    - `EncodedAlias`
    - `StableObject`
  - landed proof on this lane now covers both array and map reads:
    - `live source`
    - `cached handle`
    - `cold fallback`
  - keep `VerifiedTextSource -> TextPlan -> OwnedBytes -> KernelTextSlot` as the already-landed phase-1 canonical corridor
  - `TextLane` is now opened through `137x-E`; MIR legality remains limited to the needed contracts for that gate
- latest non-keeper:
  - `producer-side unpublished-outcome active probe regressed to 236 ms exact / 2173 ms whole and is reverted`
- latest observability split:
  - `lookup_array_store_str_source_obj` is now visible as:
    - `lookup.registry_slot_read`
    - `lookup.caller_latest_fresh_tag`
  - publish site counters now exist for the exact micro front:
    - `site.string_concat_hh.*`
    - `site.string_substring_concat_hhii.*`
  - latest raw whole observe reread proves those exact-micro sites are not the whole-kilo owner:
    - `const_suffix freeze_fallback=479728`
    - `freeze_text_plan_pieces3=60000`
    - `site.string_concat_hh.*=0`
    - `site.string_substring_concat_hhii.*=0`
  - latest exact / meso / whole slot-boundary reread shows:
    - `publish_boundary.slot_publish_handle_total=0`
    - `publish_boundary.slot_objectize_stable_box_total=0`
    - `publish_boundary.slot_empty=0`
    - `publish_boundary.slot_already_published=0`
    - `publish_reason.need_stable_object=0`
    - slot exit is now observable and inactive; owner stays upstream of the slot boundary

## Current Handoff

- current broad owner family is `array/string-store`
- duplicated producer is already fixed in trusted direct MIR; runtime publication/source-capture stayed hot
- compiler-side known string-length propagation is now landed for const / substring-window / same-length string `phi`
- active AOT entry IR on this front no longer emits `nyash.string.len_h` in `ny_main`
- `.hako` owner-side Stage A pilot is landed on the VM/reference lane; `ArrayCoreBox` now routes proven string-handle `set(...)` through `nyash.array.set_his`
- active AOT already reaches `nyash.array.set_his` without that VM/reference pilot
- slot-store boundary delayed-publication probes are rejected:
  - `v1 = 252 ms / 765 ms`
  - `v2 = 211 ms / 1807 ms`
  - wrong cut; do not reopen this before a new design decision
- helper-only keeper from that rejected card is committed as `b35382cf9`
- latest `perf-observe` reread no longer ranks `string_len_export_slow_path`; the live top stays publication/source-capture
- exact micro vs adopted middle vs whole kilo must now be read separately:
  - exact micro owner = shared generic publish/objectize behind `string_concat_hh` + `string_substring_concat_hhii`
  - adopted middle = `kilo_meso_substring_concat_array_set_loopcarry`, used to confirm the same corridor without `indexOf("line")` row-scan noise
  - whole kilo owner = `const_suffix` fallback + `freeze_text_plan(Pieces3)` publication
- landed 137x-D evidence came from the store/publication corridor around:
  - `execute_store_array_str_contract`
  - `array_get_index_encoded_i64`
  - `insert_const_mid_fallback`
- allocator / GC (`memmove` / `gc_alloc` / `_int_malloc`) opens only through `137x-G` after `137x-F` evidence keeps it structural
- `indexOf` stays a side diagnosis, not the active keeper card
- keep public ABI / legality ownership unchanged
- next kilo perf slice is `137x-H`; first run `137x-F/G` implementation gates
- current plain-release reread after reverting the failed active probe:
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 132 ms`
  - `kilo_kernel_small_hk = C 80 ms / Ny AOT 731 ms`
- latest rejected probe:
  - direct `StringBox -> handle` publish plus string-specialized host-handle payload
  - `kilo_meso_substring_concat_array_set_loopcarry = 68 ms`
  - `kilo_kernel_small = 950 ms`
  - reverted; do not reopen this seam before new owner evidence
- latest design consult is accepted in narrowed form:
  - no syntax expansion
  - no public raw string / mutable bytes
  - the old runtime-private `const_suffix` / `Pieces3` publication widening stays historical baseline; the active widening is now `137x-E`
  - semantic lock is now explicit:
    - `String = value`
    - `publish = boundary effect`
    - `freeze.str = only birth sink`
  - phased rollout is now fixed:
    - phase 1 = producer outcome -> canonical sink
    - phase 2 = cold publish effect
    - phase 2.5 = read-side alias lane split
    - phase 3 = `137x-E` `TextLane` storage/residence implementation
    - phase 4 = `137x-F/G` Value Lane bridge and allocator pilot
  - current phase 2.5 mirror:
    - map value stores now preserve borrowed string aliases
    - borrowed-alias runtime-handle cache is shared across alias lineage
    - latest strict reread came back reject-side:
      - exact stays closed
      - meso / strict whole reopened upward (`61 ms`, `809-892 ms`)
    - next decision point is `137x-E`, not another helper-local cleanup card
  - reuse existing `TextPlan` / `OwnedBytes` seams before inventing a new carrier
- hot-corridor carrier design anchor is now:
  - `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
- compare `.hako` only under:
  - `Stage A: same protocol`
  - `Stage B: same public ABI / different seam`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/phases/phase-137x/README.md`
4. `docs/development/current/main/phases/phase-137x/137x-94-textlane-value-allocator-implementation-gate.md`
5. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
6. `docs/development/current/main/phases/phase-137x/137x-93-container-primitive-design-cleanout.md`
7. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
8. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
9. `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
10. `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
11. `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
12. `docs/development/current/main/phases/phase-289x/README.md` (`137x-F` vocabulary / demand input)
13. `docs/development/current/main/design/string-value-model-phased-rollout-ssot.md`
14. `docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md`
15. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
16. `docs/development/current/main/design/string-birth-sink-ssot.md`
17. `docs/development/current/main/15-Workstream-Map.md`
18. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

## Current Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
