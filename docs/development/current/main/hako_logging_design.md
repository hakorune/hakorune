# Phase 104: .hako側ロギング設計ガイド

## 概要

Nyash(.hako)アプリケーションからのロギングを体系的に設計し、Ring0.log（Rust内部）
との役割分担を明確にする。

## Architecture: 3層ロギングの完全図

```
┌─────────────────────────────────────────────────────────┐
│  ユーザーアプリケーション (.hako)                        │
│                                                          │
│  box MyApp {                                            │
│    main() {                                             │
│      me.console.println("Result: OK")  ← ConsoleBox     │
│    }                                                    │
│  }                                                      │
└──────────────────┬──────────────────────────────────────┘
                   │
              ConsoleService
              (user-facing)
                   │
┌──────────────────▼──────────────────────────────────────┐
│  Rust Runtime (Ring0.log)                               │
│                                                          │
│  ring0.log.debug("[joinir] Processing...")      ← internal
│  ring0.log.error("VM error: {}")                ← internal
└──────────────────┬──────────────────────────────────────┘
                   │
                stderr/stdout
                   │
           ┌───────▼────────┐
           │  Terminal      │
           │  (user sees)   │
           └────────────────┘
```

## ロギングの4つのカテゴリ

### 1. User-Facing (ユーザー向け)

**属性**: 常に表示（本番環境対応）
**出力先**: ConsoleBox → stdout
**制御**: フラグなし（常時有効）

**例**:
```nyash
box FileProcessor {
    console: ConsoleBox

    process(filename) {
        me.console = new ConsoleBox()

        if file_not_found(filename) {
            me.console.println("❌ File not found: " + filename)
            return -1
        }

        me.console.println("✅ Processing: " + filename)
        return 0
    }
}
```

### 2. Dev-Debug (開発用デバッグ)

**属性**: 開発時のみ（本番環境では削除推奨）
**出力先**: println!（局所的）
**制御**: bool flag（環境変数or定数）

**例**:
```nyash
box MathBox {
    DEBUG: BoolBox = false  // 環境変数から設定

    calculate(a: Integer, b: Integer) {
        if DEBUG {
            print("[DEBUG] a=" + a + ", b=" + b)
        }
        return a + b
    }
}
```

### 3. Monitoring (運用監視)

**属性**: 定期レポート（本番環境で有効）
**出力先**: ConsoleBox → stdout
**制御**: 定期的（タイマーベース）

**例**:
```nyash
box ServiceMonitor {
    console: ConsoleBox
    requests_count: IntegerBox

    tick() {
        me.console = new ConsoleBox()
        me.console.println("[PERF] Requests/sec: " + me.requests_count)
        me.requests_count = 0
    }
}
```

### 4. Internal Rust (内部実装)

**属性**: Rust実装の内部ログ
**出力先**: Ring0.log → stderr
**制御**: NYASH_RING0_LOG_LEVEL

**例** (.hako側には関係ない、参考):
```rust
// src/mir/builder.rs
ring0.log.debug("[mir] Building function...");
```

## ロギングBox設計パターン

### パターン 1: Lightweight Logger

用途: 単純なユースケース（メッセージのみ）

```nyash
box SimpleLogger {
    console: ConsoleBox

    birth() {
        me.console = new ConsoleBox()
    }

    log(msg) {
        me.console.println(msg)
    }
}

// 使用例
box MyApp {
    main() {
        local logger = new SimpleLogger()
        logger.log("Processing...")
        logger.log("✅ Done")
    }
}
```

### パターン 2: Structured Logger

用途: レベル分け（ERROR/WARN/INFO）

```nyash
box StructuredLogger {
    console: ConsoleBox

    birth() {
        me.console = new ConsoleBox()
    }

    error(msg) {
        me.console.println("[ERROR] " + msg)
    }

    warn(msg) {
        me.console.println("[WARN] " + msg)
    }

    info(msg) {
        me.console.println("[INFO] " + msg)
    }
}

// 使用例
box DatabaseConnector {
    logger: StructuredLogger

    birth() {
        me.logger = new StructuredLogger()
    }

    connect(host) {
        if network_unavailable() {
            me.logger.error("Network unavailable")
            return false
        }

        me.logger.info("Connected to: " + host)
        return true
    }
}
```

### パターン 3: Contextual Logger

用途: コンテキスト情報を含むログ

```nyash
box ContextualLogger {
    console: ConsoleBox
    context: StringBox  // operation_id等

    birth(ctx) {
        me.console = new ConsoleBox()
        me.context = ctx
    }

    log(msg) {
        me.console.println("[" + me.context + "] " + msg)
    }
}

// 使用例
box RequestHandler {
    logger: ContextualLogger

    birth(request_id) {
        me.logger = new ContextualLogger(request_id)
    }

    process() {
        me.logger.log("Request started")
        // ... 処理 ...
        me.logger.log("Request completed")
    }
}
```

## ベストプラクティス

### ✅ DO (推奨)

1. **一元化**: ロギングBoxを1つ所有
   ```nyash
   box MyApp {
       logger: LoggerBox  // ← 一箇所だけ
       main() { ... }
   }
   ```

2. **構造化**: メッセージは構造化
   ```nyash
   logger.log("[OPERATION] " + op_name + " status=" + status)
   ```

3. **条件付き**: デバッグログにフラグ
   ```nyash
   if debug_enabled {
       logger.debug("var=" + var)
   }
   ```

### ❌ DON'T (回避)

1. **散乱**: 複数の ConsoleBox 初期化
   ```nyash
   // ❌ ダメ
   box MyApp {
       main() {
           local c1 = new ConsoleBox()
           local c2 = new ConsoleBox()  // 重複
       }
   }
   ```

2. **混在**: データ処理ロジックとログ混在
   ```nyash
   // ❌ ダメ
   box Processor {
       process(x) {
           print(x)  // ← ロジックに埋め込み
           return x + 1
       }
   }
   ```

3. **過剰**: 全行でログ出力
   ```nyash
   // ❌ ダメ
   calculate(a, b) {
       print("enter")
       local x = a + b
       print("x=" + x)
       print("exit")
       return x
   }
   ```

## ロギング制御フロー

```
┌─────────────────┐
│ .hako app start │
└────────┬────────┘
         │
    ┌────▼──────────────────────────┐
    │ Check NYASH_RING0_LOG_LEVEL env │
    └────┬──────────────────────────┘
         │
    ┌────▼────────────┐      ┌──────────────────┐
    │ DEBUG level?    │─Y─>  │ Enable full logs │
    └────┬────────────┘      └──────────────────┘
         │
         N
    ┌────▼────────────┐      ┌──────────────────┐
    │ INFO level?     │─Y─>  │ Error/Warn only  │
    └────┬────────────┘      └──────────────────┘
         │
         N  (default)
    ┌────▼──────────────┐
    │ User-facing only  │
    │ (ConsoleBox)      │
    └───────────────────┘
```

## チェックリスト: ロギング設計確認

実装時には以下をチェック:

- [ ] ConsoleBox初期化は1回のみ
- [ ] ユーザーメッセージは全て ConsoleBox経由
- [ ] デバッグログに "[DEBUG]" prefix
- [ ] エラーメッセージに "❌" 絵文字
- [ ] 成功メッセージに "✅" 絵文字
- [ ] ロギングLogicを別Boxに分離した
- [ ] テストコード内の println! は許容
- [ ] 本番環境での不要なログを削除した

## Phase 105: Logger Box Framework Integration

Phase 105 で Logger Box フレームワークが導入され、以下のパターンが正式化されました：

- **LightweightLoggerBox**: メッセージのみの単純ロギング
- **StructuredLoggerBox**: DEBUG/INFO/WARN/ERROR レベル付きロギング
- **ContextualLoggerBox**: context（request ID等）付きロギング

詳細な設計と実装例は [Logger Box Design](logger_box_design.md) を参照してください。

### Integration with Phase 104 Categories

Logger Box は以下のカテゴリに対応：

| Phase 104 Category | Logger Box Pattern | Use Case |
|-------------------|-------------------|----------|
| user-facing | Direct ConsoleBox (no Logger Box needed) | 単純なユーザーメッセージ |
| dev-debug | StructuredLogger (DEBUG level) | デバッグ出力 |
| monitoring | StructuredLogger (INFO level) | 監視情報 |
| internal Rust | Ring0.log | Rust側の内部ログ |

## ConsoleBox の使い方（Phase 122 更新）

### 基本パターン

```nyash
local console = new ConsoleBox()
console.println("Hello")  // 内部的には log と同じスロット
console.log("World")      // println と同じ動作
```

### ConsoleBox vs LoggerBox vs ConsoleService

- **ConsoleBox**: ユーザーコードで直接使用（`println` / `log`）
- **LoggerBox**: 構造化ログ・ログレベル管理
- **ConsoleService**: CLI/システム内部での出力（Ring0 経由）

### Phase 122 での統一

**背景**:
- Phase 122 以前: ユーザーコード（.hako）では `println` を使用するが、Rust VM 実装では `log` のみ実装
- 問題: selfhost Stage-3 + JoinIR Strict 経路で `Unknown method 'println'` エラー発生

**解決策**:
- `ConsoleBox.println` を `ConsoleBox.log` の完全なエイリアスとして定義
- VM の TypeRegistry で slot 400 に正規化される
- すべての経路（JSON v0 / selfhost / 通常VM）で一貫性を保つ

**Hako コンパイラでの対応**:

Hako コンパイラ（Nyash で書かれたコンパイラ）は `ConsoleBox.println` をサポートします。

```nyash
// Hako コンパイラ内でのログ出力
local console = new ConsoleBox()
console.println("Compiling: " + filename)  // ✅ 動作する

// JSON v0 経由でも同じ
// Hako → JSON v0 → Rust VM
// "println" → TypeRegistry → slot 400 → log 実装
```

**技術的詳細**:
- Hako コンパイラが生成する JSON v0 に `ConsoleBox.println` が含まれる
- Rust VM の TypeRegistry が `println` を slot 400（`log` と同じ）に解決
- VM が `log` の実装を実行

**利点**:
- Hako コンパイラで直感的な `println` が使用可能
- 他言語（JavaScript, Python など）との一貫性
- selfhost Stage-3 経路での完全動作

**参照**: [Phase 122 詳細ドキュメント](phase122_consolebox_println_unification.md)

---

## 関連ドキュメント

### ConsoleBox について知りたい場合
- [ConsoleBox 完全ガイド](consolebox_complete_guide.md) - 統合的なリファレンス
- [Phase 122-125 実装記録](phase122_consolebox_println_unification.md) - 詳細な実装背景

### ログ出力について知りたい場合
- [ログポリシー](logging_policy.md) - Nyash のログ出力全体のポリシー
- このドキュメント - Hako コンパイラ側のログ設計

### その他の関連ドキュメント
- [logger_box_design.md](logger_box_design.md) - Phase 105 Logger Box フレームワーク
- [ring0-inventory.md](ring0-inventory.md) - println!分類在庫
- [core_optional_design.md](core_optional_design.md) - Optional化設計
- [Core Boxes 設計](core_boxes_design.md) - Core Box の全体設計
