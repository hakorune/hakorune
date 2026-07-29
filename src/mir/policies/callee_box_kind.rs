//! Callee Box-kind classification policy.
//!
//! This is the sole decision owner for the static-compiler, runtime-data, and
//! user-defined Box partitions used by MIR call construction.  Callers select
//! their already-existing context once; this policy never retries another
//! context after classification.

use crate::mir::definitions::call_unified::CalleeBoxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CalleeBoxKindPolicyContextV1 {
    GeneralEmission,
    ResolverExtendedCompiler,
}

pub(crate) fn classify_callee_box_kind_v1(
    context: CalleeBoxKindPolicyContextV1,
    box_name: &str,
) -> CalleeBoxKind {
    use CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler;

    match (context, box_name) {
        // Bounded compatibility surface. Growth is forbidden; see
        // CALL-BOX-KIND-ANALYZER-COMPAT-SUNSET-001 in the active workstream.
        (ResolverExtendedCompiler, "BreakFinderBox" | "PhiInjectorBox" | "LoopSSA") => {
            CalleeBoxKind::StaticCompiler
        }

        (
            _,
            "StageBArgsBox"
            | "StageBBodyExtractorBox"
            | "StageBDriverBox"
            | "Stage1UsingResolverBox"
            | "BundleResolver"
            | "ParserBox"
            | "ParserStmtBox"
            | "ParserExprBox"
            | "ParserControlBox"
            | "ParserLiteralBox"
            | "ParserTokenBox"
            | "FuncScannerBox"
            | "MirBuilderBox"
            | "JsonFragBox"
            | "JsonCursorBox"
            | "JsonScanBox"
            | "PatternUtilBox"
            | "MethodAliasPolicy"
            | "StringHelpers"
            | "StringOps"
            | "StringScanBox"
            | "StringifyOperator"
            | "AddOperator"
            | "CompareOperator",
        ) => CalleeBoxKind::StaticCompiler,

        (
            _,
            "MapBox" | "ArrayBox" | "StringBox" | "IntegerBox" | "BoolBox" | "FloatBox" | "NullBox"
            | "VoidBox" | "UnknownBox" | "FileBox" | "ConsoleBox" | "PathBox",
        ) => CalleeBoxKind::RuntimeData,

        (_, _) => CalleeBoxKind::UserDefined,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_box_kinds_are_identical_in_both_contexts() {
        let cases = [
            ("StageBArgsBox", CalleeBoxKind::StaticCompiler),
            ("ParserBox", CalleeBoxKind::StaticCompiler),
            ("MapBox", CalleeBoxKind::RuntimeData),
            ("UnknownBox", CalleeBoxKind::RuntimeData),
            ("RuntimeDataBox", CalleeBoxKind::UserDefined),
            ("MyCustomBox", CalleeBoxKind::UserDefined),
        ];
        for context in [
            CalleeBoxKindPolicyContextV1::GeneralEmission,
            CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler,
        ] {
            for (box_name, expected) in cases {
                assert_eq!(
                    classify_callee_box_kind_v1(context, box_name),
                    expected,
                    "context={context:?} box={box_name}"
                );
            }
        }
    }

    #[test]
    fn analyzer_compatibility_surface_is_context_bounded() {
        for box_name in ["BreakFinderBox", "PhiInjectorBox", "LoopSSA"] {
            assert_eq!(
                classify_callee_box_kind_v1(
                    CalleeBoxKindPolicyContextV1::ResolverExtendedCompiler,
                    box_name,
                ),
                CalleeBoxKind::StaticCompiler
            );
            assert_eq!(
                classify_callee_box_kind_v1(
                    CalleeBoxKindPolicyContextV1::GeneralEmission,
                    box_name,
                ),
                CalleeBoxKind::UserDefined
            );
        }
    }
}
