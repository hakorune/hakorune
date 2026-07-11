---
Status: Instructions
Scope: Phase 29ax P2
---

# P2: Stabilize purity gate env

目的: regression pack 実行時に strict/dev の観測条件が揺れないよう、smoke スクリプト側で環境汚染を確実に無効化する。

## 背景

- pack は同一シェルで複数の smoke を連続実行するため、前の smoke が export した env が残留すると “tag missing” が起きうる。

## 実装方針（SSOT）

- `env -u` に依存しない（環境/シェル差で失敗しやすい）
- 各実行の直前に明示的に `unset` して、strict 実行のノイズ源を落とす

## 対象

- `tools/smokes/v2/profiles/integration/joinir/joinir_purity_gate_vm.sh`
  - `HAKO_JOINIR_DEBUG/DEV` と `NYASH_JOINIR_DEBUG/DEV` を毎回 `unset`

## 受け入れ

- `./tools/smokes/v2/profiles/integration/joinir/joinir_purity_gate_vm.sh` が安定 PASS
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` が PASS
