//! Phase 189: JoinIR ID Remapper - ValueId/BlockId ID空間変換の独立化
//!
//! 責務:
//! - ValueId/BlockId の ID割り当て
//! - JoinIR fragment → host MIR への ID変換
//! - 決定性を重視した実装

use crate::mir::{BasicBlock, BasicBlockId, MirInstruction, ValueId};
use std::collections::BTreeMap; // Phase 222.5-E: HashMap → BTreeMap for determinism

/// JoinIR ID space を host MIR ID space に変換する
pub struct JoinIrIdRemapper {
    /// (func_name, old_block_id) → new_block_id のマッピング
    /// Phase 222.5-E: HashMap → BTreeMap for determinism
    block_map: BTreeMap<(String, BasicBlockId), BasicBlockId>,
    /// old_value_id → new_value_id のマッピング
    /// Phase 222.5-E: HashMap → BTreeMap for determinism
    value_map: BTreeMap<ValueId, ValueId>,
}

impl JoinIrIdRemapper {
    pub fn new() -> Self {
        Self {
            // Phase 222.5-E: HashMap → BTreeMap for determinism
            block_map: BTreeMap::new(),
            value_map: BTreeMap::new(),
        }
    }

    /// Block ID mapping を取得
    pub fn get_block(&self, func_name: &str, old_id: BasicBlockId) -> Option<BasicBlockId> {
        self.block_map
            .get(&(func_name.to_string(), old_id))
            .copied()
    }

    /// Value ID mapping を取得
    pub fn get_value(&self, old_id: ValueId) -> Option<ValueId> {
        self.value_map.get(&old_id).copied()
    }

    /// Block mapping を設定
    pub fn set_block(&mut self, func_name: String, old_id: BasicBlockId, new_id: BasicBlockId) {
        self.block_map.insert((func_name, old_id), new_id);
    }

    /// Value mapping を設定
    pub fn set_value(&mut self, old_id: ValueId, new_id: ValueId) {
        self.value_map.insert(old_id, new_id);
    }

    /// Block 内の ValueId を収集
    pub fn collect_values_in_block(&self, block: &BasicBlock) -> Vec<ValueId> {
        let mut values = Vec::new();
        for inst in &block.instructions {
            values.extend(self.collect_values_in_instruction(inst));
        }
        if let Some(ref term) = block.terminator {
            values.extend(self.collect_values_in_instruction(term));
        }
        values
    }

    /// 命令内の ValueId を収集
    pub fn collect_values_in_instruction(&self, inst: &MirInstruction) -> Vec<ValueId> {
        use crate::mir::MirInstruction::*;

        match inst {
            Const { dst, .. } => vec![*dst],
            UnaryOp { dst, operand, .. } => vec![*dst, *operand],
            BinOp { dst, lhs, rhs, .. } => vec![*dst, *lhs, *rhs],
            Compare { dst, lhs, rhs, .. } => vec![*dst, *lhs, *rhs],
            Load { dst, ptr } => vec![*dst, *ptr],
            Store { value, ptr } => vec![*value, *ptr],
            Call {
                dst,
                func,
                callee,
                args,
                ..
            } => {
                let mut vals = Vec::new();
                if let Some(crate::mir::Callee::Method {
                    receiver: Some(r), ..
                }) = callee
                {
                    vals.push(*r);
                } else if *func != ValueId::INVALID {
                    vals.push(*func);
                }
                if let Some(d) = dst {
                    vals.push(*d);
                }
                vals.extend(args.iter().copied());
                vals
            }
            Branch {
                condition,
                then_edge_args,
                else_edge_args,
                ..
            } => {
                let mut vals = vec![*condition];
                if let Some(args) = then_edge_args {
                    vals.extend(args.values.iter().copied());
                }
                if let Some(args) = else_edge_args {
                    vals.extend(args.values.iter().copied());
                }
                vals
            }
            Jump { edge_args, .. } => edge_args
                .as_ref()
                .map(|args| args.values.clone())
                .unwrap_or_default(),
            Return { value } => value.iter().copied().collect(),
            Phi { dst, inputs, .. } => {
                let mut vals = vec![*dst];
                vals.extend(inputs.iter().map(|(_, v)| *v));
                vals
            }
            Copy { dst, src } => vec![*dst, *src],
            NewBox { dst, args, .. } => {
                let mut vals = vec![*dst];
                vals.extend(args.iter().copied());
                vals
            }
            NewClosure {
                dst, captures, me, ..
            } => {
                let mut vals = vec![*dst];
                vals.extend(captures.iter().map(|(_, v)| *v));
                if let Some(m) = me {
                    vals.push(*m);
                }
                vals
            }
            Debug { value, .. } => vec![*value],
            // Phase 287: Lifecycle management collects all values
            KeepAlive { values } => values.clone(),
            ReleaseStrong { values } => values.clone(),
            Throw { exception, .. } => vec![*exception],
            Catch {
                exception_value, ..
            } => vec![*exception_value],
            RefNew { dst, box_val } => vec![*dst, *box_val],
            WeakRef { dst, value, .. } => vec![*dst, *value],
            Barrier { ptr, .. } => vec![*ptr],
            FutureNew { dst, value } => vec![*dst, *value],
            FutureSet { future, value } => vec![*future, *value],
            Await { dst, future } => vec![*dst, *future],
            TypeOp { dst, value, .. } => vec![*dst, *value],
            // Phase 256 P1.5: Collect Select ValueIds (dst, cond, then_val, else_val)
            Select {
                dst,
                cond,
                then_val,
                else_val,
            } => vec![*dst, *cond, *then_val, *else_val],
            Safepoint => vec![],
        }
    }

    /// 命令を新しい ID空間にリマップ
    pub fn remap_instruction(&self, inst: &MirInstruction) -> MirInstruction {
        use crate::mir::EdgeArgs;
        use crate::mir::MirInstruction::*;

        let remap = |v: ValueId| self.value_map.get(&v).copied().unwrap_or(v);
        let remap_edge_args = |edge_args: &Option<EdgeArgs>| {
            edge_args.as_ref().map(|args| EdgeArgs {
                layout: args.layout,
                values: args.values.iter().map(|&v| remap(v)).collect(),
            })
        };
        let remap_callee = |callee: &crate::mir::Callee| -> crate::mir::Callee {
            match callee {
                crate::mir::Callee::Global(name) => crate::mir::Callee::Global(name.clone()),
                crate::mir::Callee::Method {
                    box_name,
                    method,
                    receiver,
                    certainty,
                    box_kind,
                } => crate::mir::Callee::Method {
                    box_name: box_name.clone(),
                    method: method.clone(),
                    receiver: receiver.map(remap),
                    certainty: *certainty,
                    box_kind: *box_kind,
                },
                crate::mir::Callee::Constructor { box_type } => {
                    crate::mir::Callee::Constructor {
                        box_type: box_type.clone(),
                    }
                }
                crate::mir::Callee::Closure {
                    params,
                    captures,
                    me_capture,
                } => crate::mir::Callee::Closure {
                    params: params.clone(),
                    captures: captures
                        .iter()
                        .map(|(name, value)| (name.clone(), remap(*value)))
                        .collect(),
                    me_capture: me_capture.map(remap),
                },
                crate::mir::Callee::Value(v) => crate::mir::Callee::Value(remap(*v)),
                crate::mir::Callee::Extern(name) => crate::mir::Callee::Extern(name.clone()),
            }
        };

        match inst {
            Const { dst, value } => Const {
                dst: remap(*dst),
                value: value.clone(),
            },
            UnaryOp { dst, op, operand } => UnaryOp {
                dst: remap(*dst),
                op: *op,
                operand: remap(*operand),
            },
            BinOp { dst, op, lhs, rhs } => BinOp {
                dst: remap(*dst),
                op: *op,
                lhs: remap(*lhs),
                rhs: remap(*rhs),
            },
            Compare { dst, op, lhs, rhs } => Compare {
                dst: remap(*dst),
                op: *op,
                lhs: remap(*lhs),
                rhs: remap(*rhs),
            },
            Load { dst, ptr } => Load {
                dst: remap(*dst),
                ptr: remap(*ptr),
            },
            Store { value, ptr } => Store {
                value: remap(*value),
                ptr: remap(*ptr),
            },
            Call {
                dst,
                func,
                callee,
                args,
                effects,
            } => Call {
                dst: dst.map(remap),
                func: remap(*func),
                callee: callee.as_ref().map(remap_callee),
                args: args.iter().map(|&a| remap(a)).collect(),
                effects: *effects,
            },
            Copy { dst, src } => Copy {
                dst: remap(*dst),
                src: remap(*src),
            },
            NewBox {
                dst,
                box_type,
                args,
            } => NewBox {
                dst: remap(*dst),
                box_type: box_type.clone(),
                args: args.iter().map(|&a| remap(a)).collect(),
            },
            NewClosure {
                dst,
                params,
                body_id,
                body,
                captures,
                me,
            } => NewClosure {
                dst: remap(*dst),
                params: params.clone(),
                body_id: *body_id,
                body: body.clone(),
                captures: captures
                    .iter()
                    .map(|(n, v)| (n.clone(), remap(*v)))
                    .collect(),
                me: me.map(remap),
            },
            Debug { value, message } => Debug {
                value: remap(*value),
                message: message.clone(),
            },
            // Phase 287: Lifecycle management remaps all values
            KeepAlive { values } => KeepAlive {
                values: values.iter().map(|&v| remap(v)).collect(),
            },
            ReleaseStrong { values } => ReleaseStrong {
                values: values.iter().map(|&v| remap(v)).collect(),
            },
            Throw { exception, effects } => Throw {
                exception: remap(*exception),
                effects: *effects,
            },
            Catch {
                exception_type,
                exception_value,
                handler_bb,
            } => Catch {
                exception_type: exception_type.clone(),
                exception_value: remap(*exception_value),
                handler_bb: *handler_bb,
            },
            RefNew { dst, box_val } => RefNew {
                dst: remap(*dst),
                box_val: remap(*box_val),
            },
            WeakRef { dst, op, value } => WeakRef {
                dst: remap(*dst),
                op: *op,
                value: remap(*value),
            },
            Barrier { op, ptr } => Barrier {
                op: *op,
                ptr: remap(*ptr),
            },
            FutureNew { dst, value } => FutureNew {
                dst: remap(*dst),
                value: remap(*value),
            },
            FutureSet { future, value } => FutureSet {
                future: remap(*future),
                value: remap(*value),
            },
            Await { dst, future } => Await {
                dst: remap(*dst),
                future: remap(*future),
            },
            TypeOp { dst, op, value, ty } => TypeOp {
                dst: remap(*dst),
                op: *op,
                value: remap(*value),
                ty: ty.clone(),
            },
            // Phase 189 FIX: Remap PHI dst and input values (BlockId remapping is done in control_flow.rs)
            Phi {
                dst,
                inputs,
                type_hint,
            } => Phi {
                dst: remap(*dst),
                inputs: inputs.iter().map(|(bb, val)| (*bb, remap(*val))).collect(),
                type_hint: type_hint.clone(),
            },
            // Phase 256 P1.5: Remap Select instruction (ternary conditional)
            Select {
                dst,
                cond,
                then_val,
                else_val,
            } => Select {
                dst: remap(*dst),
                cond: remap(*cond),
                then_val: remap(*then_val),
                else_val: remap(*else_val),
            },
            Branch {
                condition,
                then_bb,
                else_bb,
                then_edge_args,
                else_edge_args,
            } => Branch {
                condition: remap(*condition),
                then_bb: *then_bb,
                else_bb: *else_bb,
                then_edge_args: remap_edge_args(then_edge_args),
                else_edge_args: remap_edge_args(else_edge_args),
            },
            Jump { target, edge_args } => Jump {
                target: *target,
                edge_args: remap_edge_args(edge_args),
            },
            Return { value } => Return {
                value: value.map(remap),
            },
            // Pass through unchanged
            Safepoint => inst.clone(),
        }
    }

    /// Value ID をリマップ
    pub fn remap_value(&self, v: ValueId) -> ValueId {
        self.value_map.get(&v).copied().unwrap_or(v)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remapper_new() {
        let remapper = JoinIrIdRemapper::new();
        assert_eq!(remapper.get_value(ValueId(0)), None);
    }

    #[test]
    fn test_remapper_set_and_get_value() {
        let mut remapper = JoinIrIdRemapper::new();
        remapper.set_value(ValueId(0), ValueId(10));
        assert_eq!(remapper.get_value(ValueId(0)), Some(ValueId(10)));
    }

    #[test]
    fn test_remapper_set_and_get_block() {
        let mut remapper = JoinIrIdRemapper::new();
        remapper.set_block("main".to_string(), BasicBlockId(0), BasicBlockId(100));
        assert_eq!(
            remapper.get_block("main", BasicBlockId(0)),
            Some(BasicBlockId(100))
        );
    }

    #[test]
    fn test_remap_value() {
        let mut remapper = JoinIrIdRemapper::new();
        remapper.set_value(ValueId(5), ValueId(50));
        assert_eq!(remapper.remap_value(ValueId(5)), ValueId(50));
        assert_eq!(remapper.remap_value(ValueId(99)), ValueId(99)); // Unmapped returns original
    }

    #[test]
    fn test_collect_values_simple() {
        let remapper = JoinIrIdRemapper::new();
        let inst = MirInstruction::BinOp {
            dst: ValueId(1),
            op: crate::mir::types::BinaryOp::Add,
            lhs: ValueId(2),
            rhs: ValueId(3),
        };
        let values = remapper.collect_values_in_instruction(&inst);
        assert_eq!(values, vec![ValueId(1), ValueId(2), ValueId(3)]);
    }
}
