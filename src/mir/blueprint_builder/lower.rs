//! One recursive expression/statement writer over Resolve's contract.
use super::declaration_table::*;
use crate::mir::definitions::MirCall;
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, EffectMask,
    FunctionSignature, MirFunction, MirInstruction, UnaryOp, ValueId,
};
use std::collections::{BTreeMap, BTreeSet};
#[path = "lower_control.rs"]
mod control;
#[path = "workspace.rs"]
mod workspace;

type Environment = BTreeMap<BindingId, ValueId>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Cursor {
    pub block: BasicBlockId,
    pub index: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Role {
    Value,
    Control,
    Compound,
    RangeTest,
    RangeOne,
    RangeStep,
}
#[derive(Debug, Clone, Copy)]
pub(super) struct LoopImage {
    pub header: BasicBlockId,
    pub body: BasicBlockId,
    pub step: Option<BasicBlockId>,
    pub exit: BasicBlockId,
}
/// Locations and source roles only; never expected operations or producer paths.
#[derive(Debug, Default)]
pub(super) struct Correspondence {
    pub operations: BTreeMap<(NodeId, Role), Cursor>,
    pub values: BTreeMap<NodeId, Option<ValueId>>,
    pub nodes: BTreeSet<NodeId>,
    pub joins: BTreeMap<NodeId, BasicBlockId>,
    pub loops: BTreeMap<NodeId, LoopImage>,
    pub phis: BTreeMap<(BasicBlockId, BindingId), ValueId>,
}
pub(super) struct Draft {
    pub declaration: DeclarationId,
    pub function: MirFunction,
    pub correspondence: Correspondence,
}
/// Blueprint §5: no nonexistent value escapes a terminated expression.
enum ExpressionResult {
    Continued {
        block: BasicBlockId,
        value: Option<ValueId>,
        ty: Type,
    },
    Terminated,
}
impl ExpressionResult {
    fn value(self) -> Result<Option<ValueId>, String> {
        match self {
            Self::Continued {
                value: Some(value), ..
            } => Ok(Some(value)),
            Self::Terminated => Ok(None),
            _ => Err("blueprint/internal/unit-operand".into()),
        }
    }
}
struct LoopFrame {
    id: NodeId,
    image: LoopImage,
    breaks: Vec<BasicBlockId>,
    continues: Vec<BasicBlockId>,
}
struct Workspace<'a> {
    source: &'a ResolvedFunction,
    table: &'a DeclarationTable,
    function: MirFunction,
    current: Option<BasicBlockId>,
    environments: BTreeMap<BasicBlockId, Environment>,
    correspondence: Correspondence,
    loops: Vec<LoopFrame>,
}
pub(super) fn module(source: &ResolvedModule) -> Result<Vec<Draft>, String> {
    source
        .functions
        .iter()
        .map(|body| {
            let declaration = &source.declarations.entries[body.declaration.0];
            let signature = FunctionSignature {
                name: declaration.key.mir_symbol_projection(),
                params: declaration
                    .parameters
                    .iter()
                    .map(|(_, t)| t.mir())
                    .collect(),
                return_type: declaration.result.mir(),
                effects: if declaration.io {
                    EffectMask::IO
                } else {
                    EffectMask::PURE
                },
            };
            let mut function = MirFunction::new(signature, BasicBlockId::new(0));
            // The existing EXE serializer/backend reserves physical value 0 for
            // an absent operand. Source binding ordinals remain zero-based.
            let parameter_count = declaration.parameters.len() as u32;
            function.params = (1..=parameter_count).map(ValueId::new).collect();
            function.next_value_id = parameter_count + 1;
            function.metadata.is_entry_point = declaration.id == source.entry;
            let mut environment = Environment::new();
            for (index, (&value, (_, ty))) in function
                .params
                .iter()
                .zip(&declaration.parameters)
                .enumerate()
            {
                function.metadata.value_types.insert(value, ty.mir());
                environment.insert(BindingId(index), value);
            }
            let mut workspace = Workspace {
                source: body,
                table: &source.declarations,
                function,
                current: Some(BasicBlockId::new(0)),
                environments: BTreeMap::from([(BasicBlockId::new(0), environment)]),
                correspondence: Correspondence::default(),
                loops: Vec::new(),
            };
            workspace.block(&body.body)?;
            if workspace.current.is_some() {
                return Err("blueprint/internal/unresolved-normal-exit".into());
            }
            Ok(Draft {
                declaration: body.declaration,
                function: workspace.function,
                correspondence: workspace.correspondence,
            })
        })
        .collect()
}
impl Workspace<'_> {
    fn expression(&mut self, source: &Expression) -> Result<ExpressionResult, String> {
        if self.current.is_none() {
            return Ok(ExpressionResult::Terminated);
        }
        self.mark(source.id)?;
        let value = match &source.kind {
            ExpressionKind::Read(binding) => Some(self.read(*binding)?),
            ExpressionKind::Integer(n) => {
                let dst = self.value(Type::I64);
                self.emit(
                    source.id,
                    Role::Value,
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::Integer(*n),
                    },
                )?;
                Some(dst)
            }
            ExpressionKind::Bool(b) => {
                let dst = self.value(Type::Bool);
                self.emit(
                    source.id,
                    Role::Value,
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::Bool(*b),
                    },
                )?;
                Some(dst)
            }
            ExpressionKind::Unary { operator, operand } => {
                let Some(operand) = self.expression(operand)?.value()? else {
                    return Ok(ExpressionResult::Terminated);
                };
                let op = match operator {
                    Unary::Minus => UnaryOp::Neg,
                    Unary::Not => UnaryOp::Not,
                    _ => return Err("blueprint/internal/unresolved-unary".into()),
                };
                let dst = self.value(source.ty);
                self.emit(
                    source.id,
                    Role::Value,
                    MirInstruction::UnaryOp { dst, op, operand },
                )?;
                Some(dst)
            }
            ExpressionKind::Binary {
                operator,
                left,
                right,
            } if matches!(operator, Binary::And | Binary::Or) => {
                self.logical(source, left, right, matches!(operator, Binary::And))?
            }
            ExpressionKind::Binary {
                operator,
                left,
                right,
            } => {
                let Some(lhs) = self.expression(left)?.value()? else {
                    return Ok(ExpressionResult::Terminated);
                };
                let Some(rhs) = self.expression(right)?.value()? else {
                    return Ok(ExpressionResult::Terminated);
                };
                Some(self.binary(source.id, Role::Value, operator, source.ty, lhs, rhs)?)
            }
            ExpressionKind::Call(call) => self.call(source, call)?,
        };
        self.correspondence.values.insert(source.id, value);
        Ok(ExpressionResult::Continued {
            block: self.current.ok_or("blueprint/internal/lost-continuation")?,
            value,
            ty: source.ty,
        })
    }
    /// All source call forms reach this writer after Resolve chose their target.
    fn call(
        &mut self,
        source: &Expression,
        call: &ResolvedCall,
    ) -> Result<Option<ValueId>, String> {
        if call.receiver.is_some() || call.may_fault {
            return Err("blueprint/internal/unresolved-receiver".into());
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
        for source in &call.arguments {
            let Some(value) = self.expression(source)?.value()? else {
                return Ok(None);
            };
            arguments.push(value);
        }
        let result = if source.ty == Type::Unit {
            None
        } else {
            Some(self.value(source.ty))
        };
        let mut operation = MirCall::new(result, Callee::Global(target), arguments);
        operation.effects = if io { EffectMask::IO } else { EffectMask::PURE };
        self.emit(source.id, Role::Value, MirInstruction::Call(operation))?;
        Ok(result)
    }
    fn binary(
        &mut self,
        node: NodeId,
        role: Role,
        operator: &Binary,
        ty: Type,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<ValueId, String> {
        let dst = self.value(ty);
        let instruction = match operator {
            Binary::Add | Binary::Subtract | Binary::Multiply | Binary::Divide => {
                let op = match operator {
                    Binary::Add => BinaryOp::Add,
                    Binary::Subtract => BinaryOp::Sub,
                    Binary::Multiply => BinaryOp::Mul,
                    _ => BinaryOp::Div,
                };
                MirInstruction::BinOp { dst, op, lhs, rhs }
            }
            Binary::Equal
            | Binary::NotEqual
            | Binary::Less
            | Binary::LessEqual
            | Binary::Greater => {
                let op = match operator {
                    Binary::Equal => CompareOp::Eq,
                    Binary::NotEqual => CompareOp::Ne,
                    Binary::Less => CompareOp::Lt,
                    Binary::LessEqual => CompareOp::Le,
                    _ => CompareOp::Gt,
                };
                MirInstruction::Compare { dst, op, lhs, rhs }
            }
            _ => return Err("blueprint/internal/unresolved-binary".into()),
        };
        self.emit(node, role, instruction)?;
        Ok(dst)
    }
    fn block(&mut self, body: &[Statement]) -> Result<(), String> {
        for row in body {
            if self.current.is_none() {
                break;
            }
            self.statement(row)?;
        }
        Ok(())
    }
    fn statement(&mut self, source: &Statement) -> Result<(), String> {
        self.mark(source.id)?;
        match &source.kind {
            StatementKind::Local { binding, value } | StatementKind::Assign { binding, value } => {
                if let Some(value) = self.expression(value)?.value()? {
                    self.bind(*binding, value)?;
                }
            }
            StatementKind::Compound {
                binding,
                operator,
                value,
            } => {
                let lhs = self.read(*binding)?; // Evaluated Place old read precedes RHS.
                if let Some(rhs) = self.expression(value)?.value()? {
                    let result = self.binary(
                        source.id,
                        Role::Compound,
                        operator,
                        self.source.bindings[binding.0].ty,
                        lhs,
                        rhs,
                    )?;
                    self.bind(*binding, result)?;
                }
            }
            StatementKind::Expression(value) => {
                let _ = self.expression(value)?;
            }
            StatementKind::Return(value) => {
                if let Some(value) = self.expression(value)?.value()? {
                    self.terminate(
                        Some((source.id, Role::Control)),
                        MirInstruction::Return { value: Some(value) },
                    )?;
                }
            }
            StatementKind::If {
                condition,
                yes,
                no,
                carriers,
            } => self.conditional(source.id, condition, yes, no, carriers)?,
            StatementKind::Loop(contract) => self.loop_statement(source.id, contract)?,
            StatementKind::Break(target) | StatementKind::Continue(target) => {
                let is_break = matches!(source.kind, StatementKind::Break(_));
                let position = self
                    .loops
                    .iter()
                    .rposition(|f| f.id == *target)
                    .ok_or("blueprint/internal/control-target")?;
                let frame = &self.loops[position];
                let destination = if is_break {
                    frame.image.exit
                } else {
                    frame.image.step.unwrap_or(frame.image.header)
                };
                let predecessor = self.jump(Some((source.id, Role::Control)), destination)?;
                if is_break {
                    self.loops[position].breaks.push(predecessor);
                } else {
                    self.loops[position].continues.push(predecessor);
                }
            }
            StatementKind::Scope(body) => self.block(body)?,
        }
        Ok(())
    }
}
