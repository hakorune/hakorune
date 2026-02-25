//! ReturnConverterBox: Return → Jump conversion helpers
//!
//! Phase 286C-2: Extracted from instruction_rewriter.rs
//! Helper functions to determine if a Return instruction should be converted to Jump.

use crate::mir::ValueId;

/// ReturnConverterBox: Return → Jump conversion helpers
///
/// Helper functions that determine how Return instructions should be handled
/// during JoinIR→MIR merging.
pub struct ReturnConverterBox;

impl ReturnConverterBox {
    /// Check if a Return instruction should be kept as Return (not converted to Jump)
    ///
    /// Non-skippable continuations (like k_return) should keep their Return terminator
    /// instead of being converted to Jump to exit block.
    ///
    /// # Arguments
    /// * `is_continuation_candidate` - Whether this is a continuation candidate
    /// * `is_skippable_continuation` - Whether this is a skippable continuation
    ///
    /// # Returns
    /// * `true` if the Return should be kept (not converted to Jump)
    /// * `false` if the Return should be converted to Jump to exit block
    pub fn should_keep_return(is_continuation_candidate: bool, is_skippable_continuation: bool) -> bool {
        is_continuation_candidate && !is_skippable_continuation
    }

    /// Create the remapped return value for a kept Return instruction
    ///
    /// When a Return is kept (not converted to Jump), the return value needs
    /// to be remapped from JoinIR value space to HOST value space.
    ///
    /// # Arguments
    /// * `value` - The optional return value from the original Return instruction
    /// * `remap_fn` - Function to remap a ValueId (e.g., `|v| remapper.remap_value(v)`)
    ///
    /// # Returns
    /// * The remapped optional return value
    pub fn remap_return_value<F>(
        value: Option<ValueId>,
        remap_fn: F,
    ) -> Option<ValueId>
    where
        F: FnOnce(ValueId) -> ValueId,
    {
        value.map(remap_fn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_keep_return_non_skippable_continuation() {
        // Non-skippable continuation: should keep Return
        assert!(ReturnConverterBox::should_keep_return(true, false));
    }

    #[test]
    fn test_should_keep_return_skippable_continuation() {
        // Skippable continuation: should convert to Jump
        assert!(!ReturnConverterBox::should_keep_return(true, true));
    }

    #[test]
    fn test_should_keep_return_non_continuation() {
        // Not a continuation: should convert to Jump
        assert!(!ReturnConverterBox::should_keep_return(false, false));
        assert!(!ReturnConverterBox::should_keep_return(false, true));
    }

    #[test]
    fn test_remap_return_value_some() {
        let value = Some(ValueId(100));
        let result = ReturnConverterBox::remap_return_value(value, |v| ValueId(v.0 + 1000));
        assert_eq!(result, Some(ValueId(1100)));
    }

    #[test]
    fn test_remap_return_value_none() {
        let value = None;
        let result = ReturnConverterBox::remap_return_value(value, |v| ValueId(v.0 + 1000));
        assert_eq!(result, None);
    }
}
