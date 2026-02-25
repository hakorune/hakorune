# Phase 118: Loop + If-Only Merge Parity (VM + LLVM EXE)

## 目的
Phase 117で if-only nested-if + call merge が固まったので、Phase 118では "loop + if-only + merge" を固定する。loop側のPHI生成と if-only条件付き更新を組み合わせたパターンを確立し、VM + LLVM EXE の両方で動作保証する。

## 背景
- **Phase 117**: if-only nested-if + call merge parity 確立（`phase117_if_only_nested_if_call_merge_min.hako`）
- **Phase 118**: loop + if-only merge の組み合わせテスト
  - ループ内で if-only により条件付き変数更新
  - Pattern3 (if-sum) の活用（既存実装を利用）
  - ループ継続条件での PHI 生成
  - Exit での merge 処理

## テストケース

### Fixture
**ファイル**: `apps/tests/phase118_loop_nested_if_merge_min.hako`

```hako
// Phase 118: loop + if-only merge parity test
// Expected output: 2 (numeric line)
// Calculation: i=0: x=0 (skip), i=1: x=0+1→1, i=2: x=1+1→2

static box Main {
    main() {
        local i = 0
        local x = 0
        loop(i < 3) {
            if i > 0 {
                x = x + 1
            }
            i = i + 1
        }
        print(x)
        return "OK"
    }
}
```

**期待出力**: `2`（数値1行）

**計算ロジック**:
- `i=0`: `i > 0` は false、x は 0 のまま（skip）
- `i=1`: `i > 0` は true、`x = 0 + 1 → 1`（if ブランチ）
- `i=2`: `i > 0` は true、`x = 1 + 1 → 2`（if ブランチ）
- ループ終了後に `x` の値 `2` を出力

**Pattern3 活用**: このパターンは Pattern3 (if-sum) で既に対応済み。if-only で条件付き加算を行うパターンとして、既存実装を活用。

### スモークテスト

#### VM Smoke Test
**ファイル**: `tools/smokes/v2/profiles/integration/apps/phase118_loop_nested_if_merge_vm.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../lib/output_validator.sh"

FIXTURE="apps/tests/phase118_loop_nested_if_merge_min.hako"

echo "[phase118_loop_nested_if_merge_vm] Testing loop + if-else merge parity (VM)..."

# VM execution with STRICT mode
OUTPUT=$(NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend vm "$FIXTURE" 2>&1) || {
    echo "❌ VM execution failed"
    echo "$OUTPUT"
    exit 1
}

# Validate: expect 1 line with value 2
validate_numeric_output 1 "2" "$OUTPUT"

echo "✅ [phase118_loop_nested_if_merge_vm] PASS"
```

**実行条件**:
- `NYASH_DISABLE_PLUGINS=1`: プラグイン無効（core経路のみ）
- `HAKO_JOINIR_STRICT=1`: JoinIR STRICT モード有効
- `--backend vm`: Rust VM バックエンド

**検証**: `validate_numeric_output 1 "2" "$OUTPUT"`（1行、値2）

#### LLVM EXE Smoke Test
**ファイル**: `tools/smokes/v2/profiles/integration/apps/phase118_loop_nested_if_merge_llvm_exe.sh`

```bash
#!/bin/bash
# Phase 118: loop + if-else merge parity (LLVM EXE)

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/llvm_exe_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

llvm_exe_preflight_or_skip || exit 0

# Phase 97/98/100 SSOT: plugin dlopen check → build only if needed → dlopen recheck.
FILEBOX_SO="$NYASH_ROOT/plugins/nyash-filebox-plugin/libnyash_filebox_plugin.so"
MAPBOX_SO="$NYASH_ROOT/plugins/nyash-map-plugin/libnyash_map_plugin.so"
STRINGBOX_SO="$NYASH_ROOT/plugins/nyash-string-plugin/libnyash_string_plugin.so"
CONSOLEBOX_SO="$NYASH_ROOT/plugins/nyash-console-plugin/libnyash_console_plugin.so"
INTEGERBOX_SO="$NYASH_ROOT/plugins/nyash-integer-plugin/libnyash_integer_plugin.so"

LLVM_REQUIRED_PLUGINS=(
  "FileBox|$FILEBOX_SO|nyash-filebox-plugin"
  "MapBox|$MAPBOX_SO|nyash-map-plugin"
  "StringBox|$STRINGBOX_SO|nyash-string-plugin"
  "ConsoleBox|$CONSOLEBOX_SO|nyash-console-plugin"
  "IntegerBox|$INTEGERBOX_SO|nyash-integer-plugin"
)
LLVM_PLUGIN_BUILD_LOG="/tmp/phase118_loop_nested_if_merge_plugin_build.log"
llvm_exe_ensure_plugins_or_fail || exit 1

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase118_loop_nested_if_merge_min.hako"
OUTPUT_EXE="$NYASH_ROOT/tmp/phase118_loop_nested_if_merge_llvm_exe"

EXPECTED='2'
EXPECTED_LINES=1
LLVM_BUILD_LOG="/tmp/phase118_loop_nested_if_merge_build.log"
if llvm_exe_build_and_run_numeric_smoke; then
  test_pass "phase118_loop_nested_if_merge_llvm_exe: output matches expected (2)"
else
  exit 1
fi
```

**必要プラグイン**: FileBox, MapBox, StringBox, ConsoleBox, IntegerBox（Phase 117と同じセット）

**検証**: `llvm_exe_build_and_run_numeric_smoke`（EXPECTED='2', EXPECTED_LINES=1）

## 検証コマンド

### VM Smoke Test
```bash
bash tools/smokes/v2/profiles/integration/apps/phase118_loop_nested_if_merge_vm.sh
```

### LLVM EXE Smoke Test
```bash
bash tools/smokes/v2/profiles/integration/apps/phase118_loop_nested_if_merge_llvm_exe.sh
```

### 回帰テスト（Phase 117）
```bash
bash tools/smokes/v2/profiles/integration/apps/phase117_if_only_nested_if_call_merge_llvm_exe.sh
```

## 成果物
1. **Fixture**: `apps/tests/phase118_loop_nested_if_merge_min.hako`
2. **VM Smoke**: `tools/smokes/v2/profiles/integration/apps/phase118_loop_nested_if_merge_vm.sh`
3. **LLVM EXE Smoke**: `tools/smokes/v2/profiles/integration/apps/phase118_loop_nested_if_merge_llvm_exe.sh`
4. **ドキュメント**: `docs/development/current/main/phases/phase-118/README.md`（本ファイル）

## 追加（Phase 118 follow-up）：Pattern3 carrier PHI contract 固定

Pattern3（if-sum）の exit carrier PHI が欠けると、後段で `Carrier '<name>' not found in carrier_phis` が発生する。
このフェーズでは fixture + VM/LLVM EXE smoke で再現と回帰固定を行い、さらに Fail-Fast の契約チェックを追加した。

### Fixture
- `apps/tests/phase118_pattern3_if_sum_min.hako`（期待: `12`）

### Smoke
- VM: `tools/smokes/v2/profiles/integration/apps/phase118_pattern3_if_sum_vm.sh`
- LLVM EXE: `tools/smokes/v2/profiles/integration/apps/phase118_pattern3_if_sum_llvm_exe.sh`

### 契約（SSOT）
- `exit_bindings` に含まれる `LoopState` carrier は、必ず exit PHI（`exit_carrier_phis`）を持つこと。
- 違反時は `[joinir/phase118/exit_phi/missing_carrier_phi] ... Hint: ...` で Fail-Fast。

## Phase 118 完了条件
- ✅ VM Smoke Test PASS
- ✅ LLVM EXE Smoke Test PASS
- ✅ Phase 117 回帰テスト PASS
- ✅ ドキュメント更新（10-Now.md, 01-JoinIR-Selfhost-INDEX.md）

## 次のステップ
Phase 119以降で更に複雑なループ + 制御フロー構造のテストケースを追加予定。
