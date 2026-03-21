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
  - tools/smokes/v2/suites/integration/selfhost-core.txt
  - tools/smokes/v2/suites/integration/joinir-bq.txt
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

- `tools/checks/smoke_inventory_report.sh` に suite-aware summary を足す
- `integration/apps` の新規追加を凍結し、新規 smoke は semantic domain 配下へ置く
- active family を小さく `integration/<domain>/` へ移し始める
