# Ring0 Inventory & println! Categorization (Phase 99)

## Overview

This document provides an inventory of Ring0.log infrastructure and categorizes the remaining println!/eprintln! call sites across the codebase. This inventory serves as the foundation for future migration planning.

**Status**: Phase 101-B in progress — planning baseline plus partial internal/dev migration to Ring0.log (113 call sites converted across 5 batches).

---

## Section 1: Ring0.log Utilization Status

### Current Infrastructure (Phase 99 Start)

**Implemented**:
- `Ring0Context` contains `log: Arc<dyn LogApi>`
- `LogApi` trait with methods:
  - `debug(message: &str)`
  - `info(message: &str)`
  - `warn(message: &str)`
  - `error(message: &str)`
- `StdLog` implementation outputs to stdout/stderr

**Location**: `crates/nyash_kernel/src/ring0/log.rs`

---

### Phase 95-98 Usage

**ConsoleService Implementation (Phase 95-98)**:
- PluginHost established with CoreServices
- ConsoleService provides user-facing output
- `console_println!` macro consolidates user messages
- Ring0.log remains underutilized (future expansion target)

**Current Ring0.log Usage**: Minimal
- Primary logging still uses raw println!/eprintln!
- Opportunity for consolidation exists

---

## Section 2: println!/eprintln! Remaining Call Sites

### Overall Statistics

**Total**: 1776 locations (Phase 99 initial assessment)
- Test code: ~299 locations
- Production code: ~1477 locations

**Source**: Phase 99 initial assessment via `rg 'println!|eprintln!'`

**Historical Context (Phase 88-90)**:
- Phase 88-90 migrated 56 locations to ring0.log.*
- Phase 90 migrated 12 locations to ring0.fs/time/thread.*
- Previous total: 3,955 locations → Current: 1,776 locations

---

### Category Breakdown (Design Phase)

#### Category 1: user-facing (~366 locations, HIGH priority)

**Description**: CLI messages, errors, help text, and other user-visible output

**Target Migration**: `console_println!` macro

**Priority**: HIGH (directly visible to end users)

**Phase 98 Progress**: 7 locations completed (representative paths)

**Phase 100 Progress**: 29 locations completed (selfhost + LLVM runner)
- selfhost.rs: 6箇所（CoreInitError, Result 出力）
- llvm.rs: 23箇所（エラー、成功メッセージ、実行結果）

**合計完了**: Phase 98 (7) + Phase 100 (29) = **36箇所**

**残り**: ~330箇所（その他の runner/modes/*）

**Phase 101-102 Plan**: 段階的拡張継続

**Example Locations**:
- `src/runner/` - CLI entry points
- Error messages in main execution paths
- Help text and usage information
- Progress indicators

---

#### Category 2: dev-debug (~615 locations, MEDIUM priority)

**Description**: Temporary debug output for development purposes

**Target Migration**: Ring0.log (Phase 101-A で Ring0.log に統一決定)

**Priority**: MEDIUM

**Phase 99 Scope**: Assessment only - determine if Ring0.log is appropriate

**Phase 101-A Progress**: 34箇所完了（代表的なデバッグログ）
- llvm.rs: 13箇所（`[joinir/llvm]`, `[parse/context]`）
- loop_form.rs: 全 `[loopform]` ログ
- phi_core: 21箇所（`[loopform/prepare]`, `[loopform/seal_phis]`, `[Option C]`）

**合計完了**: Phase 101-A で **34箇所** → Ring0.log

**残り**: ~585箇所（全体 ~615箇所から Phase 101-A の 34箇所を除く）

**Phase 101-B/C Plan**: 段階的に Ring0.log へ移行継続

**Considerations**:
- Is this debug output needed in production?
- Should it use Ring0.log.debug()?
- Or should it be removed entirely?
- Or should it use a special dev_* macro?

---

#### Category 3: test (~299 locations, LOW priority)

**Description**: Test output and verification messages

**Target Migration**: **None - keep as-is**

**Priority**: LOW (safe in isolated environment)

**Phase 99-101 Plan**: No changes

**Rationale**:
- Tests run in isolated environments
- println! is appropriate for test output
- No migration needed

**Example Locations**:
- Test assertion messages
- Test progress indicators
- Debug output during test execution

---

#### Category 4: internal (~812 locations, TBD)

**Description**: Internal processing println! remnants

**Target Migration**: TBD

**Priority**: To be determined in Phase 99

**Phase 99 Scope**: Initial categorization

**Subcategories** (to be refined):
- Internal errors (could use Ring0.log.error)
- State transitions (could use Ring0.log.debug)
- Legacy debug output (could be removed)
- Performance metrics (could use Ring0.log.info)

---

### Phase 101-B Update (2025-12-04)

- internal/dev ログ 26 箇所を Ring0.log 化（stderr ノイズ削減）[第1バッチ]
  - runtime: provider_lock/mod.rs, type_meta.rs, deprecations.rs, leak_tracker.rs, plugin_loader_unified.rs
  - plugin loader v2: loader/config.rs, loader/library.rs, loader/metadata.rs, instance_manager.rs, ffi_bridge.rs
- internal/dev ログ 21 箇所を追加で Ring0.log 化 [第2バッチ]
  - runtime: provider_verify.rs, scheduler.rs, gc_controller.rs, box_registry.rs
  - plugin loader v2: loader/specs.rs（TypeBox ABI/trace）
  - runner: runner/trace.rs（cli_verbose トレース）
  - mir: mir/verification.rs（NYASH_BREAKFINDER_SSA_TRACE / NYASH_DEBUG_VERIFIER）
- internal/dev ログ 20 箇所を追加で Ring0.log 化 [第3バッチ]
  - mir core: basic_block.rs（insert-after-phis trace）、control_form.rs、hints.rs、effect.rs、printer.rs、optimizer.rs
- internal/dev ログ 26 箇所を追加で Ring0.log 化 [第4バッチ]
  - mir builder/region: loop_builder/phi_ops.rs、builder/type_registry.rs、region/observer.rs
  - plugin loader v2: enabled/extern_functions.rs（call/console trace）、enabled/types.rs（finalize trace）
- internal/dev ログ 20 箇所を追加で Ring0.log 化 [第5バッチ]
  - mir loop_builder JoinIR 系: joinir_if_phi_selector.rs（dry-run trace）、control.rs（LoopForm debug）
  - mir builder observe: observe/types.rs（NYASH_MIR_TYPE_TRACE）、observe/resolve.rs（NYASH_DEBUG_KPI_KNOWN）
  - joinir VM bridge: join_ir_vm_bridge_dispatch/exec_routes.rs（run_generic_joinir_route trace）
  - plugin loader v2: enabled/extern_functions.rs（NYASH_DEBUG_TRACE / runtime_checkpoint_trace / NYASH_BOX_INTROSPECT_TRACE）
- 役目: internal/diagnostic ログの責務を Ring0.log に寄せ、ConsoleService とテスト出力を分離
- 残件概算:
  - dev-debug: ~475–495 箇所（Phase 101-A の 34 箇所 + 5バッチ合計 113 箇所以外の残り）
  - internal: ~699 箇所（812 から 113 箇所控除のラフ値）
  - test: ~299 箇所（方針どおり維持）
  - hack_check (.hako) は別フェーズで扱う（Rust 側とは切り離し）

---

## Section 3: Ring0.log Expansion Plan (Sketch)

### Future Directions (Not Implemented)

This section outlines **potential** future uses of Ring0.log. Implementation is deferred to Phase 100+.

---

#### Option 1: Error Log Consolidation

**Concept**: Unify error logging through Ring0.log

**Examples**:
```rust
// VM execution errors
ring0.log.error(&format!("VM execution failed: {}", err));

// Parser errors
ring0.log.error(&format!("Parse error at line {}: {}", line, msg));

// MIR lowering errors
ring0.log.error(&format!("Lowering failed for {}: {}", node, reason));
```

**Benefits**:
- Centralized error logging
- Consistent error format
- Easy to redirect to files or monitoring systems

**Phase 99 Decision**: Concept documented, implementation deferred

---

#### Option 2: Internal State Logging

**Concept**: Use Ring0.log for runtime state tracking

**Examples**:
```rust
// Memory usage
ring0.log.debug(&format!("Memory allocated: {} bytes", size));

// GC triggers
ring0.log.info("Garbage collection triggered");

// Cache events
ring0.log.debug(&format!("Cache hit rate: {}%", rate));
```

**Benefits**:
- Non-intrusive diagnostics
- Performance monitoring
- Debugging production issues

**Phase 99 Decision**: Concept documented, implementation deferred

---

#### Option 3: Execution Tracing

**Concept**: Trace program execution through Ring0.log

**Examples**:
```rust
// Function entry/exit
ring0.log.debug(&format!("Entering function: {}", name));

// State transitions
ring0.log.info(&format!("State changed: {} -> {}", old, new));

// Performance markers
ring0.log.debug(&format!("Operation completed in {}ms", duration));
```

**Benefits**:
- Detailed execution visibility
- Performance profiling
- Bug reproduction

**Phase 99 Decision**: Concept documented, implementation deferred

---

### Implementation Priority (Future)

**Phase 100-101 Focus**: User-facing migrations (console_println!)

**Phase 102+ Candidates**:
1. Error log consolidation (HIGH impact)
2. Internal state logging (MEDIUM impact)
3. Execution tracing (LOW impact, high cost)

**Decision Point**: After console_println! migration is complete

---

## Section 4: Call Site Distribution

### By Module (Approximate)

**src/runner/**: ~366 locations
- Priority: HIGH (user-facing)
- Target: console_println!

**src/mir/**: ~200 locations
- Priority: MEDIUM (internal/debug)
- Target: Ring0.log or removal

**src/parser/**: ~150 locations
- Priority: MEDIUM (internal/debug)
- Target: Ring0.log or removal

**tests/**: ~299 locations
- Priority: LOW (keep as-is)
- Target: None

**Other**: ~761 locations
- Priority: TBD
- Target: TBD

---

### By Type (Approximate)

**Error messages**: ~400 locations
- User errors → console_println!
- Internal errors → Ring0.log.error

**Debug output**: ~600 locations
- Development debug → Ring0.log.debug or removal
- Temporary debug → removal

**Progress/info**: ~200 locations
- User progress → console_println!
- Internal progress → Ring0.log.info

**Test output**: ~299 locations
- Keep as-is

**Other**: ~277 locations
- TBD

### Phase 104: .hako側ロギング設計 (COMPLETED)

**Scope**:
- ConsoleBox適切な使い方ガイド
- ユーザー定義Boxからのロギングベストプラクティス
- 4つのロギングカテゴリ確立（user-facing/dev-debug/monitoring/internal Rust）
- 3つのロギングBoxパターン（Lightweight/Structured/Contextual）

**Files Created/Updated**:
- docs/development/current/main/logging_policy.md (Section 4 追加)
- docs/development/current/main/hako_logging_design.md (new)

**Status**: Complete - .hako側ロギング設計確立

---

### Phase 105: Logger Box Framework (COMPLETED)

**Scope**:
- Logger Box インターフェース設計（ログレベル: DEBUG/INFO/WARN/ERROR）
- 3つの設計パターン（Lightweight/Structured/Contextual）のリファレンス実装例
- Phase 99-104 ロギングポリシーとの整合確認

**Files Created/Updated**:
- docs/development/current/main/logger_box_design.md (new)
- docs/development/current/main/logging_policy.md (updated)
- docs/development/current/main/hako_logging_design.md (updated)

**Status**: Design + reference examples complete. Runtime implementation deferred to Phase 106+.

---

## Section 5: Migration Roadmap

### Phase 99 (Current): Documentation & Planning

**Objectives**:
- ✅ Document Ring0.log infrastructure
- ✅ Categorize remaining println!/eprintln! call sites
- ✅ Establish migration strategy
- ✅ Define completion criteria

**Deliverables**:
- This document (ring0-inventory.md)
- Logging policy (logging_policy.md)
- Updated core_boxes_design.md Section 15.6-A

---

### Phase 100-101: User-Facing Migrations

**Objectives**:
- Migrate ~366 user-facing println! to console_println!
- Focus on src/runner/ paths
- Ensure all CLI messages use ConsoleService

**Estimated Effort**: Medium (straightforward replacements)

**Success Criteria**:
- All user-visible messages use console_println!
- No println! in main execution paths
- Tests still pass

---

### Phase 102+: Internal Logging

**Objectives**:
- Evaluate Ring0.log usage for internal logging
- Migrate or remove dev-debug println!
- Consolidate error logging

**Estimated Effort**: Variable (requires case-by-case analysis)

**Success Criteria**:
- Clear separation: Ring0.log (internal) vs ConsoleService (user)
- No unnecessary println! in production code
- Test println! remains untouched

---

## Section 6: Grep Commands for Investigation

### Finding Call Sites

```bash
# All println!/eprintln! in source code
rg 'println!|eprintln!' --type rust

# Excluding tests
rg 'println!|eprintln!' --type rust --glob '!**/tests/**' --glob '!**/test_*.rs'

# Only in src/runner/
rg 'println!|eprintln!' --type rust src/runner/

# Only in src/mir/
rg 'println!|eprintln!' --type rust src/mir/

# Only in tests
rg 'println!|eprintln!' --type rust --glob '**/tests/**' --glob '**/test_*.rs'
```

---

### Finding console_println! Usage

```bash
# Current console_println! usage
rg 'console_println!' --type rust

# Potential candidates for console_println!
rg 'eprintln!\("Error:|Warning:|Info:' --type rust
```

---

### Finding Ring0.log Usage

```bash
# Current Ring0.log usage
rg 'ring0\.log\.(debug|info|warn|error)' --type rust

# LogApi trait implementations
rg 'impl.*LogApi' --type rust
```

---

## Section 7: Historical Migration Data (Phase 88-90)

### Phase 88-90: ring0.log.* Migration (56 locations)

**Migrated Files**:
- `src/runner/selfhost.rs`: 11 locations
- `src/runner/modes/vm.rs`: 22 locations
- `src/runner/modes/vm_fallback.rs`: 4 locations
- `src/runner/modes/common_util/hack_check.rs`: 19 locations

**Impact**: Consolidated user-facing error messages to Ring0.log.error()

---

### Phase 90: Ring0 API Migration (12 locations)

**Phase 90-A: fs API (7 locations)**:
- `src/runner/modes/common_util/resolve/strip/mod.rs`: 4 locations
- `src/runner/dispatch.rs`: 1 location
- `src/runner/mod.rs`: 3 locations
- Migration: `std::fs::read_to_string` → `ring0.fs.read_to_string`

**Phase 90-C: time API (3 locations)**:
- `src/runner/modes/common_util/selfhost_exe.rs`: 1 location
- `src/runner/modes/common_util/io.rs`: 1 location
- `src/runtime/plugin_loader_unified.rs`: 1 location
- Migration: `Instant::now() + elapsed()` → `ring0.time.monotonic_now() + ring0.time.elapsed()`

**Phase 90-D: thread API (2 locations)**:
- `src/runtime/global_hooks.rs`: 1 location
- `src/runtime/plugin_loader_unified.rs`: 1 location
- Migration: `std::thread::sleep` → `ring0.thread.sleep`

---

## Section 8: Success Metrics

### Completion Criteria

**Phase 99 Complete** when:
- ✅ Ring0.log infrastructure documented
- ✅ println!/eprintln! call sites categorized
- ✅ Migration strategy established
- ✅ Roadmap defined for Phase 100+

**Phase 100-101 Complete** when:
- All user-facing println! migrated to console_println!
- src/runner/ paths use ConsoleService exclusively
- Test println! remains untouched

**Phase 102+ Complete** when:
- Internal logging uses Ring0.log appropriately
- No unnecessary println! in production code
- Clear separation of concerns maintained

---

## Related Documentation

- [Logging Policy](logging_policy.md) - Role separation and macro policy
- [CoreBoxes Design - Section 15.6-A](core_boxes_design.md#section-156-a-logsoutput-unified-design) - Architectural context
- [Phase 85 CURRENT_TASK](../../../CURRENT_TASK.md) - Implementation timeline

---

### Phase 102: MemApi Bridge Skeleton (COMPLETED)

**Completed Tasks**:
- ✅ StdMem implementation (stdlib alloc/free)
- ✅ default_ring0() unified to use StdMem
- ✅ Unit tests for StdMem allocation and statistics
- ✅ NoopMem retained for compatibility

**Status**: Skeleton ready for Phase 102B (hakmem bridge integration)

**Files Modified**:
- src/runtime/ring0/std_impls.rs (added StdMem)
- src/runtime/ring0/mod.rs (updated default_ring0, exports)
- docs/development/current/main/phase-85-ring0-runtime/README.md (added Phase 102 section)
- docs/development/current/main/ring0-inventory.md (this file)

---

### Phase 103: CoreServices Optional化 (COMPLETED)

**Scope**:
- CoreServicesConfig struct (from_env, minimal, all_enabled)
- with_core_from_registry_optional() implementation
- Environment variable control (NYASH_CORE_DISABLE_* pattern)
- CoreBoxesImpl updated to Option<Arc<dyn XyzService>>

**Status**: Optional initialization ready for memory-constrained environments

**Files Modified**:
- src/runtime/plugin_host.rs (added CoreServicesConfig, with_core_from_registry_optional)
- src/runtime/core_services.rs (CoreBoxesImpl fields → Option type)
- docs/development/current/main/core_optional_design.md (new)
- docs/development/current/main/ring0-inventory.md (this file)

---

## Summary

Phase 85–108 で、Ring0 / CoreServices / FileBox / Logging の基礎はほぼ出揃った：

- Ring0Context: Mem/Io/Time/Log/Fs/Thread の抽象化が確立
- CoreBoxId/CoreMethodId: Box 名・メソッド名の SSOT 化
- CoreServices/PluginHost: ring1-core (String/Integer/Bool/Array/Map/Console) の service 化
- ConsoleService/Logging: 3層設計（Ring0.log / ConsoleService / test println!）が定着
- FileBox: CoreRequired 扱い + Ring0FsFileIo 経由で read/write 両対応

Phase 99 は logging infrastructure と println! call site の在庫を整理し：

1. **Ring0.log**: dev/debug ログの受け皿として拡張準備済み
2. **println!/eprintln!**: ~1400 箇所を 4 カテゴリに分類（user-facing/dev/test/internal）
3. **Migration strategy**: user-facing → Ring0.log/internal の順で段階的移行

Phase 102 では StdMem 実装により hakmem 統合の足場を用意し、  
Phase 106–108 では FileBox provider_lock / Ring0FsFileIo / write/write_all 実装により、
`FileBox → FileIo → Ring0.FsApi → std::fs` のパイプラインが完成した。

### Next Phases（計画）

今後の候補フェーズ（優先度の高い順）:

- ✅ **Phase 109: runtime profiles（default/no-fs）** (COMPLETED)
  - `NYASH_PROFILE={default|no-fs}` などのプロファイル導入
  - default では FileBox 必須、no-fs では FileBox provider 未登録（エラー文字列で通知）
- ✅ **Phase 110: FileHandleBox** (COMPLETED - 2025-12-03)
  - FileBox を「1ファイル専用」に保ちつつ、複数ファイル同時アクセスは FileHandleBox 側に切り出す
  - Ring0FsFileIo を再利用してハンドル単位の管理を行う
  - ハンドルベース複数回アクセス I/O 完全実装
  - 実装: `src/boxes/file/handle_box.rs` (7テスト全PASS)
  - API: open(path, mode) → read/write → close()
  - プロファイル対応: Default ✅、NoFs ❌
- ✅ **Phase 111: Fs metadata 拡張 + append mode** (COMPLETED - 2025-12-03)
  - FileHandleBox に append mode ("a") を追加
  - `exists/metadata/canonicalize` を FileIo / FileBox 側にきちんとエクスポート
  - Ring0.FsApi の stat 情報を Nyash 側から扱えるようにする
  - 実装: append_all() + metadata() 完全実装
  - テスト: 4個の新テスト全PASS
- ✅ **Phase 112: Ring0 Service Registry 統一化** (COMPLETED - 2025-12-03)
  - Ring0Context 初期化を Ring0Registry::build(profile) factory パターンに集約
  - NoFsApi struct 実装（Ring0 レベルで FsApi を無効化）
  - Profile 応じた実装選択（Default → StdFs、NoFs → NoFsApi）
  - default_ring0() を Ring0Registry 経由に統一（互換性維持）
  - 将来の拡張準備（TestMock/Sandbox/ReadOnly/Embedded プロファイル対応可能）
- ✅ **Phase 113: FileHandleBox Nyash 公開 API** (COMPLETED - 2025-12-04)
  - FileHandleBox の内部メソッドを Nyash (.hako) 側から使える形に公開
  - 実装: ny_* メソッド群（ny_open/ny_read/ny_write/ny_close/ny_exists/ny_size/ny_is_file/ny_is_dir）
  - エラーハンドリング: panic ベース（unwrap_or_else）
  - BoxFactory 登録: BuiltinBoxFactory に FileHandleBox 追加
  - テスト: Rust ユニット 6個 + .hako サンプル 1個（全 PASS）
  - Profile 対応: Default ✅ 全メソッド動作、NoFs ❌ open で panic
  - 設計原則確立: Rust internal vs Nyash-visible 分離パターン

さらに長期的には、Ring0 全体を「統一サービスレジストリ」として扱うフェーズ（Mem/Io/Time/Log/Fs/Thread の trait 統合）を
Phase 11x 以降で検討する予定だよ。Phase 112 で factory pattern による拡張基盤が整備された！

---

### Phase 114: FileIo trait 拡張 & メタデータ統一

**Scope**:
- FileIo trait に exists/stat/canonicalize 正式追加
- Ring0FsFileIo/NoFsFileIo の実装統一
- FileHandleBox 内部を metadata_internal() に統一
- FileStat 構造体追加（is_file/is_dir/size）

**Design Principles**:
- **FsApi = Stateless**: パスを毎回引数で受け取る（OS レイヤー）
- **FileIo = Stateful**: open() で path を保持（ハンドルレイヤー）
- **FileHandleBox**: FileIo::stat() 経由で統一（downcast 不要）

**Implementation**:
- Ring0FsFileIo: path を RwLock で管理、ring0.fs.metadata() を FileStat に変換
- NoFsFileIo: exists() → false, stat()/canonicalize() → Unsupported エラー
- FileHandleBox: metadata_internal() が FileIo::stat() を呼ぶ統一設計

**Tests**:
- Ring0FsFileIo: stat/exists/canonicalize 動作確認（Default プロファイル）
- NoFsFileIo: exists=false, stat/canonicalize エラー確認
- FileHandleBox: metadata_internal() が stat() 経由で動作確認

**Status**: 完了 - FsApi ↔ FileIo ↔ FileHandleBox の I/O 情報経路を完全統一

**Files Modified**:
- src/boxes/file/provider.rs (FileStat 構造体, FileIo trait 拡張)
- src/providers/ring1/file/ring0_fs_fileio.rs (exists/stat/canonicalize 実装)
- src/providers/ring1/file/nofs_fileio.rs (stub 実装)
- src/providers/ring1/file/core_ro.rs (exists/stat/canonicalize 実装)
- src/boxes/file/handle_box.rs (metadata_internal() 統一)
- docs/development/current/main/phase114_fileio_trait_extension.md (新規)
- docs/development/current/main/core_boxes_design.md (Phase 114 セクション追加)
- docs/development/current/main/ring0-inventory.md (this file)
