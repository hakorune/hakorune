//! Shared (`Shared(...)`) Local initializer plans built from a checked source.
use super::*;

impl HomeExpressionPlanV1 {
    pub(in super::super) fn shared_fresh_initializer(
        source: Rc<ExpressionSourcePlanV1>,
        shared: &crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
        payload: crate::mir::resolved_semantics::OwnedExprSiteV1,
        prior_homes: Rc<[LiveHomeV1]>,
    ) -> Option<Self> {
        if source.root().initializer()?.binding().owner() != source.owner()
            || payload.owner() != source.owner()
            || shared.operator() != crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Shared
            || source.nodes().first()?.site() != shared.site() || shared.operand() != payload.site()
            || !source.nodes().iter().any(|node| node.site() == payload.site()) { return None; }
        let plan = Self { source: Rc::clone(&source), scalar_operations: Vec::new(), source_fields: Vec::new(),
            print_frame: None, argument_frame: None,
            initializer_frame: Some(InitializerFrameV1::SharedFresh { source, shared: shared.clone(), payload, prior_homes }),
            rebind_frame: None, condition_frame: None, loop_recipe: None, field_write: None, source_field_homes: None };
        plan.is_consistent().then_some(plan)
    }
    pub(in super::super) fn shared_clone_initializer(
        source: Rc<ExpressionSourcePlanV1>, from: crate::mir::resolved_semantics::BindingRefV1,
        acquisition: crate::mir::resolved_semantics::OwnedExprSiteV1, prior_homes: Rc<[LiveHomeV1]>,
    ) -> Option<Self> {
        let initializer = source.root().initializer()?;
        if initializer.binding().owner() != source.owner() || from.owner() != source.owner()
            || acquisition.owner() != source.owner() || source.nodes().len() != 1
            || !matches!(source.nodes()[0].source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == from)
            || prior_homes.iter().filter(|home| home.binding() == from).count() != 1 { return None; }
        let plan = Self {
            source: Rc::clone(&source), scalar_operations: Vec::new(), source_fields: Vec::new(),
            print_frame: None, argument_frame: None,
            initializer_frame: Some(InitializerFrameV1::SharedClone { source, from, acquisition, prior_homes }),
            rebind_frame: None, condition_frame: None, loop_recipe: None, field_write: None,
            source_field_homes: None,
        };
        plan.is_consistent().then_some(plan)
    }
    pub(in super::super) fn shared_move_initializer(
        source: Rc<ExpressionSourcePlanV1>,
        shared: &crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
        moved: &crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
        from: crate::mir::resolved_semantics::BindingRefV1,
        acquisition: crate::mir::resolved_semantics::OwnedExprSiteV1,
        prior_homes: Rc<[LiveHomeV1]>,
    ) -> Option<Self> {
        let initializer = source.root().initializer()?;
        if initializer.binding().owner() != source.owner() || from.owner() != source.owner()
            || acquisition.owner() != source.owner() || source.nodes().len() != 3
            || !matches!(source.nodes()[0].source(), ExpressionNodeSourceV1::Unary)
            || !matches!(source.nodes()[1].source(), ExpressionNodeSourceV1::Unary)
            || shared.operator() != crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Shared
            || moved.operator() != crate::mir::resolved_semantics::ResolvedUnaryOperatorV1::Move
            || shared.site() != source.nodes()[0].site() || shared.operand() != moved.site()
            || moved.site() != source.nodes()[1].site() || moved.operand() != source.nodes()[2].site()
            || !matches!(source.nodes()[2].source(), ExpressionNodeSourceV1::Reference(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding)) if *binding == from)
            || prior_homes.iter().filter(|home| home.binding() == from).count() != 1 {
            return None;
        }
        let plan = Self {
            source: Rc::clone(&source), scalar_operations: Vec::new(), source_fields: Vec::new(),
            print_frame: None, argument_frame: None,
            initializer_frame: Some(InitializerFrameV1::SharedMove {
                source, shared: shared.clone(), moved: moved.clone(), from, acquisition, prior_homes,
            }),
            rebind_frame: None, condition_frame: None, loop_recipe: None, field_write: None,
            source_field_homes: None,
        };
        plan.is_consistent().then_some(plan)
    }
}
