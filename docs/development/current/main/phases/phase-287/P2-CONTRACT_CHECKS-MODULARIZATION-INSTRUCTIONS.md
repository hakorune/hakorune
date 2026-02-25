# Phase 287 P2: `merge/contract_checks.rs` 分割指示書（意味論不変）

**Date**: 2025-12-27  
**Status**: Completed ✅  
**Scope**: `src/mir/builder/control_flow/joinir/merge/contract_checks.rs`（~846行）を facade 化し、契約検証を “1 module = 1 契約” に分割  
**Non-goals**: エラータグ変更、検証条件の追加/緩和、`merge/instruction_rewriter.rs` の分割、silent fallback 追加

---

## 目的（SSOT）

- merge の “Fail-Fast 契約” を **構造で見える化**する。
- `contract_checks.rs` を facade（re-export + glue）に寄せ、検証ロジックを責務分離する。
- 既存の呼び出し点を壊さない（public 関数名/シグネチャを基本維持）。

---

## 現状

`src/mir/builder/control_flow/joinir/merge/contract_checks.rs` は複数契約を同居させており、読む側が “どの契約がどこにあるか” を追いにくい。

代表的な契約（例）:
- terminator target existence
- exit_bindings ↔ exit_phis
- carrier_inputs completeness
- boundary contract at creation（B1/C2）
- entry params consistency

---

## 目標の構造（案）

```
src/mir/builder/control_flow/joinir/merge/
├── contract_checks.rs                 # facade（旧名維持 / re-export）
└── contract_checks/                   # NEW
    ├── mod.rs
    ├── terminator_targets.rs          # verify_all_terminator_targets_exist
    ├── exit_bindings.rs               # verify_exit_bindings_have_exit_phis
    ├── carrier_inputs.rs              # verify_carrier_inputs_complete
    ├── boundary_creation.rs           # verify_boundary_contract_at_creation（B1/C2）
    └── entry_params.rs                # verify_boundary_entry_params
```

ルール:
- `contract_checks.rs` は **呼び出し側互換のための facade** に徹する。
- 新規の “総合入口” を作るなら `contract_checks::run_all_pipeline_checks()` のみ（既存があるなら整理だけ）。

---

## 手順（安全な順序）

### Step 1: `contract_checks/` を追加し facade を作る

- `contract_checks/mod.rs` に各 module を `pub(super)` で生やす
- `src/mir/builder/control_flow/joinir/merge/contract_checks.rs` から `pub(super) use ...` で既存 API を re-export

### Step 2: 低依存の契約から移す

優先:
- `verify_all_terminator_targets_exist()`（依存が少ない）
- `verify_exit_bindings_have_exit_phis()`

### Step 3: `verify_carrier_inputs_complete()` を移す

- ここは “エラータグ/ヒント文” が契約なので、文言変更は避ける。

### Step 4: boundary creation（B1/C2）を移す

- `verify_boundary_contract_at_creation()` は “入口で fail-fast する” という意味で重要なので、移設後も呼び出し位置を変えない。

### Step 5: entry params を移す

- `verify_boundary_entry_params()` は “param 順序” の SSOT なので、テストがあるなら位置だけ追従させる。

---

## テスト（仕様固定）

P2 は意味論不変が主目的なので、原則 “既存テストを壊さない” を優先する。

- 既存の `#[cfg(test)]` が `contract_checks.rs` にある場合:
  - 最小差分で `contract_checks/` の該当 module へ移動
  - もしくは facade 側に残して import だけ更新（どちらでも可）

新規テストは原則不要（既存で十分）。

---

## 検証手順（受け入れ基準）

```bash
cargo build --release
./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako   # RC=9
./tools/smokes/v2/run.sh --profile quick
```

受け入れ:
- Build: 0 errors
- quick: 154/154 PASS
- Pattern6: RC=9 維持
- 恒常ログ増加なし

---

## Out of Scope（重要）

- `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs` の物理分割
- エラータグの変更、契約の意味変更、条件追加による挙動変化
- “便利な fallback” の追加（Fail-Fast 原則に反するため）
