/*!
 * Unified MIR Call Definitions - packaging slice extracted from `src/mir`.
 *
 * This crate holds the pure call-shape substrate while `src/mir` keeps thin
 * compatibility wrappers for now.
 */

use hakorune_mir_core::{Effect, EffectMask, ValueId};

use crate::callable_key::CanonicalSameModuleCallableKeyV1;
use crate::global_target::CanonicalGlobalTargetV1;

/// Certainty of callee type information for method calls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeCertainty {
    /// Receiver class is known (from origin propagation or static context)
    Known,
    /// Receiver may be a union/merged flow; class not uniquely known
    Union,
}

/// Classification of Box types to prevent static/runtime mixing
/// Prevents Stage-B/Stage-1 compiler boxes from being confused with runtime data boxes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalleeBoxKind {
    /// Static compiler boxes (StageBArgsBox, Stage1UsingResolverBox, BundleResolver, ParserBox, etc.)
    /// These are only for compile-time static method lowering
    StaticCompiler,
    /// Runtime data boxes (MapBox, ArrayBox, StringBox, IntegerBox, etc.)
    /// These handle actual runtime values and method dispatch
    RuntimeData,
    /// User-defined boxes (neither compiler nor built-in runtime)
    UserDefined,
}

/// Call target specification for type-safe function resolution
/// Replaces runtime string-based resolution with compile-time typed targets
#[derive(Debug, Clone, PartialEq)]
pub enum Callee {
    /// Global function call selected by the declaration-backed catalog.
    ///
    /// The structural carrier is intentionally separate from source names and
    /// physical symbols.  Callers must provide an already-selected target.
    Global(CanonicalGlobalTargetV1),

    /// Box method call with explicit receiver
    /// Enables static resolution of box.method() patterns
    Method {
        box_name: String,
        method: String,
        receiver: Option<ValueId>,
        certainty: TypeCertainty,
        box_kind: CalleeBoxKind,
    },

    /// Canonical same-module instance method call.
    ///
    /// The callable key is issued by the source/catalog owner and carried
    /// without name-based reconstruction.  The receiver is mandatory and is
    /// kept separate from the source argument list; physical backends may
    /// project it to their receiver lane exactly once.
    SameModuleInstance {
        key: CanonicalSameModuleCallableKeyV1,
        receiver: ValueId,
    },

    /// Constructor hook on a freshly allocated receiver, not allocation itself.
    /// The source-issued BirthConstructor key and mandatory receiver stay
    /// separate from source arguments. This is never a Global method target.
    BirthConstructor {
        key: CanonicalSameModuleCallableKeyV1,
        receiver: ValueId,
    },

    /// Constructor call (NewBox equivalent)
    /// Creates new Box instances with birth() method
    Constructor { box_type: String },

    /// Closure creation (NewClosure equivalent)
    /// Creates function values with captured variables
    Closure {
        params: Vec<String>,
        captures: Vec<(String, ValueId)>,
        me_capture: Option<ValueId>,
    },

    /// Dynamic function value call
    /// Preserves first-class function semantics for variables containing functions
    Value(ValueId),

    /// External C ABI function call
    /// Direct mapping to host/runtime functions
    Extern(String),
}

impl Callee {
    /// Check if this is a constructor call
    pub fn is_constructor(&self) -> bool {
        matches!(self, Callee::Constructor { .. } | Callee::Closure { .. })
    }

    /// Check if this is a method call with receiver
    pub fn has_receiver(&self) -> bool {
        match self {
            Callee::Method { receiver, .. } => receiver.is_some(),
            Callee::SameModuleInstance { .. } | Callee::BirthConstructor { .. } => true,
            _ => false,
        }
    }

    /// Get the receiver if this is a method call
    pub fn receiver(&self) -> Option<ValueId> {
        match self {
            Callee::Method { receiver, .. } => *receiver,
            Callee::SameModuleInstance { receiver, .. }
            | Callee::BirthConstructor { receiver, .. } => Some(*receiver),
            _ => None,
        }
    }

    /// Visit every embedded ValueId in the canonical occurrence order.
    ///
    /// This is the immutable counterpart to `rewrite_value_operands`: method
    /// receivers, first-class value targets, and closure captures are exposed
    /// without allowing consumers to reinterpret the target shape. Duplicate
    /// occurrences are visited independently, and target-less variants are
    /// explicit no-ops.
    pub fn for_each_value_operand(&self, mut visit: impl FnMut(ValueId)) {
        match self {
            Callee::Global(_) | Callee::Extern(_) | Callee::Constructor { .. } => {}
            Callee::Method { receiver, .. } => {
                if let Some(receiver) = receiver {
                    visit(*receiver);
                }
            }
            Callee::SameModuleInstance { receiver, .. }
            | Callee::BirthConstructor { receiver, .. } => visit(*receiver),
            Callee::Closure {
                captures,
                me_capture,
                ..
            } => {
                for (_, capture) in captures {
                    visit(*capture);
                }
                if let Some(me_capture) = me_capture {
                    visit(*me_capture);
                }
            }
            Callee::Value(value) => visit(*value),
        }
    }

    /// Rewrite every embedded ValueId in the canonical occurrence order.
    ///
    /// The order is part of the call-shape contract: a method receiver, a
    /// first-class value target, or closure captures in stored order followed
    /// by the optional `me` capture.  Duplicate occurrences are intentional
    /// and are visited independently.  Target-less variants are explicit
    /// no-ops so adding a new variant cannot silently drop an operand.
    pub fn rewrite_value_operands(&mut self, mut rewrite: impl FnMut(&mut ValueId)) {
        match self {
            Callee::Global(_) | Callee::Extern(_) | Callee::Constructor { .. } => {}
            Callee::Method { receiver, .. } => {
                if let Some(receiver) = receiver {
                    rewrite(receiver);
                }
            }
            Callee::SameModuleInstance { receiver, .. }
            | Callee::BirthConstructor { receiver, .. } => rewrite(receiver),
            Callee::Closure {
                captures,
                me_capture,
                ..
            } => {
                for (_, capture) in captures {
                    rewrite(capture);
                }
                if let Some(me_capture) = me_capture {
                    rewrite(me_capture);
                }
            }
            Callee::Value(value) => rewrite(value),
        }
    }
}

/// Call flags for unified MIR Call instruction
/// Controls call behavior and optimization hints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallFlags {
    pub tail_call: bool,
    pub no_return: bool,
    pub can_inline: bool,
    pub is_constructor: bool,
}

impl CallFlags {
    pub const fn new() -> Self {
        CallFlags {
            tail_call: false,
            no_return: false,
            can_inline: false,
            is_constructor: false,
        }
    }

    pub const fn constructor() -> Self {
        CallFlags {
            tail_call: false,
            no_return: false,
            can_inline: false,
            is_constructor: true,
        }
    }

    pub const fn no_return() -> Self {
        CallFlags {
            tail_call: false,
            no_return: true,
            can_inline: false,
            is_constructor: false,
        }
    }

    pub fn with_tail_call(mut self) -> Self {
        self.tail_call = true;
        self
    }

    pub fn with_inline(mut self) -> Self {
        self.can_inline = true;
        self
    }
}

impl Default for CallFlags {
    fn default() -> Self {
        CallFlags::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Callee, CalleeBoxKind, TypeCertainty};
    use crate::global_target::CanonicalGlobalTargetV1;
    use hakorune_mir_core::ValueId;

    #[test]
    fn callee_rewrite_value_operands_preserves_occurrence_order_and_duplicates() {
        let mut callee = Callee::Closure {
            params: vec!["x".to_string()],
            captures: vec![
                ("a".to_string(), ValueId::new(7)),
                ("b".to_string(), ValueId::new(7)),
            ],
            me_capture: Some(ValueId::new(9)),
        };
        let mut visited = Vec::new();
        callee.rewrite_value_operands(|value| {
            visited.push(*value);
            if *value == ValueId::new(7) {
                *value = ValueId::new(17);
            }
        });

        assert_eq!(
            visited,
            vec![ValueId::new(7), ValueId::new(7), ValueId::new(9)]
        );
        assert_eq!(
            callee,
            Callee::Closure {
                params: vec!["x".to_string()],
                captures: vec![
                    ("a".to_string(), ValueId::new(17)),
                    ("b".to_string(), ValueId::new(17)),
                ],
                me_capture: Some(ValueId::new(9)),
            }
        );
    }

    #[test]
    fn callee_rewrite_value_operands_is_empty_for_targetless_and_missing_receiver_shapes() {
        let mut shapes = vec![
            Callee::Global(CanonicalGlobalTargetV1::new_free_function("f".into(), 0).unwrap()),
            Callee::Extern("env.f".to_string()),
            Callee::Constructor {
                box_type: "Box".to_string(),
            },
            Callee::Method {
                box_name: "Box".to_string(),
                method: "f".to_string(),
                receiver: None,
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            },
        ];
        for shape in &mut shapes {
            let mut calls = 0;
            shape.rewrite_value_operands(|_| calls += 1);
            assert_eq!(calls, 0);
        }
    }

    #[test]
    fn callee_for_each_value_operand_preserves_occurrence_order_and_duplicates() {
        let callee = Callee::Closure {
            params: vec!["x".to_string()],
            captures: vec![
                ("a".to_string(), ValueId::new(7)),
                ("b".to_string(), ValueId::new(7)),
            ],
            me_capture: Some(ValueId::new(9)),
        };
        let mut visited = Vec::new();
        callee.for_each_value_operand(|value| visited.push(value));

        assert_eq!(
            visited,
            vec![ValueId::new(7), ValueId::new(7), ValueId::new(9)]
        );
    }

    #[test]
    fn callee_for_each_value_operand_is_empty_for_targetless_and_missing_receiver_shapes() {
        let shapes = vec![
            Callee::Global(CanonicalGlobalTargetV1::new_free_function("f".into(), 0).unwrap()),
            Callee::Extern("env.f".to_string()),
            Callee::Constructor {
                box_type: "Box".to_string(),
            },
            Callee::Method {
                box_name: "Box".to_string(),
                method: "f".to_string(),
                receiver: None,
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            },
        ];
        for shape in &shapes {
            let mut calls = 0;
            shape.for_each_value_operand(|_| calls += 1);
            assert_eq!(calls, 0);
        }
    }

    #[test]
    fn same_module_instance_keeps_receiver_as_one_mandatory_operand() {
        let key = crate::callable_key::CanonicalSameModuleCallableKeyV1::instance_box_method(
            "Probe", "wrap", 1,
        );
        let mut callee = Callee::SameModuleInstance {
            key: key.clone(),
            receiver: ValueId::new(4),
        };
        assert!(callee.has_receiver());
        assert_eq!(callee.receiver(), Some(ValueId::new(4)));

        let mut visited = Vec::new();
        callee.for_each_value_operand(|value| visited.push(value));
        assert_eq!(visited, vec![ValueId::new(4)]);

        callee.rewrite_value_operands(|value| value.0 += 10);
        assert_eq!(
            callee,
            Callee::SameModuleInstance {
                key,
                receiver: ValueId::new(14),
            }
        );
        let mut birth = Callee::BirthConstructor {
            key: crate::callable_key::CanonicalSameModuleCallableKeyV1::birth_constructor("Probe", 1),
            receiver: ValueId::new(0),
        };
        assert!(birth.has_receiver());
        assert!(!birth.is_constructor(), "Birth is a hook, not NewBox allocation");
        let mut operands = Vec::new();
        birth.for_each_value_operand(|value| operands.push(value));
        assert_eq!(operands, [ValueId::new(0)]);
        birth.rewrite_value_operands(|value| value.0 += 10);
        assert_eq!(birth.receiver(), Some(ValueId::new(10)));
    }
}

/// Unified MIR Call instruction - replaces Call/BoxCall/ExternCall/NewBox/NewClosure
#[derive(Debug, Clone, PartialEq)]
pub struct MirCall {
    pub dst: Option<ValueId>,
    pub callee: Callee,
    pub args: Vec<ValueId>,
    pub flags: CallFlags,
    pub effects: EffectMask,
}

impl MirCall {
    pub fn new(dst: Option<ValueId>, callee: Callee, args: Vec<ValueId>) -> Self {
        MirCall {
            dst,
            callee,
            args,
            flags: CallFlags::new(),
            effects: EffectMask::PURE,
        }
    }

    pub fn global(
        dst: Option<ValueId>,
        target: CanonicalGlobalTargetV1,
        args: Vec<ValueId>,
    ) -> Self {
        MirCall::new(dst, Callee::Global(target), args)
    }

    pub fn method(
        dst: Option<ValueId>,
        box_name: String,
        method: String,
        receiver: ValueId,
        args: Vec<ValueId>,
    ) -> Self {
        MirCall::new(
            dst,
            Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            },
            args,
        )
    }

    pub fn external(dst: Option<ValueId>, name: String, args: Vec<ValueId>) -> Self {
        let mut call = MirCall::new(dst, Callee::Extern(name), args);
        call.effects = EffectMask::IO;
        call
    }

    pub fn constructor(dst: ValueId, box_type: String, args: Vec<ValueId>) -> Self {
        let mut call = MirCall::new(Some(dst), Callee::Constructor { box_type }, args);
        call.flags = CallFlags::constructor();
        call.effects = EffectMask::PURE.add(Effect::Alloc);
        call
    }

    pub fn closure(
        dst: ValueId,
        params: Vec<String>,
        captures: Vec<(String, ValueId)>,
        me_capture: Option<ValueId>,
    ) -> Self {
        let mut call = MirCall::new(
            Some(dst),
            Callee::Closure {
                params,
                captures,
                me_capture,
            },
            vec![],
        );
        call.flags = CallFlags::constructor();
        call.effects = EffectMask::PURE.add(Effect::Alloc);
        call
    }

    pub fn with_tail_call(mut self) -> Self {
        self.flags.tail_call = true;
        self
    }

    pub fn with_effects(mut self, effects: EffectMask) -> Self {
        self.effects = effects;
        self
    }
}
