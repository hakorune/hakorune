//! Exact source cleanup receipts through original and finishing consumers.
//! No source operation, Home obligation or replacement Invoke is inferred from MIR.
use super::*;
use crate::mir::builder::EmittedSourceLoopCleanupV2;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit::{
    physical_boundary::{self, FinishedBindings},
    root_cleanup_graph::ordered_structure,
};
use std::collections::{BTreeMap, BTreeSet};
type Binding = (BasicBlockId, MirInstruction);

impl SourceLoopCleanupPlanV2 {
    pub(in crate::mir) fn validate_original_emitted_v2(
        &self,
        emitted: &EmittedSourceLoopCleanupV2,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
    ) -> Result<(), String> {
        self.validate_emitted_v2(emitted, context, function, None)
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_emitted_v2(
        &self,
        emitted: &EmittedSourceLoopCleanupV2,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&FinishedBindings>,
    ) -> Result<(), String> {
        if !std::ptr::eq(self, &emitted.plan) {
            return Err(freeze("source-loop/cleanup-plan-substitution"));
        }
        context.require_original_source_loop_v2(&self.product)?;
        context.require_source_loop_parent_v2(
            &self.product,
            matches!(context, CallValidationContextV1::Finalized(_)),
        )?;
        let port = self
            .product
            .source
            .ports
            .get(self.port)
            .ok_or_else(|| freeze("source-loop/cleanup-port"))?;
        let homes: Vec<_> = self.product.port_fault_homes_v2(self.port)?.into_iter().rev().collect();
        let expected = super::super::super::source_home_end_selection_v2(
            &context.ledger().local_commits.borrow(),
            &homes,
            port.normal_ends(),
        )?
        .ok_or_else(|| freeze("source-loop/cleanup-end-unavailable"))?;
        if self.full != expected.0 || self.normal != expected.1 {
            return Err(freeze("source-loop/cleanup-source-plan-drift"));
        }
        if self.normal_is_empty() {
            if emitted.from != emitted.entry
                || emitted.from != emitted.continuation
                || !emitted.bindings.is_empty()
                || !emitted.operations.is_empty()
                || !emitted.normal_origins.is_empty()
            {
                return Err(freeze("source-loop/empty-cleanup-effects"));
            }
            return Ok(());
        }
        let finalized = matches!(context, CallValidationContextV1::Finalized(_));
        let coordinate = |id| match finishing {
            Some(projection) => projection
                .destination(id)
                .ok_or_else(|| freeze("source-loop/cleanup-destination")),
            None => Ok(id),
        };
        let effect = |row: &Binding| match finishing {
            Some(projection) if finalized => projection.required_effect_binding(function, row),
            Some(projection) => projection.captured_effect_binding(function, row),
            None => {
                if !physical_boundary::check_binding(function, None, row.0, &row.1)? {
                    return Err(freeze("source-loop/cleanup-effect"));
                }
                Ok(row.clone())
            }
        };
        let producer = |row: &Binding| match finishing {
            Some(projection) => projection.relation_binding(function, row),
            None => {
                if !physical_boundary::check_binding(function, None, row.0, &row.1)? {
                    return Err(freeze("source-loop/cleanup-producer"));
                }
                Ok(Some(row.clone()))
            }
        };
        let frame_binding = emitted
            .bindings
            .first()
            .ok_or_else(|| freeze("source-loop/cleanup-frame"))?;
        let MirInstruction::FaultFrameEnter { dst: frame, .. } = frame_binding.1 else {
            return Err(freeze("source-loop/cleanup-frame"));
        };
        if producer(frame_binding)?.is_none()
            || emitted
                .bindings
                .iter()
                .filter(|row| *row == frame_binding)
                .count()
                != 1
        {
            return Err(freeze("source-loop/cleanup-frame"));
        }
        let mut prefix_allowed = Vec::new();
        let mut mapped_operations = BTreeMap::new();
        for (index, loan) in emitted.operations.iter().enumerate() {
            if emitted
                .bindings
                .iter()
                .filter(|row| **row == loan.binding)
                .count()
                != 1
            {
                return Err(freeze("source-loop/cleanup-invoke-membership"));
            }
            let mut rewritten = loan.original.clone();
            let mut cursor = 0;
            let mut mismatch = false;
            rewritten.rewrite_values(|value| {
                match loan.operands.get(cursor) {
                    Some(relation) if relation.values().0 == *value => *value = relation.values().1,
                    _ => mismatch = true,
                }
                cursor += 1;
            });
            if mismatch || cursor != loan.operands.len() {
                return Err(freeze("source-loop/cleanup-operands"));
            }
            for relation in &loan.operands {
                if let Some(projection) = finishing {
                    relation.verify_projected(function, &mut |row| {
                        projection.relation_binding(function, row)
                    })?;
                } else {
                    relation.verify(function)?;
                }
                let mut observed = Vec::new();
                relation.append_observed_bindings_v1(&mut observed);
                for row in observed.into_iter().filter(|row| row.0 == loan.binding.0) {
                    if !prefix_allowed.contains(&row) {
                        prefix_allowed.push(row);
                    }
                }
            }
            let actual = effect(&loan.binding)?;
            let MirInstruction::Invoke {
                operation,
                fault_frame,
                ..
            } = &actual.1
            else {
                return Err(freeze("source-loop/cleanup-invoke"));
            };
            if operation != &rewritten
                || *fault_frame != frame
                || mapped_operations
                    .insert(actual.0, (index, loan.original.clone(), operation.clone()))
                    .is_some()
            {
                return Err(freeze("source-loop/cleanup-invoke-drift"));
            }
        }
        if emitted.bindings.last()
            != Some(&(
                emitted.from,
                MirInstruction::Jump {
                    target: emitted.entry,
                    edge_args: None,
                },
            ))
            || emitted.from == emitted.entry
            || emitted.from == emitted.continuation
            || emitted.entry == emitted.continuation
            || emitted
                .normal_origins
                .first()
                .is_none_or(|row| row.0 != emitted.entry)
        {
            return Err(freeze("source-loop/cleanup-ingress"));
        }
        let normal = self.normal_operations();
        if emitted.normal_origins.len() != normal.len() {
            return Err(freeze("source-loop/cleanup-normal-census"));
        }
        let mut normal_origins = BTreeSet::new();
        for (binding, expected) in emitted.normal_origins.iter().zip(&normal) {
            let loan = emitted
                .operations
                .iter()
                .find(|loan| &loan.binding == binding)
                .ok_or_else(|| freeze("source-loop/cleanup-normal-origin"))?;
            if &loan.original != expected || !normal_origins.insert(binding.0) {
                return Err(freeze("source-loop/cleanup-normal-origin"));
            }
        }
        let mut terminals: Vec<Binding> = Vec::new();
        let mut original_terminals = BTreeSet::new();
        let mut prefix = Vec::new();
        for row in emitted.bindings.iter().skip(1) {
            let mapped = match &row.1 {
                MirInstruction::Jump {
                    edge_args: None, ..
                } => {
                    if !original_terminals.insert(row.0) {
                        return Err(freeze("source-loop/cleanup-terminal-repeat"));
                    }
                    match finishing {
                        Some(projection) => {
                            projection.structural_jump_binding(function, row, finalized)?
                        }
                        None => Some(effect(row)?),
                    }
                }
                MirInstruction::Invoke { .. } | MirInstruction::ReturnFault { .. } => {
                    if !original_terminals.insert(row.0) {
                        return Err(freeze("source-loop/cleanup-terminal-repeat"));
                    }
                    Some(effect(row)?)
                }
                _ => {
                    if !prefix_allowed.contains(row) || prefix.contains(row) {
                        return Err(freeze("source-loop/cleanup-prefix-owner"));
                    }
                    producer(row)?;
                    prefix.push(row.clone());
                    None
                }
            };
            if let Some(mapped) = mapped {
                if let Some((_, prior)) = terminals.iter().find(|(id, _)| *id == mapped.0) {
                    if prior != &mapped.1 {
                        return Err(freeze("source-loop/cleanup-mapped-terminal"));
                    }
                } else {
                    terminals.push(mapped);
                }
            }
        }
        if prefix_allowed.iter().any(|row| !prefix.contains(row)) {
            return Err(freeze("source-loop/cleanup-prefix-missing"));
        }
        let invokes = terminals
            .iter()
            .filter(|row| matches!(row.1, MirInstruction::Invoke { .. }))
            .count();
        if invokes != mapped_operations.len() {
            return Err(freeze("source-loop/cleanup-invoke-census"));
        }
        if finishing.is_none() {
            for id in &original_terminals {
                if *id == emitted.from {
                    continue;
                }
                let expected = prefix
                    .iter()
                    .filter(|row| row.0 == *id)
                    .map(|row| &row.1)
                    .collect::<Vec<_>>();
                let block = function
                    .blocks
                    .get(id)
                    .ok_or_else(|| freeze("source-loop/cleanup-block"))?;
                if block.instructions.iter().collect::<Vec<_>>() != expected {
                    return Err(freeze("source-loop/cleanup-prefix-sequence"));
                }
            }
        }
        let start = coordinate(emitted.from)?;
        let stop = coordinate(emitted.continuation)?;
        ordered_structure::validate_to_continuation_v2(function, &terminals, start, stop)?;
        let mut used = BTreeSet::new();
        self.validate_paths_v2(function, start, stop, Some(frame), &mut |id, expected| {
            let (index, original, actual) = mapped_operations
                .get(&id)
                .ok_or_else(|| freeze("source-loop/cleanup-origin"))?;
            if original != expected {
                return Err(freeze("source-loop/cleanup-source-operation"));
            }
            used.insert(*index);
            Ok(actual.clone())
        })?;
        if used.len() != emitted.operations.len() {
            return Err(freeze("source-loop/cleanup-unconsumed-invoke"));
        }
        Ok(())
    }
}
