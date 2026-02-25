# Phase 122.5: nyash.toml ConsoleBox.println method_id 修正

## 目的
Phase 122 で ConsoleBox.println を log のエイリアスとして実装した際に、TypeRegistry では slot 400 (log と同じ) にマッピングしたが、nyash.toml では println に method_id = 2 が指定されたままになっている問題を修正する。

## 問題の詳細

### 現在の状態
**src/runtime/type_registry.rs**:
```rust
const CONSOLE_METHODS: &[MethodEntry] = &[
    MethodEntry { name: "log",     arity: 1, slot: 400 },
    MethodEntry { name: "warn",    arity: 1, slot: 401 },
    MethodEntry { name: "error",   arity: 1, slot: 402 },
    MethodEntry { name: "clear",   arity: 0, slot: 403 },
    MethodEntry { name: "println", arity: 1, slot: 400 },  // ← log と同じ slot 400
];
```

**nyash.toml (line 720-722)** ❌ **不一致！**:
```toml
[libraries."libnyash_console_plugin.so".ConsoleBox.methods]
log = { method_id = 1 }
print = { method_id = 1 }
println = { method_id = 2 }  # ❌ エイリアスなら log と同じ ID にすべき
```

### なぜこれが問題か

TypeRegistry（Rust側）では println と log が同じ slot 400 にマッピングされているため、プラグインシステムはこれを同じメソッドとして処理する。しかし nyash.toml では異なる method_id が指定されているため、不整合が発生する可能性がある：

1. **プラグイン呼び出し時**: Rust VM は TypeRegistry から slot 400 を取得
2. **プラグイン実装側**: method_id で println (2) と log (1) を区別する可能性
3. **結果**: プラグインが println に対応していない可能性がある

この不整合を解決する必要がある。

## 修正内容

### 修正対象
**ファイル**: `nyash.toml`
**行**: 722

### 修正内容

```diff
[libraries."libnyash_console_plugin.so".ConsoleBox.methods]
birth = { method_id = 0 }
log = { method_id = 1 }
print = { method_id = 1 }
-println = { method_id = 2 }  # Phase 122: alias for log (uses println internally)
+println = { method_id = 1 }  # Phase 122.5: alias for log (same method_id as log)
```

**変更点**:
- `println` の `method_id` を `2` から `1` に修正
- コメントを更新して意図を明確化

## 検証方法

### 1. 構文確認
```bash
cd /home/tomoaki/git/hakorune-selfhost
# toml パーサー確認
cargo build --release 2>&1 | grep -i toml
```

### 2. 機能確認
修正後、Phase 120 の representative tests を再実行:

```bash
# ConsoleBox.println が正常に動作することを確認
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/peek_expr_block.hako
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/loop_min_while.hako
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/esc_dirname_smoke.hako
```

すべて成功するはず（Phase 122 で既に動作確認済み）。

### 3. 追加検証（推奨）
```bash
# プラグイン側の method_id 処理を確認（libnyash_console_plugin.so）
strings /path/to/libnyash_console_plugin.so | grep -i println
```

## 実装者への注意

- **影響範囲**: nyash.toml のみ（1行修正）
- **ビルド**: 不要（toml は実行時に読み込まれる）
- **テスト**: 既存テストで十分（Phase 120 representative tests が確認）
- **ロールバック**: 簡単（1行戻すだけ）

## 所要時間
**5分程度** - 単純な設定修正

## 完了後の次のステップ

Phase 122.5 修正完了後、以下の Phase 123 (ConsoleBox WASM/非WASM コード統一) に進む。

---

**Phase 122.5 の位置付け**:
- Phase 122 の実装完了後に発見された品質改善の第1段
- 本来は Phase 122 に含めるべきだったが、実装後の品質レビューで発見
- Phase 122 の機能は既に動作済み（この修正は完全性の向上）
Status: Historical
