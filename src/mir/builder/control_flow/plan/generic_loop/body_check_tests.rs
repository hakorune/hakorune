//! Tests for body_check module

#[cfg(test)]
mod tests {
    use crate::mir::builder::control_flow::plan::generic_loop::body_check::shape_resolution::resolve_v1_shape_matches;
    use crate::mir::policies::GenericLoopV1ShapeId;

    #[test]
    fn generic_loop_v1_shape_overlap_freezes() {
        let matches = vec![
            GenericLoopV1ShapeId::ParseMap,
            GenericLoopV1ShapeId::ParseBlockExpr,
        ];
        let err = resolve_v1_shape_matches(&matches).expect_err("overlap should freeze");
        assert!(err.to_string().contains("shape overlap"));
    }
}
