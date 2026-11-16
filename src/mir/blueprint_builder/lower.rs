//! Mechanical emission from the resolved contract. No AST or source admission.
use super::declaration_table::*;
use crate::mir::{
    BasicBlockId, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirType, ValueId,
};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Cursor {
    pub block: BasicBlockId,
    pub index: usize,
}

/// Coordinates only: expected operations always come from ResolvedFunction.
#[derive(Debug)]
pub(super) struct Correspondence {
    pub operations: BTreeMap<NodeId, Cursor>,
    pub values: BTreeMap<NodeId, ValueId>,
}

pub(super) struct Draft {
    pub declaration: DeclarationId,
    pub function: MirFunction,
    pub correspondence: Correspondence,
}

pub(super) fn mir_type(ty: Type) -> MirType {
    match ty {
        Type::I64 => MirType::Integer,
        Type::Bool => MirType::Bool,
        Type::Unit => MirType::Void,
    }
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
                    .map(|(_, ty)| mir_type(*ty))
                    .collect(),
                return_type: mir_type(declaration.result),
                effects: EffectMask::PURE,
            };
            let mut function = MirFunction::new(signature, BasicBlockId::new(0));
            function.metadata.is_entry_point = declaration.key.name() == "main"
                && (declaration.owner.as_deref() == Some("Main") || declaration.owner.is_none());
            for (&value, (_, ty)) in function.params.iter().zip(&declaration.parameters) {
                function.metadata.value_types.insert(value, mir_type(*ty));
            }
            let mut workspace = Workspace {
                function,
                block: BasicBlockId::new(0),
                correspondence: Correspondence {
                    operations: BTreeMap::new(),
                    values: BTreeMap::new(),
                },
            };
            for statement in &body.body {
                workspace.statement(statement)?;
            }
            Ok(Draft {
                declaration: body.declaration,
                function: workspace.function,
                correspondence: workspace.correspondence,
            })
        })
        .collect()
}

struct Workspace {
    function: MirFunction,
    block: BasicBlockId,
    correspondence: Correspondence,
}
impl Workspace {
    fn value(&mut self, ty: Type) -> ValueId {
        let value = self.function.next_value_id();
        self.function
            .metadata
            .value_types
            .insert(value, mir_type(ty));
        value
    }
    fn write(&mut self, id: NodeId, instruction: MirInstruction) -> Result<(), String> {
        let body = self
            .function
            .blocks
            .get_mut(&self.block)
            .ok_or("blueprint/internal/missing-block")?;
        if body.terminator.is_some() {
            return Err("blueprint/internal/write-after-exit".into());
        }
        let cursor = Cursor {
            block: self.block,
            index: body.instructions.len(),
        };
        if self.correspondence.operations.insert(id, cursor).is_some() {
            return Err("blueprint/internal/duplicate-operation".into());
        }
        body.add_instruction(instruction);
        Ok(())
    }
    fn expression(&mut self, expression: &Expression) -> Result<ValueId, String> {
        let value = self.value(expression.ty);
        let instruction = match &expression.kind {
            ExpressionKind::Integer(integer) => MirInstruction::Const {
                dst: value,
                value: ConstValue::Integer(*integer),
            },
            ExpressionKind::Bool(boolean) => MirInstruction::Const {
                dst: value,
                value: ConstValue::Bool(*boolean),
            },
            ExpressionKind::Binary {
                op: Binary::Add,
                left,
                right,
            } => {
                let lhs = self.expression(left)?;
                let rhs = self.expression(right)?;
                MirInstruction::BinOp {
                    dst: value,
                    op: BinaryOp::Add,
                    lhs,
                    rhs,
                }
            }
        };
        self.write(expression.id, instruction)?;
        self.correspondence.values.insert(expression.id, value);
        Ok(value)
    }
    fn statement(&mut self, statement: &Statement) -> Result<(), String> {
        match &statement.kind {
            StatementKind::Return(expression) => {
                let value = self.expression(expression)?;
                let body = self
                    .function
                    .blocks
                    .get_mut(&self.block)
                    .ok_or("blueprint/internal/missing-block")?;
                let cursor = Cursor {
                    block: self.block,
                    index: body.instructions.len(),
                };
                if self
                    .correspondence
                    .operations
                    .insert(statement.id, cursor)
                    .is_some()
                {
                    return Err("blueprint/internal/duplicate-operation".into());
                }
                body.set_terminator(MirInstruction::Return { value: Some(value) });
                Ok(())
            }
        }
    }
}
