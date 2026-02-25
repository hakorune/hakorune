---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: X59 ABI borrowed/owned conformance matrix を Phase29x gate-first 運用へ拡張する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-78-core-cabi-delegation-inventory-guard-ssot.md
  - docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md
  - tools/checks/phase29x_abi_borrowed_owned_matrix_cases.txt
  - tools/checks/phase29x_abi_borrowed_owned_matrix_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_abi_borrowed_owned_conformance_vm.sh
---

# 29x-86: ABI Borrowed/Owned Conformance Extension (SSOT)

## 0. Goal

- X59 として、`args borrowed / return owned` 契約を matrix 化して継続検証する。
- X51（Core C ABI delegation ownership）を前提 step に固定し、ABI 境界の順序を崩さない。
- `cargo test` 単発ではなく、case inventory + guard + gate の 3点で drift を fail-fast 検出する。

## 1. Matrix SSOT

Matrix source of truth:
- `tools/checks/phase29x_abi_borrowed_owned_matrix_cases.txt`

Fixed cases:

| Case ID | nyash_kernel test | Contract focus |
| --- | --- | --- |
| `return_escape_survives_borrowed_release` | `handle_abi_borrowed_owned_conformance` | borrowed 解放後も returned-owned が生存 |
| `multi_escape_chain` | `handle_abi_borrowed_owned_multi_escape_conformance` | owned 再escape連鎖の独立 release |
| `invalid_handles_noop` | `handle_abi_borrowed_owned_invalid_handles_are_noop` | invalid retain/release は no-op で非関連 handle を壊さない |

## 2. Contract

1. Matrix file の全 case は `crates/nyash_kernel/src/tests.rs` に対応 test を持つ。
2. X59 gate は X51 guard smoke（`phase29x_core_cabi_delegation_guard_vm.sh`）を前提実行する。
3. X59 gate は `cargo test -p nyash_kernel handle_abi_borrowed_owned_ -- --nocapture --test-threads=1` を実行し、matrix の全 test 名が出力に現れることを確認する（shared host-handle registry の再利用レースを避けるため single-thread 固定）。
4. `10-ABI-SSOT.md` の契約（args borrowed / return owned）を変更しない。

## 3. Integration gate

- Guard:
  - `tools/checks/phase29x_abi_borrowed_owned_matrix_guard.sh`
- Gate:
  - `tools/smokes/v2/profiles/integration/apps/phase29x_abi_borrowed_owned_conformance_vm.sh`

Gate steps:
1. X59 guard（matrix/docs/gate wiring）
2. X51 precondition（Core C ABI delegation guard smoke）
3. borrowed/owned matrix cargo test（`handle_abi_borrowed_owned_` filter）

## 4. Evidence command

- `bash tools/checks/phase29x_abi_borrowed_owned_matrix_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29x_abi_borrowed_owned_conformance_vm.sh`

## 5. Next step

- X60 は `29x-87-rc-insertion-phase2-queue-lock-ssot.md` で完了。
- X61 は `29x-88-observability-drift-guard-ssot.md` で完了。
- X62 は `29x-89-runtime-core-integrated-gate-ssot.md` で完了。
- X63 は `29x-92-optimization-allowlist-lock-ssot.md` で完了。
- 次タスクは X64（optimization parity fixtures/reject fixtures）。
