---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: PLG-07-min1(min docs lock) + PLG-07-min2(Rust plugin parity) として FileBox.readBytes/writeBytes の plugin 契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-178-plg07-plugin-derust-cutover-order-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-177-wsm-p4-min7-buffer-file-binary-contract-lock-ssot.md
  - plugins/nyash-filebox-plugin/nyash_box.toml
  - plugins/nyash-filebox-plugin/src/constants.rs
  - plugins/nyash-filebox-plugin/src/filebox_impl.rs
  - tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_rust_route_vm.sh
  - apps/tests/phase29cc_plg07_filebox_binary_rust_route_min.hako
---

# 29cc-179 PLG-07-min1/min2 FileBox Binary Rust Parity Lock

## Purpose
plugin de-rust cutover の最初の固定として、FileBox binary API を docs-first で明文化し、
Rust plugin 実装側に同一契約を追加して route を lock する。

## Decision
1. `FileBox.readBytes` を Rust plugin の公開 method table に追加する。
   - method_id: `6`
   - 引数: `[path?: string]`
   - 戻り: `bytes`（TLV tag=7）
2. `FileBox.writeBytes` を Rust plugin の公開 method table に追加する。
   - method_id: `9`
   - 引数: `[path?: string, data: bytes]`
   - 戻り: `i32`（write count）
3. 既存 `read`/`write` は互換維持で残し、`readBytes`/`writeBytes` は同じ内部 I/O ハンドラへ合流する。
4. plugin smoke pack に PLG-07 の binary route fixture を追加し、Rust plugin route を固定する。

## Contract
1. 非破壊追加のみ（既存 method_id を変更しない）。
2. `readBytes/writeBytes` は plugin method resolve/invoke の両入口で有効。
3. VM route は FileBox Rust plugin + per-run `nyash.toml` で再現できる。
4. fail-fast 原則は維持（method missing/arg mismatch は silent fallback しない）。

## Acceptance
1. `cargo check --bin hakorune`
2. `cargo build -p nyash-filebox-plugin --release`
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29cc_plg07_filebox_binary_rust_route_vm.sh`

## Next
1. `PLG-07-min3` は `29cc-180` で lock 済み。
2. `PLG-07-min4` は `29cc-181` で lock 済み。
3. 次は `PLG-07-min5` default switch（`.hako` route 既定化）へ進む。
