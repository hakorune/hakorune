# Phase 9.51: WASM Jump/Branch実装とHTTPサーバー実用化 🚀

**優先度**: 🔴 **最高（実用性ブロッカー）**  
**期間**: 1週間  
**前提**: Phase 9 (PR #67) マージ済み

## 🎯 概要

Phase 9で実装されたWASM/AOTとHTTPサーバー機能に重大な制約があり、実用化を阻害しています。本issueではこれらを修正し、真の実用レベルに到達させます。

## 🔍 現在の問題

### 1. **WASM/AOT コンパイルエラー（最重要）**
```bash
# 現象
$ ./target/release/nyash --compile-wasm test_simple_loop.hako
❌ WASM compilation error: Unsupported instruction: Jump { target: BasicBlockId(1) }
```

**原因**: `src/backend/wasm/codegen.rs`にJump/Branch命令が未実装
**影響**: **ループ・条件分岐を含む全プログラムがWASM/AOT化不可**

### 2. **HTTPServerBox listen()常に失敗**
```nyash
// 現象
server.bind("127.0.0.1", 8080)  // ✅ true
server.listen(10)                // ❌ always false
```

**原因**: `src/boxes/socket_box.rs`のlisten()実装が不完全
**影響**: HTTPサーバーが実際には動作しない

### 3. **エラーハンドリング脆弱性**
```bash
$ grep -n "unwrap()" src/boxes/http_server_box.rs | wc -l
26
```

**原因**: 26箇所のunwrap()使用
**影響**: 本番環境でパニック多発の可能性

## 📋 実装タスク

### Task 1: WASM Jump/Branch命令実装（2日）

**ファイル**: `src/backend/wasm/codegen.rs`

```rust
// 追加実装箇所（358行目付近）
MirInstruction::Jump { target } => {
    // 無条件ジャンプ
    // WASMのbr命令を使用
    // ブロックスタック管理が必要
    Ok(vec![
        format!("br ${}", self.get_block_depth(target)?),
    ])
},

MirInstruction::Branch { cond, then_block, else_block } => {
    // 条件分岐
    // WASMのbr_if命令を使用
    self.emit_value_load(cond)?;
    Ok(vec![
        "i32.eqz".to_string(),
        format!("br_if ${}", self.get_block_depth(else_block)?),
        format!("br ${}", self.get_block_depth(then_block)?),
    ])
},
```

**必要な補助実装**:
- ブロック深度管理（`get_block_depth`メソッド）
- ループ構造のblock/loop/end生成
- Phi命令の簡易実装（変数コピーで対応）

### Task 2: SocketBox listen()修正（1日）

**ファイル**: `src/boxes/socket_box.rs`

```rust
pub fn listen(&self, backlog: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
    let backlog_num = backlog.to_string_box().value.parse::<i32>().unwrap_or(128);
    
    // 実際にlisten状態を管理
    if let Some(ref listener) = *self.listener.lock().unwrap() {
        // TcpListenerは既にlisten状態
        // 内部状態を更新
        *self.status.lock().unwrap() = SocketStatus::Listening;
        Box::new(BoolBox::new(true))
    } else {
        Box::new(BoolBox::new(false))
    }
}
```

### Task 3: エラーハンドリング改善（2日）

**対象ファイル**: 
- `src/boxes/http_server_box.rs`
- `src/boxes/socket_box.rs`
- `src/boxes/http_message_box.rs`

**変更例**:
```rust
// Before
let listener = self.listener.lock().unwrap();

// After
let listener = match self.listener.lock() {
    Ok(l) => l,
    Err(_) => return Box::new(StringBox::new("Error: Failed to acquire lock")),
};
```

### Task 4: HTTPサーバー実用化（2日）

**ファイル**: `src/boxes/http_server_box.rs`

1. **スレッドプール実装**
```rust
use std::sync::mpsc;
use std::thread::JoinHandle;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        // 固定サイズのワーカープール
    }
}
```

2. **適切なシャットダウン**
```rust
pub fn stop(&self) -> Box<dyn NyashBox> {
    *self.running.lock().unwrap() = false;
    // グレースフルシャットダウン
    // 全コネクションの終了待機
}
```

### Task 5: テストケース追加（1日）

**新規テストファイル**:

1. `test_wasm_loop.hako`
```nyash
// WASMループテスト
local sum, i
sum = 0
i = 0
loop (i < 10) {
    sum = sum + i
    i = i + 1
}
print("Sum: " + sum)  // Expected: 45
```

2. `test_http_server_real.hako`
```nyash
// 実用HTTPサーバーテスト
static box Main {
    main() {
        local server = new HTTPServerBox()
        
        // ルート設定
        server.route("/", "home")
        server.route("/api/health", "health")
        
        // サーバー起動
        if (server.bind("0.0.0.0", 8080)) {
            if (server.listen(10)) {
                print("Server started on http://0.0.0.0:8080")
                server.start()
            }
        }
    }
    
    home(req) {
        return "<h1>Nyash Server Running!</h1>"
    }
    
    health(req) {
        return "{\"status\":\"healthy\"}"
    }
}
```

## 🎯 完了条件

1. **WASM/AOT成功**
   ```bash
   $ ./target/release/nyash --compile-wasm test_wasm_loop.hako
   ✅ WASM compilation completed successfully!
   
   $ ./target/release/nyash --benchmark --iterations 100
   WASM: XX.XXms (13.5x faster than interpreter) ← 目標達成
   ```

2. **HTTPサーバー実動作**
   ```bash
   $ ./target/release/nyash test_http_server_real.hako &
   Server started on http://0.0.0.0:8080
   
   $ curl http://localhost:8080/
   <h1>Nyash Server Running!</h1>
   
   $ curl http://localhost:8080/api/health
   {"status":"healthy"}
   ```

3. **エラーハンドリング**
   - unwrap()使用箇所: 26 → 5以下
   - パニックフリーな実行

## 📊 性能目標

- **WASM実行**: 現在11.5倍 → **13.5倍以上**
- **HTTPサーバー**: 100 req/sec以上
- **起動時間**: 50ms以下

## 🔧 実装のヒント

### WASMブロック管理
```rust
struct WasmCodeGen {
    // 既存フィールド
    block_stack: Vec<BlockInfo>,  // 追加
}

struct BlockInfo {
    block_type: BlockType,  // Loop, Block, If
    label: String,
    depth: usize,
}
```

### デバッグ用出力
```rust
// MIR → WASM変換時のデバッグ
if self.debug {
    println!("MIR: {:?} -> WASM: {:?}", instruction, wasm_code);
}
```

## 📝 参考資料

- [WebAssembly Specification - Control Instructions](https://webassembly.github.io/spec/core/syntax/instructions.html#control-instructions)
- [wasmtime compile documentation](https://docs.wasmtime.dev/cli-compile.html)
- Rust std::thread::ThreadPool実装例

## 🎉 期待される成果

Phase 9.51完了により、Nyashは：
- **実用的なWebアプリケーション開発**が可能に
- **高速なAOT実行ファイル配布**が実現
- **本番環境での安定動作**を保証

Everything is Box哲学を守りながら、実用性を達成します！🐱

---
**担当**: Copilot様  
**レビュー**: Claude様  
**作成日**: 2025-08-14