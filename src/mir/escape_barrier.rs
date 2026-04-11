/*!
 * Escape barrier vocabulary and operand-role classifier.
 *
 * Keep escape meaning separate from generic def-use queries such as
 * `MirInstruction::used_values()`. This module defines only the MIR-side
 * authority for which operand roles count as publication/capture barriers for
 * the current narrow escape-analysis slice.
 *
 * These barriers are cause-side facts, not lifecycle/outcome facts. Keep that
 * split explicit so later generic extraction does not collapse both questions
 * into one vocabulary.
 */

use crate::mir::definitions::call_unified::Callee;

use super::{MirInstruction, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EscapeBarrier {
    Return,
    Throw,
    Call,
    StoreLike,
    PhiMerge,
    Capture,
    DebugObserve,
}

impl std::fmt::Display for EscapeBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Return => f.write_str("return"),
            Self::Throw => f.write_str("throw"),
            Self::Call => f.write_str("call"),
            Self::StoreLike => f.write_str("store_like"),
            Self::PhiMerge => f.write_str("phi_merge"),
            Self::Capture => f.write_str("capture"),
            Self::DebugObserve => f.write_str("debug_observe"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EscapeUse {
    pub value: ValueId,
    pub barrier: EscapeBarrier,
}

impl EscapeUse {
    const fn new(value: ValueId, barrier: EscapeBarrier) -> Self {
        Self { value, barrier }
    }
}

pub fn classify_escape_uses(inst: &MirInstruction) -> Vec<EscapeUse> {
    match inst {
        MirInstruction::Return { value: Some(value) } => {
            vec![EscapeUse::new(*value, EscapeBarrier::Return)]
        }
        MirInstruction::Throw { exception, .. } => {
            vec![EscapeUse::new(*exception, EscapeBarrier::Throw)]
        }
        MirInstruction::Call { callee, args, .. } => {
            let mut uses = Vec::with_capacity(args.len() + 1);
            if let Some(Callee::Method {
                receiver: Some(receiver),
                ..
            }) = callee
            {
                uses.push(EscapeUse::new(*receiver, EscapeBarrier::Call));
            }
            uses.extend(
                args.iter()
                    .copied()
                    .map(|value| EscapeUse::new(value, EscapeBarrier::Call)),
            );
            uses
        }
        MirInstruction::Store { value, .. } | MirInstruction::FieldSet { value, .. } => {
            vec![EscapeUse::new(*value, EscapeBarrier::StoreLike)]
        }
        MirInstruction::Phi { inputs, .. } => inputs
            .iter()
            .map(|(_, value)| EscapeUse::new(*value, EscapeBarrier::PhiMerge))
            .collect(),
        MirInstruction::NewClosure { captures, me, .. } => {
            let mut uses = Vec::with_capacity(captures.len() + usize::from(me.is_some()));
            uses.extend(
                captures
                    .iter()
                    .map(|(_, value)| EscapeUse::new(*value, EscapeBarrier::Capture)),
            );
            if let Some(value) = me {
                uses.push(EscapeUse::new(*value, EscapeBarrier::Capture));
            }
            uses
        }
        MirInstruction::Debug { value, .. } => {
            vec![EscapeUse::new(*value, EscapeBarrier::DebugObserve)]
        }
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{classify_escape_uses, EscapeBarrier, EscapeUse};
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{Callee, EffectMask, MirInstruction, ValueId};

    #[test]
    fn method_call_marks_receiver_and_args_as_call_barriers() {
        let receiver = ValueId::new(10);
        let arg = ValueId::new(11);
        let uses = classify_escape_uses(&MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Point".to_string(),
                method: "sum".to_string(),
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![arg],
            effects: EffectMask::PURE,
        });

        assert_eq!(
            uses,
            vec![
                EscapeUse {
                    value: receiver,
                    barrier: EscapeBarrier::Call,
                },
                EscapeUse {
                    value: arg,
                    barrier: EscapeBarrier::Call,
                },
            ]
        );
    }

    #[test]
    fn fieldset_marks_only_value_as_store_like() {
        let base = ValueId::new(20);
        let value = ValueId::new(21);
        let uses = classify_escape_uses(&MirInstruction::FieldSet {
            base,
            field: "child".to_string(),
            value,
            declared_type: None,
        });

        assert_eq!(
            uses,
            vec![EscapeUse {
                value,
                barrier: EscapeBarrier::StoreLike,
            }]
        );
        assert!(!uses.iter().any(|use_site| use_site.value == base));
    }
}
