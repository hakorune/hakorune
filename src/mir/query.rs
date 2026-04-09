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
            Const { .. } => Vec::new(),
            Copy { src, .. } => vec![*src],
            UnaryOp { operand, .. } => vec![*operand],
            BinOp { lhs, rhs, .. } | Compare { lhs, rhs, .. } => {
                vec![*lhs, *rhs]
            }
            TypeOp { value, .. } => vec![*value],
            FieldGet { base, .. } => vec![*base],
            FieldSet { base, value, .. } => vec![*base, *value],
            SumMake { payload, .. } => payload.iter().copied().collect(),
            SumTag { value, .. } | SumProject { value, .. } => vec![*value],
            Load { ptr, .. } => vec![*ptr],
            Store { ptr, value } => vec![*ptr, *value],
            Call {
                callee, func, args, ..
            } => {
                let mut used = Vec::new();
                if let Some(crate::mir::definitions::call_unified::Callee::Method {
                    receiver: Some(r),
                    ..
                }) = callee
                {
                    used.push(*r);
                } else if callee.is_none() {
                    used.push(*func);
                }
                used.extend(args.iter().copied());
                used
            }
            Return { value } => value.iter().copied().collect(),
            Branch { condition, .. } => vec![*condition],
            Jump { .. } => Vec::new(),
            Phi { inputs, .. } => inputs.iter().map(|(_, v)| *v).collect(),
            NewBox { args, .. } => args.clone(),
            Debug { value, .. } => vec![*value],
            // Phase 287: Lifecycle management reads all values
            KeepAlive { values } => values.clone(),
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
        use MirInstruction::*;
        match inst {
            Const { dst, .. }
            | UnaryOp { dst, .. }
            | BinOp { dst, .. }
            | Compare { dst, .. }
            | TypeOp { dst, .. }
            | FieldGet { dst, .. }
            | SumMake { dst, .. }
            | SumTag { dst, .. }
            | SumProject { dst, .. }
            | Load { dst, .. }
            | Call { dst: Some(dst), .. }
            | Phi { dst, .. }
            | NewBox { dst, .. }
            | RefNew { dst, .. }
            | WeakRef { dst, .. }
            | FutureNew { dst, .. }
            | NewClosure { dst, .. }
            | Await { dst, .. }
            | Copy { dst, .. }
            | Select { dst, .. } => vec![*dst], // Copy writes to dst, Select writes to dst
            // No writes
            Store { .. }
            | FieldSet { .. }
            | Call { dst: None, .. }
            | Return { .. }
            | Branch { .. }
            | Jump { .. }
            | Debug { .. }
            | Throw { .. }
            | Catch { .. }
            | Barrier { .. }
            | FutureSet { .. }
            | Safepoint => Vec::new(),
            _ => Vec::new(),
        }
    }
}
