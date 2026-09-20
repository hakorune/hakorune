use super::*;

impl RawInvocationChildPortV1<'_, '_> {
    pub(in crate::mir::builder) fn issue_callable_loop_binding_schedule_v1(
        &self,
        condition: &ASTNode,
    ) -> Result<
        Option<crate::mir::builder::normal_callable_loop_handoff::
            CallableLoopBindingProjectionDispositionV1>,
        String,
    >{
        let Some(ledger) = self.callable_ledger.as_ref() else {
            return Ok(None);
        };
        let loop_site = self
            .active_source
            .as_ref()
            .and_then(RawInvocationSourceContextV1::site)
            .cloned()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop-handoff/missing-loop-source]".to_owned()
            })?;
        let state = ledger.borrow();
        let projection = state.loop_binding_source_projection();
        let disposition = if matches!(
            condition,
            ASTNode::Literal {
                value: crate::ast::LiteralValue::Bool(true),
                ..
            }
        ) {
            projection.project_loop_true_disposition(loop_site)
        } else {
            projection.project_disposition(loop_site)
        }?;
        Ok(Some(disposition))
    }
}
