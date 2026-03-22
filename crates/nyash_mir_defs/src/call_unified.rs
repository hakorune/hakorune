/*!
 * Unified MIR Call Definitions - packaging slice extracted from `src/mir`.
 *
 * This crate holds the pure call-shape substrate while `src/mir` keeps thin
 * compatibility wrappers for now.
 */

use nyash_mir_core::{Effect, EffectMask, ValueId};

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
    /// Global function call (e.g., nyash.builtin.print)
    /// Resolves to built-in or global functions at compile time
    Global(String),

    /// Box method call with explicit receiver
    /// Enables static resolution of box.method() patterns
    Method {
        box_name: String,
        method: String,
        receiver: Option<ValueId>,
        certainty: TypeCertainty,
        box_kind: CalleeBoxKind,
    },

    /// Constructor call (NewBox equivalent)
    /// Creates new Box instances with birth() method
    Constructor {
        box_type: String,
    },

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
            _ => false,
        }
    }

    /// Get the receiver if this is a method call
    pub fn receiver(&self) -> Option<ValueId> {
        match self {
            Callee::Method { receiver, .. } => *receiver,
            _ => None,
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

    pub fn global(dst: Option<ValueId>, name: String, args: Vec<ValueId>) -> Self {
        MirCall::new(dst, Callee::Global(name), args)
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

