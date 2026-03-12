//! SSOT: generic_loop_v1 shape identifiers.
//! See docs/development/current/main/design/generic-loop-v1-shape-ssot.md

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericLoopV1ShapeId {
    ParseBlockExpr,
    ParseMap,
    PeekParse,
    RewriteKnownItoaComplexStep,
    RewriteKnownTrimLoopCondAndMethodCall,
    ParseProgram2NestedLoopIfReturn,
    ParseProgram2NestedLoopIfElseReturn,
    ParseProgram2NestedLoopIfReturnVar,
    ParseProgram2NestedLoopIfReturnLocal,
    ParseProgram2NestedLoopIfElseReturnVar,
    ParseProgram2NestedLoopIfElseReturnLocal,
    ParseProgram2NestedLoopIfElseIfReturn,
    WhileCapAccumSum,
    UsingCollectorLineScan,
    ScanAllBoxesNextI,
    DecodeEscapesLoop,
    DivCountdownBy10,
    ScanWhilePredicate,
    EffectStepOnly,
}

impl GenericLoopV1ShapeId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ParseBlockExpr => "parse_block_expr",
            Self::ParseMap => "parse_map",
            Self::PeekParse => "peek_parse",
            Self::RewriteKnownItoaComplexStep => "rewriteknown_itoa_complex_step",
            Self::RewriteKnownTrimLoopCondAndMethodCall => {
                "rewriteknown_trim_loop_cond_and_methodcall"
            }
            Self::ParseProgram2NestedLoopIfReturn => "parse_program2_nested_loop_if_return",
            Self::ParseProgram2NestedLoopIfElseReturn => {
                "parse_program2_nested_loop_if_else_return"
            }
            Self::ParseProgram2NestedLoopIfReturnVar => "parse_program2_nested_loop_if_return_var",
            Self::ParseProgram2NestedLoopIfReturnLocal => {
                "parse_program2_nested_loop_if_return_local"
            }
            Self::ParseProgram2NestedLoopIfElseReturnVar => {
                "parse_program2_nested_loop_if_else_return_var"
            }
            Self::ParseProgram2NestedLoopIfElseReturnLocal => {
                "parse_program2_nested_loop_if_else_return_local"
            }
            Self::ParseProgram2NestedLoopIfElseIfReturn => {
                "parse_program2_nested_loop_if_else_if_return"
            }
            Self::WhileCapAccumSum => "while_cap_accum_sum",
            Self::UsingCollectorLineScan => "usingcollector_line_scan",
            Self::ScanAllBoxesNextI => "scan_all_boxes_next_i",
            Self::DecodeEscapesLoop => "decode_escapes_loop",
            Self::DivCountdownBy10 => "div_countdown_by10",
            Self::ScanWhilePredicate => "scan_while_predicate",
            Self::EffectStepOnly => "effect_step_only",
        }
    }
}

impl fmt::Display for GenericLoopV1ShapeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
