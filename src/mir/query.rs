//! MirQuery - Read/Write/CFGビューを提供する共通窓口
//!
//! Box理論: MIR 全体の構造は MirQueryBox が保持し、他の箱（ExitLiveness など）は
//! 「見せる窓」である MirQuery トレイト越しにしか触らないようにする。
//! これにより MIR 構造への依存を最小化し、テスタビリティと疎結合を保つ。

use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

/// MIR への読み取り専用ビュー
pub trait MirQuery {
    /// ブロック内の命令列（PHI を含む）を順序付きで返す
    fn insts_in_block(&self, bb: BasicBlockId) -> &[MirInstruction];

    /// ブロックの後続（succs）を決定的順序で返す
    fn succs(&self, bb: BasicBlockId) -> Vec<BasicBlockId>;

    /// 命令が読む（use する）ValueId のリスト
    fn reads_of(&self, inst: &MirInstruction) -> Vec<ValueId>;

    /// 命令が書く（def する）ValueId のリスト
    fn writes_of(&self, inst: &MirInstruction) -> Vec<ValueId>;
}

/// MirQuery の標準実装：MirFunction 全体を抱えつつビューを提供
pub struct MirQueryBox<'m> {
    mir: &'m MirFunction,
}

impl<'m> MirQueryBox<'m> {
    pub fn new(mir: &'m MirFunction) -> Self {
        Self { mir }
    }
}

impl<'m> MirQuery for MirQueryBox<'m> {
    fn insts_in_block(&self, bb: BasicBlockId) -> &[MirInstruction] {
        static EMPTY: &[MirInstruction] = &[];
        self.mir
            .blocks
            .get(&bb)
            .map(|bb| bb.instructions.as_slice())
            .unwrap_or(EMPTY)
    }

    fn succs(&self, bb: BasicBlockId) -> Vec<BasicBlockId> {
        let mut v: Vec<_> = self
            .mir
            .blocks
            .get(&bb)
            .map(|bb| bb.successors.iter().copied().collect())
            .unwrap_or_else(Vec::new);
        v.sort_by_key(|b| b.0);
        v
    }

    fn reads_of(&self, inst: &MirInstruction) -> Vec<ValueId> {
        use MirInstruction::*;
        match inst {
            Invoke { .. }
            | InvokeNormalResult { .. }
            | ReturnFault { .. }
            | FaultFrameEnter { .. } => inst.used_values(),
            Const { .. } => Vec::new(),
            Copy { src, .. } | CopyOwned { src, .. } | LocalContractWrite { src, .. } => vec![*src],
            RecordFieldContractCheck { value, .. } => vec![*value],
            RecordValuePublish { base, fields, .. } => {
                let mut values = base.iter().copied().collect::<Vec<_>>();
                values.extend(fields.iter().copied());
                values
            }
            ArrayElementWrite {
                receiver,
                index,
                value,
                ..
            } => {
                let mut values = vec![*receiver];
                values.extend(index.iter().copied());
                values.push(*value);
                values
            }
            ArrayStateContractClaim { array, .. } => vec![*array],
            UnaryOp { operand, .. } => vec![*operand],
            BinOp { lhs, rhs, .. } | Compare { lhs, rhs, .. } => {
                vec![*lhs, *rhs]
            }
            TypeOp { value, .. } => vec![*value],
            FieldGet { base, .. } => vec![*base],
            FieldSet { base, value, .. } => vec![*base, *value],
            WeakFieldWrite { base, value, .. } => vec![*base, *value],
            VariantMake { payload, .. } => payload.iter().copied().collect(),
            VariantTag { value, .. } | VariantProject { value, .. } => vec![*value],
            Load { ptr, .. } => vec![*ptr],
            StaticDataLoad { index, .. } => vec![*index],
            Store { ptr, value } => vec![*ptr, *value],
            MemOp { operands, .. } => operands.clone(),
            PinnedTextOp { kind, .. } => kind.used_values(),
            PinnedTextResidenceFinish { .. }
            | PinnedTextResidenceEnter { .. }
            | PinnedTextResidenceTrap { .. } => Vec::new(),
            Call(_) | LegacyCallV0 { .. } => inst.used_values(),
            Return { value } => value.iter().copied().collect(),
            CheckedCallOut {
                receiver,
                arguments,
                ..
            } => {
                let mut values = vec![*receiver];
                values.extend(arguments.iter().copied());
                values
            }
            CheckedCallOutNormalResult { .. } => Vec::new(),
            CheckedCallOutEnd { .. } | CheckedCallOutFault { .. } => Vec::new(),
            Branch { condition, .. } => vec![*condition],
            Jump { .. } => Vec::new(),
            Phi { inputs, .. } => inputs.iter().map(|(_, v)| *v).collect(),
            NewBox { args, .. } => args.clone(),
            Debug { value, .. } => vec![*value],
            // Phase 287: Lifecycle management reads all values
            KeepAlive { values } => values.clone(),
            DestroyOwned { value } => vec![*value],
            ReleaseStrong { values } => values.clone(),
            Throw { exception, .. } => vec![*exception],
            Catch { .. } => Vec::new(),
            NewClosure { captures, me, .. } => {
                let mut v: Vec<ValueId> = captures.iter().map(|(_, v)| *v).collect();
                if let Some(m) = me {
                    v.push(*m);
                }
                v
            }
            RefNew { box_val, .. } => vec![*box_val],
            WeakRef { value, .. } => vec![*value],
            Barrier { ptr, .. } => {
                vec![*ptr]
            }
            FutureNew { value, .. } => vec![*value],
            FutureSet { future, value } => vec![*future, *value],
            Await { future, .. } => vec![*future],
            Safepoint => Vec::new(),
            // Phase 256 P1.5: Select instruction reads cond, then_val, else_val
            Select {
                cond,
                then_val,
                else_val,
                ..
            } => vec![*cond, *then_val, *else_val],
        }
    }

    fn writes_of(&self, inst: &MirInstruction) -> Vec<ValueId> {
        if matches!(
            inst,
            MirInstruction::Call(_) | MirInstruction::LegacyCallV0 { .. }
        ) {
            return inst.dst_value().into_iter().collect();
        }

        use MirInstruction::*;
        match inst {
            InvokeNormalResult { dst, .. } | FaultFrameEnter { dst, .. } => vec![*dst],
            Const { dst, .. }
            | UnaryOp { dst, .. }
            | BinOp { dst, .. }
            | Compare { dst, .. }
            | TypeOp { dst, .. }
            | FieldGet { dst, .. }
            | VariantMake { dst, .. }
            | VariantTag { dst, .. }
            | VariantProject { dst, .. }
            | Load { dst, .. }
            | StaticDataLoad { dst, .. }
            | MemOp { dst: Some(dst), .. }
            | PinnedTextOp { dst, .. }
            | ArrayElementWrite { dst: Some(dst), .. }
            | Phi { dst, .. }
            | NewBox { dst, .. }
            | RefNew { dst, .. }
            | WeakRef { dst, .. }
            | FutureNew { dst, .. }
            | NewClosure { dst, .. }
            | Await { dst, .. }
            | Copy { dst, .. }
            | CopyOwned { dst, .. }
            | LocalContractWrite { dst, .. }
            | RecordValuePublish { dst, .. }
            | Select { dst, .. }
            | CheckedCallOutNormalResult { dst, .. } => vec![*dst], // Copy writes to dst, Select writes to dst
            // No writes
            Store { .. }
            | MemOp { dst: None, .. }
            | FieldSet { .. }
            | Return { .. }
            | CheckedCallOut { .. }
            | PinnedTextResidenceEnter { .. }
            | PinnedTextResidenceTrap { .. }
            | PinnedTextResidenceFinish { .. }
            | Branch { .. }
            | Jump { .. }
            | Debug { .. }
            | DestroyOwned { .. }
            | Throw { .. }
            | Catch { .. }
            | Barrier { .. }
            | FutureSet { .. }
            | RecordFieldContractCheck { .. }
            | Safepoint => Vec::new(),
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MirQuery, MirQueryBox};
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlockId, Callee, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType,
        ValueId,
    };

    fn query_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "query_call_test/0".into(),
                params: Vec::new(),
                return_type: MirType::Box("QueryTestBox".into()),
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn call(callee: Option<Callee>, dst: Option<ValueId>) -> MirInstruction {
        MirInstruction::LegacyCallV0 {
            dst,
            func: ValueId::new(99),
            callee,
            args: vec![ValueId::new(40), ValueId::new(41)],
            effects: EffectMask::PURE,
        }
    }

    #[test]
    fn query_call_reads_match_canonical_used_values_for_every_shape() {
        let function = query_function();
        let query = MirQueryBox::new(&function);
        let cases = [
            (
                Callee::Global(crate::mir::test_global_target("global/2".to_string())),
                vec![],
            ),
            (Callee::Extern("env.global/2".to_string()), vec![]),
            (
                Callee::Constructor {
                    box_type: "QueryTestBox".to_string(),
                },
                vec![],
            ),
            (
                Callee::Method {
                    box_name: "QueryTestBox".to_string(),
                    method: "read".to_string(),
                    receiver: None,
                    certainty: TypeCertainty::Known,
                    box_kind: CalleeBoxKind::UserDefined,
                },
                vec![],
            ),
            (
                Callee::Method {
                    box_name: "QueryTestBox".to_string(),
                    method: "read".to_string(),
                    receiver: Some(ValueId::new(10)),
                    certainty: TypeCertainty::Known,
                    box_kind: CalleeBoxKind::RuntimeData,
                },
                vec![ValueId::new(10)],
            ),
            (Callee::Value(ValueId::new(20)), vec![ValueId::new(20)]),
            (
                Callee::Closure {
                    params: vec!["x".to_string()],
                    captures: vec![
                        ("a".to_string(), ValueId::new(30)),
                        ("b".to_string(), ValueId::new(30)),
                    ],
                    me_capture: Some(ValueId::new(31)),
                },
                vec![ValueId::new(30), ValueId::new(30), ValueId::new(31)],
            ),
        ];

        for (callee, target_operands) in cases {
            let instruction = call(Some(callee), Some(ValueId::new(1)));
            let mut expected = target_operands;
            expected.extend([ValueId::new(40), ValueId::new(41)]);
            assert_eq!(query.reads_of(&instruction), expected);
            assert_eq!(query.reads_of(&instruction), instruction.used_values());
        }

        let legacy = call(None, None);
        assert_eq!(
            query.reads_of(&legacy),
            vec![ValueId::new(99), ValueId::new(40), ValueId::new(41)]
        );
        assert_eq!(query.reads_of(&legacy), legacy.used_values());
    }

    #[test]
    fn query_call_writes_match_canonical_dst_value_and_ignore_target_shape() {
        let function = query_function();
        let query = MirQueryBox::new(&function);
        for callee in [
            Some(Callee::Global(crate::mir::test_global_target(
                "global/0".to_string(),
            ))),
            Some(Callee::Value(ValueId::new(20))),
            None,
        ] {
            for dst in [Some(ValueId::new(1)), None] {
                let instruction = call(callee.clone(), dst);
                assert_eq!(
                    query.writes_of(&instruction),
                    instruction.dst_value().into_iter().collect::<Vec<_>>()
                );
            }
        }
    }
}
