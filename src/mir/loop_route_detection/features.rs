use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::loop_form::LoopForm;

/// Feature vector extracted from loop structure.
///
/// This structure captures all relevant properties needed for pattern classification.
/// It is name-agnostic and purely structure-based.
#[derive(Debug, Clone)]
pub struct LoopFeatures {
    /// Has break statement(s)?
    pub has_break: bool,

    /// Has continue statement(s)?
    pub has_continue: bool,

    /// Has if statement(s) in body?
    pub has_if: bool,

    /// Has if-else statement with PHI nodes?
    /// (detected via multiple carriers or specific CFG patterns)
    pub has_if_else_phi: bool,

    /// Number of carrier variables (loop variables that are updated)
    pub carrier_count: usize,

    /// Number of break targets
    pub break_count: usize,

    /// Number of continue targets
    pub continue_count: usize,

    /// Phase 131-11: Is this an infinite loop? (condition == true)
    pub is_infinite_loop: bool,

    /// Phase 188.1: Nesting depth (1 = single loop, 2 = 1-level nested, etc.)
    pub max_loop_depth: u32,

    /// Phase 188.1: Has inner loops?
    pub has_inner_loops: bool,
}

impl Default for LoopFeatures {
    fn default() -> Self {
        Self {
            has_break: false,
            has_continue: false,
            has_if: false,
            has_if_else_phi: false,
            carrier_count: 0,
            break_count: 0,
            continue_count: 0,
            is_infinite_loop: false,
            max_loop_depth: 1,      // Phase 188.1: Default (no nesting)
            has_inner_loops: false, // Phase 188.1: Default (no nesting)
        }
    }
}

impl LoopFeatures {
    /// Phase 193-3: Get debug statistics string
    ///
    /// Returns a formatted string showing all feature values for debugging.
    pub fn debug_stats(&self) -> String {
        format!(
            "LoopFeatures {{ break: {}, continue: {}, if: {}, if_else_phi: {}, carriers: {}, break_count: {}, continue_count: {}, infinite: {}, depth: {}, inner: {} }}",
            self.has_break,
            self.has_continue,
            self.has_if,
            self.has_if_else_phi,
            self.carrier_count,
            self.break_count,
            self.continue_count,
            self.is_infinite_loop,
            self.max_loop_depth,
            self.has_inner_loops
        )
    }

    /// Phase 193-3: Count total control flow divergences
    ///
    /// Returns the total number of break + continue targets.
    /// Useful for determining loop complexity.
    pub fn total_divergences(&self) -> usize {
        self.break_count + self.continue_count
    }

    /// Phase 193-3: Check if loop has complex control flow
    ///
    /// Returns true if loop has multiple divergences or multiple carriers.
    pub fn is_complex(&self) -> bool {
        self.total_divergences() > 1 || self.carrier_count > 1
    }

    /// Phase 193-3: Check if loop is simple (no special features)
    ///
    /// Returns true if loop is purely sequential.
    pub fn is_simple(&self) -> bool {
        !self.has_break && !self.has_continue && !self.has_if_else_phi && self.carrier_count <= 1
    }
}

/// Phase 264 P0: Detect IfPhiJoin route signature
///
/// Returns true if loop has if-else with arithmetic accumulation shape:
/// - Same variable updated in both if and else branches
/// - Update involves arithmetic operations (BinOp: Add, Sub, etc.)
///
/// Example: sum = sum + (if x then 1 else 0)
///
/// Simple conditional assignment (seg = if x then "A" else "B") returns false.
///
/// # Phase 264 P0: Conservative Implementation
///
/// For now, this function returns false to allow carrier_count > 1 loops
/// to continue through baseline route checks instead of being misclassified
/// as IfPhiJoin.
///
/// # Phase 264 P1: TODO
///
/// Implement accurate IfPhiJoin signature detection via AST/CFG analysis.
/// (legacy "if-sum" terminology is traceability-only)
fn has_if_phi_join_signature(_scope: Option<&LoopScopeShape>) -> bool {
    // Phase 264 P0: Conservative - always return false
    // This keeps multi-carrier loops with simple conditional assignment
    // from being eagerly classified as IfPhiJoin.
    false
}

/// Extract features from LoopForm for route classification.
///
/// This function is the entry point for structure-based pattern detection.
/// It analyzes the CFG structure without relying on variable names.
///
/// # Arguments
/// * `loop_form` - The loop structure to analyze
/// * `scope` - Optional LoopScopeShape for carrier analysis
///
/// # Returns
/// * `LoopFeatures` - Feature vector for route classification
pub(crate) fn extract_features(
    loop_form: &LoopForm,
    scope: Option<&LoopScopeShape>,
) -> LoopFeatures {
    // Phase 194: Basic feature extraction from LoopForm
    let has_break = !loop_form.break_targets.is_empty();
    let has_continue = !loop_form.continue_targets.is_empty();
    let break_count = loop_form.break_targets.len();
    let continue_count = loop_form.continue_targets.len();

    // Phase 194+: Extract carrier_count from LoopScopeShape if available
    let carrier_count = scope.map(|s| s.carriers.len()).unwrap_or(0);

    // Phase 264 P0: Improved if-else PHI detection
    // IfPhiJoin route heuristic: has_if_else_phi only when IfPhiJoin
    // signature is observed
    // - Multiple carriers (carrier_count > 1)
    // - AND at least one carrier updated in both if/else branches with arithmetic
    //
    // Simple conditional assignment (seg = if x then "A" else "B") should NOT
    // be classified as IfPhiJoin.
    let has_if_else_phi = carrier_count > 1 && has_if_phi_join_signature(scope);

    // TODO: Implement has_if detection via CFG analysis
    // For now, infer from has_if_else_phi (IfPhiJoin signature heuristic)
    let has_if = has_if_else_phi;

    // Phase 188.1: Nesting detection
    // TODO: Detect from LoopForm structure (nested LoopForm presence)
    // For now, default to no nesting (will be detected in lowering phase)
    let max_loop_depth = 1;
    let has_inner_loops = false;

    LoopFeatures {
        has_break,
        has_continue,
        has_if,
        has_if_else_phi,
        carrier_count,
        break_count,
        continue_count,
        is_infinite_loop: false, // Phase 131-11: LoopForm doesn't have condition info, default to false
        max_loop_depth,
        has_inner_loops,
    }
}
