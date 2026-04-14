use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::policies::GenericLoopV1ShapeId;

pub(in crate::mir::builder) fn detect_generic_loop_v1_shape(
    body: &[ASTNode],
    loop_var: &str,
    loop_increment: &ASTNode,
    condition: &ASTNode,
) -> Result<Option<GenericLoopV1ShapeId>, Freeze> {
    let mut matches = Vec::new();
    if super::super::body_check_shape_detectors::matches_parse_block_expr_shape(
        body,
        loop_var,
        loop_increment,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseBlockExpr);
    }
    if super::super::body_check_shape_detectors::matches_parse_map_shape(
        body,
        loop_var,
        loop_increment,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseMap);
    }
    if super::super::body_check_shape_detectors::matches_peek_parse_shape(
        body,
        loop_var,
        loop_increment,
    ) {
        matches.push(GenericLoopV1ShapeId::PeekParse);
    }
    if super::super::body_check_shape_detectors::matches_rewriteknown_itoa_complex_step_shape(
        body,
        loop_var,
        loop_increment,
    ) {
        matches.push(GenericLoopV1ShapeId::RewriteKnownItoaComplexStep);
    }
    if super::super::body_check_shape_detectors::matches_rewriteknown_trim_and_methodcall_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::RewriteKnownTrimLoopCondAndMethodCall);
    }
    if super::super::body_check_shape_detectors::matches_while_cap_accum_sum_shape(
        body,
        loop_var,
        loop_increment,
    ) {
        matches.push(GenericLoopV1ShapeId::WhileCapAccumSum);
    }
    if super::super::body_check_shape_detectors::matches_decode_escapes_loop_shape(
        body,
        loop_var,
        loop_increment,
    ) {
        matches.push(GenericLoopV1ShapeId::DecodeEscapesLoop);
    }
    if super::super::body_check_shape_detectors::matches_scan_all_boxes_next_i_shape(
        body,
        loop_var,
        loop_increment,
    ) {
        matches.push(GenericLoopV1ShapeId::ScanAllBoxesNextI);
    }
    if super::super::body_check_shape_detectors::matches_scan_while_predicate_shape(
        body,
        loop_var,
        loop_increment,
    ) {
        matches.push(GenericLoopV1ShapeId::ScanWhilePredicate);
    }
    if super::super::body_check_shape_detectors::matches_effect_step_only_shape(
        body,
        loop_var,
        loop_increment,
    ) {
        matches.push(GenericLoopV1ShapeId::EffectStepOnly);
    }
    if super::super::body_check_shape_detectors::matches_div_countdown_by10_shape(
        body,
        loop_var,
        loop_increment,
    ) {
        matches.push(GenericLoopV1ShapeId::DivCountdownBy10);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_return_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfReturn);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_else_return_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfElseReturn);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_return_var_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfReturnVar);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_return_local_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfReturnLocal);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_else_return_var_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfElseReturnVar);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_else_return_local_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfElseReturnLocal);
    }
    if super::super::body_check_shape_detectors::matches_parse_program2_nested_loop_if_else_if_return_shape(
        body,
        loop_var,
        loop_increment,
        condition,
    ) {
        matches.push(GenericLoopV1ShapeId::ParseProgram2NestedLoopIfElseIfReturn);
    }

    resolve_v1_shape_matches(&matches)
}

pub fn resolve_v1_shape_matches(
    matches: &[GenericLoopV1ShapeId],
) -> Result<Option<GenericLoopV1ShapeId>, Freeze> {
    use crate::mir::policies::generic_loop_overlap_policy::{
        classify_v1_shape_matches, V1ShapeMatch,
    };

    match classify_v1_shape_matches(matches) {
        V1ShapeMatch::None => Ok(None),
        V1ShapeMatch::Single(shape) => Ok(Some(shape)),
        V1ShapeMatch::Overlap(overlaps) => Err(Freeze::ambiguous(format!(
            "generic_loop_v1: shape overlap: {}",
            overlaps
                .iter()
                .map(|shape| shape.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ))),
    }
}
