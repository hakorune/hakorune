//! Neutral JoinIR loop-target classification.
//!
//! This module owns only the five source-function identities selected for
//! Loop lowering and for exclusion from If lowering. VM execution policy is
//! intentionally outside this source-classification authority.

pub(crate) const MAIN_SKIP: &str = "Main.skip/1";
pub(crate) const FUNCSCANNER_TRIM: &str = "FuncScannerBox.trim/1";
pub(crate) const STAGE1_USING_RESOLVER: &str = "Stage1UsingResolverBox.resolve_for_source/5";
pub(crate) const STAGEB_BODY_EXTRACTOR: &str = "StageBBodyExtractorBox.build_body_src/2";
pub(crate) const STAGEB_FUNC_SCANNER: &str = "StageBFuncScannerBox.scan_all_boxes/1";

pub(crate) const LOOP_LOWERING_TARGETS: [&str; 5] = [
    MAIN_SKIP,
    FUNCSCANNER_TRIM,
    STAGE1_USING_RESOLVER,
    STAGEB_BODY_EXTRACTOR,
    STAGEB_FUNC_SCANNER,
];

pub(crate) fn is_loop_lowering_target(name: &str) -> bool {
    LOOP_LOWERING_TARGETS.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_keeps_exact_five_loop_targets() {
        assert_eq!(LOOP_LOWERING_TARGETS.len(), 5);
        for name in LOOP_LOWERING_TARGETS {
            assert!(is_loop_lowering_target(name));
        }
    }

    #[test]
    fn policy_excludes_if_and_arbitrary_functions() {
        for name in [
            "IfSelectTest.simple_return/0",
            "IfMergeTest.multiple_true/0",
            "JsonShapeToMap._read_value_from_pair/1",
            "FuncScannerBox.append_defs/2",
            "SomeBox.some_method/3",
            "Main.main/0",
        ] {
            assert!(!is_loop_lowering_target(name));
        }
    }
}
