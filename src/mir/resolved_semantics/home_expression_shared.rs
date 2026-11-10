//! Shared local initializer plan built from a checked source.
//! `is_consistent` is the single judge of every form (fresh, move, clone).
use super::*;

impl HomeExpressionPlanV1 {
    pub(in crate::mir::resolved_semantics) fn shared_initializer(
        source: Rc<ExpressionSourcePlanV1>,
        payload_source: super::super::SharedPayloadSourceV1,
        shared: crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
        moved: Option<crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1>,
        prior_homes: Rc<[LiveHomeV1]>,
    ) -> Option<Self> {
        let plan = Self {
            source: Rc::clone(&source),
            scalar_operations: Vec::new(),
            source_fields: Vec::new(),
            print_frame: None,
            argument_frame: None,
            initializer_frame: Some(InitializerFrameV1::Shared {
                source,
                payload_source,
                shared,
                moved,
                prior_homes,
            }),
            rebind_frame: None,
            condition_frame: None,
            loop_recipe: None,
            field_write: None,
            source_field_homes: None,
        };
        plan.is_consistent().then_some(plan)
    }
}

impl HomeExpressionPlanV1 {
    pub(super) fn shared_clone_source_is_consistent_v1(
        source: &Rc<ExpressionSourcePlanV1>,
        from: crate::mir::resolved_semantics::BindingRefV1,
        acquisition: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        shared: &crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
        prior_homes: &[LiveHomeV1],
    ) -> bool {
        let nodes = source.nodes();
        from.owner() == source.owner()
            && acquisition.owner() == source.owner()
            && prior_homes
                .iter()
                .filter(|home| home.binding() == from)
                .count()
                == 1
            && nodes.len() == 2
            && shared.operator() == crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Shared
            && shared.site() == nodes[0].site()
            && shared.operand() == nodes[1].site()
            && matches!(nodes[1].source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == from)
    }
}

impl HomeExpressionPlanV1 {
    /// This bounded operation has one source position: a complete Local initializer.
    /// Check it once before expression-purpose dispatch can obscure its diagnostic.
    pub(crate) fn validate_shared_local_positions_v1(
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<(), super::super::HomePrefixUnavailableV1> {
        let expressions = input.function().expression_source();
        for unary in expressions.unaries().filter(|unary| {
            unary.operator() == crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Shared
        }) {
            if !expressions.initializers().any(|initializer| {
                initializer.initializer_site() == Some(unary.site())
                    && matches!(
                        initializer.declaration_site(),
                        crate::mir::resolved_semantics::SourceBindingSiteV1::Local { .. }
                    )
            }) {
                return Err(
                    super::super::HomePrefixUnavailableV1::SharedOperandUnsupported {
                        site: unary.site().clone(),
                        guidance: "share is supported only as a complete local initializer",
                    },
                );
            }
        }
        Ok(())
    }
}
