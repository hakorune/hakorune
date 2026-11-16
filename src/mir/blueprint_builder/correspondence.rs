//! Independent source correspondence: contracts are Resolve's, not Lower's.
use super::{
    declaration_table::*,
    lower::{mir_type, Correspondence, Cursor},
};
use crate::mir::{
    BasicBlockId, BinaryOp, ConstValue, MirFunction, MirInstruction, MirModule, ValueId,
};
use std::collections::BTreeSet;

pub(super) fn verify(
    source: &ResolvedModule,
    module: &MirModule,
    records: &[Correspondence],
) -> Result<(), String> {
    if records.len() != source.functions.len() || module.functions.len() != source.functions.len() {
        return Err("blueprint/internal/correspondence/function-census".into());
    }
    for (body, records) in source.functions.iter().zip(records) {
        let declaration = &source.declarations.entries[body.declaration.0];
        let function = module
            .functions
            .get(&declaration.key.mir_symbol_projection())
            .ok_or("blueprint/internal/correspondence/function-missing")?;
        if function.signature.params
            != declaration
                .parameters
                .iter()
                .map(|(_, t)| mir_type(*t))
                .collect::<Vec<_>>()
            || function.signature.return_type != mir_type(declaration.result)
            || module.canonical_callable_definition_symbol(&declaration.key)
                != Some(&function.signature.name)
        {
            return Err("blueprint/internal/correspondence/signature".into());
        }
        let mut checker = Checker {
            function,
            records,
            cursor: Cursor {
                block: function.entry_block,
                index: 0,
            },
            seen: BTreeSet::new(),
        };
        for statement in &body.body {
            let StatementKind::Return(expression) = &statement.kind;
            let value = checker.expression(expression)?;
            let actual = checker.operation(statement.id)?;
            if !matches!(actual, MirInstruction::Return { value: Some(actual) } if *actual == value)
            {
                return Err("blueprint/internal/correspondence/return".into());
            }
        }
        let count: usize = function
            .blocks
            .values()
            .map(|b| b.instructions.len() + usize::from(b.terminator.is_some()))
            .sum();
        if checker.seen.len() != count
            || checker.seen.len() != records.operations.len()
            || function.blocks.len() != 1
            || records.values.len() + body.body.len() != checker.seen.len()
        {
            return Err("blueprint/internal/correspondence/extra-operation".into());
        }
    }
    Ok(())
}

struct Checker<'a> {
    function: &'a MirFunction,
    records: &'a Correspondence,
    cursor: Cursor,
    seen: BTreeSet<Cursor>,
}
impl Checker<'_> {
    fn operation(&mut self, id: NodeId) -> Result<&MirInstruction, String> {
        if self.records.operations.get(&id) != Some(&self.cursor) || !self.seen.insert(self.cursor)
        {
            return Err("blueprint/internal/correspondence/operation-position".into());
        }
        let body = self
            .function
            .blocks
            .get(&self.cursor.block)
            .ok_or("blueprint/internal/correspondence/block")?;
        let operation = if self.cursor.index == body.instructions.len() {
            body.terminator.as_ref()
        } else {
            body.instructions.get(self.cursor.index)
        }
        .ok_or("blueprint/internal/correspondence/operation-missing")?;
        self.cursor.index += 1;
        Ok(operation)
    }
    fn expression(&mut self, source: &Expression) -> Result<ValueId, String> {
        let value = *self
            .records
            .values
            .get(&source.id)
            .ok_or("blueprint/internal/correspondence/result-missing")?;
        if self.function.metadata.value_types.get(&value) != Some(&mir_type(source.ty)) {
            return Err("blueprint/internal/correspondence/value-type".into());
        }
        let operands = match &source.kind {
            ExpressionKind::Binary { left, right, .. } => {
                Some((self.expression(left)?, self.expression(right)?))
            }
            _ => None,
        };
        let actual = self.operation(source.id)?;
        let correct = match (&source.kind, actual) {
            (
                ExpressionKind::Integer(expected),
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(actual),
                },
            ) => *dst == value && actual == expected,
            (
                ExpressionKind::Bool(expected),
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Bool(actual),
                },
            ) => *dst == value && actual == expected,
            (
                ExpressionKind::Binary {
                    op: Binary::Add, ..
                },
                MirInstruction::BinOp {
                    dst,
                    op: BinaryOp::Add,
                    lhs,
                    rhs,
                },
            ) => *dst == value && operands == Some((*lhs, *rhs)),
            _ => false,
        };
        if !correct {
            return Err("blueprint/internal/correspondence/value-operation".into());
        }
        Ok(value)
    }
}
