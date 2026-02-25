# Phase 284 P2: "return in loop" smoke 固定（手順書）

## 目的

- return を含む loop を **VM と LLVM harness** の両方で同一結果にする
- integration smoke で意味論を固定し、退行を防止
- quick gate (154/154 PASS) は絶対に壊さない

## 対象 fixture（既存再利用優先）

| Fixture | Pattern | 期待値 | 備考 |
|---------|---------|--------|------|
| `apps/tests/phase286_pattern5_return_min.hako` | Pattern5 (infinite + early return) | exit 7 | 既存（Phase 286 P3.2 で作成）|

**新規 fixture は追加しない**（既存で十分）

## smoke スクリプト

### VM 版
- ファイル: `tools/smokes/v2/profiles/integration/apps/phase284_p2_return_in_loop_vm.sh`
- 実行: `./target/release/hakorune --backend vm <fixture>`
- 判定: exit code または stdout で固定

### LLVM 版
- ファイル: `tools/smokes/v2/profiles/integration/apps/phase284_p2_return_in_loop_llvm.sh`
- 実行: `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm <fixture>`
- SKIP ポリシー: LLVM feature がビルドに含まれていない場合は `test_skip` で理由付き SKIP

```bash
# SKIP 流儀（v2 smoke 既存パターン準拠）
if ! "$NYASH_BIN" --version 2>/dev/null | grep -q "features.*llvm"; then
  test_skip "LLVM backend not available in this build"; exit 0
fi
```

## 受け入れ条件

- [ ] `tools/smokes/v2/run.sh --profile quick` → 154/154 PASS
- [ ] `phase284_p2_return_in_loop_vm.sh` → PASS (exit 7)
- [ ] `phase284_p2_return_in_loop_llvm.sh` → PASS (exit 7) または理由付き SKIP

## 検証コマンド

```bash
# quick gate（必須）
./tools/smokes/v2/run.sh --profile quick

# integration 単発
bash tools/smokes/v2/profiles/integration/apps/phase284_p2_return_in_loop_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase284_p2_return_in_loop_llvm.sh

# integration 全体（フィルタ付き）
./tools/smokes/v2/run.sh --profile integration --filter "*phase284*"
```

## 完了後

1. `docs/development/current/main/phases/phase-284/README.md` の P2 を `✅ COMPLETE` に更新
2. `docs/development/current/main/10-Now.md` の Current Focus を次タスクに更新
