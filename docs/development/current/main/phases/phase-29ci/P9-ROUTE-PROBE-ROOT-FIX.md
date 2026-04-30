---
Status: Accepted
Decision: accepted
Date: 2026-04-30
Scope: `tools/checks/route_env_probe.sh` の repo-root/source 解決を current route proof 用に直す。
Related:
  - docs/development/current/main/phases/phase-29ci/P8-HELPER-LEGACY-DELEGATE-RETIRE.md
  - tools/checks/route_env_probe.sh
  - tools/checks/route_no_fallback_guard.sh
  - docs/tools/README.md
---

# P9 Route Probe Root Fix

## Goal

P8 の検証中に、`route_env_probe.sh --run` が repo root を `tools/` として解決し、
`tools/target/release/hakorune` や存在しない source path を見に行くことが分かった。

route proof helper は cleanup の判定源なので、repo-root/source 解決を小さく直す。

## Decision

- script root は `tools/checks/../..` を repo root とする。
- `--source` が相対 path の場合は repo root 相対へ正規化する。
- examples は存在する fixture を指す。
- route vocabulary や compiler behavior は変えない。

## Acceptance

```bash
bash tools/checks/route_env_probe.sh --route hako-mainline \
  --source apps/tests/phase29bq_hako_mirbuilder_phase1_literal_return_min.hako \
  --run --require-no-fallback
bash tools/checks/route_no_fallback_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
