//! SSOT helper for canonical method-call construction.
//!
//! RCL-3-min2:
//! - Stop constructing legacy `MirInstruction::BoxCall` at emit sites.
//! - Emit canonical `MirInstruction::Call { callee: Some(Callee::Method { .. }) }`.

use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{Callee, EffectMask, MirInstruction, ValueId};

/// Physical encoding used by a method-call argument list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MethodOperandEncoding {
    /// `args` contains only explicit source-level arguments.
    ExplicitArgs,
    /// `args[0]` contains the receiver for a backend/runtime ABI shape.
    ReceiverPrefixed,
}

/// Read-only view over the two method-call operand encodings currently found in MIR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MethodCallOperandView<'a> {
    pub(crate) callee_receiver: ValueId,
    pub(crate) operand_receiver: ValueId,
    pub(crate) explicit_args: &'a [ValueId],
    pub(crate) encoding: MethodOperandEncoding,
}

/// Normalize method operands without mutating MIR.
///
/// The canonical method-call form stores the receiver in `Callee::Method.receiver`
/// and keeps only explicit arguments in `args`. Some VM/backend compatibility paths
/// also prefix the receiver into `args[0]`. Consumers should use this view instead
/// of comparing the two receiver ValueIds.
pub(crate) fn method_call_operand_view(
    callee_receiver: ValueId,
    args: &[ValueId],
    explicit_arity: usize,
) -> Option<MethodCallOperandView<'_>> {
    if args.len() == explicit_arity {
        return Some(MethodCallOperandView {
            callee_receiver,
            operand_receiver: callee_receiver,
            explicit_args: args,
            encoding: MethodOperandEncoding::ExplicitArgs,
        });
    }

    if args.len() == explicit_arity + 1 {
        return Some(MethodCallOperandView {
            callee_receiver,
            operand_receiver: args[0],
            explicit_args: &args[1..],
            encoding: MethodOperandEncoding::ReceiverPrefixed,
        });
    }

    None
}

/// Build a canonical method call instruction in a single place.
pub fn method_call(
    dst: Option<ValueId>,
    receiver: ValueId,
    box_name: impl Into<String>,
    method: impl Into<String>,
    args: Vec<ValueId>,
    effects: EffectMask,
    certainty: TypeCertainty,
    box_kind: CalleeBoxKind,
) -> MirInstruction {
    MirInstruction::Call {
        dst,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.into(),
            method: method.into(),
            receiver: Some(receiver),
            certainty,
            box_kind,
        }),
        args,
        effects,
    }
}

/// Runtime-dispatch method call helper with conservative box_kind.
pub fn runtime_method_call(
    dst: Option<ValueId>,
    receiver: ValueId,
    box_name: impl Into<String>,
    method: impl Into<String>,
    args: Vec<ValueId>,
    effects: EffectMask,
    certainty: TypeCertainty,
) -> MirInstruction {
    method_call(
        dst,
        receiver,
        box_name,
        method,
        args,
        effects,
        certainty,
        CalleeBoxKind::RuntimeData,
    )
}

#[cfg(test)]
mod tests {
    use super::{method_call_operand_view, MethodOperandEncoding};
    use crate::mir::ValueId;

    #[test]
    fn operand_view_accepts_explicit_args() {
        let receiver = ValueId(10);
        let args = [ValueId(20), ValueId(21)];

        let view = method_call_operand_view(receiver, &args, 2).expect("explicit args view");

        assert_eq!(view.callee_receiver, receiver);
        assert_eq!(view.operand_receiver, receiver);
        assert_eq!(view.explicit_args, &args);
        assert_eq!(view.encoding, MethodOperandEncoding::ExplicitArgs);
    }

    #[test]
    fn operand_view_accepts_receiver_prefixed_args_without_value_equality() {
        let receiver = ValueId(10);
        let rematerialized_receiver = ValueId(11);
        let args = [rematerialized_receiver, ValueId(20), ValueId(21)];

        let view =
            method_call_operand_view(receiver, &args, 2).expect("receiver-prefixed args view");

        assert_eq!(view.callee_receiver, receiver);
        assert_eq!(view.operand_receiver, rematerialized_receiver);
        assert_eq!(view.explicit_args, &args[1..]);
        assert_eq!(view.encoding, MethodOperandEncoding::ReceiverPrefixed);
    }

    #[test]
    fn operand_view_rejects_wrong_arity() {
        let receiver = ValueId(10);
        let args = [ValueId(20)];

        assert!(method_call_operand_view(receiver, &args, 2).is_none());
    }
}
