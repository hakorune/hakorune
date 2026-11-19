//! Lower an already located local declaration and publish its completed binding.

use std::{cell::RefCell, ops::ControlFlow, rc::Rc};

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::ExprChildRoleV1;

use super::normal_callable_binding_materialization::freeze;
use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1,
};
use super::recursive_child_lowering::{RawInvocationChildPortV1, RecursiveChildLoweringPortV1};
use super::stmts::variable_stmt::LocalValuePlacement;
use super::stmts::{
    drive_local_statement_with_placement_v1, CompletedLocalStatementV1, RawLegacyLocalInputV1,
};
use super::{MirBuilder, ValueId};

impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn lower_source_local_if_present_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ControlFlow<ValueId, ASTNode>, String> {
        match input {
            local @ ASTNode::Local { .. }
                if self.callable_ledger.is_some() || self.semantic_ledger.is_some() =>
            {
                Ok(ControlFlow::Break(
                    self.lower_source_local_v1(builder, local)?.result(),
                ))
            }
            other => Ok(ControlFlow::Continue(other)),
        }
    }

    /// The source frame owns the binding. An enclosing Script can retain its
    /// ledger while a constructor lowers a distinct function-body site.
    pub(in crate::mir::builder) fn lower_source_local_v1(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<CompletedLocalStatementV1, String> {
        let source = self.active_source.as_ref();
        let script_root = matches!(
            source.and_then(RawInvocationSourceContextV1::root_lineage),
            Some(RawInvocationRootLineageV1::ScriptRoot)
        );
        let callable = self
            .callable_ledger
            .as_ref()
            .filter(|_| !script_root || self.semantic_ledger.is_none())
            .cloned();
        let site = source
            .and_then(RawInvocationSourceContextV1::site)
            .cloned()
            .ok_or_else(|| match &callable {
                Some(_) => "[freeze:contract][callable-semantic-lowering/local-site]",
                None => "[freeze:contract][script-lexical/local-site]",
            })?;
        let (input, script_local) = if callable.is_none() {
            let ledger = self
                .semantic_ledger
                .clone()
                .expect("script local lowering requires semantic ledger");
            let relation = ledger
                .borrow()
                .local_relation(&site)
                .cloned()
                .ok_or("[freeze:contract][script-lexical/local-binding]")?;
            let initializer_source = if relation.initializer_site().is_some() {
                Some(self.prepare_expression_child_source_v1(
                    &input,
                    ExprChildRoleV1::LocalInitializer(0),
                )?)
            } else {
                None
            };
            let observations = Rc::new(RefCell::new(Vec::new()));
            let array_recipe = ledger
                .borrow_mut()
                .prepare_array_local_emission(builder, &relation)?;
            let input = RawLegacyLocalInputV1::from_script_relation(
                input,
                relation.clone(),
                &site,
                initializer_source.as_ref(),
                array_recipe,
            )?
            .observe_into(Rc::clone(&observations));
            (input, Some((ledger, relation, observations)))
        } else {
            let input = RawLegacyLocalInputV1::new(input)
                .observe_into(self.local_initializer_observation_sink());
            (input, None)
        };
        let claims = self.ordinary_new_claim_ledger.clone();
        let mut completed = drive_local_statement_with_placement_v1(
            builder,
            self,
            input,
            |ordinal, value| match (&callable, &claims) {
                (Some(ledger), Some(claims)) => ledger
                    .borrow()
                    .local_placement(&site, ordinal, value, claims),
                _ => Ok(LocalValuePlacement::Copy),
            },
        )?;
        if let Some((ledger, relation, observations)) = script_local {
            let value = completed.result();
            let observations = Rc::try_unwrap(observations)
                .map_err(|_| "[freeze:contract][script-array/observer-alias]")?
                .into_inner();
            let mut ledger = ledger.borrow_mut();
            ledger.record_array_local_emission(builder, &relation, value, observations)?;
            ledger.record(relation.binding(), value)?;
            ledger.complete_array_local(&relation)?;
            return Ok(completed);
        }
        let ledger = callable.expect("callable local lowering has a source ledger");
        ledger
            .borrow_mut()
            .record_completed_local(&site, &completed)?;
        if let Some(claims) = self.ordinary_new_claim_ledger.as_ref() {
            ledger
                .borrow()
                .record_completed_ordinary_new_local(&site, &completed, claims)?;
            for emission in completed.take_checked_initializers() {
                let receipt = completed
                    .bindings()
                    .iter()
                    .find(|row| row.ordinal() == emission.ordinal())
                    .ok_or_else(|| freeze("initializer-expression/completed-ordinal"))?;
                emission.publish(
                    receipt,
                    claims,
                    builder
                        .function_state
                        .current_function
                        .as_ref()
                        .ok_or_else(|| freeze("no-function"))?,
                )?;
            }
        } else if !completed.take_checked_initializers().is_empty() {
            return Err(freeze("initializer-expression/claim-ledger-missing"));
        }
        Ok(completed)
    }
}
