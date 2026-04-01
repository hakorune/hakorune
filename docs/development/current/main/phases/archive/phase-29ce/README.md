---
Status: Accepted
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
   - selfhost subset TSV では optional `filter_alias` column と fixture-derived semantic alias で semantic basename を前に出す
   - exact historical basename は inventory-only に寄せる
2. Program JSON by-name fixture key
    - live key / retired key / inventory-only key を分ける
    - `current runtime keep` / `retired Program JSON compat key` / `retired historical docs-private-only key` /
     `current dev key` / `retired dev-gated compat key` の bucket を固定する
3. semantic fixture alias
   - active docs / gate / selfhost subset の先頭に置く

## Current live-token hotspots

1. selfhost filter contract
   - `tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh`
   - `SMOKES_SELFHOST_FILTER` は `fixture + planner_tag + reason + filter_alias + fixture-derived semantic alias`
     を部分一致で見るので、semantic substring を前面化できる一方で、
     `reason` に残る exact historical token も live compat token になりうる
2. selfhost subset TSV
   - `tools/smokes/v2/profiles/integration/selfhost/planner_required_selfhost_subset.tsv`
   - semantic fixture alias を前面に置いていても、`reason` 列の legacy stem は filter 契約へ波及する
3. regression pack filters
   - `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
   - current pack でも phase-prefixed filter family が残るため、archive replay lane とは別に retire authority が必要
4. Program JSON by-name key
   - `src/mir/join_ir/frontend/ast_lowerer/route.rs`
   - runtime code 内に live string-key contract が埋め込まれている
   - current bucket authority は
     `docs/development/current/main/design/joinir-frontend-legacy-fixture-key-retirement-ssot.md`
     を正本とする

## Current Program JSON key matrix

| Bucket | Exact keys | Meaning |
| --- | --- | --- |
| current runtime keep | `simple` | current runtime keep |
| current fixture/test keep | `test`, `local`, `_read_value_from_pair` | current fixture/test keep |
| current dev key | `nested_if_merge`, `read_quoted` | dev-gated current key |
| retired dev-gated compat key | `parse_loop`, `read_quoted_from` | retired; reject in current runtime |
| retired Program JSON compat key | `filter`, `print_tokens`, `map`, `reduce`, `fold` | retired; reject in current runtime |
| retired historical docs/private key | `pattern3_if_sum_multi_min`, `jsonparser_if_sum_min`, `selfhost_if_sum_p3`, `selfhost_if_sum_p3_ext`, `jsonparser_skip_ws_mini`, `jsonparser_skip_ws_real`, `jsonparser_atoi_mini`, `jsonparser_atoi_real`, `jsonparser_parse_number_real` | historical docs/private only |

## Non-goals

- archive replay forwarder の hard-delete
- `docs/private` nested repo の drift cleanup
- generic な `pattern` 一般語の絶滅

## Exit criteria

- active how-to/checklist は semantic route substring または semantic fixture alias を先頭に置く
- exact historical basename は inventory/retirement SSOT にだけ残る
- by-name key の live set / retired set が SSOT で一意に読める
- `phase29bq_fast_gate_vm.sh --only bq` と `phase29x-probe` が緑のまま

## Closeout decision

- semantic route substring / semantic fixture alias is now the current live contract
- fixture-derived semantic alias is part of the current live contract
- historical exact basename examples are inventory-only unless a replay note says otherwise
- future retire work is optional and should be handled as dedicated follow-up slices

## Instructions

- P0: `docs/development/current/main/phases/phase-29ce/P0-LIVE-COMPAT-RETIREMENT-INSTRUCTIONS.md`
