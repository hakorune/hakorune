//! ValueId Range Allocation for JoinIR Lowering Modules
//!
//! This module manages ValueId ranges for each JoinIR lowering module to prevent
//! ID conflicts when multiple lowerings coexist.
//!
//! ## Current Allocations
//!
//! | Module                  | Range      | Entry  | Loop   | Notes |
//! |-------------------------|------------|--------|--------|-------|
//! | skip_ws                 | 3000-4999  | 3000+  | 4000+  | Skip whitespace |
//! | funcscanner_trim        | 5000-6999  | 5000+  | 6000+  | Trim whitespace |
//! | stage1_using_resolver   | 7000-8999  | 7000+  | 8000+  | Stage-1 using resolver |
//! | funcscanner_append_defs | 9000-10999 | 9000+  | 10000+ | FuncScanner append defs |
//! | stageb_body_extract     | 11000-12999| 11000+ | 12000+ | Stage-B body extractor |
//! | stageb_funcscanner      | 13000-14999| 13000+ | 14000+ | Stage-B FuncScanner (scan_all_boxes) |
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use crate::mir::join_ir::lowering::value_id_ranges::stage1_using_resolver as vid;
//!
//! let entries_param = vid::entry(0);      // ValueId(7000)
//! let n_param = vid::entry(1);            // ValueId(7001)
//! let entries_loop = vid::loop_step(0);   // ValueId(8000)
//! let n_loop = vid::loop_step(1);         // ValueId(8001)
//! ```
//!
//! ## Future Extensions
//!
//! When adding new lowering modules, allocate ranges in increments of 2000:
//! - 15000-16999 (next available)
//! - etc.

use crate::mir::ValueId;

/// Base addresses for each lowering module's ValueId range
pub mod base {
    /// skip_ws: Skip whitespace loop (3000-4999)
    pub const SKIP_WS: u32 = 3000;

    /// funcscanner_trim: Trim whitespace loop (5000-6999)
    pub const FUNCSCANNER_TRIM: u32 = 5000;

    /// stage1_using_resolver: Stage-1 using resolver entries loop (7000-8999)
    pub const STAGE1_USING_RESOLVER: u32 = 7000;

    /// funcscanner_append_defs: FuncScanner append defs loop (9000-10999)
    pub const FUNCSCANNER_APPEND_DEFS: u32 = 9000;

    /// stageb_body_extract: Stage-B body extractor loop (11000-12999)
    pub const STAGEB_BODY_EXTRACT: u32 = 11000;

    /// stageb_funcscanner: Stage-B FuncScanner scan_all_boxes loop (13000-14999)
    pub const STAGEB_FUNCSCANNER: u32 = 13000;
}

/// Helper function to create ValueId from base + offset
///
/// This is a const fn, so it's computed at compile time with zero runtime cost.
#[inline]
pub const fn id(base: u32, offset: u32) -> ValueId {
    ValueId(base + offset)
}

/// ValueId helpers for skip_ws lowering module
pub mod skip_ws {
    use super::{base, id};
    use crate::mir::ValueId;

    /// Entry function ValueIds (3000-3999)
    #[inline]
    pub const fn entry(offset: u32) -> ValueId {
        id(base::SKIP_WS, offset)
    }

    /// Loop function ValueIds (4000-4999)
    #[inline]
    pub const fn loop_step(offset: u32) -> ValueId {
        id(base::SKIP_WS, 1000 + offset)
    }
}

/// ValueId helpers for funcscanner_trim lowering module
pub mod funcscanner_trim {
    use super::{base, id};
    use crate::mir::ValueId;

    /// Entry function ValueIds (5000-5999)
    #[inline]
    pub const fn entry(offset: u32) -> ValueId {
        id(base::FUNCSCANNER_TRIM, offset)
    }

    /// Loop function ValueIds (6000-6999)
    #[inline]
    pub const fn loop_step(offset: u32) -> ValueId {
        id(base::FUNCSCANNER_TRIM, 1000 + offset)
    }
}

/// ValueId helpers for stage1_using_resolver lowering module
pub mod stage1_using_resolver {
    use super::{base, id};
    use crate::mir::ValueId;

    /// Entry function ValueIds (7000-7999)
    #[inline]
    pub const fn entry(offset: u32) -> ValueId {
        id(base::STAGE1_USING_RESOLVER, offset)
    }

    /// Loop function ValueIds (8000-8999)
    #[inline]
    pub const fn loop_step(offset: u32) -> ValueId {
        id(base::STAGE1_USING_RESOLVER, 1000 + offset)
    }
}

/// ValueId helpers for funcscanner_append_defs lowering module
pub mod funcscanner_append_defs {
    use super::{base, id};
    use crate::mir::ValueId;

    /// Entry function ValueIds (9000-9999)
    #[inline]
    pub const fn entry(offset: u32) -> ValueId {
        id(base::FUNCSCANNER_APPEND_DEFS, offset)
    }

    /// Loop function ValueIds (10000-10999)
    #[inline]
    pub const fn loop_step(offset: u32) -> ValueId {
        id(base::FUNCSCANNER_APPEND_DEFS, 1000 + offset)
    }
}

/// ValueId helpers for Stage-B body extractor lowering module
pub mod stageb_body_extract {
    use super::{base, id};
    use crate::mir::ValueId;

    /// Entry function ValueIds (11000-11999)
    #[inline]
    pub const fn entry(offset: u32) -> ValueId {
        id(base::STAGEB_BODY_EXTRACT, offset)
    }

    /// Loop function ValueIds (12000-12999)
    #[inline]
    pub const fn loop_step(offset: u32) -> ValueId {
        id(base::STAGEB_BODY_EXTRACT, 1000 + offset)
    }
}

/// ValueId helpers for Stage-B FuncScanner lowering module
pub mod stageb_funcscanner {
    use super::{base, id};
    use crate::mir::ValueId;

    /// Entry function ValueIds (13000-13999)
    #[inline]
    pub const fn entry(offset: u32) -> ValueId {
        id(base::STAGEB_FUNCSCANNER, offset)
    }

    /// Loop function ValueIds (14000-14999)
    #[inline]
    pub const fn loop_step(offset: u32) -> ValueId {
        id(base::STAGEB_FUNCSCANNER, 1000 + offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Macro to test ValueId range boundaries for a lowering module
    ///
    /// Verifies that entry(0) and loop_step(999) produce the expected ValueIds
    /// based on the module's allocated range.
    macro_rules! test_value_id_range {
        ($module:ident, $entry_base:expr, $loop_base:expr) => {
            assert_eq!(
                $module::entry(0).0,
                $entry_base,
                "{} entry(0) should be {}",
                stringify!($module),
                $entry_base
            );
            assert_eq!(
                $module::loop_step(999).0,
                $loop_base + 999,
                "{} loop_step(999) should be {}",
                stringify!($module),
                $loop_base + 999
            );
        };
    }

    #[test]
    fn test_value_id_ranges_no_overlap() {
        // Test each module's range boundaries
        test_value_id_range!(skip_ws, 3000, 4000);
        test_value_id_range!(funcscanner_trim, 5000, 6000);
        test_value_id_range!(stage1_using_resolver, 7000, 8000);
        test_value_id_range!(funcscanner_append_defs, 9000, 10000);
        test_value_id_range!(stageb_body_extract, 11000, 12000);
        test_value_id_range!(stageb_funcscanner, 13000, 14000);

        // Automated overlap detection
        // Each range is 2000 units: (base, base+1999)
        let ranges = vec![
            (3000, 4999),   // skip_ws
            (5000, 6999),   // funcscanner_trim
            (7000, 8999),   // stage1_using_resolver
            (9000, 10999),  // funcscanner_append_defs
            (11000, 12999), // stageb_body_extract
            (13000, 14999), // stageb_funcscanner
        ];

        // Verify no overlaps between consecutive ranges
        for i in 0..ranges.len() - 1 {
            let (_, end_i) = ranges[i];
            let (start_next, _) = ranges[i + 1];
            assert!(
                end_i < start_next,
                "Overlap detected: range {} ends at {} but range {} starts at {}",
                i,
                end_i,
                i + 1,
                start_next
            );
        }
    }
}
