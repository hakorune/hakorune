//! Attach the original call result only to post-acquisition Fault obligations.
use super::super::super::super::completion_index::DirectRootCleanupSourceV1;
use super::super::super::physical_boundary::FinishedBindings;
use super::super::{RootHomeExitEntry, RootHomeReleaseSubjectV1};
use super::*;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};

impl RootHomeCleanupOrderV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn ingress_result_kind(
        &self,
    ) -> InvokeCallResultKind {
        if let Some((_, loan, _)) = &self.scalar {
            return match loan.kind() {
                crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer => InvokeCallResultKind::I64,
                crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Bool => InvokeCallResultKind::Bool,
            };
        }
        if let Some(source) = &self.stored_map_get {
            return InvokeCallResultKind::BorrowedNullableHandle { support: source.parts().0.result_support_v1() };
        }
        self.direct
            .as_ref()
            .map_or(InvokeCallResultKind::I64, |(source, _)| source.kind())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn retain_direct_source(
        &mut self,
        source: DirectRootCleanupSourceV1,
    ) -> Result<(), String> {
        if self.direct.is_some() || self.scalar.is_some()
            || self
                .full
                .iter()
                .any(|origin| origin.exit() != source.exit())
        {
            return Err(freeze("direct-result-source-drift"));
        }
        self.direct = Some((source, None));
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn retain_scalar_source(
        &mut self,
        exit: crate::mir::resolved_semantics::SourceStmtSiteV1,
        loan: crate::mir::normal_callable_semantic_package::BorrowedScalarResultSourceLoanV1,
    ) -> Result<(), String> {
        if self.direct.is_some() || self.scalar.is_some()
            || self.full.iter().any(|origin| origin.exit() != &exit || matches!(origin.subject(),
                RootHomeReleaseSubjectV1::DirectResult { .. } | RootHomeReleaseSubjectV1::DirectResultField { .. }))
        {
            return Err(freeze("scalar-result-source-drift"));
        }
        self.scalar = Some((exit, loan, None));
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn bind_direct_result(
        &mut self,
        value: ValueId,
        kind: InvokeCallResultKind,
    ) -> Result<(), String> {
        if self.scalar.is_some() {
            if self.ingress_result_kind() != kind || self.scalar.as_ref().unwrap().2.is_some() {
                return Err(freeze("scalar-result-duplicate-or-kind"));
            }
            // Scalar Normal results acquire no Home and no result End obligation.
            self.scalar.as_mut().unwrap().2 = Some(value);
            return Ok(());
        }
        let Some((source, bound)) = &self.direct else {
            return if kind == InvokeCallResultKind::I64 {
                Ok(())
            } else {
                Err(freeze("direct-result-source-missing"))
            };
        };
        if bound.is_some() || source.kind() != kind {
            return Err(freeze("direct-result-duplicate-or-kind"));
        }
        let prefix: Vec<_> = source
            .end_plan(value)?
            .into_iter()
            .map(|(subject, operation)| RootHomeReleaseOriginV1 {
                subject,
                operation,
                exit: source.exit().clone().into(),
            })
            .collect();
        self.prepend_fault_only_result(prefix)?;
        self.direct.as_mut().unwrap().1 = Some(value);
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_direct_entry(
        &self,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        exit: &crate::mir::resolved_semantics::SourceStmtSiteV1,
        entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        mapping: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        self.validate_fresh_result_cleanup(ledger, owner, exit, entry)?;
        if let Some((original_exit, loan, value)) = &self.scalar {
            let current = ledger.verified_terminal_call_source_v1(owner, exit)?
                .ok_or_else(|| freeze("scalar-result-terminal-missing"))?;
            if original_exit != exit || loan.source().call_site().owner() != owner
                || current.legacy_terminal().and_then(|terminal| terminal.source_scalar()) != Some(loan)
                || current.result() != self.ingress_result_kind()
                || ledger.callable_scalar_result_source_v1(loan.source().callee_owner())?.is_none()
            { return Err(freeze("scalar-result-terminal-drift")); }
            let value = value.ok_or_else(|| freeze("scalar-result-unbound"))?;
            let RootHomeExitEntry::Call { row: crate::mir::normal_callable_semantic_package::RootCallDispositionV1::Lexical(packet), invoke, projection, .. } = entry
            else { return Err(freeze("scalar-result-entry-kind")); };
            let row = packet.original_row()?;
            if row.scalar_result_source_v1() != Some(loan) || row.source_target() != loan.source()
                || row.result() != Some(self.ingress_result_kind())
                || packet.call_site() != loan.source().call_site()
                || self.full.iter().any(|origin| matches!(origin.subject(),
                    RootHomeReleaseSubjectV1::DirectResult { .. } | RootHomeReleaseSubjectV1::DirectResultField { .. }))
            { return Err(freeze("scalar-result-packet-identity")); }
            let project = |binding: &(BasicBlockId, MirInstruction)| match mapping {
                Some(mapping) => mapping.binding(binding.0, &binding.1)?.ok_or_else(|| freeze("scalar-result-binding-missing")),
                None => Ok(binding.clone()),
            };
            let invoke = project(invoke)?;
            let projection = project(projection)?;
            let returns = bindings.iter().filter_map(|binding| match &binding.1 {
                MirInstruction::Return { .. } => Some(project(binding)), _ => None,
            }).collect::<Result<Vec<_>, _>>()?;
            if !matches!(&invoke.1, MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { result, call }, .. }
                if *result == self.ingress_result_kind() && call.dst.is_none())
                || !matches!(projection.1, MirInstruction::InvokeNormalResult { dst, invoke_block } if dst == value && invoke_block == invoke.0)
                || returns.len() != 1 || !matches!(returns[0].1, MirInstruction::Return { value: Some(dst) } if dst == value)
            { return Err(freeze("scalar-result-physical-attachment")); }
            return Ok(());
        }
        let Some((source, value)) = &self.direct else {
            return Ok(());
        };
        let value = value.ok_or_else(|| freeze("direct-result-unbound"))?;
        let RootHomeExitEntry::Call {
            row,
            invoke,
            projection,
            ..
        } = entry
        else {
            return Err(freeze("direct-result-entry-kind"));
        };
        let crate::mir::normal_callable_semantic_package::RootCallDispositionV1::Lexical(packet) =
            row
        else {
            return Err(freeze("direct-result-call-source"));
        };
        if packet.call_site() != source.call() {
            return Err(freeze("direct-result-call-site"));
        }
        let map = |binding: &(BasicBlockId, MirInstruction)| match mapping {
            Some(p) => p
                .binding(binding.0, &binding.1)?
                .ok_or_else(|| freeze("direct-result-binding-missing")),
            None => Ok(binding.clone()),
        };
        let invoke = map(invoke)?;
        let projection = map(projection)?;
        if !matches!(&invoke.1, MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::Call { result, call }, .. } if *result == source.kind() && call.dst.is_none())
            || !matches!(projection.1, MirInstruction::InvokeNormalResult { dst, invoke_block } if dst == value && invoke_block == invoke.0)
            || !bindings.iter().any(
                |(_, i)| matches!(i, MirInstruction::Return { value: Some(dst) } if *dst == value),
            )
        {
            return Err(freeze("direct-result-physical-attachment"));
        }
        let expected = source.end_plan(value)?;
        if self.full.len() < expected.len()
            || !self
                .full
                .iter()
                .zip(expected.iter())
                .all(|(origin, (subject, operation))| {
                    origin.subject() == subject && origin.operation() == operation
                })
            || self
                .normal()
                .iter()
                .chain(self.acquisition_fault().iter())
                .any(|origin| {
                    matches!(
                        origin.subject(),
                        RootHomeReleaseSubjectV1::DirectResult { .. }
                            | RootHomeReleaseSubjectV1::DirectResultField { .. }
                    )
                })
        {
            return Err(freeze("direct-result-cleanup-order"));
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "root_home_direct_result_tests.rs"]
mod tests;
