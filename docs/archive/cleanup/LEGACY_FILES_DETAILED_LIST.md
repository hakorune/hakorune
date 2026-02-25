Status: Historical

# レガシーファイル詳細リスト

**調査日**: 2025-11-06
**分類基準**: Safe / Investigate / Risky

---

## 🟢 Safe削除 (即実行可能)

### Cranelift/JIT Backend

#### Runner Modes
| ファイル | 行数 | 理由 |
|---------|-----|------|
| `src/runner/modes/cranelift.rs` | 46 | feature削除済み |
| `src/runner/modes/aot.rs` | 66 | cranelift依存 |
| `src/runner/jit_direct.rs` | ~200 | JIT直接実行 (アーカイブ済み) |

#### Tests
| ファイル | 行数 | 理由 |
|---------|-----|------|
| `src/tests/core13_smoke_jit.rs` | ~100 | #[cfg(feature = "cranelift-jit")] |
| `src/tests/core13_smoke_jit_map.rs` | ~100 | #[cfg(feature = "cranelift-jit")] |

#### Backend Module
| ファイル | 削除箇所 | 理由 |
|---------|---------|------|
| `src/backend/mod.rs` | 29-52行 | cranelift再エクスポート |

#### CLI/Runner
| ファイル | 削除箇所 | 理由 |
|---------|---------|------|
| `src/cli/args.rs` | --backend cranelift | オプション削除 |
| `src/runner/dispatch.rs` | cranelift分岐 | 実行経路削除 |
| `src/runner/modes/mod.rs` | aot module | feature依存 |

---

### BID Copilot Modules (アーカイブ推奨)

#### bid-codegen-from-copilot/ (88KB, 9ファイル)
```
src/bid-codegen-from-copilot/
├── README.md                                    (39行)
├── schema.rs                                    (~300行)
└── codegen/
    ├── mod.rs                                   (~50行)
    ├── generator.rs                             (~500行)
    └── targets/
        ├── mod.rs                               (~30行)
        ├── vm.rs                                (~150行)
        ├── wasm.rs                              (~400行)
        ├── llvm.rs                              (~100行, スタブ)
        ├── python.rs                            (~100行, スタブ)
        └── typescript.rs                        (~100行, スタブ)
```

**移動先**: `archive/bid-copilot-prototype/bid-codegen-from-copilot/`

#### bid-converter-copilot/ (36KB, 4ファイル)
```
src/bid-converter-copilot/
├── README.md                                    (29行)
├── mod.rs                                       (~20行)
├── tlv.rs                                       (~400行)
├── types.rs                                     (~300行)
└── error.rs                                     (~100行)
```

**移動先**: `archive/bid-copilot-prototype/bid-converter-copilot/`

---

### Legacy MIR Builder

| ファイル | 削除対象 | 理由 |
|---------|---------|------|
| `src/mir/builder/exprs_legacy.rs` | 全体 (~150行) | Phase 15でCore-13/14/15に統一済み |
| `src/mir/builder.rs` | `mod exprs_legacy;` 宣言 | モジュール参照削除 |
| `src/mir/builder.rs` | `build_expression_impl_legacy()` 呼び出し | 関数呼び出し削除 |

---

### Dead Code (明確に未使用)

| ファイル | 関数/構造体 | 行数 | 理由 |
|---------|-----------|-----|------|
| `src/runner/json_v1_bridge.rs` | `try_parse_v1_to_module()` | ~700 | #[allow(dead_code)] |
| `src/runner/trace.rs` | `cli_verbose()` | ~10 | #[allow(dead_code)] |
| `src/runner/box_index.rs` | `get_plugin_meta()` | ~20 | #[allow(dead_code)] |
| `src/parser/common.rs` | `unknown_span()` | ~10 | #[allow(dead_code)] |

---

## 🟡 Investigate (調査が必要)

### JSON v1 Bridge

| ファイル | 行数 | 調査項目 |
|---------|-----|---------|
| `src/runner/json_v1_bridge.rs` | 734 | 使用状況確認・JSON v1出力コードの有無 |

**調査コマンド**:
```bash
# JSON v1使用確認
grep -r "json_v1_bridge\|try_parse_v1_to_module" src --include="*.rs"
grep -r "schema_version.*1" apps --include="*.json"

# テスト実行
cargo test json_v1
```

---

### Legacy Test Files

| ファイル | 行数 | 調査項目 |
|---------|-----|---------|
| `src/tests/identical_exec.rs` | ~300 | cranelift依存部分の分離可能性 |
| `src/tests/identical_exec_collections.rs` | 199 | 同上 |
| `src/tests/identical_exec_instance.rs` | 175 | 同上 |
| `src/tests/identical_exec_string.rs` | ~150 | 同上 |
| `src/tests/policy_mutdeny.rs` | 206 | 一部cranelift依存 (分割可能?) |

**調査項目**:
1. cranelift依存部分の特定
2. VM/LLVM比較テストとして有用か?
3. 分割 or 完全削除の判断

---

### Parser Dead Code

| ファイル | 関数 | 調査項目 |
|---------|-----|---------|
| `src/parser/sugar.rs` | 複数関数 | 実使用確認 |
| `src/parser/declarations/*/validators.rs` | バリデーター関数 | 実使用確認 |

**調査コマンド**:
```bash
# 各関数の使用確認
grep -r "関数名" src --include="*.rs" | grep -v "^Binary\|def\|fn "
```

---

## 🔴 Risky (Phase戦略依存)

### WASM Backend

#### ディレクトリ構造
```
src/backend/
├── wasm/                                        (84KB)
│   ├── codegen.rs                               (851行)
│   ├── memory.rs                                (426行)
│   ├── runtime.rs                               (369行)
│   ├── host.rs                                  (~200行)
│   └── mod.rs                                   (~50行)
├── wasm_v2/                                     (16KB)
│   ├── mod.rs                                   (~50行)
│   ├── unified_dispatch.rs                      (~100行)
│   └── vtable_codegen.rs                        (~100行)
└── aot/                                         (40KB)
    ├── compiler.rs                              (~200行)
    ├── config.rs                                (~100行)
    ├── executable.rs                            (301行)
    └── mod.rs                                   (~50行)
```

**総行数**: 約3,170行
**判定**: Phase 21.0完成後に評価

**調査項目**:
1. WASM backendは実際に動作するか?
2. Phase 21.0でWASM対応の計画はあるか?
3. 動作しない場合: アーカイブ推奨

---

### Builtin Box DEPRECATED

| ファイル | DEPRECATEDマーカー | 行数 | Plugin状態 |
|---------|------------------|-----|-----------|
| `src/box_factory/builtin_impls/console_box.rs` | 3箇所 | ~40 | nyash-console-plugin (既存) |
| `src/box_factory/builtin_impls/string_box.rs` | 3箇所 | ~50 | nyash-string-plugin (未?) |
| `src/box_factory/builtin_impls/array_box.rs` | 3箇所 | ~45 | nyash-array-plugin (未?) |
| `src/box_factory/builtin_impls/bool_box.rs` | 3箇所 | ~35 | nyash-bool-plugin (未作成) |
| `src/box_factory/builtin_impls/integer_box.rs` | 3箇所 | ~40 | nyash-integer-plugin (未?) |
| `src/box_factory/builtin_impls/map_box.rs` | 3箇所 | ~54 | nyash-map-plugin (未?) |

**総行数**: 約264行
**判定**: Phase 15.5-B完了後にプラグイン移行戦略確定

**Phase 16計画**:
1. 各Pluginの実装状況確認
2. Builtin→Plugin完全移行
3. Builtinコード削除

---

## 📊 サマリー

### Safe削除 (即実行可能)
| カテゴリ | ファイル数 | 行数 |
|---------|----------|-----|
| Cranelift/JIT | 8ファイル | ~1,500行 |
| BID Copilot | 13ファイル | ~1,900行 |
| Dead Code | 4関数 | ~500行 |
| **合計** | **25+** | **~3,900行** |

### Investigate (調査後削除)
| カテゴリ | ファイル数 | 行数 |
|---------|----------|-----|
| JSON v1 Bridge | 1ファイル | 734行 |
| Legacy Tests | 5ファイル | ~1,000行 |
| Parser Dead Code | 複数 | ~500行 |
| **合計** | **7+** | **~2,200行** |

### Risky (Phase戦略依存)
| カテゴリ | ファイル数 | 行数 |
|---------|----------|-----|
| WASM Backend | 12ファイル | 3,170行 |
| Builtin Box | 6ファイル | 264行 |
| **合計** | **18** | **3,434行** |

---

## 🎯 削除優先順位

### 優先度1: 今すぐ削除
1. ✅ Cranelift/JITファイル (8ファイル, ~1,500行)
2. ✅ BID Copilotアーカイブ (13ファイル, ~1,900行)
3. ✅ 明確なDead Code (4関数, ~500行)

**合計**: 約3,900行

### 優先度2: 1週間以内 (調査後)
1. 🔍 JSON v1 Bridge (1ファイル, 734行)
2. 🔍 Legacy Tests (5ファイル, ~1,000行)
3. 🔍 Parser Dead Code (複数, ~500行)

**合計**: 約2,200行

### 優先度3: Phase 16以降
1. ⏳ WASM Backend評価 (12ファイル, 3,170行)
2. ⏳ Builtin Box移行 (6ファイル, 264行)

**合計**: 約3,434行

---

## 📋 実行チェックリスト

### Phase A: Safe削除
- [ ] Cranelift/JITファイル削除
- [ ] BID Copilotアーカイブ
- [ ] Dead Code削除
- [ ] ビルドテスト成功
- [ ] スモークテスト成功

### Phase B: Investigate
- [ ] JSON v1 Bridge使用状況調査
- [ ] Legacy Tests整理
- [ ] Parser Dead Code確認
- [ ] 削除判断・実行

### Phase C: Risky
- [ ] WASM Backend動作確認
- [ ] Phase 21.0戦略確認
- [ ] Builtin Box移行計画確定
- [ ] 段階的削除実施

---

**最終更新**: 2025-11-06
**調査者**: Claude Code
**次のアクション**: Phase A実行推奨
