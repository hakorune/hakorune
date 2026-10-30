//! Phase 29ao P0: CorePlan composer scaffold (CanonicalLoopFacts -> CorePlan)
//!
//! SSOT: docs/development/current/main/design/plan-dir-shallowing-ssot.md
//! Old cfg-test versioned coreloop shelves retired in 291x-754.

mod branchn_return;

pub(in crate::mir::builder) use branchn_return::{compose_match_return_branchn, MatchReturnPlan};
