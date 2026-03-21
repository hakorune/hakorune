---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-G2-min1 browser demo-run baseline を bridge crate で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md
  - docs/development/current/main/phases/phase-29cc/29cc-130-wsm02d-min2-demo-min-fixture-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-131-wsm02d-min3-demo-unsupported-boundary-lock-ssot.md
  - projects/nyash-wasm/build.sh
  - projects/nyash-wasm/bridge/Cargo.toml
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_min1_bridge_build_vm.sh
---

# 29cc-134 WSM-G2-min1 Bridge Run Loop Lock

## Purpose
`projects/nyash-wasm` の run loop 最小構成を、ルート crate の wasm 互換状態に依存しない形で固定する。`ConsoleBox` 5メソッド（log/warn/error/info/debug）の demo contract を browser 側で再実行可能にする。

## Decision
1. `projects/nyash-wasm/bridge` を独立 wasm bridge crate として追加し、`wasm-pack` のビルド対象をここに固定する。
2. `projects/nyash-wasm/build.sh` は bridge crate を build して `projects/nyash-wasm/pkg/` に成果物を出力する。
3. `nyash_playground.html` の初期コードを `phase29cc_wsm02d_demo_min.hako` と同じ 5メソッド contract に揃える。
4. `tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_min1_bridge_build_vm.sh` を追加し、build + export + playground marker を fail-fast で固定する。

## Acceptance
- `bash projects/nyash-wasm/build.sh`
- `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_min1_bridge_build_vm.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh`

## Notes
- この時点の bridge は `ConsoleBox` 5メソッド run loop 専用の最小実装で、scope-out は fail-fast する（WSM-02d boundary 維持）。
- 次は `WSM-G2-min2`（headless run automation smoke）へ進む。
