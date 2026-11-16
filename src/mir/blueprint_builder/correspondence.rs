//! Source-directed checking of the finished MIR, independent of its writer.
use super::{
    declaration_table::*,
    lower::{Correspondence, Cursor, LoopImage, Role},
};
use crate::mir::{
    BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, EffectMask, MirFunction, MirInstruction,
    MirModule, UnaryOp, ValueId,
};
use std::collections::{BTreeMap, BTreeSet};
#[path = "correspondence_control.rs"]
mod control;
type Environment = BTreeMap<BindingId, ValueId>;
#[derive(Clone)]
struct State {
    cursor: Cursor,
    values: Environment,
}
struct LoopFrame {
    id: NodeId,
    image: LoopImage,
    breaks: Vec<State>,
    continues: Vec<State>,
}
struct Checker<'a> {
    source: &'a ResolvedFunction,
    table: &'a DeclarationTable,
    function: &'a MirFunction,
    records: &'a Correspondence,
    current: Option<State>,
    seen: BTreeSet<Cursor>,
    nodes: BTreeSet<NodeId>,
    expressions: BTreeSet<NodeId>,
    roles: BTreeSet<(NodeId, Role)>,
    phis: BTreeSet<(BasicBlockId, BindingId)>,
    joins: BTreeSet<NodeId>,
    loop_nodes: BTreeSet<NodeId>,
    loops: Vec<LoopFrame>,
}
fn bad(reason: &str) -> String {
    format!("blueprint/internal/correspondence/{reason}")
}
pub(super) fn verify(
    source: &ResolvedModule,
    module: &MirModule,
    records: &[Correspondence],
) -> Result<(), String> {
    if records.len() != source.functions.len() || module.functions.len() != source.functions.len() {
        return Err(bad("function-census"));
    }
    for (body, records) in source.functions.iter().zip(records) {
        let declaration = &source.declarations.entries[body.declaration.0];
        let function = module
            .functions
            .get(&declaration.key.mir_symbol_projection())
            .ok_or_else(|| bad("function-missing"))?;
        if function.signature.params
            != declaration
                .parameters
                .iter()
                .map(|(_, t)| t.mir())
                .collect::<Vec<_>>()
            || function.params.len() != declaration.parameters.len()
            || function.signature.return_type != declaration.result.mir()
            || function.signature.effects
                != if declaration.io {
                    EffectMask::IO
                } else {
                    EffectMask::PURE
                }
            || function.metadata.is_entry_point != (declaration.id == source.entry)
            || module.canonical_callable_definition_symbol(&declaration.key)
                != Some(&function.signature.name)
        {
            return Err(bad("signature"));
        }
        let mut environment = Environment::new();
        for (index, (&value, (_, ty))) in function
            .params
            .iter()
            .zip(&declaration.parameters)
            .enumerate()
        {
            if function.metadata.value_types.get(&value) != Some(&ty.mir()) {
                return Err(bad("parameter-type"));
            }
            environment.insert(BindingId(index), value);
        }
        let mut checker = Checker {
            source: body,
            table: &source.declarations,
            function,
            records,
            current: Some(State {
                cursor: Cursor {
                    block: function.entry_block,
                    index: 0,
                },
                values: environment,
            }),
            seen: BTreeSet::new(),
            nodes: BTreeSet::new(),
            expressions: BTreeSet::new(),
            roles: BTreeSet::new(),
            phis: BTreeSet::new(),
            joins: BTreeSet::new(),
            loop_nodes: BTreeSet::new(),
            loops: Vec::new(),
        };
        checker.block(&body.body)?;
        let count: usize = function
            .blocks
            .values()
            .map(|b| b.instructions.len() + usize::from(b.terminator.is_some()))
            .sum();
        if checker.current.is_some()
            || checker.seen.len() != count
            || checker.nodes != records.nodes
            || checker.roles != records.operations.keys().copied().collect()
            || checker.phis != records.phis.keys().copied().collect()
            || checker.joins != records.joins.keys().copied().collect()
            || checker.loop_nodes != records.loops.keys().copied().collect()
            || checker.expressions != records.values.keys().copied().collect()
        {
            return Err(bad("coverage"));
        }
    }
    Ok(())
}
impl Checker<'_> {
    fn mark(&mut self, node: NodeId) -> Result<(), String> {
        if !self.nodes.insert(node) {
            return Err(bad("duplicate-source"));
        }
        Ok(())
    }
    fn state(&self) -> Result<&State, String> {
        self.current
            .as_ref()
            .ok_or_else(|| bad("missing-continuation"))
    }
    fn read(&self, binding: BindingId) -> Result<ValueId, String> {
        self.state()?
            .values
            .get(&binding)
            .copied()
            .ok_or_else(|| bad("binding-missing"))
    }
    fn bind(&mut self, binding: BindingId, value: ValueId) -> Result<(), String> {
        self.current
            .as_mut()
            .ok_or_else(|| bad("missing-continuation"))?
            .values
            .insert(binding, value);
        Ok(())
    }
    fn typed(&self, value: ValueId, ty: Type) -> Result<(), String> {
        if self.function.metadata.value_types.get(&value) != Some(&ty.mir()) {
            Err(bad("value-type"))
        } else {
            Ok(())
        }
    }
    fn take(&mut self, role: Option<(NodeId, Role)>) -> Result<MirInstruction, String> {
        let cursor = self.state()?.cursor;
        if !self.seen.insert(cursor) {
            return Err(bad("repeated-operation"));
        }
        if let Some(role) = role {
            if self.records.operations.get(&role) != Some(&cursor) || !self.roles.insert(role) {
                return Err(bad("operation-position"));
            }
        }
        let block = self
            .function
            .blocks
            .get(&cursor.block)
            .ok_or_else(|| bad("block-missing"))?;
        let operation = if cursor.index == block.instructions.len() {
            block.terminator.as_ref()
        } else {
            block.instructions.get(cursor.index)
        }
        .ok_or_else(|| bad("operation-missing"))?
        .clone();
        self.current.as_mut().unwrap().cursor.index += 1;
        Ok(operation)
    }
    fn jump(
        &mut self,
        role: Option<(NodeId, Role)>,
        expected: BasicBlockId,
    ) -> Result<State, String> {
        if !matches!(self.take(role)?,MirInstruction::Jump{target,edge_args:None}if target==expected)
        {
            return Err(bad("jump-target"));
        }
        Ok(self.current.take().unwrap())
    }
    fn branch(
        &mut self,
        node: NodeId,
        condition: ValueId,
    ) -> Result<(State, BasicBlockId, BasicBlockId), String> {
        let MirInstruction::Branch {
            condition: actual,
            then_bb,
            else_bb,
            then_edge_args: None,
            else_edge_args: None,
        } = self.take(Some((node, Role::Control)))?
        else {
            return Err(bad("branch-kind"));
        };
        if actual != condition || then_bb == else_bb {
            return Err(bad("branch-condition"));
        }
        Ok((self.current.take().unwrap(), then_bb, else_bb))
    }
    fn enter(&mut self, block: BasicBlockId, values: Environment) {
        self.current = Some(State {
            cursor: Cursor { block, index: 0 },
            values,
        });
    }
    fn expression(&mut self, source: &Expression) -> Result<Option<ValueId>, String> {
        self.mark(source.id)?;
        self.expressions.insert(source.id);
        let value = match &source.kind {
            ExpressionKind::Read(binding) => Some(self.read(*binding)?),
            ExpressionKind::Integer(expected) => {
                let MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(actual),
                } = self.take(Some((source.id, Role::Value)))?
                else {
                    return Err(bad("integer-kind"));
                };
                if actual != *expected {
                    return Err(bad("integer-value"));
                }
                Some(dst)
            }
            ExpressionKind::Bool(expected) => {
                let MirInstruction::Const {
                    dst,
                    value: ConstValue::Bool(actual),
                } = self.take(Some((source.id, Role::Value)))?
                else {
                    return Err(bad("bool-kind"));
                };
                if actual != *expected {
                    return Err(bad("bool-value"));
                }
                Some(dst)
            }
            ExpressionKind::Unary { operator, operand } => {
                let operand = self.operand(operand)?;
                let expected = match operator {
                    Unary::Minus => UnaryOp::Neg,
                    Unary::Not => UnaryOp::Not,
                    _ => return Err(bad("unresolved-unary")),
                };
                let MirInstruction::UnaryOp {
                    dst,
                    op,
                    operand: actual,
                } = self.take(Some((source.id, Role::Value)))?
                else {
                    return Err(bad("unary-kind"));
                };
                if op != expected || actual != operand {
                    return Err(bad("unary-operands"));
                }
                Some(dst)
            }
            ExpressionKind::Binary {
                operator,
                left,
                right,
            } if matches!(operator, Binary::And | Binary::Or) => {
                Some(self.logical(source, left, right, matches!(operator, Binary::And))?)
            }
            ExpressionKind::Binary {
                operator,
                left,
                right,
            } => {
                let left = self.operand(left)?;
                let right = self.operand(right)?;
                Some(self.binary(source.id, Role::Value, operator, left, right)?)
            }
            ExpressionKind::Call(call) => self.call(source, call)?,
        };
        if let Some(value) = value {
            self.typed(value, source.ty)?;
        }
        if self.records.values.get(&source.id) != Some(&value) {
            return Err(bad("result-relation"));
        }
        Ok(value)
    }
    fn operand(&mut self, source: &Expression) -> Result<ValueId, String> {
        self.expression(source)?.ok_or_else(|| bad("unit-operand"))
    }
    fn binary(
        &mut self,
        node: NodeId,
        role: Role,
        operator: &Binary,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<ValueId, String> {
        // Expected operator is read here from the trusted Resolved expression,
        // never from a Lower helper or its positional record.
        let actual = self.take(Some((node, role)))?;
        let (dst, left, right, correct) = match actual {
            MirInstruction::BinOp { dst, op, lhs, rhs } => {
                let expected = match operator {
                    Binary::Add => BinaryOp::Add,
                    Binary::Subtract => BinaryOp::Sub,
                    Binary::Multiply => BinaryOp::Mul,
                    Binary::Divide => BinaryOp::Div,
                    _ => return Err(bad("arithmetic-kind")),
                };
                (dst, lhs, rhs, op == expected)
            }
            MirInstruction::Compare { dst, op, lhs, rhs } => {
                let expected = match operator {
                    Binary::Equal => CompareOp::Eq,
                    Binary::NotEqual => CompareOp::Ne,
                    Binary::Less => CompareOp::Lt,
                    Binary::LessEqual => CompareOp::Le,
                    Binary::Greater => CompareOp::Gt,
                    _ => return Err(bad("comparison-kind")),
                };
                (dst, lhs, rhs, op == expected)
            }
            _ => return Err(bad("binary-kind")),
        };
        if !correct || left != lhs || right != rhs {
            return Err(bad("binary-operands"));
        }
        Ok(dst)
    }
    fn call(
        &mut self,
        source: &Expression,
        call: &ResolvedCall,
    ) -> Result<Option<ValueId>, String> {
        if call.receiver.is_some() || call.may_fault {
            return Err(bad("receiver"));
        }
        let (target, io) = match call.target {
            CallTarget::Print => (
                hakorune_mir_defs::CanonicalGlobalTargetV1::builtin_print(),
                true,
            ),
            CallTarget::Declaration(id) => {
                let d = &self.table.entries[id.0];
                (d.key.canonical_global_target_v1()?, d.io)
            }
        };
        let mut arguments = Vec::new();
        for arg in &call.arguments {
            arguments.push(self.operand(arg)?);
        }
        let MirInstruction::Call(actual) = self.take(Some((source.id, Role::Value)))? else {
            return Err(bad("call-kind"));
        };
        if actual.callee != Callee::Global(target)
            || actual.args != arguments
            || !actual.text_argument_cleanup.is_empty()
            || actual.effects != if io { EffectMask::IO } else { EffectMask::PURE }
            || actual.flags != crate::mir::definitions::CallFlags::new()
            || (source.ty == Type::Unit) != actual.dst.is_none()
        {
            return Err(bad("call-contract"));
        }
        Ok(actual.dst)
    }
    fn block(&mut self, body: &[Statement]) -> Result<(), String> {
        for statement in body {
            if self.current.is_none() {
                break;
            }
            self.statement(statement)?;
        }
        Ok(())
    }
    fn statement(&mut self, source: &Statement) -> Result<(), String> {
        self.mark(source.id)?;
        match &source.kind {
            StatementKind::Local { binding, value } | StatementKind::Assign { binding, value } => {
                let value = self.operand(value)?;
                self.bind(*binding, value)?;
            }
            StatementKind::Compound {
                binding,
                operator,
                value,
            } => {
                let old = self.read(*binding)?;
                let rhs = self.operand(value)?;
                let value = self.binary(source.id, Role::Compound, operator, old, rhs)?;
                self.typed(value, self.source.bindings[binding.0].ty)?;
                self.bind(*binding, value)?;
            }
            StatementKind::Expression(value) => {
                let _ = self.expression(value)?;
            }
            StatementKind::Return(value) => {
                let value = self.operand(value)?;
                if !matches!(self.take(Some((source.id,Role::Control)))?,MirInstruction::Return{value:Some(actual)}if actual==value)
                {
                    return Err(bad("return-value"));
                }
                self.current = None;
            }
            StatementKind::If {
                condition,
                yes,
                no,
                carriers,
            } => self.conditional(source.id, condition, yes, no, carriers)?,
            StatementKind::Loop(contract) => self.loop_statement(source.id, contract)?,
            StatementKind::Break(target) | StatementKind::Continue(target) => {
                let position = self
                    .loops
                    .iter()
                    .rposition(|l| l.id == *target)
                    .ok_or_else(|| bad("control-target"))?;
                let frame = &self.loops[position];
                let break_ = matches!(source.kind, StatementKind::Break(_));
                let expected = if break_ {
                    frame.image.exit
                } else {
                    frame.image.step.unwrap_or(frame.image.header)
                };
                let state = self.jump(Some((source.id, Role::Control)), expected)?;
                if break_ {
                    self.loops[position].breaks.push(state);
                } else {
                    self.loops[position].continues.push(state);
                }
            }
            StatementKind::Scope(body) => self.block(body)?,
        }
        Ok(())
    }
}
