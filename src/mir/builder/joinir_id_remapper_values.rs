//! Value discovery for the existing JoinIR ID remapper.
use super::JoinIrIdRemapper;
use crate::mir::{BasicBlock, MirInstruction, ValueId};

impl JoinIrIdRemapper {
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
            StaticDataLoad { dst, index, .. } => vec![*dst, *index],
            Store { value, ptr } => vec![*value, *ptr],
            ArrayElementWrite {
                dst,
                receiver,
                index,
                value,
                ..
            } => {
                let mut vals = Vec::new();
                vals.extend(dst.iter().copied());
                vals.push(*receiver);
                vals.extend(index.iter().copied());
                vals.push(*value);
                vals
            }
            ArrayStateContractClaim { array, .. } => vec![*array],
            MemOp { dst, operands, .. } => {
                let mut vals = Vec::new();
                vals.extend(dst.iter().copied());
                vals.extend(operands.iter().copied());
                vals
            }
            PinnedTextOp { dst, kind, .. } => {
                let mut vals = vec![*dst];
                vals.extend(kind.used_values());
                vals
            }
            FieldGet { dst, base, .. } | ObjectFieldGet { dst, base, .. } => vec![*dst, *base],
            FieldSet { base, value, .. } => vec![*base, *value],
            WeakFieldWrite { base, value, .. } => vec![*base, *value],
            VariantMake { dst, payload, .. } => {
                let mut vals = vec![*dst];
                vals.extend(payload.iter().copied());
                vals
            }
            VariantTag { dst, value, .. } | VariantProject { dst, value, .. } => vec![*dst, *value],
            Call(call) => {
                let mut vals = Vec::new();
                call.callee.for_each_value_operand(|value| vals.push(value));
                vals.extend(call.dst.iter().copied());
                vals.extend(call.args.iter().copied());
                vals
            }
            LegacyCallV0 {
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
            Invoke { .. } | ReturnFault { .. } => inst.used_values(),
            InvokeNormalResult { dst, .. } | FaultFrameEnter { dst, .. } => vec![*dst],
            CheckedCallOut {
                receiver,
                arguments,
                ..
            } => {
                let mut vals = vec![*receiver];
                vals.extend(arguments.iter().copied());
                vals
            }
            CheckedCallOutNormalResult { dst, .. } => vec![*dst],
            CheckedCallOutEnd { .. }
            | CheckedCallOutFault { .. }
            | PinnedTextResidenceFinish { .. }
            | PinnedTextResidenceEnter { .. }
            | PinnedTextResidenceTrap { .. } => Vec::new(),
            Phi { dst, inputs, .. } => {
                let mut vals = vec![*dst];
                vals.extend(inputs.iter().map(|(_, v)| *v));
                vals
            }
            Copy { dst, src } | CopyOwned { dst, src } | LocalContractWrite { dst, src, .. } => {
                vec![*dst, *src]
            }
            RecordFieldContractCheck { value, .. } => vec![*value],
            RecordValuePublish {
                dst, base, fields, ..
            } => {
                let mut values = vec![*dst];
                values.extend(base.iter().copied());
                values.extend(fields.iter().copied());
                values
            }
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
            DestroyOwned { value } => vec![*value],
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
}
