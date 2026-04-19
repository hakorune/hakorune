---
Status: Active
Date: 2026-04-19
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
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
  - `phase-137x-B container / primitive design cleanout before owner-first optimization return`
  - execution mode:
    - docs-first BoxShape gate; no perf implementation until `137x-C`
- blocker:
  - `137x-B design cleanout` before perf return
- worktree:
  - clean is expected; do not resurrect `stash@{0}` unless you are explicitly reopening the rejected slot-store boundary probe
- current snapshot:
  - `kilo_micro_substring_concat = C 2 ms / Ny AOT 3 ms`
  - `kilo_micro_array_string_store = C 10 ms / Ny AOT 4 ms`
  - `kilo_meso_substring_concat_array_set_loopcarry = C 3 ms / Ny AOT 57 ms`
  - adopted middle bridge:
    - `substring + concat + array.set + loopcarry`
    - use it to confirm store/publication cuts without the whole-front `indexOf("line")` row-scan noise
  - `kilo_kernel_small = C 80 ms / Ny AOT 739 ms`
- immediate next:
  - `finish 137x-B design cleanout: map demand vs typed-lane boundary, primitive residuals, and container identity/residence contract`
  - done in this cleanout:
    - `array-typed-slot-truth-sync`
- method anchor:
  - `docs/development/current/main/design/perf-owner-first-optimization-ssot.md` (`137x-C` only)
- rollout anchor:
  - `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
  - `docs/development/current/main/design/string-value-model-phased-rollout-ssot.md`
- successor planning anchor:
  - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
  - `docs/development/current/main/phases/phase-289x/README.md`
  - `docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md`
  - `docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md`
  - read as parked planning only; phase-0 authority/vocabulary lock is docs-only
  - current docs focus is shared runtime vocabulary:
    - `Ref / Owned / Cell / Immediate / Stable`
    - `get / set / call` as demand verbs
  - array/map remain identity containers; only internal residence may become lane-hosted later
  - `publish` / `promote` stay boundary effects; `freeze.str` stays the only string birth sink
  - do not start runtime-wide implementation before 137x-B design cleanout is closed and phase-137x is judged again
- taskboard:
  - `docs/development/current/main/phases/phase-137x/137x-91-task-board.md`
  - `docs/development/current/main/phases/phase-137x/137x-93-container-primitive-design-cleanout.md`
  - `docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md`
- immediate follow-on:
  - `keep phase order intact: canonical sink before cold publish effect, cold publish before read-side alias split, read-side alias split before TextLane, TextLane before MIR legality`
- deferred 137x-C code seam:
  - phase 2.5 read-side alias lane stays next only after 137x-B closes:
    - `TextReadOnly`
    - `EncodedAlias`
    - `StableObject`
  - landed proof on this lane now covers both array and map reads:
    - `live source`
    - `cached handle`
    - `cold fallback`
  - keep `VerifiedTextSource -> TextPlan -> OwnedBytes -> KernelTextSlot` as the already-landed phase-1 canonical corridor
  - do not jump to `TextLane` or MIR legality first
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
- deferred 137x-C narrow cut candidate is the store/publication corridor around:
  - `execute_store_array_str_contract`
  - `array_get_index_encoded_i64`
  - `insert_const_mid_fallback`
- allocator / GC (`memmove` / `gc_alloc` / `_int_malloc`) stays secondary diagnosis until that corridor is disproved
- `indexOf` stays a side diagnosis, not the active keeper card
- keep public ABI / legality ownership unchanged
- next perf slice is no longer `len_h` removal; when 137x-C opens, restart from publication/source-capture with the compiler-known-length lane fixed
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
  - the next widening stays inside runtime-private `const_suffix` / `Pieces3` publication, not helper-site specialization
  - semantic lock is now explicit:
    - `String = value`
    - `publish = boundary effect`
    - `freeze.str = only birth sink`
  - phased rollout is now fixed:
    - phase 1 = producer outcome -> canonical sink
    - phase 2 = cold publish effect
    - phase 2.5 = read-side alias lane split
    - phase 3 = future `TextLane`
    - phase 4 = MIR legality / sink-aware AOT
  - current phase 2.5 mirror:
    - map value stores now preserve borrowed string aliases
    - borrowed-alias runtime-handle cache is shared across alias lineage
    - latest strict reread came back reject-side:
      - exact stays closed
      - meso / strict whole reopened upward (`61 ms`, `809-892 ms`)
    - next decision point is the smallest cleanup cards on this lane, not another semantics/storage expansion
  - reuse existing `TextPlan` / `OwnedBytes` seams before inventing a new carrier
- hot-corridor carrier design anchor is now:
  - `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
- compare `.hako` only under:
  - `Stage A: same protocol`
  - `Stage B: same public ABI / different seam`

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/10-Now.md`
3. `docs/development/current/main/investigations/phase137x-array-store-owner-snapshot-2026-04-18.md`
4. `docs/development/current/main/phases/phase-137x/README.md`
5. `docs/development/current/main/phases/phase-137x/137x-93-container-primitive-design-cleanout.md`
6. `docs/development/current/main/design/kernel-observability-and-two-stage-pilot-ssot.md`
7. `docs/development/current/main/design/runtime-hot-lane-optimization-patterns-ssot.md`
8. `docs/development/current/main/design/string-hot-corridor-runtime-carrier-ssot.md`
9. `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
10. `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
11. `docs/development/current/main/phases/phase-289x/README.md` (`runtime-wide value/object` planning only)
12. `docs/development/current/main/design/string-value-model-phased-rollout-ssot.md`
13. `docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md`
13. `docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md`
14. `docs/development/current/main/design/string-birth-sink-ssot.md`
15. `docs/development/current/main/15-Workstream-Map.md`
16. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md` (`phase-29bq` に戻るときだけ)

## Current Proof Bundle

```bash
git status -sb
tools/checks/dev_gate.sh quick
cargo test -p nyash_kernel --lib string_helpers::tests:: -- --nocapture
cargo check --features perf-observe -p nyash_kernel
cargo test -p nyash_kernel --lib --tests --no-run
```
