//! Canonical physical materialization of the source-backed initial index seed.
//!
//! This child owns only one entry Const and one exact declaration publication.
//! It does not emit a compare, branch, edge, or loop block.

use crate::mir::builder::emission::constant;
use crate::mir::loop_recipe_contract::PreparedLoopV2InitialIndexSeedRelationV1;
use crate::mir::{BasicBlockId, MirBuilder, MirType, ValueId};

use super::CommonV2CanonicalSessionRefV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum InitialIndexSeedMaterializationRejectV1 {
    AlreadyIssued,
    OwnerMismatch,
    SourceShapeMismatch,
    MissingFunction,
    EntryBlockDrift,
    Value(String),
    Declaration(String),
}

/// One callback-scoped witness for the canonical initial declaration/value.
/// The mutable session borrow prevents a second session from consuming or
/// re-pairing this seed before the surrounding unpublished transaction closes.
pub(in crate::mir::builder) struct CanonicalInitialIndexSeedReceiptV1<'seed, 'source, 'envelope> {
    _session: &'seed mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    carrier_entry: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    physical_block: BasicBlockId,
    value: ValueId,
}

impl CanonicalInitialIndexSeedReceiptV1<'_, '_, '_> {
    pub(in crate::mir::builder) const fn owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn binding(
        &self,
    ) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.binding
    }

    pub(in crate::mir::builder) const fn carrier_entry(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.carrier_entry
    }

    pub(in crate::mir::builder) const fn physical_block(&self) -> BasicBlockId {
        self.physical_block
    }

    pub(in crate::mir::builder) const fn value(&self) -> ValueId {
        self.value
    }
}

impl<'source, 'envelope> CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    /// Materialize exactly one source-backed `ConstI64(0)` at the function
    /// entry and publish that value through the canonical identity/SSA owner.
    /// The surrounding function transaction remains the sole discard owner.
    pub(in crate::mir::builder) fn emit_initial_index_seed<'seed>(
        &'seed mut self,
        builder: &mut MirBuilder,
    ) -> Result<
        CanonicalInitialIndexSeedReceiptV1<'seed, 'source, 'envelope>,
        InitialIndexSeedMaterializationRejectV1,
    > {
        if self.initial_index_seed_issued {
            return Err(InitialIndexSeedMaterializationRejectV1::AlreadyIssued);
        }

        let owner = self.session.owner();
        let seed: &PreparedLoopV2InitialIndexSeedRelationV1<'_> =
            self.envelope.initial_index_seed();
        if seed.owner() != owner
            || seed.binding().owner() != owner
            || self.envelope.owner() != owner
        {
            return Err(InitialIndexSeedMaterializationRejectV1::OwnerMismatch);
        }
        if seed.declared_type_name() != Some("i64")
            || seed.literal()
                != &crate::mir::resolved_semantics::ResolvedLiteralSourceV1::Integer(0)
        {
            return Err(InitialIndexSeedMaterializationRejectV1::SourceShapeMismatch);
        }

        let entry = self
            .session
            .physical_execution_entry(builder)
            .map_err(|_| InitialIndexSeedMaterializationRejectV1::MissingFunction)?;
        if builder.function_state.current_block != Some(entry) {
            return Err(InitialIndexSeedMaterializationRejectV1::EntryBlockDrift);
        }

        // Reserve the one-shot state before any physical effect. A failure
        // after this point poisons this unpublished session rather than
        // exposing a retry path that could leave a duplicate Const behind.
        self.initial_index_seed_issued = true;
        let value = self
            .session
            .issue_physical_value_id(builder)
            .map_err(InitialIndexSeedMaterializationRejectV1::Value)?;
        constant::emit_integer_at_with_dst(builder, entry, value, 0)
            .map_err(InitialIndexSeedMaterializationRejectV1::Value)?;
        self.session
            .publish_physical_value_type(builder, value, MirType::Integer)
            .map_err(InitialIndexSeedMaterializationRejectV1::Value)?;
        self.session
            .identity
            .publish_declaration_exact(seed.declaration_site(), seed.binding(), entry, value)
            .map_err(InitialIndexSeedMaterializationRejectV1::Declaration)?;

        Ok(CanonicalInitialIndexSeedReceiptV1 {
            _session: self,
            owner,
            binding: seed.binding(),
            carrier_entry: seed.index_carrier_entry(),
            physical_block: entry,
            value,
        })
    }
}
