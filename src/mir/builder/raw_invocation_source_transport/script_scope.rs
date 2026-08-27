use super::*;

impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn lower_script_binding_rebind_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        let role = match &input {
            ASTNode::Assignment { target, .. }
                if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
            {
                ExprChildRoleV1::AssignmentTarget
            }
            ASTNode::CompoundAssignment { target, .. }
                if matches!(target.as_ref(), ASTNode::Variable { .. }) =>
            {
                ExprChildRoleV1::CompoundAssignmentTarget
            }
            ASTNode::GroupedAssignmentExpr { .. } => ExprChildRoleV1::GroupedAssignmentTarget,
            _ => return Err("[freeze:contract][script-lexical/rebind-source-drift]".to_owned()),
        };
        let ledger = self
            .semantic_ledger
            .clone()
            .expect("script rebind lowering requires semantic ledger");
        let target_site = self
            .current_source_context_v1()
            .ok_or_else(|| "[freeze:contract][script-lexical/rebind-parent-site]".to_owned())?
            .child_expression(&input, role)?
            .site()
            .cloned()
            .ok_or_else(|| "[freeze:contract][script-lexical/rebind-target-site]".to_owned())?;
        let binding = ledger
            .borrow()
            .assignment_binding(&target_site)
            .ok_or_else(|| "[freeze:contract][script-lexical/rebind-binding]".to_owned())?;
        let value = lower_raw_expression_with_recursion_guard_v1(builder, self, input)?;
        ledger.borrow_mut().rebind(binding, value)?;
        Ok(value)
    }

    pub(in crate::mir::builder) fn with_script_semantic_source_v1<R>(
        &mut self,
        source: CanonicalScriptCPreparedLoweringSourceV1<'_>,
        execute: impl FnOnce(&mut Self) -> Result<R, String>,
    ) -> Result<R, String> {
        let [root] = source.source().forest().roots() else {
            return Err("[freeze:contract][mir/script-semantic/root-cardinality]".to_owned());
        };
        if source
            .source()
            .projection()
            .owner_root(source.source().source(), *root)
            .is_err()
            || source
                .source()
                .runtime_source_indices()
                .iter()
                .any(|index| {
                    !matches!(
                        source.source().source(),
                        ASTNode::Program { statements, .. } if statements.get(*index).is_some()
                    )
                })
        {
            return Err("[freeze:contract][mir/script-semantic/source-proof]".to_owned());
        }
        let state = Rc::new(RefCell::new(ScriptSemanticLoweringState::new(
            source.into_lowering_input(),
        )?));
        let finish_state = Rc::clone(&state);
        let parent = std::mem::replace(&mut self.semantic_ledger, Some(state));
        let result = self.with_source_transport_v1(
            RawInvocationSourceTransportV1::script_semantic_root(()),
            |port, ()| execute(port),
        );
        let result = match result {
            Ok(value) => finish_state
                .try_borrow_mut()
                .map_err(|_| {
                    "[freeze:contract][script-direct-static/claim-finish-borrow]".to_owned()
                })?
                .finish_direct_static_claims()
                .map(|()| value),
            Err(error) => Err(error),
        };
        self.semantic_ledger = parent;
        result
    }
}
