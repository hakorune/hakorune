# Phase 131-12: Case C (LLVM wrong result) Investigation Notes

Status: Active  
Scope: `apps/tests/llvm_stage3_loop_only.hako` が **VM では正しいが LLVM では結果が一致しない**問題の切り分け。
Related:
- SSOT (LLVM棚卸し): `docs/development/current/main/phase131-3-llvm-lowering-inventory.md`
- Case C (pattern): `docs/development/current/main/phase131-11-case-c-summary.md`
- PHI type cycle report (historical): `docs/development/current/main/phase-131-11-g-phi-type-bug-report.md`
- ENV: `docs/reference/environment-variables.md`（`NYASH_LLVM_DUMP_IR`, `NYASH_LLVM_TRACE_*`）

## 事象

- VM: `Result: 3`（期待通り）
- LLVM: `Result: 0`（不一致）

前提:
- MIR の PHI 型（loop-carrier）が循環で `String` になる問題は Phase 131-11-H で修正済み。
- それでも LLVM で結果不一致が残るため、次は **LLVM backend 側の value/phi/exit 値の取り回し**を疑う。

## 切り分け（最優先）

### 1) 文字列連結経路の影響を切る

Case C は `print("Result: " + counter)` を含むため、以下の2系統を分けて確認する:

- **Loop 値そのもの**が壊れているのか？
- **String concat / print** の coercion 経路が壊れているのか？

最小の派生ケース（新規fixtureにせず /tmp でOK）:

1. `return counter`（出力なし、戻り値のみ）
2. `print(counter)`（文字列連結なし）
3. `print("Result: " + counter)`（元の形）

VM/LLVM で挙動を揃えて比較する。

### 2) LLVM IR を必ず保存して diff する

同一入力に対して:

- `NYASH_LLVM_DUMP_IR=/tmp/case_c.ll tools/build_llvm.sh apps/tests/llvm_stage3_loop_only.hako -o /tmp/case_c`
- 必要に応じて `NYASH_LLVM_TRACE_PHI=1 NYASH_LLVM_TRACE_VALUES=1 NYASH_LLVM_TRACE_OUT=/tmp/case_c.trace`

確認点（IR）:
- loop-carrier に対応する `phi` が **正しい incoming** を持っているか
- ループ exit 後に参照される値が **backedge の最終値**になっているか（init 値のままになっていないか）
- `print`/`concat` 直前で `counter` が `0` に固定されていないか（Constant folding ではなく wiring 問題）

## 期待される原因クラス

- **Exit value wiring**: JoinIR→MIR→LLVM のどこかで exit 後の “host slot” へ値が戻っていない
- **PHI/value resolution**: LLVM backend の `vmap` / `resolve_*` が exit 後の ValueId を誤解決している
- **String concat coercion**: `counter` を string へ変換する経路で別の ValueId を参照している

## 受け入れ基準（この調査のDone）

- `return counter` と `print(counter)` が VM/LLVM で一致するまで、問題を局所化できていること。
- その状態で、必要な修正点（どのファイル/どの関数）が特定できていること。

