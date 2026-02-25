# Phase 286: llvm.rs 箱化モジュール化命令書

**Date**: 2025-12-24
**Target**: `src/runner/modes/llvm.rs` (449行の巨大関数)
**Goal**: Phase 33スタイルの箱化モジュール化で、単一責任・テスト容易性・可読性を達成

---

## 📋 背景・目的

### 現状の問題

**ファイル**: `src/runner/modes/llvm.rs` (449行)
**問題**: `execute_llvm_mode()` が単一関数で12個の責務を持つ

```rust
impl NyashRunner {
    pub(crate) fn execute_llvm_mode(&self, filename: &str) {
        // 1. Plugin初期化 (lines 12-20)
        // 2. Using処理・prelude merge (lines 31-108)
        // 3. Macro expansion (lines 109-111)
        // 4. MIR compilation (lines 113-138)
        // 5. method_id injection (lines 131-137)
        // 6. JoinIR experiment (lines 139-232, 94行, feature-gated)
        // 7. PyVM harness dev/test (lines 234-245, 12行)
        // 8. Object emit (lines 247-325, 79行)
        // 9. LLVM harness execution (lines 327-380, 54行)
        // 10. Inkwell legacy (lines 384-407, 24行)
        // 11. Mock execution (lines 408-445, 38行)
        // 12. Leak report (lines 356-358, 365-366, Phase 285LLVM-0)
    }
}
```

**影響**:
- ❌ 単一責任の原則違反（12個の責務）
- ❌ テスト困難（449行の巨大関数）
- ❌ 再利用不可（他のbackendで使えない）
- ❌ 可読性低下（責務の境界が不明確）
- ❌ 変更リスク大（1箇所変更で全体影響）

### 目的

**Phase 33成功事例を再現**:
- ✅ 単一責任の原則（各Boxが1つの関心事のみ）
- ✅ テスト容易性（独立したBoxで単体テスト可能）
- ✅ 再利用性（他のbackendでも使えるBox）
- ✅ 可読性（50-80行/Box、責務明確）
- ✅ 変更安全性（Boxの独立性で影響範囲限定）

---

## 🎯 Phase 33 成功事例（参考）

**ファイル**: `docs/development/architecture/phase-33-modularization.md`

**Before**: `mod.rs` 511行の巨大ファイル
**After**: 221行のオーケストレーター + 10個の独立Box

```
src/mir/builder/control_flow/
├── mod.rs (221行, -57%削減)
├── joinir/
│   ├── loop_patterns/
│   │   ├── pattern1_minimal.rs      # Pattern 1専用
│   │   ├── pattern2_flat_scan.rs     # Pattern 2専用
│   │   ├── pattern3_nested_if.rs     # Pattern 3専用
│   │   └── pattern4_multi_exit.rs    # Pattern 4専用
│   ├── merge/
│   │   ├── exit_line_reconnector.rs  # Exit line再接続Box
│   │   ├── exit_meta_collector.rs    # Exit metadata収集Box
│   │   └── exit_line_orchestrator.rs # Exit line調整オーケストレーター
```

**成功の鍵**:
1. **単一責任**: 各Boxが1つの明確な責務のみ
2. **Box境界**: 入力・出力・副作用を明確化
3. **再利用性**: 共通処理は別Boxに分離（ExitLineReconnectorなど）
4. **段階的移行**: 既存コードを動かしながら少しずつ分離

---

## 🏗️ llvm.rs 箱化設計

### ファイル構成（最終形）

```
src/runner/modes/llvm/
├── mod.rs (150-200行)           # オーケストレーター（execute_llvm_mode）
├── plugin_init.rs               # Plugin初期化Box
├── using_resolver.rs            # Using/prelude処理Box
├── mir_compiler.rs              # MIR compilation Box
├── method_id_injector.rs        # method_id injection Box
├── joinir_experiment.rs         # JoinIR experiment Box (feature-gated)
├── pyvm_executor.rs             # PyVM harness Box (dev/test)
├── object_emitter.rs            # Object emit Box
├── harness_executor.rs          # LLVM harness実行Box
├── exit_reporter.rs             # Leak report Box (Phase 285LLVM-0)
└── fallback_executor.rs         # Mock/legacy実行Box
```

### 各Boxの責務と設計

#### 1. `plugin_init.rs` - Plugin初期化Box

**責務**: Plugin初期化・診断
**入力**: なし
**出力**: `Result<(), String>`
**現在の場所**: `llvm.rs:12-20`

```rust
//! Plugin initialization for LLVM mode

pub struct PluginInitBox;

impl PluginInitBox {
    /// Initialize plugins and run diagnostics
    pub fn init() -> Result<(), String> {
        // Phase 15.5: Initialize bid plugins
        crate::runner_plugin_init::init_bid_plugins();

        // Friendly plugin guard (non-strict)
        crate::runner::modes::common_util::plugin_guard::check_and_report(
            false,
            crate::config::env::env_bool("NYASH_JSON_ONLY"),
            "llvm",
        );

        Ok(())
    }
}
```

#### 2. `using_resolver.rs` - Using/Prelude処理Box

**責務**: `using` 構文の解決とprelude merge
**入力**: `code: &str, filename: &str`
**出力**: `Result<(String, Vec<ASTNode>), String>`
**現在の場所**: `llvm.rs:31-108`

```rust
//! Using/prelude resolution for LLVM mode

use nyash_rust::ast::ASTNode;

pub struct UsingResolverBox;

impl UsingResolverBox {
    /// Resolve `using` statements and merge preludes
    pub fn resolve(
        runner: &crate::runner::NyashRunner,
        code: &str,
        filename: &str,
    ) -> Result<(String, Vec<ASTNode>), String> {
        let use_ast = crate::config::env::using_ast_enabled();
        let mut code_ref: &str = code;
        let cleaned_code_owned;
        let mut prelude_asts: Vec<ASTNode> = Vec::new();

        if crate::config::env::enable_using() {
            // Using resolution logic (lines 36-99)
            // ...
        }

        Ok((code_ref.to_string(), prelude_asts))
    }
}
```

#### 3. `mir_compiler.rs` - MIR Compilation Box

**責務**: AST → MIR compilation
**入力**: `ast: ASTNode, filename: Option<&str>`
**出力**: `Result<MirModule, String>`
**現在の場所**: `llvm.rs:113-138`

```rust
//! MIR compilation for LLVM mode

use nyash_rust::mir::{MirCompiler, MirModule};
use nyash_rust::ast::ASTNode;

pub struct MirCompilerBox;

impl MirCompilerBox {
    /// Compile AST to MIR
    pub fn compile(ast: ASTNode, filename: Option<&str>) -> Result<MirModule, String> {
        let mut mir_compiler = MirCompiler::new();

        let compile_result = crate::runner::modes::common_util::source_hint::compile_with_source_hint(
            &mut mir_compiler,
            ast,
            filename,
        ).map_err(|e| format!("MIR compilation error: {}", e))?;

        crate::console_println!("📊 MIR Module compiled successfully!");
        crate::console_println!("📊 Functions: {}", compile_result.module.functions.len());

        Ok(compile_result.module)
    }
}
```

#### 4. `method_id_injector.rs` - Method ID Injection Box

**責務**: BoxCall/PluginInvokeにmethod_id注入
**入力**: `&mut MirModule`
**出力**: `usize` (注入箇所数)
**現在の場所**: `llvm.rs:131-137`

```rust
//! Method ID injection for LLVM mode

use nyash_rust::mir::MirModule;
use nyash_rust::mir::passes::method_id_inject::inject_method_ids;

pub struct MethodIdInjectorBox;

impl MethodIdInjectorBox {
    /// Inject method_id for BoxCall/PluginInvoke
    pub fn inject(module: &mut MirModule) -> usize {
        let injected = inject_method_ids(module);
        if injected > 0 {
            crate::cli_v!("[LLVM] method_id injected: {} places", injected);
        }
        injected
    }
}
```

#### 5. `joinir_experiment.rs` - JoinIR Experiment Box

**責務**: JoinIR経路の実験的処理（feature-gated）
**入力**: `MirModule`
**出力**: `MirModule` (変換後、失敗時は元のまま)
**現在の場所**: `llvm.rs:139-232` (94行)

```rust
//! JoinIR experiment for LLVM mode (Phase 32 L-4.3a)

#[cfg(feature = "llvm-harness")]
use nyash_rust::mir::MirModule;

pub struct JoinIrExperimentBox;

impl JoinIrExperimentBox {
    /// Apply JoinIR experiment if enabled
    #[cfg(feature = "llvm-harness")]
    pub fn apply(module: MirModule) -> MirModule {
        if !crate::config::env::joinir_experiment_enabled()
            || !crate::config::env::joinir_llvm_experiment_enabled()
            || !crate::config::env::llvm_use_harness()
        {
            return module;
        }

        use nyash_rust::mir::join_ir::lower_skip_ws_to_joinir;
        use nyash_rust::mir::join_ir_vm_bridge::bridge_joinir_to_mir;

        crate::runtime::get_global_ring0()
            .log
            .debug("[joinir/llvm] Attempting JoinIR path for LLVM execution");

        // JoinIR conversion logic (lines 156-223)
        // ...

        module
    }

    #[cfg(not(feature = "llvm-harness"))]
    pub fn apply(module: MirModule) -> MirModule {
        module
    }
}
```

#### 6. `pyvm_executor.rs` - PyVM Harness Box

**責務**: PyVM harness経由での実行（dev/test用）
**入力**: `&MirModule`
**出力**: `Option<i32>` (実行した場合のみexit code)
**現在の場所**: `llvm.rs:234-245` (12行)
**重要**: **削除不可！8個のJSON ASTスモークテストで使用中**

```rust
//! PyVM harness executor (dev/test helper)

use nyash_rust::mir::MirModule;

pub struct PyVmExecutorBox;

impl PyVmExecutorBox {
    /// Execute via PyVM harness if requested
    pub fn try_execute(module: &MirModule) -> Option<i32> {
        if std::env::var("SMOKES_USE_PYVM").ok().as_deref() != Some("1") {
            return None;
        }

        match super::super::common_util::pyvm::run_pyvm_harness_lib(module, "llvm-ast") {
            Ok(code) => Some(code),
            Err(e) => {
                crate::console_println!("❌ PyVM harness error: {}", e);
                Some(1)
            }
        }
    }
}
```

#### 7. `object_emitter.rs` - Object Emit Box

**責務**: LLVM object file生成
**入力**: `&MirModule, out_path: &str`
**出力**: `Result<(), String>`
**現在の場所**: `llvm.rs:247-325` (79行)

```rust
//! Object file emitter for LLVM mode

use nyash_rust::mir::MirModule;

pub struct ObjectEmitterBox;

impl ObjectEmitterBox {
    /// Emit LLVM object file if requested
    #[cfg(feature = "llvm-harness")]
    pub fn try_emit(module: &MirModule) -> Result<bool, String> {
        let out_path = match std::env::var("NYASH_LLVM_OBJ_OUT") {
            Ok(p) => p,
            Err(_) => return Ok(false), // Not requested
        };

        if crate::config::env::llvm_use_harness() {
            crate::runner::modes::common_util::exec::llvmlite_emit_object(
                module, &out_path, 20_000
            )?;

            // Verify object file
            Self::verify_object(&out_path)?;
            return Ok(true);
        }

        Ok(false)
    }

    #[cfg(feature = "llvm-harness")]
    fn verify_object(path: &str) -> Result<(), String> {
        match std::fs::metadata(path) {
            Ok(meta) if meta.len() > 0 => {
                if std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1") {
                    crate::console_println!(
                        "[LLVM] object emitted: {} ({} bytes)",
                        path, meta.len()
                    );
                }
                Ok(())
            }
            Ok(_) => Err(format!("harness object is empty: {}", path)),
            Err(e) => Err(format!("harness output not found: {} ({})", path, e)),
        }
    }
}
```

#### 8. `harness_executor.rs` - LLVM Harness実行Box

**責務**: LLVM harness経由でnative executable生成・実行
**入力**: `&MirModule`
**出力**: `Option<i32>` (実行した場合のみexit code)
**現在の場所**: `llvm.rs:327-380` (54行)

```rust
//! LLVM harness executor (native executable generation and execution)

use nyash_rust::mir::MirModule;

pub struct HarnessExecutorBox;

impl HarnessExecutorBox {
    /// Execute via LLVM harness if available
    #[cfg(feature = "llvm-harness")]
    pub fn try_execute(module: &MirModule) -> Option<i32> {
        if !crate::config::env::llvm_use_harness() {
            return None;
        }

        // Prefer producing a native executable via ny-llvmc, then execute it
        let exe_out = "tmp/nyash_llvm_run";
        let libs = std::env::var("NYASH_LLVM_EXE_LIBS").ok();

        match crate::runner::modes::common_util::exec::ny_llvmc_emit_exe_lib(
            module,
            exe_out,
            None,
            libs.as_deref(),
        ) {
            Ok(()) => {
                match crate::runner::modes::common_util::exec::run_executable(
                    exe_out,
                    &[],
                    20_000,
                ) {
                    Ok((code, _timed_out, stdout_text)) => {
                        // Forward program stdout for parity tests
                        if !stdout_text.is_empty() {
                            print!("{}", stdout_text);
                        }
                        crate::console_println!(
                            "✅ LLVM (harness) execution completed (exit={})",
                            code
                        );

                        Some(code)
                    }
                    Err(e) => {
                        crate::console_println!("❌ run executable error: {}", e);
                        Some(1)
                    }
                }
            }
            Err(e) => {
                crate::console_println!("❌ ny-llvmc emit-exe error: {}", e);
                crate::console_println!(
                    "   Hint: build ny-llvmc: cargo build -p nyash-llvm-compiler --release"
                );
                Some(1)
            }
        }
    }

    #[cfg(not(feature = "llvm-harness"))]
    pub fn try_execute(_module: &MirModule) -> Option<i32> {
        None
    }
}
```

#### 9. `exit_reporter.rs` - Leak Report Box

**責務**: Phase 285LLVM-0のleak report出力
**入力**: なし
**出力**: なし（副作用のみ）
**現在の場所**: `llvm.rs:356-358, 365-366`

```rust
//! Exit reporter for LLVM mode (Phase 285LLVM-0)

pub struct ExitReporterBox;

impl ExitReporterBox {
    /// Emit leak report before exit
    pub fn emit_and_exit(code: i32) -> ! {
        // Phase 285LLVM-0: Emit Rust-side leak report before exit (if enabled)
        // Note: Only reports Rust VM-side roots (modules, host_handles, plugin_boxes).
        crate::runtime::leak_tracker::emit_leak_report();

        std::process::exit(code);
    }
}
```

#### 10. `fallback_executor.rs` - Fallback/Mock実行Box

**責務**: feature無効時のfallback実行（mock含む）
**入力**: `&MirModule`
**出力**: `i32` (exit code)
**現在の場所**: `llvm.rs:408-445` (38行)

```rust
//! Fallback executor for LLVM mode (mock/legacy)

use nyash_rust::mir::{MirModule, MirInstruction};

pub struct FallbackExecutorBox;

impl FallbackExecutorBox {
    /// Execute fallback path (feature check + mock)
    pub fn execute(module: &MirModule) -> i32 {
        // Fail-fast: if the user explicitly requested the llvmlite harness
        // but this binary was built without the `llvm-harness` feature,
        // do not silently fall back to mock.
        if crate::config::env::env_bool("NYASH_LLVM_USE_HARNESS") {
            crate::console_println!(
                "❌ LLVM harness requested (NYASH_LLVM_USE_HARNESS=1), but this binary was built without `--features llvm` (llvm-harness)."
            );
            crate::console_println!(
                "   Fix: cargo build --release --features llvm"
            );
            return 1;
        }

        crate::console_println!("🔧 Mock LLVM Backend Execution:");
        crate::console_println!("   Build with --features llvm for real backend.");

        // NamingBox SSOT: Select entry (arity-aware, Main.main → main fallback)
        let entry = crate::runner::modes::common_util::entry_selection::select_entry_function(module);

        if let Some(main_func) = module.functions.get(&entry) {
            for (_bid, block) in &main_func.blocks {
                for inst in &block.instructions {
                    match inst {
                        MirInstruction::Return { value: Some(_) } => {
                            crate::console_println!("✅ Mock exit code: 42");
                            return 42;
                        }
                        MirInstruction::Return { value: None } => {
                            crate::console_println!("✅ Mock exit code: 0");
                            return 0;
                        }
                        _ => {}
                    }
                }
            }
        }

        crate::console_println!("✅ Mock exit code: 0");
        0
    }
}
```

### オーケストレーター（mod.rs）

**責務**: 各Boxを適切な順序で呼び出し、全体フロー制御
**サイズ**: 150-200行（元の449行から55-60%削減）

```rust
//! LLVM mode executor (modularized)
//!
//! Phase 286: Modularization following Phase 33 success pattern
//! - Single responsibility per box
//! - Testability through isolated boxes
//! - Reusability across backends

use super::super::NyashRunner;
use nyash_rust::{mir::MirModule, parser::NyashParser};
use std::{fs, process};

mod plugin_init;
mod using_resolver;
mod mir_compiler;
mod method_id_injector;
mod joinir_experiment;
mod pyvm_executor;
mod object_emitter;
mod harness_executor;
mod exit_reporter;
mod fallback_executor;

use plugin_init::PluginInitBox;
use using_resolver::UsingResolverBox;
use mir_compiler::MirCompilerBox;
use method_id_injector::MethodIdInjectorBox;
use joinir_experiment::JoinIrExperimentBox;
use pyvm_executor::PyVmExecutorBox;
use object_emitter::ObjectEmitterBox;
use harness_executor::HarnessExecutorBox;
use exit_reporter::ExitReporterBox;
use fallback_executor::FallbackExecutorBox;

impl NyashRunner {
    /// Execute LLVM mode (modularized orchestrator)
    pub(crate) fn execute_llvm_mode(&self, filename: &str) {
        // Step 1: Plugin initialization
        if let Err(e) = PluginInitBox::init() {
            crate::console_println!("❌ Plugin init error: {}", e);
            ExitReporterBox::emit_and_exit(1);
        }

        // Step 2: Read file
        let code = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                crate::console_println!("❌ Error reading file {}: {}", filename, e);
                ExitReporterBox::emit_and_exit(1);
            }
        };

        // Step 3: Using resolution and prelude merge
        let (clean_code, prelude_asts) = match UsingResolverBox::resolve(self, &code, filename) {
            Ok(result) => result,
            Err(e) => {
                crate::console_println!("❌ Using resolution error: {}", e);
                ExitReporterBox::emit_and_exit(1);
            }
        };

        // Step 4: Parse to AST
        let mut parser = NyashParser::new(&clean_code);
        let main_ast = match parser.parse_entry() {
            Ok(ast) => ast,
            Err(e) => {
                crate::console_println!("❌ Parse error: {}", e);
                ExitReporterBox::emit_and_exit(1);
            }
        };

        // Step 5: Merge preludes with main AST
        let ast = if !prelude_asts.is_empty() {
            crate::runner::modes::common_util::resolve::merge_prelude_asts_with_main(
                prelude_asts,
                &main_ast,
            )
        } else {
            main_ast
        };

        // Step 6: Macro expansion
        let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);
        let ast = crate::runner::modes::macro_child::normalize_core_pass(&ast);

        // Step 7: MIR compilation
        let mut module = match MirCompilerBox::compile(ast, Some(filename)) {
            Ok(m) => m,
            Err(e) => {
                crate::console_println!("❌ {}", e);
                ExitReporterBox::emit_and_exit(1);
            }
        };

        // Step 8: Method ID injection
        MethodIdInjectorBox::inject(&mut module);

        // Step 9: JoinIR experiment (optional, feature-gated)
        let module = JoinIrExperimentBox::apply(module);

        // Step 10: PyVM harness (dev/test helper)
        if let Some(code) = PyVmExecutorBox::try_execute(&module) {
            ExitReporterBox::emit_and_exit(code);
        }

        // Step 11: Object emit (if requested)
        #[cfg(feature = "llvm-harness")]
        if let Ok(true) = ObjectEmitterBox::try_emit(&module) {
            return; // Object emit only, no execution
        }

        // Step 12: LLVM harness execution (preferred)
        if let Some(code) = HarnessExecutorBox::try_execute(&module) {
            ExitReporterBox::emit_and_exit(code);
        }

        // Step 13: Fallback execution (mock/legacy)
        let code = FallbackExecutorBox::execute(&module);
        ExitReporterBox::emit_and_exit(code);
    }
}
```

---

## 🚀 実装手順（段階的移行）

### Phase 1: 準備（30分）

1. **ディレクトリ作成**
   ```bash
   mkdir -p src/runner/modes/llvm
   ```

2. **mod.rs作成**
   - 現在の `llvm.rs` を `llvm/mod.rs` にコピー
   - Git履歴保持: `git mv src/runner/modes/llvm.rs src/runner/modes/llvm/mod.rs`

3. **親モジュール更新**
   - `src/runner/modes/mod.rs` の `mod llvm;` を確認

### Phase 2: Box分離（各30-60分、順次実行）

**順序**: 依存関係の少ないBoxから分離

#### Step 1: `plugin_init.rs` 分離
- Lines 12-20 を抽出
- `PluginInitBox::init()` 実装
- `mod.rs` から呼び出し
- テスト実行（`cargo build --release`）
- Commit

#### Step 2: `exit_reporter.rs` 分離
- Lines 356-358, 365-366 を抽出
- `ExitReporterBox::emit_and_exit()` 実装
- 全ての `process::exit()` を置き換え
- テスト実行
- Commit

#### Step 3: `method_id_injector.rs` 分離
- Lines 131-137 を抽出
- `MethodIdInjectorBox::inject()` 実装
- テスト実行
- Commit

#### Step 4: `using_resolver.rs` 分離
- Lines 31-108 を抽出
- `UsingResolverBox::resolve()` 実装
- テスト実行
- Commit

#### Step 5: `mir_compiler.rs` 分離
- Lines 113-130 を抽出
- `MirCompilerBox::compile()` 実装
- テスト実行
- Commit

#### Step 6: `pyvm_executor.rs` 分離
- Lines 234-245 を抽出
- `PyVmExecutorBox::try_execute()` 実装
- **重要**: JSON ASTスモークテスト実行確認
- Commit

#### Step 7: `joinir_experiment.rs` 分離
- Lines 139-232 を抽出
- `JoinIrExperimentBox::apply()` 実装
- feature-gated コンパイル確認
- テスト実行
- Commit

#### Step 8: `object_emitter.rs` 分離
- Lines 247-325 を抽出
- `ObjectEmitterBox::try_emit()` 実装
- feature-gated コンパイル確認
- テスト実行
- Commit

#### Step 9: `harness_executor.rs` 分離
- Lines 327-380 を抽出
- `HarnessExecutorBox::try_execute()` 実装
- feature-gated コンパイル確認
- テスト実行
- Commit

#### Step 10: `fallback_executor.rs` 分離
- Lines 408-445 を抽出
- `FallbackExecutorBox::execute()` 実装
- テスト実行
- Commit

### Phase 3: オーケストレーター整理（1時間）

1. **mod.rs簡素化**
   - 各Boxのuse文追加
   - `execute_llvm_mode()` を12ステップのオーケストレーターに書き換え
   - 古いコード削除

2. **最終テスト**
   ```bash
   # ビルド確認
   cargo build --release
   cargo build --release --features llvm

   # スモークテスト（重要！）
   tools/smokes/v2/run.sh --profile quick --filter "json_*_ast"  # PyVM確認
   tools/smokes/v2/run.sh --profile quick --filter "llvm"         # LLVM確認
   ```

3. **Commit**
   ```bash
   git add src/runner/modes/llvm/
   git commit -m "refactor(llvm): Phase 286 - Modularize llvm.rs into 10 boxes

   Following Phase 33 success pattern:
   - Single responsibility per box
   - Testability through isolated boxes
   - Reusability across backends

   Before: 449 lines monolithic function
   After: 150-200 line orchestrator + 10 focused boxes

   Boxes created:
   - plugin_init.rs (plugin initialization)
   - using_resolver.rs (using/prelude handling)
   - mir_compiler.rs (AST → MIR compilation)
   - method_id_injector.rs (method_id injection)
   - joinir_experiment.rs (JoinIR experiment, feature-gated)
   - pyvm_executor.rs (PyVM harness, dev/test)
   - object_emitter.rs (LLVM object emit)
   - harness_executor.rs (LLVM harness execution)
   - exit_reporter.rs (leak report, Phase 285LLVM-0)
   - fallback_executor.rs (mock/legacy execution)

   Test Results:
   - ✅ cargo build --release
   - ✅ cargo build --release --features llvm
   - ✅ JSON AST smoke tests (PyVM確認)
   - ✅ LLVM smoke tests
   - ✅ No regressions

   🤖 Generated with [Claude Code](https://claude.com/claude-code)

   Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
   ```

---

## ✅ Acceptance Criteria

### 必須条件

- [ ] **ビルド成功**: `cargo build --release` エラー0
- [ ] **ビルド成功（LLVM）**: `cargo build --release --features llvm` エラー0
- [ ] **PyVMスモークテスト**: 8個のJSON ASTテスト全てPASS
  ```bash
  tools/smokes/v2/run.sh --profile quick --filter "json_errors_ast"
  tools/smokes/v2/run.sh --profile quick --filter "json_large_array_ast"
  tools/smokes/v2/run.sh --profile quick --filter "json_deep_nesting_ast"
  tools/smokes/v2/run.sh --profile quick --filter "json_error_messages_ast"
  tools/smokes/v2/run.sh --profile quick --filter "json_unicode_basic_ast"
  tools/smokes/v2/run.sh --profile quick --filter "json_roundtrip_ast"
  tools/smokes/v2/run.sh --profile quick --filter "json_long_string_ast"
  tools/smokes/v2/run.sh --profile quick --filter "json_nested_ast"
  ```
- [ ] **LLVMスモークテスト**: Phase 285LLVM-0テスト含めてPASS
  ```bash
  tools/smokes/v2/run.sh --profile quick --filter "phase285_leak_report_llvm"
  ```
- [ ] **回帰なし**: 既知FAIL以外増えない

### 品質条件

- [ ] **単一責任**: 各Boxが1つの明確な責務のみ
- [ ] **サイズ**: 各Box 50-120行（オーケストレーター除く）
- [ ] **可読性**: 責務・入力・出力がコメントで明示
- [ ] **再利用性**: Boxが独立して使える設計
- [ ] **テスト容易性**: 各Boxが単体テスト可能な設計

### ドキュメント条件

- [ ] **各Boxにドキュメントコメント**: `//!` でモジュール説明
- [ ] **各関数にドキュメントコメント**: `///` で関数説明
- [ ] **Phase 286記録**: `docs/development/current/main/phases/phase-286/README.md` 作成

---

## ⚠️ 重要な注意点

### 1. PyVM削除禁止

**理由**: 8個のJSON ASTスモークテストで使用中
**対象**: `pyvm_executor.rs` は保持必須
**確認**: 実装後に必ずJSON ASTテスト実行

### 2. Feature-gated コード

**対象**:
- `joinir_experiment.rs` (#[cfg(feature = "llvm-harness")])
- `object_emitter.rs` (#[cfg(feature = "llvm-harness")])
- `harness_executor.rs` (#[cfg(feature = "llvm-harness")])

**確認**: 両方のfeature組み合わせでビルド
```bash
cargo build --release                    # llvm-harness無効
cargo build --release --features llvm    # llvm-harness有効
```

### 3. Leak Report統合

**Phase 285LLVM-0**: `exit_reporter.rs` で統一
**全ての `process::exit()` を `ExitReporterBox::emit_and_exit()` に置き換え**

### 4. 段階的移行

**一度に全部やらない**:
- 1 Boxずつ分離
- 各ステップでビルド・テスト確認
- 問題があれば即座に戻せる

### 5. Git履歴保持

**`git mv` 使用**:
```bash
git mv src/runner/modes/llvm.rs src/runner/modes/llvm/mod.rs
```

---

## 📚 参考資料

### Phase 33 成功事例
- **ドキュメント**: `docs/development/architecture/phase-33-modularization.md`
- **実装**: `src/mir/builder/control_flow/`
- **Commit**: `2d5607930` など5個のcommit

### 関連Phase
- **Phase 285LLVM-0**: Leak Report実装（`exit_reporter.rs` の基礎）
- **Phase 15**: Rust VM + LLVM 2本柱体制

### コーディング規約
- **Fail-Fast原則**: エラーは早期に明示的に失敗
- **Box-First原則**: 箱化・モジュール化で足場を積む
- **SSOT原則**: 単一真実の源（ドキュメント・実装の整合性）

---

## 🎯 期待効果

### コード品質

| 項目 | Before | After | 改善 |
|------|--------|-------|------|
| ファイル数 | 1 | 11 | +10 |
| 最大行数/ファイル | 449行 | 150-200行 | -55% |
| 平均行数/Box | N/A | 50-80行 | 可読性↑ |
| 責務数/関数 | 12 | 1 | 単一責任↑ |

### 開発効率

- ✅ **テスト容易性**: 各Box独立テスト可能
- ✅ **変更安全性**: Box境界で影響範囲限定
- ✅ **再利用性**: 他backendでBox再利用可能
- ✅ **可読性**: 責務明確で理解しやすい

### 保守性

- ✅ **バグ修正**: Box単位で修正可能
- ✅ **機能追加**: 新Boxとして追加可能
- ✅ **リファクタリング**: Box単位で実施可能

---

## 🤖 AI実装者への追加指示

### 実装スタイル

1. **段階的に実装**: 一度に全部やらない
2. **各ステップでテスト**: ビルド・スモークテスト確認
3. **Commitは小さく**: 1 Box分離ごとにcommit
4. **エラーメッセージ保持**: 既存のエラーメッセージを変更しない
5. **コメント充実**: 各Boxの責務・入力・出力を明記

### デバッグ時の確認事項

**ビルドエラー時**:
1. use文の追加忘れチェック
2. feature-gated コードの `#[cfg]` 確認
3. モジュール宣言の追加確認

**実行時エラー時**:
1. PyVMスモークテスト確認（`SMOKES_USE_PYVM=1`）
2. Leak report出力確認（`NYASH_LEAK_LOG=2`）
3. LLVM harness実行確認（`NYASH_LLVM_USE_HARNESS=1`）

### 質問・不明点があれば

- Phase 33実装 (`src/mir/builder/control_flow/`) を参照
- 既存コードの動作を変更しない
- 疑問があれば実装前に質問

---

**この命令書は完全自己完結型です。他のAI（ChatGPT等）に渡して実装可能です。**

**実装完了後、Phase 285LLVM-0と合わせてコミットしてください。**
