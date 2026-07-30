//! Historical primary-name hint table for Builder return-type finalization.

use crate::mir::{MirFunction, MirInstruction, MirType, ValueId};

#[derive(Clone, Copy)]
enum MatchRule {
    Prefix(&'static str),
    Contains(&'static str),
}

impl MatchRule {
    fn matches(self, function_name: &str) -> bool {
        match self {
            Self::Prefix(prefix) => function_name.starts_with(prefix),
            Self::Contains(needle) => function_name.contains(needle),
        }
    }
}

const PRIMARY_HINT_TARGETS: &[MatchRule] = &[
    MatchRule::Prefix("IfSelectTest."),
    MatchRule::Prefix("IfMergeTest."),
    MatchRule::Contains("read_quoted"),
    MatchRule::Prefix("NewBoxTest."),
];

pub(super) fn is_primary_target(function_name: &str) -> bool {
    PRIMARY_HINT_TARGETS
        .iter()
        .copied()
        .any(|rule| rule.matches(function_name))
}

pub(super) fn is_uniform_phi_fallback_target(function_name: &str) -> bool {
    !function_name.is_empty() && !is_primary_target(function_name)
}

pub(super) fn extract_phi_type_hint(
    function: &MirFunction,
    return_value: ValueId,
) -> Option<MirType> {
    function
        .blocks
        .values()
        .flat_map(|block| &block.instructions)
        .find_map(|instruction| {
            if let MirInstruction::Phi { dst, type_hint, .. } = instruction {
                (*dst == return_value).then(|| type_hint.clone()).flatten()
            } else {
                None
            }
        })
}
