# 🚫 WASM Backend 未実装機能一覧

## 📅 最終更新: 2025-08-15

## 🎯 **概要**
NyashのWASM Backend で現在未実装のMIR命令・機能・Nyash言語機能を体系的にまとめました。

---

## 🔴 **緊急度: 高 - 基本機能**

### 1. **BoxCall命令**
**MIR命令**: `BoxCall { dst, box_val, method, args, effects }`

**未実装メソッド**:
```rust
// 基本Box操作
toString()          // ❌ 文字列変換
equals(other)       // ❌ 等価比較  
clone()            // ❌ オブジェクト複製

// StringBox
length()           // ❌ 文字列長
substring(start, end) // ❌ 部分文字列
indexOf(str)       // ❌ 文字列検索

// IntegerBox/Math
add(other)         // ❌ 加算
subtract(other)    // ❌ 減算
multiply(other)    // ❌ 乗算
divide(other)      // ❌ 除算

// ArrayBox  
push(item)         // ❌ 要素追加
pop()              // ❌ 要素削除
get(index)         // ❌ 要素取得
length()           // ❌ 配列長

// ConsoleBox
log(message)       // ❌ コンソール出力
warn(message)      // ❌ 警告出力
error(message)     // ❌ エラー出力
```

**修正ファイル**: `src/backend/wasm/codegen.rs`

---

### 2. **ExternCall命令**  
**MIR命令**: `ExternCall { dst, extern_name, method, args, effects }`

**未実装機能**:
```rust
// ブラウザーAPI
console_log(msg)    // ❌ JavaScript console.log
canvas_fillRect()   // ❌ Canvas描画
fetch(url)          // ❌ HTTP通信

// システムAPI  
file_read(path)     // ❌ ファイル読み取り
file_write(path, data) // ❌ ファイル書き込み
```

**修正ファイル**: `src/backend/wasm/runtime.rs`

---

## 🟠 **緊急度: 中 - 制御フロー**

### 3. **条件分岐最適化**
**MIR命令**: `Branch`, `Jump`, `Compare`

**問題**:
- ネストした条件分岐のブロック管理
- Switch文相当の最適化
- 短絡評価 (and, or) の効率化

### 4. **ループ最適化**
**MIR命令**: `Loop`, `Phi`

**未実装**:
- ループ内変数の最適化
- 無限ループ検出・対策
- ループアンローリング

---

## 🟡 **緊急度: 低 - 高級機能**

### 5. **メモリ管理高級機能**
**未実装機能**:
```rust
// 弱参照
WeakNew, WeakLoad, WeakCheck    // ❌ 弱参照システム

// メモリ同期
MemCopy, AtomicFence           // ❌ メモリ操作・同期

// ガベージコレクション
// 自動メモリ解放、循環参照検出
```

### 6. **並行処理**
**未実装機能**:
```rust
// 非同期・並行
Send, Recv                     // ❌ メッセージパッシング
Safepoint                      // ❌ GC安全点

// スレッド・タスク
spawn_task()                   // ❌ タスク生成
await_result()                 // ❌ 非同期待機
```

### 7. **例外処理**
**未実装機能**:
```rust
// 例外・エラーハンドリング  
try_catch()                    // ❌ 例外キャッチ
throw_error()                  // ❌ 例外スロー
finally_block()                // ❌ finally実行
```

---

## 📊 **実装優先度マトリクス**

| 機能カテゴリ | 緊急度 | 実装工数 | ユーザー影響 | 優先順位 |
|--------------|--------|----------|--------------|----------|
| **BoxCall基本** | 高 | 中 | 致命的 | **1** |
| **ExternCall** | 高 | 高 | 高 | **2** |
| **条件分岐** | 中 | 低 | 中 | **3** |
| **ループ最適化** | 中 | 中 | 中 | **4** |
| **メモリ管理** | 低 | 高 | 低 | **5** |
| **並行処理** | 低 | 高 | 低 | **6** |
| **例外処理** | 低 | 中 | 低 | **7** |

---

## 🛠️ **実装戦略**

### Phase 1: BoxCall基本実装 (1週間)
```rust
// 目標: 基本的なNyashプログラムがWASMで動作
impl WasmCodegen {
    fn generate_box_call(&mut self, dst: Option<ValueId>, box_val: ValueId, 
                        method: &str, args: Vec<ValueId>) -> Result<(), WasmError> {
        match method {
            "toString" => self.generate_to_string_call(dst, box_val),
            "equals" => self.generate_equals_call(dst, box_val, args),
            "length" => self.generate_length_call(dst, box_val),
            // ... 基本メソッド追加
            _ => Err(WasmError::UnsupportedInstruction(format!("Unknown method: {}", method)))
        }
    }
}
```

### Phase 2: ExternCall統合 (2週間)
```rust
// 目標: ブラウザーとの連携動作
impl RuntimeImports {
    fn register_browser_apis(&mut self) {
        self.register("console_log", console_log_impl);
        self.register("canvas_fillRect", canvas_fill_rect_impl);
        // ... ブラウザーAPI追加
    }
}
```

### Phase 3: 最適化・高級機能 (1ヶ月)
- 制御フロー最適化
- メモリ管理改善
- パフォーマンス向上

---

## 📋 **テストケース**

### Level 1: 基本BoxCall
```nyash
# test_basic_boxcall.hako
local str = "Hello"
local len = str.length()         # BoxCall実装必要
print("Length: " + len.toString()) # BoxCall + ExternCall
```

### Level 2: Box操作
```nyash  
# test_box_operations.hako
local arr = new ArrayBox()
arr.push("item1")               # BoxCall実装必要
local item = arr.get(0)         # BoxCall実装必要
print(item.toString())          # BoxCall実装必要
```

### Level 3: 外部連携
```nyash
# test_extern_integration.hako
local console = new ExternBox("console")
console.call("log", "Hello Browser!")  # ExternCall実装必要
```

---

## ✅ **実装完了判定基準**

### 基本機能復旧
```bash
# 以下が全て成功すること
./target/release/hakorune --compile-wasm test_basic_boxcall.hako
./target/release/hakorune --compile-wasm test_box_operations.hako  
./target/release/hakorune --compile-wasm test_extern_integration.hako

# WASM実行成功
wasmtime test_basic_boxcall.wasm
wasmtime test_box_operations.wasm
wasmtime test_extern_integration.wasm
```

### パフォーマンス基準
- コンパイル時間: インタープリター比 2倍以内
- 実行速度: インタープリター比 5倍以上高速
- メモリ使用量: 合理的範囲内

---

**🎯 目標**: Phase 1完了でNyash WASM基本機能が実用レベルに到達し、Phase 2でブラウザー連携が完全動作する状態を実現