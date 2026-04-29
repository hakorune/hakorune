//! JoinIR development / experimental flags (SSOT).
//! Phase 72-C: Consolidate all NYASH_JOINIR_* dev flags through centralized helpers.

use crate::config::env::env_bool;

/// NYASH_JOINIR_LOWER_GENERIC=1 - Enable generic lowering path for JoinIR.
pub fn lower_generic_enabled() -> bool {
    env_bool("NYASH_JOINIR_LOWER_GENERIC")
}

/// NYASH_JOINIR_MAINLINE_DEBUG=1 - Debug output for mainline JoinIR lowering
pub fn mainline_debug_enabled() -> bool {
    env_bool("NYASH_JOINIR_MAINLINE_DEBUG")
}

/// NYASH_JOINIR_IF_MERGE=1 - Enable If-merge experimental mode
pub fn if_merge_enabled() -> bool {
    env_bool("NYASH_JOINIR_IF_MERGE")
}

/// JoinIR debug output switch (compat).
///
/// - Primary: `HAKO_JOINIR_DEBUG=1..3` (recommended)
/// - Legacy: `NYASH_JOINIR_DEBUG=1` (deprecated, but still supported)
pub fn debug_enabled() -> bool {
    crate::config::env::joinir_trace_enabled()
}

/// NYASH_JOINIR_VM_BRIDGE=1 - Enable VM bridge mode
pub fn vm_bridge_enabled() -> bool {
    env_bool("NYASH_JOINIR_VM_BRIDGE")
}

/// HAKO_JOINIR_STRICT=1 or NYASH_JOINIR_STRICT=1 - Strict validation mode
///
/// Phase 138-P2-A: Supports both HAKO_ and NYASH_ prefixes for compatibility
pub fn strict_enabled() -> bool {
    env_bool("HAKO_JOINIR_STRICT") || env_bool("NYASH_JOINIR_STRICT")
}

/// HAKO_JOINIR_PLANNER_REQUIRED=1 - Disallow legacy fallback when planner returns None (dev/strict gate only)
///
/// Default: OFF. Use in single-case / diagnostics gates to enforce "no silent fallback" without changing release behavior.
pub fn planner_required_enabled() -> bool {
    env_bool("HAKO_JOINIR_PLANNER_REQUIRED")
}

pub fn strict_planner_required_debug_enabled() -> bool {
    strict_enabled() && planner_required_enabled() && debug_enabled()
}

/// Strict + planner_required gate mode (debug output may still be disabled).
///
/// Use this for "gate sentinel" logs that must appear in strict+planner_required
/// runs even when general debug output is turned off.
pub fn strict_planner_required_enabled() -> bool {
    strict_enabled() && planner_required_enabled()
}

/// NYASH_JOINIR_SNAPSHOT_GENERATE=1 - Generate snapshot for testing
pub fn snapshot_generate_enabled() -> bool {
    env_bool("NYASH_JOINIR_SNAPSHOT_GENERATE")
}

/// NYASH_JOINIR_SNAPSHOT_TEST=1 - Test using snapshot
pub fn snapshot_test_enabled() -> bool {
    env_bool("NYASH_JOINIR_SNAPSHOT_TEST")
}

/// NYASH_JOINIR_INPUT=* - Input source or mode
pub fn input_mode() -> Option<String> {
    std::env::var("NYASH_JOINIR_INPUT").ok()
}

/// NYASH_JOINIR_LOWER_FROM_MIR=1 - Enable lowering from MIR mode
pub fn lower_from_mir_enabled() -> bool {
    env_bool("NYASH_JOINIR_LOWER_FROM_MIR")
}

/// NYASH_JOINIR_LLVM_EXPERIMENT=1 - LLVM experimental mode
pub fn llvm_experiment_enabled() -> bool {
    env_bool("NYASH_JOINIR_LLVM_EXPERIMENT")
}

/// HAKO_JOINIR_IF_TOPLEVEL=1 - Enable If-select for top-level if statements
pub fn if_toplevel_enabled() -> bool {
    env_bool("HAKO_JOINIR_IF_TOPLEVEL")
}

/// HAKO_JOINIR_IF_TOPLEVEL_TRACE=1 - Debug trace for top-level if
pub fn if_toplevel_trace_enabled() -> bool {
    env_bool("HAKO_JOINIR_IF_TOPLEVEL_TRACE")
}

/// HAKO_JOINIR_IF_IN_LOOP_TRACE=1 - Debug trace for if in loop
pub fn if_in_loop_trace_enabled() -> bool {
    env_bool("HAKO_JOINIR_IF_IN_LOOP_TRACE")
}

/// HAKO_JOINIR_NESTED_IF=1 - Enable nested if lowering
pub fn nested_if_enabled() -> bool {
    env_bool("HAKO_JOINIR_NESTED_IF")
}

/// HAKO_JOINIR_PRINT_TOKENS_MAIN=1 - Print tokens for main
pub fn print_tokens_main_enabled() -> bool {
    env_bool("HAKO_JOINIR_PRINT_TOKENS_MAIN")
}

/// HAKO_JOINIR_ARRAY_FILTER_MAIN=1 - Array filter main mode
pub fn array_filter_main_enabled() -> bool {
    env_bool("HAKO_JOINIR_ARRAY_FILTER_MAIN")
}

/// HAKO_JOINIR_READ_QUOTED=1 - Read quoted mode
pub fn read_quoted_enabled() -> bool {
    env_bool("HAKO_JOINIR_READ_QUOTED")
}

/// HAKO_JOINIR_READ_QUOTED_IFMERGE=1 - Read quoted with if-merge
pub fn read_quoted_ifmerge_enabled() -> bool {
    env_bool("HAKO_JOINIR_READ_QUOTED_IFMERGE")
}

/// JOINIR_TEST_DEBUG=1 (or NYASH_JOINIR_TEST_DEBUG=1) - Verbose logging for normalized dev tests
pub fn joinir_test_debug_enabled() -> bool {
    env_bool("JOINIR_TEST_DEBUG") || env_bool("NYASH_JOINIR_TEST_DEBUG")
}

/// Phase 183: NYASH_LEGACY_LOOPBUILDER=1 - Legacy LoopBuilder 経路を明示的に opt-in
///
/// デフォルトはJoinIR優先。どうしても古いLoopBuilder経路を使う必要がある場合のみ設定。
/// 本線では使用しない開発専用フラグ。
pub fn legacy_loopbuilder_enabled() -> bool {
    env_bool("NYASH_LEGACY_LOOPBUILDER")
}

/// Phase 145 P0: HAKO_ANF_DEV=1 - ANF (A-Normal Form) transformation development mode
///
/// Enables ANF transformation routing in NormalizedExprLowererBox.
/// P0: Debug logging only (execute_box is stub, returns Ok(None)).
/// P1+: Actual transformation (String.length() hoist, compound expression ANF).
///
/// # Usage
///
/// ```bash
/// HAKO_ANF_DEV=1 cargo test --release
/// HAKO_ANF_DEV=1 ./target/release/hakorune program.hako
/// ```
///
/// # Expected Behavior (P0)
///
/// - ANF routing enabled: AnfPlanBox detects impure expressions
/// - Debug log: "[phase145/debug] ANF plan found but execute returned None (P0 stub)"
/// - Existing behavior unchanged: execute_box returns Ok(None) → fallback to legacy
///
/// # Future Behavior (P1+)
///
/// - String.length() hoist: `x + s.length()` → ANF transformation
/// - Compound expression ANF: Recursive left-to-right linearization
pub fn anf_dev_enabled() -> bool {
    env_bool("HAKO_ANF_DEV")
}

/// Phase 145 P2: ANF strict mode (fail-fast on violations)
///
/// When enabled, ANF transformation errors result in immediate failure
/// instead of graceful fallback to legacy lowering.
///
/// # Environment Variable
///
/// `HAKO_ANF_STRICT=1` enables strict mode (default: OFF)
///
/// # Behavior
///
/// - **ON**: ANF violations return Err() with detailed error tags
/// - **OFF**: ANF violations gracefully fallback to legacy lowering (Ok(None))
///
/// # Use Cases
///
/// - **Development**: Catch order violations, pure-required violations early
/// - **Testing**: Verify ANF transformation correctness with fail-fast
/// - **Production**: Keep OFF for backward compatibility
///
/// # Example
///
/// ```bash
/// # Strict mode: Fail on `f() + g()` without ANF
/// HAKO_ANF_STRICT=1 ./hakorune program.hako
/// # Error: [joinir/anf/order_violation] f() + g(): both calls not hoisted
///
/// # Graceful mode (default): Fallback to legacy
/// ./hakorune program.hako
/// # OK: Legacy lowering used
/// ```
pub fn anf_strict_enabled() -> bool {
    env_bool("HAKO_ANF_STRICT")
}

/// Phase 146 P1: HAKO_ANF_ALLOW_PURE=1 - Allow ANF in PureOnly scope (dev-only)
///
/// Enables ANF transformation for PureOnly expression scopes (e.g., loop/if conditions).
/// Requires HAKO_ANF_DEV=1 to be set as well.
///
/// # Environment Variable
///
/// `HAKO_ANF_ALLOW_PURE=1` enables PureOnly ANF routing (default: OFF)
///
/// # Behavior
///
/// - **OFF (P0)**: ANF only for WithImpure scope (compound assignments)
/// - **ON (P1)**: ANF also for PureOnly scope (loop/if conditions with Compare)
///
/// # Use Cases
///
/// - **P1**: Enable `if (s.length() == 3)` ANF transformation
/// - **P147**: Enable `if (s1.length() < s2.length())` compound condition ANF
///
/// # Example
///
/// ```bash
/// # Enable condition ANF
/// HAKO_ANF_DEV=1 HAKO_ANF_ALLOW_PURE=1 ./hakorune program.hako
/// ```
pub fn anf_allow_pure_enabled() -> bool {
    env_bool("HAKO_ANF_ALLOW_PURE")
}
