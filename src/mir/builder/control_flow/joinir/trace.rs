//! JoinLoopTrace - Unified tracing for JoinIR loop lowering
//!
//! This module consolidates all debug output for JoinIR loop operations into a single
//! interface, making tracing consistent and controllable through environment variables.
//!
//! # Environment Variables
//!
//! - `NYASH_TRACE_VARMAP=1`: Enable variable_map tracing (shows variable → ValueId mappings)
//! - `NYASH_JOINIR_DEBUG=1`: Enable general JoinIR debug output (pattern routing, merge stats)
//! - `NYASH_OPTION_C_DEBUG=1`: Enable PHI-related debug (Option C PHI generation)
//! - `NYASH_JOINIR_MAINLINE_DEBUG=1`: Enable mainline routing debug (function name matching)
//! - `NYASH_LOOPFORM_DEBUG=1`: Enable LoopForm debug (control flow structure)
//!
//! # Output Format
//!
//! All trace output uses prefixed tags for easy filtering:
//!
//! ```text
//! [trace:pattern] route: Pattern3_WithIfPhi MATCHED
//! [trace:varmap] pattern3_before_merge: i→r4, sum→r7
//! [trace:joinir] merge_start: 3 functions, 45 blocks
//! [trace:phi] pattern3: PHI already exists, skipping
//! [trace:merge] pattern3: starting JoinIR merge
//! [trace:exit_phi] pattern3: sum r7→r15
//! [trace:debug] router: Current function name: 'main'
//! ```
//!
//! # Examples
//!
//! ```bash
//! # Enable variable_map tracing only
//! NYASH_TRACE_VARMAP=1 ./target/release/hakorune test.hako 2>&1 | grep "\[trace:"
//!
//! # Enable all JoinIR debug output
//! NYASH_JOINIR_DEBUG=1 ./target/release/hakorune test.hako 2>&1 | grep "\[trace:"
//!
//! # Enable PHI tracing
//! NYASH_OPTION_C_DEBUG=1 ./target/release/hakorune test.hako 2>&1 | grep "\[trace:phi\]"
//!
//! # Enable multiple trace categories
//! NYASH_TRACE_VARMAP=1 NYASH_JOINIR_DEBUG=1 ./target/release/hakorune test.hako
//! ```

use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Unified tracer for JoinIR loop operations.
///
/// Consolidates all debug output through a single interface, reading environment
/// variables to control which trace categories are enabled.
pub struct JoinLoopTrace {
    /// Whether varmap tracing is enabled (NYASH_TRACE_VARMAP)
    varmap_enabled: bool,
    /// Whether general JoinIR debug is enabled (NYASH_JOINIR_DEBUG)
    joinir_enabled: bool,
    /// Whether PHI debug is enabled (NYASH_OPTION_C_DEBUG)
    phi_enabled: bool,
    /// Whether mainline routing debug is enabled (NYASH_JOINIR_MAINLINE_DEBUG)
    mainline_enabled: bool,
    /// Whether LoopForm debug is enabled (NYASH_LOOPFORM_DEBUG)
    loopform_enabled: bool,
    /// Whether JoinIR dev mode is enabled (NYASH_JOINIR_DEV)
    dev_enabled: bool,
    /// Whether capture/ConditionEnv construction debug is enabled (NYASH_CAPTURE_DEBUG)
    capture_enabled: bool,
}

impl JoinLoopTrace {
    /// Create a new tracer, reading environment variables.
    pub fn new() -> Self {
        use crate::config::env::is_joinir_debug;
        let varmap_enabled = crate::config::env::builder_trace_varmap();
        let joinir_enabled = is_joinir_debug();
        let phi_enabled = crate::config::env::builder_option_c_debug();
        let mainline_enabled = crate::config::env::joinir_dev::mainline_debug_enabled();
        let loopform_enabled = crate::config::env::builder_loopform_debug();
        let capture_enabled = crate::config::env::builder_capture_debug();

        // IMPORTANT:
        // `NYASH_JOINIR_DEV=1` is a semantic/feature toggle used by the smoke SSOT.
        // It must not implicitly enable noisy stderr traces.
        //
        // Dev traces are enabled only when JoinIR debug is explicitly requested.
        let dev_enabled = crate::config::env::joinir_dev_enabled() && joinir_enabled;

        Self {
            varmap_enabled,
            joinir_enabled,
            phi_enabled,
            mainline_enabled,
            loopform_enabled,
            dev_enabled,
            capture_enabled,
        }
    }

    /// Check if any tracing is enabled
    pub fn is_enabled(&self) -> bool {
        self.varmap_enabled
            || self.joinir_enabled
            || self.phi_enabled
            || self.mainline_enabled
            || self.loopform_enabled
            || self.capture_enabled
    }

    /// Check if varmap tracing is enabled
    #[allow(dead_code)]
    pub fn is_varmap_enabled(&self) -> bool {
        self.varmap_enabled
    }

    /// Check if general joinir debug is enabled
    pub fn is_joinir_enabled(&self) -> bool {
        self.joinir_enabled
    }

    /// Check if mainline routing debug is enabled
    pub fn is_mainline_enabled(&self) -> bool {
        self.mainline_enabled
    }

    /// Check if loopform debug is enabled (legacy compatibility)
    pub fn is_loopform_enabled(&self) -> bool {
        self.loopform_enabled
    }

    /// Trace pattern detection/selection
    ///
    /// # Arguments
    /// - `tag`: Context identifier (e.g., "route", "pattern3")
    /// - `pattern_name`: Name of the pattern (e.g., "Pattern3_WithIfPhi")
    /// - `matched`: Whether the pattern matched (true) or was skipped (false)
    pub fn pattern(&self, tag: &str, pattern_name: &str, matched: bool) {
        if self.joinir_enabled || self.varmap_enabled {
            let status = if matched { "MATCHED" } else { "skipped" };
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[trace:pattern] {}: {} {}",
                tag, pattern_name, status
            ));
        }
    }

    /// Trace variable_map state
    ///
    /// # Arguments
    /// - `tag`: Context identifier (e.g., "pattern3_before_merge", "after_phi")
    /// - `vars`: The variable_map to display (variable name → ValueId)
    pub fn varmap(&self, tag: &str, vars: &BTreeMap<String, ValueId>) {
        if self.varmap_enabled {
            let entries: Vec<String> = vars
                .iter()
                .map(|(k, v)| format!("{}→r{}", k, v.0))
                .collect();
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[trace:varmap] {}: {}", tag, entries.join(", ")));
        }
    }

    /// Trace JoinIR function/block counts
    ///
    /// # Arguments
    /// - `tag`: Context identifier (e.g., "merge_start", "after_allocation")
    /// - `func_count`: Number of functions in the JoinModule
    /// - `block_count`: Total number of blocks across all functions
    pub fn joinir_stats(&self, tag: &str, func_count: usize, block_count: usize) {
        if self.joinir_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[trace:joinir] {}: {} functions, {} blocks",
                tag, func_count, block_count
            ));
        }
    }

    /// Trace PHI operations
    ///
    /// # Arguments
    /// - `tag`: Context identifier (e.g., "pattern3", "exit_block")
    /// - `msg`: Human-readable message about the PHI operation
    #[allow(dead_code)]
    pub fn phi(&self, tag: &str, msg: &str) {
        if self.phi_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[trace:phi] {}: {}", tag, msg));
        }
    }

    /// Trace merge operations
    ///
    /// # Arguments
    /// - `tag`: Context identifier (e.g., "pattern3", "block_allocation")
    /// - `msg`: Human-readable message about the merge operation
    #[allow(dead_code)]
    pub fn merge(&self, tag: &str, msg: &str) {
        if self.joinir_enabled || self.varmap_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[trace:merge] {}: {}", tag, msg));
        }
    }

    /// Trace exit PHI connection (variable_map update)
    ///
    /// # Arguments
    /// - `tag`: Context identifier (e.g., "pattern3", "exit_reconnect")
    /// - `var_name`: Name of the variable being reconnected
    /// - `old_id`: Old ValueId (before exit PHI)
    /// - `new_id`: New ValueId (after exit PHI)
    #[allow(dead_code)]
    pub fn exit_phi(&self, tag: &str, var_name: &str, old_id: ValueId, new_id: ValueId) {
        if self.varmap_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[trace:exit_phi] {}: {} r{}→r{}",
                tag, var_name, old_id.0, new_id.0
            ));
        }
    }

    /// Generic debug message (only if any tracing enabled)
    ///
    /// # Arguments
    /// - `tag`: Context identifier (e.g., "router", "pattern1")
    /// - `msg`: Human-readable debug message
    pub fn debug(&self, tag: &str, msg: &str) {
        if self.is_enabled() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[trace:debug] {}: {}", tag, msg));
        }
    }

    /// Dev-only trace message (NYASH_JOINIR_DEV=1).
    ///
    /// This is for diagnostics that should never appear in default runs, but are
    /// useful while developing JoinIR lowering.
    pub fn dev(&self, tag: &str, msg: &str) {
        if self.dev_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[trace:dev] {}: {}", tag, msg));
        }
    }

    /// Capture/debug output (NYASH_CAPTURE_DEBUG=1).
    pub fn capture(&self, tag: &str, msg: &str) {
        if self.capture_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[trace:capture] {}: {}", tag, msg));
        }
    }

    /// Emit a message when the caller explicitly enables it (no env checks).
    ///
    /// This is useful for routing `debug: bool` parameters through a single formatting point,
    /// instead of scattering ad-hoc `eprintln!`.
    pub fn emit_if(&self, channel: &str, tag: &str, msg: &str, enabled: bool) {
        if enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[trace:{}] {}: {}", channel, tag, msg));
        }
    }

    /// Emit a raw line to stderr when enabled (no formatting).
    ///
    /// Use this to preserve existing log formats while consolidating the actual `eprintln!`
    /// call sites into this tracer.
    pub fn stderr_if(&self, msg: &str, enabled: bool) {
        if enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("{}", msg));
        }
    }

    /// Trace function routing decisions
    ///
    /// # Arguments
    /// - `tag`: Context identifier (e.g., "router", "mainline")
    /// - `func_name`: Name of the function being routed
    /// - `msg`: Human-readable message about the routing decision
    pub fn routing(&self, tag: &str, func_name: &str, msg: &str) {
        if self.joinir_enabled || self.mainline_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[trace:routing] {}: function '{}' - {}",
                tag, func_name, msg
            ));
        }
    }

    /// Trace block allocation and remapping
    ///
    /// # Arguments
    /// - `tag`: Context identifier (e.g., "allocator", "remap")
    /// - `msg`: Human-readable message about block operations
    pub fn blocks(&self, tag: &str, msg: &str) {
        if self.joinir_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[trace:blocks] {}: {}", tag, msg));
        }
    }

    /// Trace instruction rewriting
    ///
    /// # Arguments
    /// - `tag`: Context identifier (e.g., "rewriter", "phi_inject")
    /// - `msg`: Human-readable message about instruction operations
    pub fn instructions(&self, tag: &str, msg: &str) {
        if self.joinir_enabled {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[trace:instructions] {}: {}", tag, msg));
        }
    }
}

impl Default for JoinLoopTrace {
    fn default() -> Self {
        Self::new()
    }
}

/// Global singleton for easy access (lazy initialized)
///
/// This provides a convenient way to access the tracer without passing it around:
///
/// ```rust
/// trace::trace().varmap("my_tag", &variable_map);
/// trace::trace().pattern("route", "Pattern1_Minimal", true);
/// ```
pub fn trace() -> &'static JoinLoopTrace {
    use std::sync::OnceLock;
    static TRACE: OnceLock<JoinLoopTrace> = OnceLock::new();
    TRACE.get_or_init(JoinLoopTrace::new)
}
