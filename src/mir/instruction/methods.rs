//! Method implementations for MIR Instructions
//!
//! Contains utility methods for MIR instruction analysis including:
//! - Effect tracking (effects())
//! - Destination value extraction (dst_value())
//! - Used values collection (used_values())

use super::super::{Effect, EffectMask, ValueId};
use crate::mir::instruction::MirInstruction;
use crate::mir::instruction_kinds as inst_meta;
use crate::mir::types::{BarrierOp, ConstValue, WeakRefOp};

impl MirInstruction {
    /// Get the effect mask for this instruction
    pub fn effects(&self) -> EffectMask {
        if let Some(eff) = inst_meta::effects_via_meta(self) {
            return eff;
        }
        match self {
            // Pure operations
            MirInstruction::Const { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::UnaryOp { .. }
            | MirInstruction::Compare { .. }
            | MirInstruction::TypeOp { .. }
            | MirInstruction::Copy { .. }
            | MirInstruction::Phi { .. }
            | MirInstruction::Select { .. }
            | MirInstruction::KeepAlive { .. } => EffectMask::PURE,

            // Memory operations
            MirInstruction::Load { .. } => EffectMask::READ,
            MirInstruction::Store { .. } => EffectMask::WRITE,

            // Phase 287: Reference lifecycle
            MirInstruction::ReleaseStrong { .. } => EffectMask::WRITE,

            // Function calls use provided effect mask
            MirInstruction::Call { effects, .. } => *effects,

            // Control flow (pure but affects execution)
            MirInstruction::Branch { .. }
            | MirInstruction::Jump { .. }
            | MirInstruction::Return { .. } => EffectMask::PURE,

            // Box creation may allocate
            MirInstruction::NewBox { .. } => EffectMask::PURE.add(Effect::Alloc),

            // Debug has debug effect
            MirInstruction::Debug { .. } => EffectMask::PURE.add(Effect::Debug),

            // Phase 5: Control flow & exception handling
            MirInstruction::Throw { effects, .. } => *effects,
            MirInstruction::Catch { .. } => EffectMask::CONTROL, // Handler setup affects control handling
            MirInstruction::Safepoint => EffectMask::PURE,       // No-op for now

            // Phase 6: Box reference operations
            MirInstruction::RefNew { .. } => EffectMask::PURE, // Creating reference is pure
            // PoC unified ops mirror legacy effects
            MirInstruction::WeakRef { op, .. } => match op {
                WeakRefOp::New => EffectMask::PURE,
                WeakRefOp::Load => EffectMask::READ,
            },
            MirInstruction::Barrier { op, .. } => match op {
                BarrierOp::Read => EffectMask::READ.add(Effect::Barrier),
                BarrierOp::Write => EffectMask::WRITE.add(Effect::Barrier),
            },

            // Phase 7: Async/Future Operations
            MirInstruction::FutureNew { .. } => EffectMask::PURE.add(Effect::Alloc), // Creating future may allocate
            MirInstruction::FutureSet { .. } => EffectMask::WRITE, // Setting future has write effects
            MirInstruction::Await { .. } => EffectMask::READ.add(Effect::Async), // Await blocks and reads

            // Function value construction: treat as pure with allocation
            MirInstruction::NewClosure { .. } => EffectMask::PURE.add(Effect::Alloc),
        }
    }

    /// Get the destination ValueId if this instruction produces a value
    pub fn dst_value(&self) -> Option<ValueId> {
        if let Some(dst) = inst_meta::dst_via_meta(self) {
            return Some(dst);
        }
        match self {
            MirInstruction::Const { dst, .. }
            | MirInstruction::BinOp { dst, .. }
            | MirInstruction::UnaryOp { dst, .. }
            | MirInstruction::Compare { dst, .. }
            | MirInstruction::Load { dst, .. }
            | MirInstruction::Phi { dst, .. }
            | MirInstruction::NewBox { dst, .. }
            | MirInstruction::TypeOp { dst, .. }
            | MirInstruction::Copy { dst, .. }
            | MirInstruction::RefNew { dst, .. }
            | MirInstruction::WeakRef { dst, .. }
            | MirInstruction::FutureNew { dst, .. }
            | MirInstruction::Await { dst, .. }
            | MirInstruction::Select { dst, .. } => Some(*dst),
            MirInstruction::NewClosure { dst, .. } => Some(*dst),

            MirInstruction::Call { dst, .. } => *dst,

            MirInstruction::Store { .. }
            | MirInstruction::Branch { .. }
            | MirInstruction::Jump { .. }
            | MirInstruction::Return { .. }
            | MirInstruction::Debug { .. }
            | MirInstruction::KeepAlive { .. }
            | MirInstruction::ReleaseStrong { .. }
            | MirInstruction::Throw { .. }
            | MirInstruction::Barrier { .. }
            | MirInstruction::FutureSet { .. }
            | MirInstruction::Safepoint => None,

            MirInstruction::Catch {
                exception_value, ..
            } => Some(*exception_value),
        }
    }

    /// Get all ValueIds used by this instruction
    pub fn used_values(&self) -> Vec<ValueId> {
        // Handle Call instructions here (not in inst_meta) because CallLikeInst
        // doesn't have the callee field needed for Callee::Method receiver handling.
        // This is the single source of truth for Call's used values.
        if let MirInstruction::Call {
            callee, func, args, ..
        } = self
        {
            use crate::mir::definitions::call_unified::Callee;
            let mut used: Vec<ValueId> = Vec::new();
            match callee {
                // Unified path: Callee::Method with receiver
                Some(Callee::Method {
                    receiver: Some(r), ..
                }) => {
                    used.push(*r);
                }
                // Legacy path: func ValueId is the callable
                None => {
                    used.push(*func);
                }
                // Other Callee variants (Global, Value, Extern) - no extra used values
                _ => {}
            }
            used.extend(args.iter().copied());
            return used;
        }

        match self {
            MirInstruction::Branch {
                condition,
                then_edge_args,
                else_edge_args,
                ..
            } => {
                let mut used = vec![*condition];
                if let Some(args) = then_edge_args {
                    used.extend(args.values.iter().copied());
                }
                if let Some(args) = else_edge_args {
                    used.extend(args.values.iter().copied());
                }
                return used;
            }
            MirInstruction::Jump { edge_args, .. } => {
                return edge_args
                    .as_ref()
                    .map(|args| args.values.clone())
                    .unwrap_or_default();
            }
            _ => {}
        }

        if let Some(used) = inst_meta::used_via_meta(self) {
            return used;
        }
        match self {
            MirInstruction::Const { .. } | MirInstruction::Jump { .. } => {
                Vec::new()
            }

            MirInstruction::UnaryOp { operand, .. }
            | MirInstruction::Load { ptr: operand, .. }
            | MirInstruction::TypeOp { value: operand, .. }
            | MirInstruction::Copy { src: operand, .. }
            | MirInstruction::Debug { value: operand, .. } => vec![*operand],

            MirInstruction::BinOp { lhs, rhs, .. }
            | MirInstruction::Compare { lhs, rhs, .. }
            | MirInstruction::Store {
                value: lhs,
                ptr: rhs,
                ..
            } => vec![*lhs, *rhs],

            // Phase 287: Lifecycle management uses all values
            MirInstruction::KeepAlive { values } => values.clone(),
            MirInstruction::ReleaseStrong { values } => values.clone(),

            // Phase 256 P1.5: Select instruction uses cond, then_val, else_val
            MirInstruction::Select {
                cond,
                then_val,
                else_val,
                ..
            } => vec![*cond, *then_val, *else_val],

            MirInstruction::Branch { condition, .. } => vec![*condition],

            MirInstruction::Return { value } => value.map(|v| vec![v]).unwrap_or_default(),

            // Call is handled by early return above (single source of truth)
            MirInstruction::Call { .. } => {
                unreachable!("Call should be handled by early return in used_values()")
            }
            MirInstruction::NewClosure { captures, me, .. } => {
                let mut used: Vec<ValueId> = Vec::new();
                used.extend(captures.iter().map(|(_, v)| *v));
                if let Some(m) = me {
                    used.push(*m);
                }
                used
            }

            MirInstruction::NewBox { args, .. } => args.clone(),

            MirInstruction::Phi { inputs, .. } => inputs.iter().map(|(_, value)| *value).collect(),

            // Phase 5: Control flow & exception handling
            MirInstruction::Throw { exception, .. } => vec![*exception],
            MirInstruction::Catch { .. } => Vec::new(), // Handler setup doesn't use values
            MirInstruction::Safepoint => Vec::new(),

            // Phase 6: Box reference operations
            MirInstruction::RefNew { box_val, .. } => vec![*box_val],
            MirInstruction::WeakRef { value, .. } => vec![*value],
            MirInstruction::Barrier { ptr, .. } => vec![*ptr],

            // Phase 7: Async/Future Operations
            MirInstruction::FutureNew { value, .. } => vec![*value],
            MirInstruction::FutureSet { future, value } => vec![*future, *value],
            MirInstruction::Await { future, .. } => vec![*future],

        }
    }
}

impl ConstValue {
    /*
    /// Convert to NyashValue
    pub fn to_nyash_value(&self) -> NyashValue {
        match self {
            ConstValue::Integer(n) => NyashValue::new_integer(*n),
            ConstValue::Float(f) => NyashValue::new_float(*f),
            ConstValue::Bool(b) => NyashValue::new_bool(*b),
            ConstValue::String(s) => NyashValue::new_string(s.clone()),
            ConstValue::Null => NyashValue::new_null(),
            ConstValue::Void => NyashValue::new_void(),
        }
    }

    /// Create from NyashValue
    pub fn from_nyash_value(value: &NyashValue) -> Option<Self> {
        match value {
            NyashValue::Integer(n) => Some(ConstValue::Integer(*n)),
            NyashValue::Float(f) => Some(ConstValue::Float(*f)),
            NyashValue::Bool(b) => Some(ConstValue::Bool(*b)),
            NyashValue::String(s) => Some(ConstValue::String(s.clone())),
            NyashValue::Null => Some(ConstValue::Null),
            NyashValue::Void => Some(ConstValue::Void),
            _ => None, // Collections and Boxes can't be constants
        }
    }
    */
}
