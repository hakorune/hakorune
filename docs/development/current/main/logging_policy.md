# Logging & Output Policy (Phase 99)

## Overview

This document establishes the clear separation of concerns between three logging/output layers in the Nyash runtime, and provides guidelines for transitioning the remaining ~1477 println!/eprintln! call sites to the appropriate mechanism.

**Status**: Phase 101-B in progress — documentation plus partial internal log migration (Ring0.log) and test-output policy fixed.

---

## Section 1: Three-Layer Log/Output Role Separation

The Nyash runtime uses three distinct layers for logging and output, each with a specific purpose:

### Layer 1: Ring0.log (OS API Layer)

**Purpose**: Runtime/OS layer internal logging

**Use Cases**:
- Memory management state
- Garbage collection events
- Thread management
- Cache information
- Internal state tracking

**Target Audience**: Developers, debugging, measurement, internal state tracking

**API**:
```rust
ring0.log.debug("message");
ring0.log.info("message");
ring0.log.warn("message");
ring0.log.error("message");
```

**Characteristics**:
- Not intended for direct user visibility
- Controlled by log levels
- Can be redirected to files or monitoring systems
- Developer-facing diagnostics

---

### Layer 2: ConsoleService (Box Layer - User-Facing)

**Purpose**: Direct CLI output for end users

**Use Cases**:
- Error messages displayed to users
- Success notifications
- Help text
- Progress information
- Command execution results

**Target Audience**: End users of the Nyash CLI

**Access Methods**:
1. Via `console_println!` macro (Rust code)
2. Via `host.core.console.println(...)` (Box code)

**API**:
```rust
// Rust side
console_println!("User-visible message");

// Box side
host.core.console.println("User-visible message");
```

**Characteristics**:
- Active after PluginHost initialization
- Falls back to `eprintln!` before initialization (Graceful Degradation)
- User-friendly messages
- Respects output redirection

---

### Layer 3: Raw println!/eprintln! (Restricted Use)

**Purpose**: Limited to tests and temporary debugging only

**Restrictions**:
- Should be removed from production paths
- Should be removed from selfhost/hack_check/VM runner paths
- Permitted in test code (isolated environment)

**Current Status**:
- Test code: ~299 instances (permitted)
- Production code: ~1477 instances (to be migrated)

**Phase 99 Stance**:
- Judgment only in Phase 99
- Implementation deferred to Phase 100+
- Test code println! remains as-is (safe in isolated environment)

---

## Section 2: Macro Policy

### console_println! Macro (Implemented in Phase 98)

**Purpose**: Safe entry point for user-facing messages

**Implementation**:
```rust
macro_rules! console_println {
    ($($arg:tt)*) => {{
        if let Some(host) = try_get_core_plugin_host() {
            host.core.console.println(&format!($($arg)*));
        } else {
            eprintln!($($arg)*);  // Graceful fallback
        }
    }};
}
```

**Usage Locations**:
- selfhost runner main output
- hack_check result display
- VM runner main output (RC, errors, etc.)

**Design Principle**:
- Follows Fail-Fast principle with exception for output destination selection
- Dynamic fallback is permitted for "where to output" decision only
- Core logic remains static and deterministic

---

### dev_eprintln! / Other dev_* Macros (Under Consideration)

**Purpose**: Temporary debug output during development

**Phase 99 Decision**:
- Evaluate necessity only
- No implementation in Phase 99
- Implementation deferred to Phase 100+

**Rationale**:
- Most use cases can be covered by Ring0.log or console_println!
- May be redundant with existing logging infrastructure
- Need to assess actual developer needs before implementing

---

## Section 3: Test Code println! Policy

### Current Status

- Test code: ~299 instances of println!/eprintln!
- Purpose: Test output and result visualization
- Location: Throughout test modules

### Policy

**Phase 99 Stance**: **Leave as-is - OK to use println! in tests**

**Rationale**:
1. Tests run in isolated environments
2. Test output is separate from production output
3. println! is safe and appropriate for test diagnostics
4. No need for special test macros (println_test!, etc.)

**Future Considerations**: No changes planned

---

## Section 4: Complete Integration Criteria

The "CoreServices Log/Output Complete Integration" is considered achieved when:

### Console (Required)

- ✅ All selfhost runner user-facing output uses console_println!
- ✅ All hack_check result display uses console_println!
- ✅ All VM runner main output (RC, errors) uses console_println!

**Current Status (Phase 98)**:
- ✅ 7 locations completed
- ✅ Representative paths covered
- 🔄 ~359 locations remaining (Phase 100-101)

---

### Ring0.log (Phased)

**Existing Infrastructure**:
- debug/info/warn/error API available
- StdLog implementation outputs to stdout/stderr

**Migration Priority**:
1. Internal errors → ring0.log.error(...)
2. VM execution logs → ring0.log.info(...)
3. Memory/GC information → ring0.log.debug(...)

**Phase 99 Scope**: Planning only - document migration strategy

---

### Tests (Permitted)

- Test code println! (~299 locations): **Keep as-is**
- Production paths: Removed (migration complete)

---

## Section 5: Migration Strategy

### Phase-by-Phase Approach

**Phase 99 (Current)**:
- Document policy and categorization
- No code changes
- Establish migration framework

**Phase 100-101**:
- Migrate user-facing println! to console_println! (~366 locations)
- Prioritize src/runner/ paths
- Focus on visible user messages

**Phase 102+**:
- Evaluate Ring0.log usage for internal logging
- Migrate dev-debug println! as needed
- Leave test println! unchanged

---

### Categorization of Remaining println!/eprintln!

Total: 1776 locations (1477 excluding tests)

**Categories**:

1. **user-facing** (~366 locations, priority: high)
   - CLI messages, errors, help text
   - Target: console_println!
   - Phase 98: 7 completed (representative paths)
   - Phase 100-101: Gradual expansion

2. **dev-debug** (TBD, Ring0.log candidates)
   - Temporary debug output
   - Target: Ring0.log or dev_* macros (to be decided)
   - Priority: Medium
   - Phase 99: Assessment only

3. **test** (~299 locations, priority: low)
   - Test output and verification
   - Target: Keep as-is (isolated environment)
   - Phase 99-101: No changes

4. **internal** (~812 locations remaining)
   - Internal processing println! remnants
   - Target: TBD in Phase 99
   - Phase 99: Uncategorized

---

## Section 6: Design Principles

### Graceful Degradation

The console_println! macro implements graceful degradation:
- Primary: Use ConsoleService when available
- Fallback: Use eprintln! before PluginHost initialization
- Rationale: Output should always work, even during early initialization

This is the **only permitted exception** to the Fail-Fast principle, as it only affects output destination, not core logic.

---

### Separation of Concerns

Each layer has a clear responsibility:
- **Ring0.log**: Developer diagnostics
- **ConsoleService**: User communication
- **println!**: Test-only convenience

This separation ensures:
- Maintainability: Clear ownership of output
- Testability: Test output isolated from production
- Flexibility: Each layer can be configured independently

---

### Migration Philosophy

**80/20 Rule Applied**:
- Phase 98: 7 locations = covering representative paths
- Phase 100-101: ~366 locations = user-visible improvements
- Phase 102+: Remaining locations = diminishing returns

Focus on high-impact migrations first, defer low-priority changes.

---

## Section 4: .hako側ロギング方針 (Phase 104)

### 概要

Nyash（.hako）アプリケーションからのロギングは、ConsoleBoxを通じてユーザーに伝える方式に統一。

**Note**: Phase 105では Logger Box フレームワークが正式化され、より構造化された
ロギングが可能になりました。詳細は [Logger Box Design](logger_box_design.md) を参照。

### パターン分類

#### Pattern 1: ユーザー向けメッセージ（user-facing）

**用途**: ユーザーに表示される情報
- エラーメッセージ
- 進捗情報
- 成功/完了通知

**実装方法**:
```nyash
box MyApp {
    console: ConsoleBox

    main() {
        me.console = new ConsoleBox()

        // ✅ 正しい方法: ConsoleBox経由
        me.console.println("Processing started...")

        // 実装例: エラーハンドリング
        if error_occurred {
            me.console.println("❌ Error: " + error_msg)
            return 1
        }

        me.console.println("✅ Success!")
        return 0
    }
}
```

**ガイドライン**:
- すべてのユーザー向けメッセージは ConsoleBox.println() 使用
- エラーメッセージは "❌" 絵文字で開始
- 成功メッセージは "✅" 絵文字で開始
- 進捗は "[1/10]" などの形式で表示

#### Pattern 2: デバッグ/診断ログ（dev-debug）

**用途**: 開発時のデバッグ情報
- 変数値のダンプ
- 関数entry/exit
- 分岐の選択記録

**実装方法** (Phase 104では ユーザーアプリ側は println! 許容):
```nyash
box MyApp {
    main() {
        // 現在: ユーザーアプリが debug print 時は println! OK
        // 将来 (Phase 105): ユーザー定義ロギングフレームワーク検討

        local debug_enabled = false  // 環境変数から読む
        if debug_enabled {
            print("[DEBUG] Processing item: " + item_id)
        }
    }
}
```

**ガイドライン**:
- デバッグ出力は "[DEBUG]" prefix で統一
- bool flag で on/off 制御（環境変数参照）
- 本番環境では全削除推奨

#### Pattern 3: 監視/メトリクス（monitoring）

**用途**: 運用時の監視情報
- 実行時間計測
- メモリ使用量
- 処理レート

**実装方法**:
```nyash
box PerformanceMonitor {
    console: ConsoleBox
    start_time: IntegerBox

    begin() {
        me.console = new ConsoleBox()
        me.start_time = runtime.now()
    }

    report() {
        local elapsed = runtime.now() - me.start_time
        me.console.println("[PERF] Elapsed: " + elapsed + "ms")
    }
}
```

**ガイドライン**:
- メトリクスは "[PERF]" prefix で統一
- 定期レポーティング推奨
- Ring0.log（Rust側）と連携可能（将来拡張）

### 設計原則（.hako側視点）

1. **Single Responsibility**: 各Boxは1つのロギング目的に専念
2. **Testability**: ログ出力をテスト可能にする（interface化）
3. **Composability**: ロギングBoxを組み合わせる
4. **Zero Config Default**: 環境変数なしでもロギング動作

### アンチパターン（避けるべき書き方）

```nyash
// ❌ ダメな例 1: raw println!を散乱
print("value: " + x)  // → Boxにロギング機能を出す

// ❌ ダメな例 2: ConsoleBoxの多重初期化
box MyBox {
    main() {
        local console = new ConsoleBox()
        console.println("msg1")

        local console2 = new ConsoleBox()  // ← 重複初期化
        console2.println("msg2")
    }
}

// ❌ ダメな例 3: ロギングロジック混在
box DataProcessor {
    process(data) {
        if validate(data) {
            print("OK")  // ← 出力ロジックが混在
            return process_data(data)
        }
    }
}

// ✅ 正しい例: 責務分離
box DataProcessor {
    logger: LoggerBox

    process(data) {
        if validate(data) {
            return process_data(data)
        } else {
            me.logger.error("Validation failed")
            return null
        }
    }
}
```

### 今後の拡張予定

**Phase 105**: ユーザー定義ロギングフレームワーク
- Logger Box interface化
- ログレベル統一（DEBUG/INFO/WARN/ERROR）
- Structured logging (JSON形式)

**Phase 106**: ログ出力先の切り替え
- ConsoleBox → FileBox
- ConsoleBox → NetworkBox
- Pluggable logger architecture

### テストでのロギング

テストコード内のロギング/println!は許容:
```nyash
// ✅ テストコード内
test my_test {
    local result = my_function()
    print("Result: " + result)  // テスト内 OK
    assert(result == expected)
}
```

**理由**: テストは隔離環境実行のため、ログ管理の必要がない

---

## Phase 122: ConsoleBox.println / log の統一

### 使い分けガイドライン

| 用途 | 推奨 API | 理由 |
|------|---------|------|
| **selfhost / CLI** | `ConsoleService` / `console_println!` | Ring0 経由で安定 |
| **ユーザーコード** | `ConsoleBox.println` | ユーザー向け sugar |
| **内部実装** | `ConsoleBox.log` | VM レベルでは同じ |

### 正規化ルール

- `ConsoleBox.println` は VM の TypeRegistry で `ConsoleBox.log`（slot 400）に正規化される
- JSON v0 / selfhost / 通常VM のすべての経路で同じ動作を保証
- Rust から直接使用する場合も `println` / `log` の両方が使用可能

---

## 関連ドキュメント

- [Logger Box Design](logger_box_design.md) - Phase 105 structured logging framework
- [.hako側ロギング設計](hako_logging_design.md) - Phase 104 ユーザーアプリ視点
- [ring0-inventory.md](ring0-inventory.md) - Ring0.log利用状況の在庫管理
- [core_optional_design.md](core_optional_design.md) - CoreServices optional化
- [CoreBoxes Design - Section 15.6-A](core_boxes_design.md#section-156-a-logsoutput-unified-design) - Architectural context
- [Phase 85 CURRENT_TASK](../../../CURRENT_TASK.md) - Implementation timeline

---

## Summary

Phase 99 establishes the **documentation foundation** for future logging/output migrations:

1. **Clear role separation**: Ring0.log (internal) vs ConsoleService (user-facing) vs println! (test-only)
2. **Macro policy**: console_println! (implemented), dev_* macros (under consideration)
3. **Test policy**: println! in tests is OK (isolated environment)
4. **Migration strategy**: Phased approach with clear priorities

**Next Steps**: Phase 100+ will implement gradual migrations based on this policy framework.

---

## Section 7: Phase 100 Implementation Complete (2025-12-03)

### user-facing 出力の CoreServices 化完了

**実装概要**: selfhost と LLVM runner の主要なユーザー向け出力を ConsoleService (console_println!) 経由に統一

**完了箇所**:
- **selfhost.rs**: 6箇所 → console_println!
  - Line 57: CoreInitError 出力
  - Line 194, 363, 418, 519, 570: Result 出力
- **llvm.rs**: 23箇所（ユーザー向けメッセージ） → console_println!
  - Line 26, 44, 53, 60, 116: エラーメッセージ（❌）
  - Line 121-122: 成功メッセージ（📊）
  - Line 215, 230, 239, 275, 287, 295: LLVM/harness エラー
  - Line 324, 328, 334-335, 353-354, 357-358, 362: 実行結果
  - Line 369-370, 379, 383, 391: Mock LLVM メッセージ
- **vm.rs**: 1箇所（Phase 98 で完了済み）
- **core_bridge.rs**: 2箇所（Phase 98 で完了済み）
- **selfhost 関連**: 5箇所（Phase 98 で完了済み）

**合計**: Phase 98 (7箇所) + Phase 100 (29箇所) = **36箇所完了**

**除外箇所**（意図的に残した）:
- llvm.rs の `[joinir/llvm]`, `[parse/context]` デバッグログ（Phase 101 対象）
- hack_check: .hako アプリ（Nyash言語の ConsoleBox 経由、別フェーズ）

**テスト結果**:
- ✅ cargo build --release 成功
- ✅ core_services テスト: 11 passed
- ✅ plugin_host テスト: 7 passed
- ✅ 代表ケース動作確認:
  - loop_min_while.hako: "📊 MIR Module compiled successfully!" 等が console_println! 経由で出力
  - エラーケース: "❌ Error reading file..." が console_println! 経由で出力

**残りの user-facing 出力**:
- 推定: ~330箇所（その他の runner/modes/*）
- 優先度: HIGH → Phase 101-102 で段階的拡張

**技術的成果**:
- selfhost/LLVM runner のユーザー向けメッセージが ConsoleService に統一
- Phase 99 で確立したログ/出力ポリシーが実装レベルで実現
- デバッグログとユーザー向け出力の明確な分離

---

## Section 7-A: Phase 101-A dev-debug ログの Ring0.log 統一（2025-12-03）

### dev-debug ログ ~34箇所を Ring0.log に統一

**実装概要**: llvm.rs, loop_form.rs, phi_core モジュールの代表的なデバッグログを Ring0.log に移行

**完了箇所**:
- **llvm.rs**: 13箇所 → Ring0.log
  - `[joinir/llvm]` デバッグログ: 12箇所（JoinIR 実験パス関連）
  - `[parse/context]` デバッグログ: 1箇所（プリロードファイル一覧）

- **loop_form.rs**: 全 [loopform] ログ → Ring0.log
  - `[loopform]` 基本ログ: 変数マップ、ブロックID等
  - `[loopform/condition]` ログ: 条件式処理関連
  - `[loopform/writes]` ログ: 変数書き込み収集
  - `[loopform/27.4-C]` ログ: JoinIR ヘッダーφバイパス

- **phi_core モジュール**: 21箇所 → Ring0.log
  - **loopform_builder.rs**: 16箇所
    - `[loopform/prepare]` ログ: 構造準備、変数分類、サマリー
    - `[loopform/seal_phis]` ログ: PHI シール処理
  - **loop_snapshot_merge.rs**: 5箇所
    - `[Option C]` ログ: Exit PHI 分類、変数解析

**実装パターン**:
```rust
// Before
eprintln!("[loopform] variable_map size={}", size);

// After
crate::runtime::get_global_ring0().log.debug(&format!(
    "[loopform] variable_map size={}", size
));
```

**合計**: Phase 101-A で **34箇所**のデバッグログを Ring0.log に統一

**テスト結果**:
- ✅ cargo build --release 成功（警告のみ、エラーなし）
- ✅ VM実行テスト: loop_min_while.hako 正常動作
- ✅ デバッグログが stderr に出なくなることを確認

**環境変数制御**:
- `NYASH_LOOPFORM_DEBUG=1`: LoopForm 関連デバッグログ有効化
- `NYASH_OPTION_C_DEBUG=1`: Option C 関連デバッグログ有効化
- Ring0.log のログレベル設定で出力制御可能

**残りの dev-debug ログ**:
- 推定: ~585箇所（全体 ~615箇所から Phase 101-A の 34箇所を除く）
- 対象外: test 出力（~347箇所、Phase 101-B で別途検討）
- 対象外: internal 出力（Phase 101-B で別途検討）

**技術的成果**:
- Ring0.log で dev-debug ログを一元管理
- 環境に応じた出力制御が可能（将来の活用に向けて）
- stderr の cleanness 向上（ユーザー向けメッセージのみになる）
- Phase 99-100 で確立した 3層設計を実装レベルで完成

---

## Section 7-B: Phase 101-B internal/test ログ整理（2025-12-04）

### 内部ログを Ring0.log に寄せ、テスト出力ポリシーを固定

**実装概要**:
- internal/dev ログ 26 箇所を Ring0.log に移行（stderr 汚染を削減） → 第1バッチ
  - 対象: provider_lock/mod.rs, plugin_loader_unified.rs, type_meta.rs, deprecations.rs, leak_tracker.rs
  - Plugin loader v2 系: loader/config.rs, loader/library.rs, loader/metadata.rs, instance_manager.rs, ffi_bridge.rs
- internal/dev ログ 21 箇所を追加で Ring0.log 化 → 第2バッチ
  - 対象: provider_verify.rs, scheduler.rs, gc_controller.rs, box_registry.rs
  - Plugin loader v2 specs: loader/specs.rs（TypeBox ABI/trace）
  - Runner trace: runner/trace.rs（cli_verbose トレース）
  - MIR verifier dev-trace: mir/verification.rs（NYASH_BREAKFINDER_SSA_TRACE/NYASH_DEBUG_VERIFIER）
- internal/dev ログ 20 箇所を追加で Ring0.log 化 → 第3バッチ
  - MIR core: basic_block.rs, control_form.rs, hints.rs, effect.rs, printer.rs, optimizer.rs
- internal/dev ログ 26 箇所を追加で Ring0.log 化 → 第4バッチ
  - MIR builder/region: loop_builder/phi_ops.rs, builder/type_registry.rs, region/observer.rs
  - Plugin loader v2: enabled/extern_functions.rs（trace）/types.rs（finalize trace）
- internal/dev ログ 20 箇所を追加で Ring0.log 化 → 第5バッチ
  - MIR loop_builder JoinIR: joinir_if_phi_selector.rs（dry-run trace）, control.rs（LoopForm debug）
  - MIR builder observe: observe/types.rs（NYASH_MIR_TYPE_TRACE）, observe/resolve.rs（NYASH_DEBUG_KPI_KNOWN）
  - joinir VM bridge: join_ir_vm_bridge_dispatch/exec_routes.rs（run_generic_joinir_route trace）
  - Plugin loader v2: enabled/extern_functions.rs（NYASH_DEBUG_TRACE / runtime_checkpoint_trace / NYASH_BOX_INTROSPECT_TRACE）
- ログレベル整理: init/loader 失敗は error、warn-once 系は warn、トレースは debug/info に整理
- ログレベル整理: init/loader 失敗は error、warn-once 系は warn、トレースは debug/info に整理

**テスト出力方針**:
- Rust テスト内（src/tests/, tests/）の println!/eprintln! は原則許容（大きな出力のみ将来検討）
- 本フェーズではテストコードは無変更、ポリシーを docs に明文化

**残件**:
- internal/dev ログ残量: 概算で ~475–495 箇所（引き続き段階的に Ring0.log へ移行/削除）
- user-facing: console_println! 移行は別ラインで継続
- .hako/hack_check: Rust とは別フェーズで整理

**成果**:
- Ring0/Ring1/Core の責務分離を保ったまま internal ログを OS 抽象層に集約
- 環境変数ベースのデバッグトレース（PLUGIN_DEBUG, HAKO_*）も Ring0.log 経由に統一
- stderr のノイズ低減とログ観測の一元化を達成

---

## Section 8: Phase 122 println/log 統一化

### 背景

従来、ConsoleBox の `println` と `log` は別々のメソッドとして扱われていました。しかし、ユーザーコード（.hako）では `println` を使用することが多く、Rust VM 実装では `log` のみが実装されていたため、selfhost Stage-3 + JoinIR Strict 経路で `Unknown method 'println'` エラーが発生していました。

### 実装内容

**Phase 122 の解決策**:
- `println` を `log` のエイリアスとして統一
- TypeRegistry で両者を同じ slot (400) に割り当て
- すべての経路（JSON v0 / selfhost / 通常VM）で一貫性を保証

**技術的詳細**:
```rust
// src/runtime/type_registry.rs
const CONSOLE_METHODS: &[MethodEntry] = &[
    MethodEntry { name: "log",     arity: 1, slot: 400 },
    MethodEntry { name: "println", arity: 1, slot: 400 },  // log と同じ slot
    MethodEntry { name: "warn",    arity: 1, slot: 401 },
    MethodEntry { name: "error",   arity: 1, slot: 402 },
    MethodEntry { name: "clear",   arity: 0, slot: 403 },
];
```

**nyash.toml での統一**（Phase 122.5）:
```toml
[libraries."libnyash_console_plugin.so".ConsoleBox.methods]
log = { method_id = 400 }
println = { method_id = 400 }  # log と同じ
warn = { method_id = 401 }
error = { method_id = 402 }
clear = { method_id = 403 }
```

### 使用ガイドライン

| 用途 | 推奨 API | 理由 |
|------|---------|------|
| **ユーザーコード（.hako）** | `ConsoleBox.println` | ユーザー向け sugar、他言語との一貫性 |
| **内部実装（Rust）** | `ConsoleBox.log` または `console_println!` | VM レベルでは同じ、マクロ推奨 |
| **selfhost / CLI** | `ConsoleService` / `console_println!` | Ring0 経由で安定 |

### 正規化ルール

- `ConsoleBox.println` は VM の TypeRegistry で `ConsoleBox.log`（slot 400）に正規化される
- JSON v0 / selfhost / 通常VM のすべての経路で同じ動作を保証
- Rust から直接使用する場合も `println` / `log` の両方が使用可能

### 3層ロギングとの関係

Phase 122 の println/log 統一は、Phase 99-101 で確立された3層ロギングシステムの **Layer 2（ConsoleService）** に該当します。

**3層ロギングの位置付け**:
1. **Layer 1（Ring0.log）**: Runtime/OS層内部ログ（開発者向け）
2. **Layer 2（ConsoleService）**: ユーザー向けCLI出力 ← **Phase 122 の対象**
3. **Layer 3（Raw println!）**: テスト・デバッグ専用（本番では制限）

### 実装完了日

**Phase 122 実装完了日**: 2025-12-04

### 参照

- [Phase 122 詳細ドキュメント](phase122_consolebox_println_unification.md)
- [Phase 122.5 詳細ドキュメント](phase122_5_nyash_toml_fix.md)
- [ConsoleBox 完全ガイド](consolebox_complete_guide.md) - 統合的なリファレンス

---

## Section 9: JoinIR/Loop Trace System (Phase 194)

### 概要

JoinIR（関数型IR）とループ最適化のデバッグ専用トレースシステム。
開発時のみ有効化し、本番環境では常にOFFにする。

**実装**: `src/mir/builder/control_flow/joinir/trace.rs` (250行)

### トレースカテゴリと環境変数

| 環境変数 | カテゴリ | 内容 | 出力例 |
|---------|---------|------|--------|
| `NYASH_JOINIR_DEBUG=1` | 全般デバッグ | JoinIR lowering全般の詳細 | `[trace:debug] pattern4: CarrierInfo: ...` |
| `NYASH_TRACE_VARMAP=1` | 変数マップ | variable_mapのスナップショット | `[trace:varmap] i=ValueId(123) sum=ValueId(456)` |
| `NYASH_TRACE_PHI=1` | PHI生成 | PHI命令生成の詳細 | `[trace:phi] Generated PHI for carrier 'sum'` |
| `NYASH_TRACE_MERGE=1` | ブロック結合 | MIRブロックマージ処理 | `[trace:merge] Remapping block BB5 → BB142` |
| `NYASH_TRACE_JOINIR_STATS=1` | 統計情報 | 関数数・ブロック数等 | `[trace:joinir_stats] 3 functions, 12 blocks` |

### 主要トレースメソッド

```rust
// シングルトンアクセス
use crate::mir::builder::control_flow::joinir::trace;

// パターン検出
trace::trace().pattern("pattern4", "Lowering loop with continue");

// 変数マップ
trace::trace().varmap("before_loop", &builder.variable_map);

// JoinIR統計
trace::trace().joinir_stats("pattern4", fn_count, block_count);

// PHI生成
trace::trace().phi("sum_exit", phi_id, &incoming_values);

// 汎用デバッグ
trace::trace().debug("pattern4", &format!("Carrier: {:?}", carrier_info));
```

### 出力フォーマット

**基本フォーマット**: `[trace:<category>] <tag>: <message>`

**出力例**:
```
[trace:pattern] pattern4: Detected loop with continue
[trace:varmap] pattern4_start: i=ValueId(100), sum=ValueId(101)
[trace:debug] pattern4: CarrierInfo: loop_var=i, carriers=["sum"]
[trace:debug] pattern4: Analyzed 1 carrier update expressions
[trace:debug] pattern4:   sum → BinOp { lhs: "sum", op: Add, rhs: Variable("i") }
[trace:joinir_stats] pattern4: 3 functions, 12 blocks
[trace:phi] sum_exit: PHI ValueId(456) from [BB10:ValueId(123), BB15:ValueId(234)]
[trace:merge] Remapping ValueId(5) → ValueId(789)
[trace:exit_phi] Building exit PHI from 2 incoming values
```

### 運用ポリシー

#### 開発時（推奨）
```bash
# Pattern 4デバッグ
NYASH_JOINIR_DEBUG=1 ./target/release/hakorune test.hako

# 変数追跡
NYASH_TRACE_VARMAP=1 ./target/release/hakorune test.hako

# 全トレース有効化（詳細デバッグ）
NYASH_JOINIR_DEBUG=1 NYASH_TRACE_VARMAP=1 NYASH_TRACE_PHI=1 \
  ./target/release/hakorune test.hako
```

#### 本番環境（必須）
```bash
# すべてのトレース環境変数をOFFにする（デフォルト）
./target/release/hakorune test.hako
```

#### テスト実行時
```bash
# 通常テスト: トレースOFF（デフォルト）
cargo test --release

# 特定テストのトレース有効化
NYASH_JOINIR_DEBUG=1 cargo test --release loop_continue_multi_carrier
```

### 実装箇所

**トレースAPI実装**:
- `src/mir/builder/control_flow/joinir/trace.rs` - JoinLoopTrace構造体・シングルトン

**トレース利用箇所**:
- `src/mir/builder/control_flow/joinir/patterns/pattern4_with_continue.rs` - Pattern 4 lowering
- `src/mir/builder/control_flow/joinir/merge/mod.rs` - MIRブロック結合
- `src/mir/join_ir/lowering/loop_with_continue_minimal.rs` - JoinIR生成
- その他JoinIR関連モジュール

### 設計原則

1. **開発時のみON、本番OFF**: 環境変数なしではトレース出力なし（ZeroConfig Default）
2. **カテゴリ別制御**: 必要なトレースのみ有効化（粒度の細かい制御）
3. **一貫フォーマット**: `[trace:<category>]` prefix統一（grep/filterしやすい）
4. **シングルトン**: `trace::trace()` で全モジュールから統一アクセス
5. **環境変数駆動**: コンパイル不要、実行時制御のみ

### Phase 194実装完了内容

**Task 194-3 完了**: Pattern 4 lowering内の全eprintln!をtrace()に置き換え
- 8箇所のeprintln!削除
- trace().debug()統一化
- 環境変数制御で出力ON/OFF

**Task 194-4 完了**: このドキュメントへの反映

---

## 📚 Related Documents

### ConsoleBox について知りたい場合
- [ConsoleBox 完全ガイド](consolebox_complete_guide.md) - 統合的なリファレンス
- [Phase 122-125 実装記録](phase122_consolebox_println_unification.md) - 詳細な実装背景

### ログ出力について知りたい場合
- このドキュメント - Nyash のログ出力全体のポリシー
- [Hako ログ設計](hako_logging_design.md) - Hako コンパイラ側のログ設計

### Core Boxes 設計について知りたい場合
- [Core Boxes 設計](core_boxes_design.md) - Core Box の全体設計
- [TypeRegistry 設計](../architecture/type-registry-design.md) - TypeRegistry の詳細設計
