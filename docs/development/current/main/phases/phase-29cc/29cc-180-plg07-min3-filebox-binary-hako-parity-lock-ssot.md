---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: PLG-07-min3 として FileBox binary API の `.hako` parity fixture/smoke を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-178-plg07-plugin-derust-cutover-order-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-179-plg07-min1-min2-filebox-binary-rust-parity-lock-ssot.md
  - apps/tests/phase29cc_plg07_filebox_binary_hako_route_min.hako
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_hako_route_vm.sh
---

# 29cc-180 PLG-07-min3 FileBox Binary `.hako` Parity Lock

## Purpose
PLG-07 の `.hako` parity を FileBox binary API 1件で固定し、min4 dual-run 比較へ進む前提を作る。

## Decision
1. `.hako` parity fixture を追加し、`readBytes` ルートを実行する。
2. `.hako` parity smoke を追加し、strict-plugin-first provider policy で実行を固定する。
3. Rust route と同じ payload 契約（`PLG07_BINARY_OK`）を使い、比較可能性を維持する。

## Contract
1. `.hako` parity fixture は `FileBox.readBytes` を 1 回以上実行する。
2. smoke は `HAKO_PROVIDER_POLICY=strict-plugin-first` で動く。
3. payload file は実行前後で `PLG07_BINARY_OK` を保持する（非破壊）。

## Acceptance
1. `cargo check --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_hako_route_vm.sh`

## Next
1. `PLG-07-min4` dual-run parity gate（Rust route vs `.hako` route）を lock する。
