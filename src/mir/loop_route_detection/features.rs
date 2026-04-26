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

    /// If/phi route signal. This is not "any if"; producers must use the
    /// precise route recognizer for the current frontend.
    pub has_if: bool,

    /// Number of carrier variables (loop variables that are updated)
    pub carrier_count: usize,

    /// Number of break targets
    pub break_count: usize,

    /// Number of continue targets
    pub continue_count: usize,

    /// Phase 131-11: Is this an infinite loop? (condition == true)
    pub is_infinite_loop: bool,
}

impl Default for LoopFeatures {
    fn default() -> Self {
        Self {
            has_break: false,
            has_continue: false,
            has_if: false,
            carrier_count: 0,
            break_count: 0,
            continue_count: 0,
            is_infinite_loop: false,
        }
    }
}

/// Extract features from LoopForm for route classification.
///
/// This function is the entry point for structure-based pattern detection.
/// It analyzes the CFG structure without relying on variable names.
///
/// # Arguments
/// * `loop_form` - The loop structure to analyze
///
/// # Returns
/// * `LoopFeatures` - Feature vector for route classification
pub(crate) fn extract_features(loop_form: &LoopForm) -> LoopFeatures {
    // Phase 194: Basic feature extraction from LoopForm
    let has_break = !loop_form.break_targets.is_empty();
    let has_continue = !loop_form.continue_targets.is_empty();
    let break_count = loop_form.break_targets.len();
    let continue_count = loop_form.continue_targets.len();

    // LoopForm currently does not carry AST assignment/update observations.
    // Keep IfPhiJoin recognition in the AST feature extractor.
    let carrier_count = 0;
    let has_if = false;

    LoopFeatures {
        has_break,
        has_continue,
        has_if,
        carrier_count,
        break_count,
        continue_count,
        is_infinite_loop: false, // Phase 131-11: LoopForm doesn't have condition info, default to false
    }
}
