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
            if let Some(callee) = callee {
                let barrier = if matches!(callee, Callee::Closure { .. }) {
                    EscapeBarrier::Capture
                } else {
                    EscapeBarrier::Call
                };
                callee.for_each_value_operand(|value| {
                    uses.push(EscapeUse::new(value, barrier));
                });
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
        MirInstruction::Phi { inputs, .. } if inputs.len() == 1 => Vec::new(),
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
    fn typed_value_target_is_a_call_barrier_and_stale_func_is_ignored() {
        let target = ValueId::new(12);
        let arg = ValueId::new(13);
        let uses = classify_escape_uses(&MirInstruction::Call {
            dst: Some(ValueId::new(14)),
            func: ValueId::new(99),
            callee: Some(crate::mir::Callee::Value(target)),
            args: vec![arg],
            effects: EffectMask::PURE,
        });

        assert_eq!(
            uses,
            vec![
                EscapeUse {
                    value: target,
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
    fn closure_target_marks_captures_as_capture_before_call_args() {
        let capture = ValueId::new(20);
        let me = ValueId::new(21);
        let arg = ValueId::new(22);
        let uses = classify_escape_uses(&MirInstruction::Call {
            dst: Some(ValueId::new(23)),
            func: ValueId::new(98),
            callee: Some(crate::mir::Callee::Closure {
                params: vec!["x".to_string()],
                captures: vec![
                    ("capture".to_string(), capture),
                    ("duplicate".to_string(), capture),
                ],
                me_capture: Some(me),
            }),
            args: vec![arg],
            effects: EffectMask::PURE,
        });

        assert_eq!(
            uses,
            vec![
                EscapeUse {
                    value: capture,
                    barrier: EscapeBarrier::Capture,
                },
                EscapeUse {
                    value: capture,
                    barrier: EscapeBarrier::Capture,
                },
                EscapeUse {
                    value: me,
                    barrier: EscapeBarrier::Capture,
                },
                EscapeUse {
                    value: arg,
                    barrier: EscapeBarrier::Call,
                },
            ]
        );
    }

    #[test]
    fn legacy_missing_callee_keeps_func_out_of_shared_barriers() {
        let legacy_func = ValueId::new(30);
        let arg = ValueId::new(31);
        let uses = classify_escape_uses(&MirInstruction::Call {
            dst: None,
            func: legacy_func,
            callee: None,
            args: vec![arg],
            effects: EffectMask::PURE,
        });

        assert_eq!(
            uses,
            vec![EscapeUse {
                value: arg,
                barrier: EscapeBarrier::Call,
            }]
        );
        assert!(!uses.iter().any(|use_site| use_site.value == legacy_func));
    }

    #[test]
    fn targetless_typed_callees_add_no_target_barrier() {
        let arg = ValueId::new(40);
        let callees = [
            crate::mir::Callee::Global(crate::mir::test_global_target("global".to_string())),
            crate::mir::Callee::Extern("env.test".to_string()),
            crate::mir::Callee::Constructor {
                box_type: "Point".to_string(),
            },
            crate::mir::Callee::Method {
                box_name: "Point".to_string(),
                method: "static_sum".to_string(),
                receiver: None,
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            },
        ];

        for callee in callees {
            let uses = classify_escape_uses(&MirInstruction::Call {
                dst: None,
                func: ValueId::new(99),
                callee: Some(callee),
                args: vec![arg],
                effects: EffectMask::PURE,
            });
            assert_eq!(
                uses,
                vec![EscapeUse {
                    value: arg,
                    barrier: EscapeBarrier::Call,
                }]
            );
        }
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

    #[test]
    fn single_input_phi_is_passthrough_not_merge_barrier() {
        let uses = classify_escape_uses(&MirInstruction::Phi {
            dst: ValueId::new(30),
            inputs: vec![(crate::mir::BasicBlockId::new(0), ValueId::new(31))],
            type_hint: None,
        });

        assert!(uses.is_empty());
    }

    #[test]
    fn multi_input_phi_stays_merge_barrier() {
        let lhs = ValueId::new(40);
        let rhs = ValueId::new(41);
        let uses = classify_escape_uses(&MirInstruction::Phi {
            dst: ValueId::new(42),
            inputs: vec![
                (crate::mir::BasicBlockId::new(0), lhs),
                (crate::mir::BasicBlockId::new(1), rhs),
            ],
            type_hint: None,
        });

        assert_eq!(
            uses,
            vec![
                EscapeUse {
                    value: lhs,
                    barrier: EscapeBarrier::PhiMerge,
                },
                EscapeUse {
                    value: rhs,
                    barrier: EscapeBarrier::PhiMerge,
                },
            ]
        );
    }
}
