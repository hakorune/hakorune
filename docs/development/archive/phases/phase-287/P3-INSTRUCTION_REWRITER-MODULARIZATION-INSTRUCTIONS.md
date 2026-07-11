# Phase 287 P3: `merge/instruction_rewriter.rs` 分割指示書（意味論不変）

**Date**: 2025-12-27  
**Status**: Completed ✅  
**Scope**: `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs`（~1297行）を “Scan/Plan/Apply” の stage 単位に分割し、`instruction_rewriter.rs` を facade（orchestrator）へ縮退する。  
**Non-goals**: 仕様変更、エラータグ/ヒント文の変更、検出条件の追加/緩和、silent fallback 追加、ログの恒常増加、Plan line / Pattern の挙動変更

---

## 目的（SSOT）

- JoinIR merge の “3-stage pipeline” を **物理構造**で見える化し、責務を分離する。
- 巨大ファイルを “入口（orchestrator）” と “stage 実装” に分けて、読む導線を短くする。
- 意味論不変（behavior unchanged）を守りつつ、後続の小改修（Box 抽出/契約追加）を安全にする。

---

## 現状（問題点）

- `instruction_rewriter.rs` が大きく、Scan/Plan/Apply の境界がコード上で追いにくい。
- stage 実装が 1 ファイルに同居しているため、変更時に “どこまでが read-only / どこからが mutation” かが崩れやすい。

※ 既に `rewriter/scan_box.rs` / `rewriter/plan_box.rs` 等の土台はあり、分割は “移動 + import 整理” が主になるはず。

---

## 目標の構造（案）

最小の増分で “1 stage = 1 module” を実現する。

```
src/mir/builder/control_flow/joinir/merge/
├── instruction_rewriter.rs            # facade（merge_and_rewrite の入口 + stage 呼び出し）
└── rewriter/
    └── stages/                        # NEW
        ├── mod.rs
        ├── scan.rs                    # scan_blocks()
        ├── plan.rs                    # plan_rewrites()
        └── apply.rs                   # apply_rewrites()
```

ルール:
- facade は “順番” と “受け渡し” のみを担い、ロジックは stage 側へ移す。
- `RewriteContext` は既存の SSOT を使い続ける（散らさない）。

---

## 手順（安全な順序）

### Step 1: `rewriter/stages/` を追加して “空の mod” を作る

- `rewriter/stages/mod.rs` を追加し、`pub(super) mod scan; pub(super) mod plan; pub(super) mod apply;` を置く。
- 先にコンパイルが通る状態を作る（未使用は `pub(super)` に寄せる）。

### Step 2: Stage 1 を移す（scan）

- `instruction_rewriter.rs` の `scan_blocks(...)` を `rewriter/stages/scan.rs` へ移動する。
- 署名と返り値は維持する（意味論不変のため）。

### Step 3: Stage 2 を移す（plan）

- `plan_rewrites(...)` を `rewriter/stages/plan.rs` へ移動する。
- `plan.rs` 側の “ローカル helper” が肥大化する場合:
  - まずは `plan_helpers.rs` へ移動して SSOT を 1 箇所に寄せる（新規 helper の乱立を避ける）。

### Step 4: Stage 3 を移す（apply）

- `apply_rewrites(...)` を `rewriter/stages/apply.rs` へ移動する。
- boundary injection のブロックは動かし過ぎない（挙動差が出やすい）。

### Step 5: facade を縮退する（orchestrator only）

- `instruction_rewriter.rs` に残すのは:
  - `merge_and_rewrite(...)`（外部 API）
  - stage 呼び出し（scan → plan → apply）
  - stage 間の contract check 呼び出し（例: `contract_checks::verify_carrier_inputs_complete(...)`）
- “大きなローカル関数” は原則 stage 側へ移す。

---

## テスト（仕様固定）

P3 は意味論不変が主目的のため、原則は既存 smoke を維持する。

新規テストを足すなら “構造テスト” を 1 つだけ（任意）:
- `scan_blocks` が “read-only” であること（builder を触らない）を保証する形のテストは難しいので、代替として
  - `classify_tail_call()` の境界条件ユニットテスト
  - `latch_incoming_recorder` の invariants テスト

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

- 既存の `TODO` 群（exit_collection への統合など）を進めること
- `ReturnConverterBox` / `ParameterBindingBox` のさらなる箱化（P3 は “stage 物理分割” に限定）
- 新しい env var / debug トグルの追加
