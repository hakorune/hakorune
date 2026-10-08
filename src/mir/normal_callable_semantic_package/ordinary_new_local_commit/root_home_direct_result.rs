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
        self.direct
            .as_ref()
            .map_or(InvokeCallResultKind::I64, |(source, _)| source.kind())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn retain_direct_source(
        &mut self,
        source: DirectRootCleanupSourceV1,
    ) -> Result<(), String> {
        if self.direct.is_some()
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
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn bind_direct_result(
        &mut self,
        value: ValueId,
        kind: InvokeCallResultKind,
    ) -> Result<(), String> {
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
        let mut full: Vec<_> = source
            .end_plan(value)?
            .into_iter()
            .map(|(subject, operation)| RootHomeReleaseOriginV1 {
                subject,
                operation,
                exit: source.exit().clone(),
            })
            .collect();
        full.extend_from_slice(&self.full);
        let mut next = Self::from_sequences(full, &self.normal(), &self.acquisition_fault())?;
        next.direct = self.direct.take();
        next.direct.as_mut().unwrap().1 = Some(value);
        *self = next;
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_direct_entry(
        &self,
        entry: &RootHomeExitEntry,
        bindings: &[(BasicBlockId, MirInstruction)],
        mapping: Option<&FinishedBindings>,
    ) -> Result<(), String> {
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
