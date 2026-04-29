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
            is_infinite_loop: false,
        }
    }
}
