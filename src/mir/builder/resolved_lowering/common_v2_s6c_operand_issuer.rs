//! Canonical physical issuer for the S6C TextEq integer operands.
//!
//! This child owns only the source-backed `V6 -> V7 -> V8` body prefix:
//! `ReadBinding(index)`, `ConstI64(1)`, and `Add(V6, V7)`.  It does not issue
//! the Substring result, a text lease, TextEq, Bool, or any control-flow
//! instruction.  The surrounding unpublished function transaction remains
//! the rollback owner.

use crate::mir::builder::emission::{constant, loop_operation};
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV2, LoopOperationV2, LoopValueClassV2, S6CLogicalCallArgsV1,
    S6CLogicalCallRoleV1, S6CLogicalItemV1,
};
use crate::mir::{MirBuilder, MirType, ValueId};

use super::super::common_v2_segment_block_allocation::{
    PreparedSegmentBlockReceiptV1, SegmentBlockAllocationBrandV1,
};
use super::s6c_substring_callout_materializer::{
    CommonV2SubstringCallOutMirMaterializerRejectV1, CommonV2SubstringCallOutNormalResultRefV1,
};
use super::CommonV2CanonicalSessionRefV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum S6CTextEqOperandIssuerRejectV1 {
    AlreadyIssued,
    OwnerMismatch,
    MissingPhysicalEntryStamp,
    SegmentScopeMismatch,
    BodySegmentMissing,
    BodySegmentDuplicate,
    SourceShapeMismatch(&'static str),
    OperationShapeMismatch(&'static str),
    OperandType(Option<MirType>),
    Read(String),
    PhysicalValue(String),
    Const(String),
    Add(String),
    Callback(String),
}

/// Callback-scoped proof that the canonical session emitted the exact S6C
/// integer operands needed by the later source-backed Substring issuer.
/// Keeping the mutable session borrow in the receipt prevents detached value
/// tuples from being re-paired with a different segment or session.
pub(in crate::mir::builder) struct S6CTextEqOperandReceiptV1<'receipt, 'source, 'envelope> {
    _session: &'receipt mut CommonV2CanonicalSessionRefV1<'source, 'envelope>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    body_block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    physical_block: crate::mir::BasicBlockId,
    index_value: ValueId,
    one_value: ValueId,
    end_value: ValueId,
    receiver_key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    index_key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    end_key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    substring_result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    segment_brand: SegmentBlockAllocationBrandV1,
}

impl S6CTextEqOperandReceiptV1<'_, '_, '_> {
    pub(in crate::mir::builder) const fn owner(
        &self,
    ) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(in crate::mir::builder) const fn body_block(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopBlockKeyV1 {
        self.body_block
    }

    pub(in crate::mir::builder) const fn physical_block(&self) -> crate::mir::BasicBlockId {
        self.physical_block
    }

    pub(in crate::mir::builder) const fn index_value(&self) -> ValueId {
        self.index_value
    }

    pub(in crate::mir::builder) const fn one_value(&self) -> ValueId {
        self.one_value
    }

    pub(in crate::mir::builder) const fn end_value(&self) -> ValueId {
        self.end_value
    }

    pub(in crate::mir::builder) const fn index_key(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.index_key
    }

    pub(in crate::mir::builder) const fn end_key(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.end_key
    }

    pub(in crate::mir::builder) const fn substring_result(
        &self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.substring_result
    }

    pub(in crate::mir::builder) fn with_s6c_substring_callout_mir<R>(
        self,
        builder: &mut MirBuilder,
        segment_receipt: &PreparedSegmentBlockReceiptV1,
        physical_effects: &crate::mir::normal_callable_semantic_package::
            VerifiedS6CPhysicalFunctionEffectsV1,
        callback: impl FnOnce(
            &mut MirBuilder,
            CommonV2SubstringCallOutNormalResultRefV1,
        ) -> Result<R, String>,
    ) -> Result<R, CommonV2SubstringCallOutMirMaterializerRejectV1> {
        let Self {
            _session,
            owner: _,
            body_block,
            physical_block,
            index_value,
            end_value,
            receiver_key,
            index_key,
            end_key,
            substring_result,
            segment_brand: _,
            ..
        } = self;
        super::s6c_substring_callout_materializer::emit(
            _session,
            builder,
            segment_receipt,
            physical_effects,
            body_block,
            physical_block,
            receiver_key,
            ValueId::new(0),
            index_key,
            index_value,
            end_key,
            end_value,
            substring_result,
            callback,
        )
    }

    pub(in crate::mir::builder) fn segment_brand(&self) -> SegmentBlockAllocationBrandV1 {
        self.segment_brand.clone()
    }
}

impl<'source, 'envelope> CommonV2CanonicalSessionRefV1<'source, 'envelope> {
    /// Emit exactly the S6C body prefix required by the later Substring row.
    /// All source/Recipe/layout checks happen before the first MIR effect.
    pub(in crate::mir::builder) fn with_s6c_text_eq_operands<R>(
        &mut self,
        builder: &mut MirBuilder,
        segment_receipt: &PreparedSegmentBlockReceiptV1,
        callback: impl for<'receipt> FnOnce(
            &mut MirBuilder,
            S6CTextEqOperandReceiptV1<'receipt, 'source, 'envelope>,
        ) -> Result<R, String>,
    ) -> Result<R, S6CTextEqOperandIssuerRejectV1> {
        if self.s6c_text_eq_operands_issued {
            return Err(S6CTextEqOperandIssuerRejectV1::AlreadyIssued);
        }

        let owner = self.session.owner();
        let stamp = self
            .session
            .physical_entry_stamp()
            .map_err(|_| S6CTextEqOperandIssuerRejectV1::MissingPhysicalEntryStamp)?;
        if stamp.owner() != owner
            || self.envelope.owner() != owner
            || self.envelope.operations().owner() != owner
            || self.envelope.initial_index_seed().owner() != owner
            || self.envelope.initial_index_seed().binding().owner() != owner
            || segment_receipt.owner() != owner
        {
            return Err(S6CTextEqOperandIssuerRejectV1::OwnerMismatch);
        }
        if !self.session.owns_segment_receipt(segment_receipt) {
            return Err(S6CTextEqOperandIssuerRejectV1::SegmentScopeMismatch);
        }

        let (substring, index, add, one) = self.source_prefix_rows()?;
        let body_block = substring.block;
        if index.block != body_block || add.block != body_block || one.block != body_block {
            return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                "operand blocks",
            ));
        }
        let Some(layout_segment) = self.envelope.layout().segment_for_block(body_block) else {
            return Err(S6CTextEqOperandIssuerRejectV1::BodySegmentMissing);
        };
        if self
            .envelope
            .layout()
            .loops()
            .iter()
            .filter(|loop_row| loop_row.body() == body_block)
            .count()
            != 1
        {
            return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                "body loop",
            ));
        }
        let mut body_rows = segment_receipt
            .rows()
            .iter()
            .filter(|row| row.logical_block() == body_block);
        let Some(body_row) = body_rows.next() else {
            return Err(S6CTextEqOperandIssuerRejectV1::BodySegmentMissing);
        };
        if body_rows.next().is_some() {
            return Err(S6CTextEqOperandIssuerRejectV1::BodySegmentDuplicate);
        }
        if body_row.loop_key() != layout_segment.loop_key()
            || body_row.split_ordinal() != layout_segment.split_ordinal()
            || !layout_segment.items().contains(&substring.item)
            || !layout_segment.items().contains(&index.item)
            || !layout_segment.items().contains(&add.item)
            || !layout_segment.items().contains(&one.item)
        {
            return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                "body layout",
            ));
        }

        // The source ingress already co-sealed the logical binding with the
        // resolver BindingRef.  Compare the logical key only against the
        // retained layout/Join binding; never invent a BindingRef from it.
        let seed_binding = self.envelope.initial_index_seed().binding();
        if index.binding != self.envelope.layout().after().1 {
            return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                "index binding",
            ));
        }

        // Poison the one-shot seam before identity/SSA or instruction writes.
        self.s6c_text_eq_operands_issued = true;
        let index_read = self
            .session
            .identity
            .read_entry_receipt(
                builder,
                &mut self.session.phis,
                body_row.physical_block(),
                seed_binding,
            )
            .map_err(S6CTextEqOperandIssuerRejectV1::Read)?;
        if index_read.owner() != owner
            || index_read.binding() != seed_binding
            || index_read.physical_block() != body_row.physical_block()
        {
            return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                "canonical index read",
            ));
        }
        self.session
            .publish_physical_value_type(builder, index_read.physical_value(), MirType::Integer)
            .map_err(S6CTextEqOperandIssuerRejectV1::PhysicalValue)?;

        let one_value = self
            .session
            .issue_physical_value_id(builder)
            .map_err(S6CTextEqOperandIssuerRejectV1::PhysicalValue)?;
        constant::emit_integer_at_with_dst(builder, body_row.physical_block(), one_value, 1)
            .map_err(S6CTextEqOperandIssuerRejectV1::Const)?;
        self.session
            .publish_physical_value_type(builder, one_value, MirType::Integer)
            .map_err(S6CTextEqOperandIssuerRejectV1::PhysicalValue)?;

        let end_value = self
            .session
            .issue_physical_value_id(builder)
            .map_err(S6CTextEqOperandIssuerRejectV1::PhysicalValue)?;
        loop_operation::emit_add_i64_at_with_dst(
            builder,
            body_row.physical_block(),
            end_value,
            index_read.physical_value(),
            one_value,
        )
        .map_err(S6CTextEqOperandIssuerRejectV1::Add)?;
        self.session
            .publish_physical_value_type(builder, end_value, MirType::Integer)
            .map_err(S6CTextEqOperandIssuerRejectV1::PhysicalValue)?;

        let receipt = S6CTextEqOperandReceiptV1 {
            _session: self,
            owner,
            body_block,
            physical_block: body_row.physical_block(),
            index_value: index_read.physical_value(),
            one_value,
            end_value,
            receiver_key: substring.receiver,
            index_key: index.result,
            end_key: add.result,
            substring_result: substring.result,
            segment_brand: segment_receipt.brand(),
        };
        callback(builder, receipt).map_err(S6CTextEqOperandIssuerRejectV1::Callback)
    }

    fn source_prefix_rows(
        &self,
    ) -> Result<
        (SourceCallRow, SourceReadRow, SourceAddRow, SourceConstRow),
        S6CTextEqOperandIssuerRejectV1,
    > {
        let rows = self.envelope.operations().rows();
        let mut substring = None;
        for row in rows {
            match (row.source(), row.operation()) {
                (
                    S6CLogicalItemV1::CallSlot(call),
                    LoopOperationV2::CallSlot {
                        receiver: Some(receiver),
                        args,
                        result: Some(result),
                    },
                ) if call.role == S6CLogicalCallRoleV1::Substring
                    && *receiver == call.receiver
                    && matches!(call.args, S6CLogicalCallArgsV1::Pair(expected) if args.as_slice() == expected)
                    && *result == call.result
                    && call.result_class == LoopValueClassV2::Text =>
                {
                    if substring.replace(SourceCallRow::from_call(*call)).is_some() {
                        return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                            "duplicate substring",
                        ));
                    }
                }
                _ => {}
            }
        }
        let substring = substring.ok_or(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
            "substring",
        ))?;
        let S6CLogicalCallArgsV1::Pair([index_key, end_key]) = substring.args else {
            return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                "substring arity",
            ));
        };

        let mut index = None;
        let mut add = None;
        for row in rows {
            match (row.source(), row.operation()) {
                (
                    S6CLogicalItemV1::ReadBinding {
                        item,
                        block,
                        binding,
                        result,
                    },
                    LoopOperationV2::ReadBinding {
                        binding: operation_binding,
                        result: operation_result,
                    },
                ) if *result == index_key => {
                    if *operation_binding != *binding || *operation_result != *result {
                        return Err(S6CTextEqOperandIssuerRejectV1::OperationShapeMismatch(
                            "index read",
                        ));
                    }
                    if index
                        .replace(SourceReadRow {
                            item: *item,
                            block: *block,
                            binding: *binding,
                            result: *result,
                        })
                        .is_some()
                    {
                        return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                            "duplicate index read",
                        ));
                    }
                }
                (
                    S6CLogicalItemV1::BinaryI64 {
                        item,
                        block,
                        op,
                        left,
                        result,
                        ..
                    },
                    LoopOperationV2::BinaryI64 {
                        op: operation_op,
                        left: operation_left,
                        result: operation_result,
                        ..
                    },
                ) if *result == end_key && *left == index_key => {
                    if *op != LoopBinaryI64OpV2::Add
                        || *operation_op != *op
                        || *operation_left != *left
                        || *operation_result != *result
                    {
                        return Err(S6CTextEqOperandIssuerRejectV1::OperationShapeMismatch(
                            "slice end add",
                        ));
                    }
                    if add
                        .replace(SourceAddRow {
                            item: *item,
                            block: *block,
                            left: *left,
                            right: match row.source() {
                                S6CLogicalItemV1::BinaryI64 { right, .. } => *right,
                                _ => unreachable!(),
                            },
                            result: *result,
                        })
                        .is_some()
                    {
                        return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                            "duplicate add",
                        ));
                    }
                }
                _ => {}
            }
        }
        let index = index.ok_or(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
            "index read",
        ))?;
        let add = add.ok_or(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch("add"))?;
        let mut one = None;
        for row in rows {
            match (row.source(), row.operation()) {
                (
                    S6CLogicalItemV1::ConstI64 {
                        item,
                        block,
                        result,
                        value,
                    },
                    LoopOperationV2::ConstI64 {
                        result: operation_result,
                        value: operation_value,
                    },
                ) if *result == add.right => {
                    if *value != 1 || *operation_result != *result || *operation_value != *value {
                        return Err(S6CTextEqOperandIssuerRejectV1::OperationShapeMismatch(
                            "slice one",
                        ));
                    }
                    if one
                        .replace(SourceConstRow {
                            item: *item,
                            block: *block,
                            result: *result,
                        })
                        .is_some()
                    {
                        return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                            "duplicate one",
                        ));
                    }
                }
                _ => {}
            }
        }
        let one = one.ok_or(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch("one"))?;
        if index.result != index_key
            || add.left != index_key
            || add.result != end_key
            || one.result != add.right
        {
            return Err(S6CTextEqOperandIssuerRejectV1::SourceShapeMismatch(
                "operand relation",
            ));
        }
        Ok((substring, index, add, one))
    }
}

#[derive(Clone, Copy)]
struct SourceCallRow {
    item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    receiver: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    args: S6CLogicalCallArgsV1,
    result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
}

impl SourceCallRow {
    fn from_call(call: crate::mir::loop_recipe_contract::S6CLogicalCallSlotV1) -> Self {
        Self {
            item: call.item,
            block: call.block,
            receiver: call.receiver,
            args: call.args,
            result: call.result,
        }
    }
}

#[derive(Clone, Copy)]
struct SourceReadRow {
    item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    binding: crate::mir::loop_recipe_contract::LoopBindingKeyV1,
    result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
}

#[derive(Clone, Copy)]
struct SourceAddRow {
    item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    left: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    right: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
}

#[derive(Clone, Copy)]
struct SourceConstRow {
    item: crate::mir::loop_recipe_contract::LoopItemKeyV1,
    block: crate::mir::loop_recipe_contract::LoopBlockKeyV1,
    result: crate::mir::loop_recipe_contract::LoopValueKeyV1,
}
