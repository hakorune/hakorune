---
Status: Active
Decision: provisional
Date: 2026-03-21
Scope: smoke 実行契約を path-first recursive discovery から suite-manifest first へ寄せる first slice。`--profile` 互換は維持し、mass move はまだしない。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/smoke-taxonomy-and-discovery-ssot.md
  - tools/smokes/v2/run.sh
  - tools/checks/smoke_inventory_report.sh
  - tools/smokes/v2/suites/README.md
  - tools/smokes/v2/suites/integration/presubmit.txt
  - tools/smokes/v2/suites/integration/collection-core.txt
  - tools/smokes/v2/suites/integration/vm-hako-core.txt
  - tools/smokes/v2/suites/integration/vm-hako-caps.txt
  - tools/smokes/v2/suites/integration/selfhost-core.txt
  - tools/smokes/v2/suites/integration/joinir-bq.txt
  - tools/smokes/v2/suites/integration/phase29ck-boundary.txt
  - tools/smokes/v2/profiles/integration/vm_hako_caps/README.md
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/README.md
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/README.md
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/README.md
  - tools/smokes/v2/profiles/integration/phase21_5/perf/README.md
  - tools/smokes/v2/profiles/integration/phase21_5/perf/apps/README.md
  - tools/smokes/v2/profiles/integration/phase21_5/perf/apps/entry_mode/README.md
  - tools/smokes/v2/profiles/integration/phase21_5/perf/apps/mir_mode/README.md
  - tools/smokes/v2/suites/integration/phase29cc-wsm-g3-canvas.txt
  - tools/smokes/v2/suites/integration/phase29cc-wsm-g2-browser.txt
  - tools/smokes/v2/suites/integration/phase29cc-wsm-g4.txt
  - tools/smokes/v2/suites/integration/phase29cc-wsm-p10.txt
  - tools/smokes/v2/suites/integration/phase21_5-perf-chip8.txt
  - tools/smokes/v2/suites/integration/phase21_5-perf-kilo.txt
  - docs/development/testing/smoke-tests-v2.md
  - docs/tools/README.md
---

# Phase 29cq: Smoke Suite Manifest Cutover

## Goal

- `path = 保管場所`, `suite = 実行契約` を first-class にする。
- `tools/smokes/v2/run.sh` は `--profile` 互換を維持したまま `--suite` allowlist を持つ。
- live discovery は残すが、daily/presubmit の人間向け導線は suite manifest から呼ぶ。

## Non-Goals

- `tools/smokes/v2/profiles/**` の大量移動
- 各 `.sh` への metadata header の一括追加
- `integration/apps` の一括 rename
- `archive/lib/tmp/fixtures` の再活性化
- gate script の意味変更

## Fixed Order

1. suite manifest format を固定する
2. `run.sh --suite <name>` を opt-in で追加する
3. integration の seed suites を追加する
4. docs / `CURRENT_TASK.md` を suite-first 読みに更新する
5. inventory / semantic split は次 slice へ送る

## First Slice Contract

- manifest path:
  - `tools/smokes/v2/suites/<profile>/<suite>.txt`
- manifest format:
  - `#` comment 可
  - 1 行 1 path
  - path は `tools/smokes/v2/profiles/<profile>/` 相対
- runner contract:
  - `--suite` なし: 既存の live discovery を維持
  - `--suite` あり: live discovery に対する allowlist intersection
  - manifest が missing / duplicate / non-live entry を含む場合は fail-fast

## Seed Suites

- `integration/presubmit`
- `integration/collection-core`
- `integration/vm-hako-core`
- `integration/vm-hako-caps`
- `integration/selfhost-core`
- `integration/joinir-bq`

## Acceptance

- `bash tools/smokes/v2/run.sh --profile integration --suite presubmit --dry-run`
- `bash tools/smokes/v2/run.sh --profile integration --suite collection-core --dry-run`
- `bash tools/smokes/v2/run.sh --profile integration --suite vm-hako-core --dry-run`
- `bash tools/smokes/v2/run.sh --profile integration --suite selfhost-core --dry-run`
- `bash tools/smokes/v2/run.sh --profile integration --suite joinir-bq --dry-run`
- invalid manifest entry は fail-fast
- `git diff --check`

## Next

- `tools/checks/smoke_inventory_report.sh` は suite-aware summary 済み。`rc_gc_alignment` / `json` / `mir_shape_guard` / `ring1_providers` / `phase29ck_boundary` / `vm_hako_caps` / `phase29cc_wsm/g3_canvas` / `phase29cc_wsm/g2_browser` / `phase29cc_wsm/g4` / `phase29cc_wsm/p10` / `phase29cc_wsm/p5` / `phase29cc_wsm/p6` / `phase29cc_wsm/p7` / `phase21_5/perf/chip8` / `phase21_5/perf/kilo` の first live split はそれぞれ `integration/rc_gc_alignment/` / `integration/json/` / `integration/mir_shape/` / `integration/ring1_providers/` / `integration/phase29ck_boundary/{entry,string,runtime_data}/` / `integration/vm_hako_caps/{app1,args,compare,env,file,gate,lib,mapbox,misc,open_handle_phi,select_emit}/` / `integration/phase29cc_wsm/g3_canvas/` / `integration/phase29cc_wsm/g2_browser/` / `integration/phase29cc_wsm/g4/` / `integration/phase29cc_wsm/p10/` / `integration/phase29cc_wsm/p5/` / `integration/phase29cc_wsm/p6/` / `integration/phase29cc_wsm/p7/` / `integration/phase21_5/perf/chip8/` / `integration/phase21_5/perf/kilo/` に出したので、次は `integration/apps` の残り family を semantic split する
- `integration/apps` の新規追加を凍結し、新規 smoke は semantic domain 配下へ置く
- `phase29cc_wsm/p8` の切り出しも完了し、`phase21_5/perf/{chip8,kilo}` と `phase21_5/perf/apps/{entry_mode,mir_mode,case_breakdown,compile_run_split,crosslang_bundle,emit_mir_jsonfile_route,startup_subtract}` も landed したので、`integration/apps` への新規追加は凍結したまま、必要なら次の大きな domain family を切る
