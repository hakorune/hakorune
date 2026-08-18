//! Callback-scoped source view for the S6C scalar-scan corridor.
//!
//! The existing prephysical ingress remains the only issuer.  This module
//! checks the already-issued typed/Facts/Recipe/Join cohort once more at that
//! boundary and lends a compact relation view.  It owns no Recipe key, MIR
//! value, runtime wire, pointer, or physical effect.

use super::ids::{LoopItemKeyV1, LoopValueKeyV1};
use super::s6c_prephysical_ingress::{
    S6CPrephysicalCompletionParityRefV2, S6CPrephysicalIngressRefV2, S6CPrephysicalOperationRoleV2,
};
use super::s6c_scan_with_init_joinir::{S6CLogicalCallInputRefV1, S6CLogicalCallRoleV1};
use super::s6c_scan_with_init_joinir_output_rows::S6CLogicalItemV1;
use super::schema_v2::LoopBinaryI64OpV2;
use crate::mir::callable_semantic_batch::S6CBinaryRoleV1;
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::CoreMethodSemanticLawV2;
use crate::mir::resolved_semantics::ResolvedLiteralSourceV1;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CScalarScanSourceRejectV1 {
    InputRoles,
    InitialIndex,
    LengthLaw,
    SubstringLaw,
    LengthRow,
    SubstringRow,
    PredicateRow,
    SliceRelation,
    TextEqRelation,
    IfRelation,
    StepRelation,
    EscapeRelation,
    TransferRelation,
    CompletionRelation,
}

/// One source-backed relation for the complete first scalar-scan cohort.
///
/// `substring_result` is deliberately represented only as a derived relation
/// between the subject root, the current index, and the scalar end.  It is not
/// a runtime root or a new logical value owner.
#[derive(Debug, Clone, Copy)]
pub(crate) struct S6CScalarScanSourceRefV1<'a, 'rows, 'facts> {
    _scope: PhantomData<(&'a (), &'rows ())>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    subject_binding: crate::mir::resolved_semantics::BindingRefV1,
    needle_binding: crate::mir::resolved_semantics::BindingRefV1,
    index_binding: crate::mir::resolved_semantics::BindingRefV1,
    subject_input: LoopValueKeyV1,
    needle_input: LoopValueKeyV1,
    index_input: LoopValueKeyV1,
    initial_index: i64,
    length: S6CLogicalCallInputRefV1<'facts>,
    substring: S6CLogicalCallInputRefV1<'facts>,
    length_result: LoopValueKeyV1,
    substring_result: LoopValueKeyV1,
    slice_end: LoopValueKeyV1,
    text_equal_item: LoopItemKeyV1,
    text_equal_result: LoopValueKeyV1,
    text_equal_if: LoopItemKeyV1,
    step_add: LoopValueKeyV1,
    length_law: CoreMethodSemanticLawV2,
    substring_law: CoreMethodSemanticLawV2,
    transfer: &'facts super::join_sig::LoopJoinLogicalTransferViewV2<'facts>,
    completion: S6CPrephysicalCompletionParityRefV2,
}

impl<'a, 'rows, 'facts> S6CScalarScanSourceRefV1<'a, 'rows, 'facts> {
    pub(crate) const fn owner(self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn subject_binding(self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.subject_binding
    }

    pub(crate) const fn needle_binding(self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.needle_binding
    }

    pub(crate) const fn index_binding(self) -> crate::mir::resolved_semantics::BindingRefV1 {
        self.index_binding
    }

    pub(crate) const fn subject_input(self) -> LoopValueKeyV1 {
        self.subject_input
    }

    pub(crate) const fn needle_input(self) -> LoopValueKeyV1 {
        self.needle_input
    }

    pub(crate) const fn index_input(self) -> LoopValueKeyV1 {
        self.index_input
    }

    pub(crate) const fn initial_index(self) -> i64 {
        self.initial_index
    }

    pub(crate) const fn length(self) -> S6CLogicalCallInputRefV1<'facts> {
        self.length
    }

    pub(crate) const fn substring(self) -> S6CLogicalCallInputRefV1<'facts> {
        self.substring
    }

    pub(crate) const fn length_result(self) -> LoopValueKeyV1 {
        self.length_result
    }

    pub(crate) const fn substring_result(self) -> LoopValueKeyV1 {
        self.substring_result
    }

    pub(crate) const fn slice_end(self) -> LoopValueKeyV1 {
        self.slice_end
    }

    pub(crate) const fn text_equal_item(self) -> LoopItemKeyV1 {
        self.text_equal_item
    }

    pub(crate) const fn text_equal_result(self) -> LoopValueKeyV1 {
        self.text_equal_result
    }

    pub(crate) const fn text_equal_if(self) -> LoopItemKeyV1 {
        self.text_equal_if
    }

    pub(crate) const fn step_add(self) -> LoopValueKeyV1 {
        self.step_add
    }

    pub(crate) const fn length_law(self) -> CoreMethodSemanticLawV2 {
        self.length_law
    }

    pub(crate) const fn substring_law(self) -> CoreMethodSemanticLawV2 {
        self.substring_law
    }

    pub(crate) fn logical_transfer(
        self,
    ) -> &'facts super::join_sig::LoopJoinLogicalTransferViewV2<'facts> {
        self.transfer
    }

    pub(crate) const fn completion(self) -> S6CPrephysicalCompletionParityRefV2 {
        self.completion
    }
}

pub(super) fn issue_s6c_scalar_scan_source_v1<'a, 'rows, 'facts>(
    ingress: S6CPrephysicalIngressRefV2<'a, 'rows, 'facts>,
) -> Result<S6CScalarScanSourceRefV1<'a, 'rows, 'facts>, S6CScalarScanSourceRejectV1> {
    let typed = ingress.typed_input_relation();
    let inputs = typed.inputs();
    let subject = inputs
        .iter()
        .find(|input| {
            input.role() == crate::mir::callable_semantic_batch::S6CTypedInputRoleV1::Subject
        })
        .filter(|input| {
            input.class() == crate::mir::callable_semantic_batch::S6CLogicalValueClassV1::Text
        })
        .ok_or(S6CScalarScanSourceRejectV1::InputRoles)?;
    let needle = inputs
        .iter()
        .find(|input| {
            input.role() == crate::mir::callable_semantic_batch::S6CTypedInputRoleV1::Needle
        })
        .filter(|input| {
            input.class() == crate::mir::callable_semantic_batch::S6CLogicalValueClassV1::Text
        })
        .ok_or(S6CScalarScanSourceRejectV1::InputRoles)?;
    let index = inputs
        .iter()
        .find(|input| {
            input.role() == crate::mir::callable_semantic_batch::S6CTypedInputRoleV1::Index
        })
        .filter(|input| {
            input.class() == crate::mir::callable_semantic_batch::S6CLogicalValueClassV1::I64
        })
        .ok_or(S6CScalarScanSourceRejectV1::InputRoles)?;
    if ingress.input_bindings() != [subject.binding(), needle.binding(), index.binding()] {
        return Err(S6CScalarScanSourceRejectV1::InputRoles);
    }

    if typed.initializer_literal() != &ResolvedLiteralSourceV1::Integer(0) {
        return Err(S6CScalarScanSourceRejectV1::InitialIndex);
    }

    let length = ingress.length_source();
    let substring = ingress.substring_source();
    let length_law = exact_law(
        length,
        CoreMethodOp::StringLen,
        0,
        CoreMethodSemanticLawV2::CodePointCount,
    )
    .ok_or(S6CScalarScanSourceRejectV1::LengthLaw)?;
    let substring_law = exact_law(
        substring,
        CoreMethodOp::StringSubstring,
        2,
        CoreMethodSemanticLawV2::CodePointHalfOpenClamped,
    )
    .ok_or(S6CScalarScanSourceRejectV1::SubstringLaw)?;

    let length_item = find_item(
        ingress.logical_items(),
        ingress
            .operation(S6CPrephysicalOperationRoleV2::LengthCall)
            .item(),
    )
    .ok_or(S6CScalarScanSourceRejectV1::LengthRow)?;
    let expected_length_result = match length.recipe_row() {
        super::s6c_scan_with_init_rows::S6CRecipeOperationRowRefV2::CallSlot {
            receiver: Some(receiver),
            args,
            result: Some(result),
        } if receiver == ingress.subject_input() && args.is_empty() => result,
        _ => return Err(S6CScalarScanSourceRejectV1::LengthRow),
    };
    let (length_result, length_receiver) = match length_item {
        S6CLogicalItemV1::CallSlot(call)
            if call.role() == S6CLogicalCallRoleV1::Length
                && matches!(
                    call.args,
                    super::s6c_scan_with_init_joinir_output_rows::S6CLogicalCallArgsV1::Empty
                ) =>
        {
            (call.result, call.receiver)
        }
        _ => return Err(S6CScalarScanSourceRejectV1::LengthRow),
    };
    if length_receiver != ingress.subject_input() || length_result != expected_length_result {
        return Err(S6CScalarScanSourceRejectV1::LengthRow);
    }
    let substring_item = find_item(
        ingress.logical_items(),
        ingress
            .operation(S6CPrephysicalOperationRoleV2::SubstringCall)
            .item(),
    )
    .ok_or(S6CScalarScanSourceRejectV1::SubstringRow)?;
    let expected_substring_result = match substring.recipe_row() {
        super::s6c_scan_with_init_rows::S6CRecipeOperationRowRefV2::CallSlot {
            receiver: Some(receiver),
            args,
            result: Some(result),
        } if receiver == ingress.subject_input() && args.len() == 2 => result,
        _ => return Err(S6CScalarScanSourceRejectV1::SubstringRow),
    };
    let (substring_result, slice_end) = match substring_item {
        S6CLogicalItemV1::CallSlot(call)
            if call.role() == S6CLogicalCallRoleV1::Substring
                && call.receiver == ingress.subject_input()
                && matches!(
                    call.args,
                    super::s6c_scan_with_init_joinir_output_rows::S6CLogicalCallArgsV1::Pair(_)
                ) =>
        {
            let super::s6c_scan_with_init_joinir_output_rows::S6CLogicalCallArgsV1::Pair(args) =
                call.args
            else {
                unreachable!("checked pair arguments")
            };
            (call.result, args[1])
        }
        _ => return Err(S6CScalarScanSourceRejectV1::SubstringRow),
    };
    let slice_end_binary = typed_binary(typed, S6CBinaryRoleV1::SliceEndAdd)?;
    if substring_result != expected_substring_result
        || substring.arguments().len() != 2
        || substring.arguments()[1].site() != slice_end_binary.source().site()
    {
        return Err(S6CScalarScanSourceRejectV1::SliceRelation);
    }

    let less = typed_binary(typed, S6CBinaryRoleV1::LoopConditionLess)?;
    if less.source().rhs() != length.result_site() {
        return Err(S6CScalarScanSourceRejectV1::PredicateRow);
    }
    let equal = typed_binary(typed, S6CBinaryRoleV1::TextEqual)?;
    if equal.source().lhs() != substring.result_site() {
        return Err(S6CScalarScanSourceRejectV1::TextEqRelation);
    }

    let text_equal_item = ingress
        .operation(S6CPrephysicalOperationRoleV2::TextEqual)
        .item();
    let text_equal_result = match find_item(ingress.logical_items(), text_equal_item) {
        Some(item) => match *item {
            S6CLogicalItemV1::TextEq {
                left,
                right,
                result,
                ..
            } if left == substring_result && right == ingress.needle_input() => result,
            _ => return Err(S6CScalarScanSourceRejectV1::TextEqRelation),
        },
        None => return Err(S6CScalarScanSourceRejectV1::TextEqRelation),
    };
    let text_equal_if = ingress
        .logical_items()
        .iter()
        .find_map(|item| match *item {
            S6CLogicalItemV1::If {
                item,
                condition,
                else_block: None,
                ..
            } if condition == text_equal_result => Some(item),
            _ => None,
        })
        .ok_or(S6CScalarScanSourceRejectV1::IfRelation)?;

    let step_add_item = ingress
        .operation(S6CPrephysicalOperationRoleV2::StepAdd)
        .item();
    let step_add = match find_item(ingress.logical_items(), step_add_item) {
        Some(item) => match *item {
            S6CLogicalItemV1::BinaryI64 {
                op: LoopBinaryI64OpV2::Add,
                result,
                ..
            } => result,
            _ => return Err(S6CScalarScanSourceRejectV1::StepRelation),
        },
        None => return Err(S6CScalarScanSourceRejectV1::StepRelation),
    };
    let step_write = ingress
        .operation(S6CPrephysicalOperationRoleV2::StepWrite)
        .item();
    if !ingress.logical_items().iter().any(|item| {
        matches!(
            *item,
            S6CLogicalItemV1::WriteBinding {
                item,
                binding,
                value,
                ..
            }
                if item == step_write && binding == ingress.index_binding() && value == step_add
        )
    }) {
        return Err(S6CScalarScanSourceRejectV1::StepRelation);
    }

    let body_method_calls = typed
        .body_shape()
        .expressions()
        .iter()
        .filter(|expression| {
            matches!(
                expression,
                crate::mir::resolved_semantics::BodyExpressionShapeV1::MethodCall { .. }
            )
        })
        .count();
    if body_method_calls != 2 || substring_result == ingress.needle_input() {
        return Err(S6CScalarScanSourceRejectV1::EscapeRelation);
    }

    let transfer = ingress.logical_transfer();
    if transfer.branches().len() != 1
        || transfer.summary_transfers().len() != 1
        || transfer
            .boundaries()
            .iter()
            .filter(|row| row.role == super::join_sig::LoopJoinEdgeRoleV1::Backedge)
            .count()
            != 1
        || ingress.completion().explicit_exit_count() != 2
        || !ingress.completion().cleanup_empty()
    {
        return Err(S6CScalarScanSourceRejectV1::TransferRelation);
    }

    Ok(S6CScalarScanSourceRefV1 {
        _scope: PhantomData,
        owner: ingress.source_owner(),
        subject_binding: subject.binding(),
        needle_binding: needle.binding(),
        index_binding: index.binding(),
        subject_input: ingress.subject_input(),
        needle_input: ingress.needle_input(),
        index_input: ingress.index_input(),
        initial_index: 0,
        length,
        substring,
        length_result,
        substring_result,
        slice_end,
        text_equal_item,
        text_equal_result,
        text_equal_if,
        step_add,
        length_law,
        substring_law,
        transfer,
        completion: ingress.completion(),
    })
}

fn exact_law(
    call: S6CLogicalCallInputRefV1<'_>,
    operation: CoreMethodOp,
    arity: u32,
    expected: CoreMethodSemanticLawV2,
) -> Option<CoreMethodSemanticLawV2> {
    (call.operation() == operation
        && call.arity() == arity
        && call.target().row().semantic_law_for_arity() == Some(expected))
    .then_some(expected)
}

fn typed_binary<'a>(
    typed: &'a crate::mir::callable_semantic_batch::VerifiedS6CTypedInputRelationV1,
    role: S6CBinaryRoleV1,
) -> Result<&'a crate::mir::callable_semantic_batch::S6CBinaryRelationV1, S6CScalarScanSourceRejectV1>
{
    typed
        .binaries()
        .iter()
        .find(|binary| binary.role() == role)
        .ok_or(S6CScalarScanSourceRejectV1::SliceRelation)
}

fn find_item<'a>(
    items: &'a [S6CLogicalItemV1],
    key: LoopItemKeyV1,
) -> Option<&'a S6CLogicalItemV1> {
    items.iter().find(|item| item_key(**item) == key)
}

fn item_key(item: S6CLogicalItemV1) -> LoopItemKeyV1 {
    match item {
        S6CLogicalItemV1::ReadBinding { item, .. }
        | S6CLogicalItemV1::ConstI64 { item, .. }
        | S6CLogicalItemV1::BinaryI64 { item, .. }
        | S6CLogicalItemV1::CompareI64 { item, .. }
        | S6CLogicalItemV1::TextEq { item, .. }
        | S6CLogicalItemV1::If { item, .. }
        | S6CLogicalItemV1::WriteBinding { item, .. }
        | S6CLogicalItemV1::Exit { item, .. } => item,
        S6CLogicalItemV1::CallSlot(call) => call.item,
    }
}
