---
Status: Active
Scope: live compat retirement (`SMOKES_SELFHOST_FILTER` / by-name fixture key / semantic fixture alias)
Related:
- CURRENT_TASK.md
- docs/development/current/main/phases/phase-29cd/README.md
- docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md
- docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md
- docs/development/current/main/design/joinir-smoke-legacy-stem-retirement-ssot.md
---

# Phase 29ce: live compat retirement

## Goal

current semantic wrapper / semantic fixture alias / semantic route substring を正本に保ったまま、
まだ live contract として動いている compat token を retire 可能な形まで分離する。

## Why this is separate from aftercare

`phase-29cd` は aftercare 全体の closeout を扱う。ここではその中でも
`live compat contract lane` を独立させる。

理由:
- `SMOKES_SELFHOST_FILTER` は selfhost gate の live contract で、archive replay lane と責務が違う
- Program JSON の by-name fixture key は frontend entry contract で、smoke stem と retire 条件が違う
- semantic fixture alias は current lane の正本なので、old pin token とは逆向きに守る必要がある

## Scope

1. `SMOKES_SELFHOST_FILTER`
   - semantic route substring / semantic fixture alias を current contract として固定
   - exact historical basename は inventory-only に寄せる
2. Program JSON by-name fixture key
   - live key / retired key / inventory-only key を分ける
3. semantic fixture alias
   - active docs / gate / selfhost subset の先頭に置く

## Current live-token hotspots

1. selfhost filter contract
   - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
   - `SMOKES_SELFHOST_FILTER` は `fixture + planner_tag + reason` を部分一致で見るので、
     `reason` に残る exact historical token も live compat token になる
2. selfhost subset TSV
   - `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
   - semantic fixture alias を前面に置いていても、`reason` 列の legacy stem は filter 契約へ波及する
3. regression pack filters
   - `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
   - current pack でも phase-prefixed filter family が残るため、archive replay lane とは別に retire authority が必要
4. Program JSON by-name key
   - `src/mir/join_ir/frontend/ast_lowerer/route.rs`
   - runtime code 内に live string-key contract が埋め込まれている

## Non-goals

- archive replay forwarder の hard-delete
- `docs/private` nested repo の drift cleanup
- generic な `pattern` 一般語の絶滅

## Exit criteria

- active how-to/checklist は semantic route substring または semantic fixture alias を先頭に置く
- exact historical basename は inventory/retirement SSOT にだけ残る
- by-name key の live set / retired set が SSOT で一意に読める
- `phase29bq_fast_gate_vm.sh --only bq` と `phase29x-probe` が緑のまま

## Instructions

- P0: `docs/development/current/main/phases/phase-29ce/P0-LIVE-COMPAT-RETIREMENT-INSTRUCTIONS.md`
