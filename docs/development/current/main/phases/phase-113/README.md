# Phase 113: if-only partial assign parity

## Background

if-only (no else) で片側だけ代入がある場合、else 側では変数の元の値を「保持」する必要がある。
これは PHI ノードの incoming として、else 側に元の ValueId を渡す「保持 merge」パターン。

## What is Fixed

- `x=1; if flag==1 { x=2 } print(x)` パターン
- flag=0 のとき x は 1 のまま（else 側の暗黙保持）
- flag=1 のとき x は 2 に更新

## Verification

```bash
# VM
bash tools/smokes/v2/profiles/integration/apps/phase113_if_only_partial_assign_vm.sh

# LLVM EXE
bash tools/smokes/v2/profiles/integration/apps/phase113_if_only_partial_assign_llvm_exe.sh
```

Expected output: `1\n2`
