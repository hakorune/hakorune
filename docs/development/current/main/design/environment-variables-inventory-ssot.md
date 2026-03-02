# NYASH_* 環境変数棚卸し SSOT

## 目的

`NYASH_*` 環境変数の「目的不明/未使用/重複」を一覧化し、削除 or 非推奨の判断材料を SSOT に固定する。

## 監査コマンド（SSOT）

```bash
tools/checks/env_dead_accessors_report.sh
```

- `dead`: 実コード/ツール参照ゼロの no-op 候補
- `doc-only`: docs や一部スクリプトのみで観測される候補

## 現状

- **120個以上**の環境変数が存在
- **エイリアス混在**: `NYASH_*` ↔ `HAKO_*` (8組以上)
- **未使用候補**: `NYASH_SYNTAX_SUGAR_LEVEL`、`NYASH_TEST_*` 系
- 2026-03-02 no-op削減: `NYASH_GC_BARRIER_TRACE` / `NYASH_GC_METRICS_JSON` / `NYASH_GC_ALLOC_THRESHOLD` を撤去
- **分散定義**: 13ファイルに実装が分散
- **ドキュメント**: `docs/reference/environment-variables.md` (556行)

---

## エイリアス対応表 (非推奨化対象)

| 非推奨 | 推奨 | 移行計画 | 備考 |
|--------|------|----------|------|
| `HAKO_ENABLE_USING` | `NYASH_ENABLE_USING` | 非推奨化 | using 機能 |
| `HAKO_PARSER_STAGE3` | `NYASH_PARSER_STAGE3` | 非推奨化 | パーサーStage3 |
| `HAKO_JOINIR_DEBUG` | `HAKO_SILENT_TAGS=0` + `HAKO_JOINIR_DEBUG` | 統合検討 | JoinIR デバッグ |
| `NYASH_JOINIR_DEBUG` | `HAKO_JOINIR_DEBUG` | レガシー | 旧名 |
| `HAKO_LLVM_USE_HARNESS` | `NYASH_LLVM_USE_HARNESS` | 非推奨化 | LLVM ハーネス |

---

## 未使用/不明確な変数 (要調査)

| 変数名 | 状態 | アクション | 優先度 |
|--------|------|----------|--------|
| `NYASH_SYNTAX_SUGAR_LEVEL` | 使用箇所不明 | 削除検討 | 高 |
| `NYASH_LEGACY_LOOPBUILDER` | 開発用のみ | ドキュ化 or 削除 | 中 |
| `NYASH_PHI_METRICS` | 統計用 | ドキュ化 | 低 |
| `NYASH_JOINIR_NORMALIZED_DEV_RUN` | 実験的 | ドキュ化 or 削除 | 中 |
| `NYASH_TEST_*` (6個) | テスト用のみ | スコープ限定 | 低 |

---

## ドキュメント不足 (補完必要)

| 変数名 | 不足内容 | 優先度 |
|--------|----------|--------|
| `NYASH_MACRO_DERIVE` | デフォルト値 | 中 |
| `NYASH_BOX_FACTORY_POLICY` | 有効な値の一覧 | 高 |
| `NYASH_FAIL_FAST` | 影響範囲 | 中 |

---

## カテゴリ別一覧

### A. ダンプ/診断系 (21個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_CLI_VERBOSE` | u8 | 0 | CLI verbose level (0-3) | 高 |
| `NYASH_VM_DUMP_MIR` | bool | false | Dump MIR before VM | 中 |
| `NYASH_DUMP_JSON_IR` | bool | false | Dump JSON IR | 低 |
| `NYASH_LEAK_LOG` | bool | false | Enable leak logging | 中 |
| `RUST_MIR_DUMP_PATH` | string | - | RUST MIR dump path | 低 |
| `NYASH_DEBUG_STACK_OVERFLOW` | bool | false | Debug stack overflow | 低 |
| `NYASH_VM_TRACE` | bool | false | VM execution trace | 低 |
| `NYASH_MIR_TEST_DUMP` | bool | false | MIR test dump | 低 |
| `NYASH_TRACE_VARMAP` | bool | false | Trace variable map | 低 |
| `NYASH_DCE_TRACE` | bool | false | DCE trace | 低 |
| `NYASH_OPTION_C_DEBUG` | bool | false | Option C debug | 低 |
| `NYASH_LOOPFORM_DEBUG` | bool | false | LoopForm debug | 低 |
| `NYASH_PRINT_CFG` | bool | false | Print CFG | 低 |
| `NYASH_DUMP_DOMINATOR_TREE` | bool | false | Dump dominator tree | 低 |
| `NYASH_DUMP_DOMINANCE_FRONTIER` | bool | false | Dump dominance frontier | 低 |
| `NYASH_DUMP_LOOPS` | bool | false | Dump loops | 低 |
| `NYASH_STEPS_VERBOSE` | bool | false | Steps verbose | 低 |
| `NYASH_RECIPE_TRACE` | bool | false | Recipe trace | 低 |
| `NYASH_VERIFIER_DEBUG` | bool | false | Verifier debug | 低 |
| `NYASH_DEBUG_FUEL` | usize | 100000 | Debug fuel limit | 低 |
| `HAKO_SILENT_TAGS` | bool | true | Silent tags (falseで全表示) | 中 |

### B. Parser/言語機能系 (8個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_FEATURES` | string | - | Feature flags | 高 |
| `NYASH_PARSER_STAGE3` | bool | false | Enable Stage3 parser | 低 |
| `HAKO_PARSER_STAGE3` | bool | false | (非推奨) Use NYASH_PARSER_STAGE3 | 低 |
| `NYASH_ENABLE_USING` | bool | true | Enable using system | 高 |
| `HAKO_ENABLE_USING` | bool | true | (非推奨) Use NYASH_ENABLE_USING | 低 |
| `NYASH_RESOLVE_TRACE` | bool | false | Trace name resolution | 低 |
| `NYASH_STR_CP` | bool | false | String copy semantics | 低 |
| `NYASH_BLOCK_CATCH` | bool | false | Block catch support | 低 |

### C. JoinIR系 (18個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_JOINIR_EXPERIMENT` | bool | false | JoinIR experiment | 高 |
| `HAKO_JOINIR_IF_SELECT` | bool | false | If-select lowering | 高 |
| `NYASH_JOINIR_VM_BRIDGE` | bool | false | VM bridge for JoinIR | 中 |
| `HAKO_JOINIR_DEBUG` | bool | false | JoinIR debug | 高 |
| `NYASH_JOINIR_DEBUG` | bool | false | (レガシー) Use HAKO_JOINIR_DEBUG | 低 |
| `HAKO_JOINIR_STRICT` | bool | false | JoinIR strict mode | 中 |
| `NYASH_JOINIR_LLVM_EXPERIMENT` | bool | false | JoinIR LLVM experiment | 低 |
| `HAKO_JOINIR_PLANNER_REQUIRED` | bool | false | Planner required mode | 低 |
| `HAKO_JOINIR_CANON_STRICT` | bool | false | Canon strict mode | 低 |
| `HAKO_JOINIR_FACTS_ONLY` | bool | false | Facts only mode | 低 |
| `NYASH_JOINIR_NORMALIZED_DEV_RUN` | bool | false | Normalized dev run (実験的) | 低 |
| `HAKO_JOINIR_PATTERN1` | bool | false | Pattern1 explicit | 低 |
| `HAKO_JOINIR_PATTERN2` | bool | false | Pattern2 explicit | 低 |
| `HAKO_JOINIR_PATTERN3` | bool | false | Pattern3 explicit | 低 |
| `HAKO_JOINIR_PATTERN4` | bool | false | Pattern4 explicit | 低 |
| `HAKO_JOINIR_PATTERN5` | bool | false | Pattern5 explicit | 低 |
| `HAKO_JOINIR_PATTERN8` | bool | false | Pattern8 explicit | 低 |
| `HAKO_JOINIR_PATTERN9` | bool | false | Pattern9 explicit | 低 |

### D. Macro系 (21個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_MACRO_PATHS` | string | - | Macro paths | 中 |
| `NYASH_MACRO_ENABLE` | string | - | Enable macros | 中 |
| `NYASH_MACRO_DISABLE` | string | - | Disable macros | 中 |
| `NYASH_MACRO_TRACE` | bool | false | Macro trace | 低 |
| `NYASH_MACRO_STRICT` | bool | false | Macro strict mode | 中 |
| `NYASH_MACRO_BOX` | bool | false | Macro box | 中 |
| `NYASH_MACRO_CAP_IO` | bool | true | Macro IO capability | 低 |
| `NYASH_MACRO_CAP_NET` | bool | true | Macro network capability | 低 |
| `NYASH_MACRO_CAP_ENV` | bool | true | Macro env capability | 低 |
| `NYASH_MACRO_DERIVE` | bool | ? | Macro derive (デフォルト不明) | 中 |
| `NYASH_MACRO_DERIVE_TRACE` | bool | false | Macro derive trace | 低 |
| `NYASH_MACRO_DERIVE_MINIMAL` | bool | false | Macro derive minimal | 低 |
| `NYASH_MACRO_DERIVE_ONCE` | bool | false | Macro derive once | 低 |
| `NYASH_MACRO_CHECK_OPS` | bool | false | Macro check ops | 低 |
| `NYASH_MACRO_LINT` | bool | false | Macro lint | 低 |
| `NYASH_MACRO_WARN_SHADOW` | bool | false | Macro warn shadow | 低 |
| `NYASH_MACRO_WARN_UNUSED` | bool | false | Macro warn unused | 低 |
| `NYASH_MACRO_WARN_BORROW` | bool | false | Macro warn borrow | 低 |
| `NYASH_MACRO_WARN_LIFETIME` | bool | false | Macro warn lifetime | 低 |
| `NYASH_MACRO_DENY_UNSAFE` | bool | false | Macro deny unsafe | 低 |
| `NYASH_MACRO_DENY_DEPRECATED` | bool | false | Macro deny deprecated | 低 |

### E. Box/Plugin系 (8個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_DISABLE_PLUGINS` | bool | false | Disable plugins | 高 |
| `NYASH_BOX_FACTORY_POLICY` | enum | ? | Box factory policy (値不明) | 中 |
| `NYASH_USE_PLUGIN_BUILTINS` | bool | false | Use plugin builtins | 低 |
| `NYASH_PLUGIN_OVERRIDE_TYPES` | bool | false | Plugin override types | 低 |
| `NYASH_DEBUG_PLUGIN` | bool | false | Debug plugin | 低 |
| `NYASH_DEV_PROVIDER_TRACE` | bool | false | Provider trace | 低 |
| `NYASH_LOAD_NY_PLUGINS` | bool | false | Load ny plugins | 低 |
| `NYASH_PLUGIN_PATH` | string | - | Plugin path | 低 |

### F. Selfhost/Ny Compiler系 (9個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_USE_NY_COMPILER` | bool | false | Use ny compiler | 中 |
| `NYASH_NY_COMPILER_TIMEOUT_MS` | u64 | 30000 | Ny compiler timeout | 中 |
| `NYASH_NY_COMPILER_EMIT_ONLY` | bool | false | Ny compiler emit only | 中 |
| `NYASH_NY_COMPILER_MIN_JSON` | bool | false | Ny compiler min JSON | 低 |
| `NYASH_NY_COMPILER_CHILD_ARGS` | string | - | Ny compiler child args | 低 |
| `NYASH_SELFHOST_EXEC` | bool | false | Selfhost exec | 低 |
| `NYASH_NY_COMPILER_TRACE` | bool | false | Ny compiler trace | 低 |
| `NYASH_NY_COMPILER_DEBUG` | bool | false | Ny compiler debug | 低 |
| `NYASH_NY_COMPILER_VERBOSE` | bool | false | Ny compiler verbose | 低 |

### G. MIR/検証系 (12個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_VERIFY_ALLOW_NO_PHI` | bool | false | Allow no PHI | 中 |
| `NYASH_VERIFY_RET_PURITY` | bool | false | Verify return purity | 低 |
| `NYASH_MIR_DISABLE_OPT` | bool | false | Disable MIR optimization | 低 |
| `NYASH_TRACE_VARMAP` | bool | false | Trace variable map | 低 |
| `NYASH_DCE_TRACE` | bool | false | DCE trace | 低 |
| `NYASH_FAIL_FAST` | bool | true | Fail-fast mode | 高 |
| `NYASH_MIR_UNIFIED_CALL` | bool | false | Unified call | 低 |
| `NYASH_SYNTAX_SUGAR_LEVEL` | ? | ? | (使用箇所不明) | - |
| `NYASH_LEGACY_LOOPBUILDER` | bool | false | Legacy loop builder (開発用) | 低 |
| `NYASH_PHI_METRICS` | bool | false | PHI metrics (統計用) | 低 |
| `NYASH_CHECK_MIR` | bool | false | Check MIR | 低 |
| `NYASH_CHECK_BB` | bool | false | Check basic block | 低 |

### H. GC系 (3個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_GC_MODE` | enum | ? | GC mode | 中 |
| `NYASH_GC_TRACE` | bool | false | GC trace | 低 |
| `NYASH_GC_STRESS` | bool | false | GC stress test | 低 |

### I. LLVM系 (5個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_LLVM_USE_HARNESS` | bool | false | Use LLVM harness | 高 |
| `HAKO_LLVM_USE_HARNESS` | bool | false | (非推奨) Use NYASH_LLVM_USE_HARNESS | 低 |
| `NYASH_LLVM_DUMP_IR` | string | - | Dump LLVM IR path | 低 |
| `NYASH_LLVM_VERIFY` | bool | true | Verify LLVM IR | 低 |
| `NYASH_LLVM_OPT_LEVEL` | string | ? | LLVM optimization level | 低 |

### J. VM系 (3個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_VM_USE_PY` | bool | false | Removed no-op（runtime/pipe では未使用、historical direct route 参照用） | 低 |
| `NYASH_VM_BACKEND` | enum | ? | VM backend | 低 |
| `NYASH_VM_JIT` | bool | false | Enable JIT | 低 |

### K. その他 (10個)

| 変数名 | 型 | デフォルト | 目的 | 使用頻度 |
|--------|------|----------|------|----------|
| `NYASH_ROOT` | string | - | Root directory | 低 |
| `NYASH_FILEBOX_MODE` | enum | ? | FileBox mode | 低 |
| `NYASH_CONFIG_PATH` | string | - | Config path | 低 |
| `NYASH_PROFILE` | bool | false | Enable profiling | 低 |
| `NYASH_BENCHMARK` | bool | false | Enable benchmark | 低 |
| `NYASH_TEST_*` (6個) | - | - | Test framework only | 低 |

---

## 使用箇所のマッピング (主要モジュール)

| モジュール | 主な環境変数 |
|-----------|-------------|
| `src/runner/` | `NYASH_CLI_VERBOSE`, `NYASH_ENABLE_USING`, `NYASH_LLVM_USE_HARNESS` |
| `src/mir/builder/control_flow/joinir/` | `HAKO_JOINIR_*`, `NYASH_JOINIR_*` |
| `src/mir/builder/control_flow/plan/` | `NYASH_VERIFIER_DEBUG`, `NYASH_RECIPE_TRACE` |
| `src/mir/policies/` | `NYASH_FAIL_FAST`, `NYASH_VERIFY_*` |
| `src/macro_system/` | `NYASH_MACRO_*` |
| `src/config/env/` | 全環境変数の定義 |

---

## 移行ガイド

### HAKO_* → NYASH_* 移行

1. `HAKO_ENABLE_USING` → `NYASH_ENABLE_USING`
   ```bash
   # 旧 (非推奨)
   HAKO_ENABLE_USING=1 ./hakorune
   # 新
   NYASH_ENABLE_USING=1 ./hakorune
   ```

2. `HAKO_PARSER_STAGE3` → `NYASH_PARSER_STAGE3`
   ```bash
   # 旧 (非推奨)
   HAKO_PARSER_STAGE3=1 ./hakorune
   # 新
   NYASH_PARSER_STAGE3=1 ./hakorune
   ```

3. `HAKO_LLVM_USE_HARNESS` → `NYASH_LLVM_USE_HARNESS`
   ```bash
   # 旧 (非推奨)
   HAKO_LLVM_USE_HARNESS=1 ./hakorune
   # 新
   NYASH_LLVM_USE_HARNESS=1 ./hakorune
   ```

---

## アクションアイテム

### 優先度高

1. `NYASH_SYNTAX_SUGAR_LEVEL` の使用箇所調査と削除検討
2. `NYASH_BOX_FACTORY_POLICY` の有効値をドキュメントに追加
3. `HAKO_*` エイリアスの非推奨化 (警告ログ追加)

### 優先度中

1. `NYASH_MACRO_DERIVE` のデフォルト値を調査
2. `NYASH_LEGACY_LOOPBUILDER` のドキュ化 or 削除
3. `NYASH_FAIL_FAST` の影響範囲をドキュメント化

### 優先度低

1. `NYASH_PHI_METRICS` のドキュ化
2. `NYASH_TEST_*` 系のスコープ限定
3. 各変数の使用箇所自動生成

---

## 参考ファイル

- ドキュメント: `docs/reference/environment-variables.md`
- 実装: `src/config/env/**` (13モジュール)
- カタログ: `src/config/env/catalog.rs`
