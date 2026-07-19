//! Disconnected semantic vocabulary for one PHI completion transaction.
//!
//! PHI0-PRED0-S0 deliberately owns no Builder or MIR mutation. Generic input
//! completion validates only facts knowable from the supplied rows, delegates
//! the type-only decision to `phi_type_publication`, and makes the
//! post-instruction transition explicit for later facade integration. Exact
//! predecessor-row validation is a separate route-owned CFG-ready capability.

use std::collections::BTreeMap;

use crate::mir::builder::phi_type_publication::{
    PhiConcreteTypeConflictV1, PhiTransientTypeDecisionV1, PreparedPhiTypePublicationV1,
};
use crate::mir::{BasicBlockId, MirType, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct PhiDraftV1 {
    block: BasicBlockId,
    dst: ValueId,
    type_hint: Option<MirType>,
}

impl PhiDraftV1 {
    pub(in crate::mir::builder) const fn new(
        block: BasicBlockId,
        dst: ValueId,
        type_hint: Option<MirType>,
    ) -> Self {
        Self {
            block,
            dst,
            type_hint,
        }
    }

    pub(in crate::mir::builder) const fn block(&self) -> BasicBlockId {
        self.block
    }

    pub(in crate::mir::builder) const fn dst(&self) -> ValueId {
        self.dst
    }

    /// Prepare generic input/type completion without mutating an instruction
    /// or a fact map.
    ///
    /// This is intentionally not CFG-ready: a provisional patch can lawfully
    /// name rows before the surrounding CFG publishes its predecessor set.
    pub(in crate::mir::builder) fn prepare_input_completion(
        &self,
        logical_inputs: &[(BasicBlockId, ValueId)],
        value_types: &BTreeMap<ValueId, MirType>,
        existing_destination: Option<&MirType>,
    ) -> Result<PreparedPhiCompletionV1, PhiCompletionPreparationErrorV1> {
        validate_unique_incoming_predecessors(logical_inputs)?;
        self.prepare_normalized(logical_inputs, value_types, existing_destination)
    }

    /// Prepare a route-owned CFG-ready completion from a sealed row product.
    /// This stays private until a route-specific owner is selected.
    pub(super) fn prepare_cfg_ready(
        &self,
        cfg_ready_rows: CfgReadyPhiRowsV1,
        value_types: &BTreeMap<ValueId, MirType>,
        existing_destination: Option<&MirType>,
    ) -> Result<PreparedPhiCompletionV1, PhiCompletionPreparationErrorV1> {
        self.prepare_normalized(
            cfg_ready_rows.logical_inputs(),
            value_types,
            existing_destination,
        )
    }

    fn prepare_normalized(
        &self,
        logical_inputs: &[(BasicBlockId, ValueId)],
        value_types: &BTreeMap<ValueId, MirType>,
        existing_destination: Option<&MirType>,
    ) -> Result<PreparedPhiCompletionV1, PhiCompletionPreparationErrorV1> {
        let mut normalized_inputs = logical_inputs.to_vec();
        normalized_inputs.sort_unstable_by_key(|(predecessor, value)| (*predecessor, *value));
        let prepared_type = PhiTransientTypeDecisionV1::prepare(
            self.dst,
            &normalized_inputs,
            value_types,
            existing_destination,
            self.type_hint.as_ref(),
        )
        .map_err(PhiCompletionPreparationErrorV1::ConcreteTypeConflict)?;

        Ok(PreparedPhiCompletionV1 {
            draft: self.clone(),
            logical_inputs: normalized_inputs.into_boxed_slice(),
            prepared_type,
        })
    }
}

/// Exact predecessor/input rows sealed by an existing route owner.
///
/// The constructor is private to this disconnected vocabulary: PRED0-S0 has
/// no production route consumer. A later CFGREADY0 row must expose only a
/// route-specific constructor, never a generic raw-row escape hatch.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct CfgReadyPhiRowsV1 {
    logical_inputs: Box<[(BasicBlockId, ValueId)]>,
}

impl CfgReadyPhiRowsV1 {
    fn verify(
        expected_predecessors: &[BasicBlockId],
        logical_inputs: &[(BasicBlockId, ValueId)],
    ) -> Result<Self, PhiCompletionPreparationErrorV1> {
        validate_cfg_ready_predecessor_rows(expected_predecessors, logical_inputs)?;
        let mut normalized_inputs = logical_inputs.to_vec();
        normalized_inputs.sort_unstable_by_key(|(predecessor, value)| (*predecessor, *value));
        Ok(Self {
            logical_inputs: normalized_inputs.into_boxed_slice(),
        })
    }

    fn logical_inputs(&self) -> &[(BasicBlockId, ValueId)] {
        &self.logical_inputs
    }
}

/// A validated, still-uncommitted PHI completion request.
///
/// The type decision is prepared from logical values. A later owner must
/// materialize physical values and commit the instruction before it commits
/// this prepared type fact.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct PreparedPhiCompletionV1 {
    draft: PhiDraftV1,
    logical_inputs: Box<[(BasicBlockId, ValueId)]>,
    prepared_type: PreparedPhiTypePublicationV1,
}

impl PreparedPhiCompletionV1 {
    pub(in crate::mir::builder) fn draft(&self) -> &PhiDraftV1 {
        &self.draft
    }

    pub(in crate::mir::builder) fn logical_inputs(&self) -> &[(BasicBlockId, ValueId)] {
        &self.logical_inputs
    }

    pub(in crate::mir::builder) fn prepared_type(&self) -> &PreparedPhiTypePublicationV1 {
        &self.prepared_type
    }

    /// This transition is available only after a facade has committed its
    /// instruction mutation. It has no side effect by itself.
    pub(in crate::mir::builder) fn after_instruction_commit(self) -> CompletedPhiV1 {
        CompletedPhiV1 {
            draft: self.draft,
            logical_inputs: self.logical_inputs,
            prepared_type: self.prepared_type,
        }
    }
}

/// A completed PHI transaction whose prepared type fact may now be committed
/// by the existing fact-publication owner.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct CompletedPhiV1 {
    draft: PhiDraftV1,
    logical_inputs: Box<[(BasicBlockId, ValueId)]>,
    prepared_type: PreparedPhiTypePublicationV1,
}

impl CompletedPhiV1 {
    pub(in crate::mir::builder) fn draft(&self) -> &PhiDraftV1 {
        &self.draft
    }

    pub(in crate::mir::builder) fn logical_inputs(&self) -> &[(BasicBlockId, ValueId)] {
        &self.logical_inputs
    }

    pub(in crate::mir::builder) fn prepared_type(&self) -> &PreparedPhiTypePublicationV1 {
        &self.prepared_type
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum PhiCompletionPreparationErrorV1 {
    DuplicateExpectedPredecessor { predecessor: BasicBlockId },
    DuplicateIncomingPredecessor { predecessor: BasicBlockId },
    PhantomIncomingPredecessor { predecessor: BasicBlockId },
    MissingIncomingPredecessor { predecessor: BasicBlockId },
    ConcreteTypeConflict(PhiConcreteTypeConflictV1),
}

fn validate_unique_incoming_predecessors(
    logical_inputs: &[(BasicBlockId, ValueId)],
) -> Result<(), PhiCompletionPreparationErrorV1> {
    let mut actual = logical_inputs
        .iter()
        .map(|(predecessor, _)| *predecessor)
        .collect::<Vec<_>>();
    actual.sort_unstable();
    if let Some(predecessor) = actual
        .windows(2)
        .find_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
    {
        return Err(PhiCompletionPreparationErrorV1::DuplicateIncomingPredecessor { predecessor });
    }
    Ok(())
}

fn validate_cfg_ready_predecessor_rows(
    expected_predecessors: &[BasicBlockId],
    logical_inputs: &[(BasicBlockId, ValueId)],
) -> Result<(), PhiCompletionPreparationErrorV1> {
    let mut expected = expected_predecessors.to_vec();
    expected.sort_unstable();
    if let Some(predecessor) = expected
        .windows(2)
        .find_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
    {
        return Err(PhiCompletionPreparationErrorV1::DuplicateExpectedPredecessor { predecessor });
    }

    validate_unique_incoming_predecessors(logical_inputs)?;

    let mut actual = logical_inputs
        .iter()
        .map(|(predecessor, _)| *predecessor)
        .collect::<Vec<_>>();
    actual.sort_unstable();

    if let Some(predecessor) = actual
        .iter()
        .find(|predecessor| !expected.contains(predecessor))
    {
        return Err(
            PhiCompletionPreparationErrorV1::PhantomIncomingPredecessor {
                predecessor: *predecessor,
            },
        );
    }

    if let Some(predecessor) = expected
        .iter()
        .find(|predecessor| !actual.contains(predecessor))
    {
        return Err(
            PhiCompletionPreparationErrorV1::MissingIncomingPredecessor {
                predecessor: *predecessor,
            },
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests;
