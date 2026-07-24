//! Test-only Legacy-vs-Raw parity snapshot.
//!
//! This module is intentionally a passive observer.  It owns the normalized
//! comparison vocabulary for PARITY0; production lowering, printers, JSON,
//! and backend serializers are not parity authorities.

use crate::mir::{
    BasicBlockId, ConstValue, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ModuleSnapshotV1 {
    pub(super) name: String,
    pub(super) source_file: Option<String>,
    pub(super) functions: Vec<FunctionSnapshotV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FunctionSnapshotV1 {
    pub(super) name: String,
    pub(super) params: Vec<TypeSnapshotV1>,
    pub(super) return_type: TypeSnapshotV1,
    pub(super) effects: u16,
    pub(super) locals: Vec<TypeSnapshotV1>,
    pub(super) blocks: Vec<BlockSnapshotV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct BlockSnapshotV1 {
    pub(super) index: usize,
    pub(super) successors: Vec<usize>,
    pub(super) instructions: Vec<InstructionSnapshotV1>,
    pub(super) terminator: Option<InstructionSnapshotV1>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum TypeSnapshotV1 {
    Integer,
    Float,
    Bool,
    String,
    Box(String),
    Array(Box<TypeSnapshotV1>),
    Future(Box<TypeSnapshotV1>),
    WeakRef,
    Void,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ConstSnapshotV1 {
    Integer(i64),
    FloatBits(u64),
    Bool(bool),
    String(String),
    Null,
    Void,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum InstructionSnapshotV1 {
    Const { dst: u32, value: ConstSnapshotV1 },
    BinOp { dst: u32, op: &'static str, lhs: u32, rhs: u32 },
    UnaryOp { dst: u32, op: &'static str, operand: u32 },
    Compare { dst: u32, op: &'static str, lhs: u32, rhs: u32 },
    Copy { dst: u32, src: u32 },
    Phi { dst: u32, inputs: Vec<(usize, u32)> },
    Jump { target: usize, args: Vec<u32> },
    Branch { condition: u32, then_bb: usize, else_bb: usize },
    Return { value: Option<u32> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SnapshotErrorV1 {
    UnknownValue { function: String, value: u32 },
    UnsupportedInstruction { function: String, block: u32 },
    UnknownBlock { function: String, block: u32 },
}

pub(super) fn snapshot_module(module: &MirModule) -> Result<ModuleSnapshotV1, SnapshotErrorV1> {
    let mut functions = Vec::with_capacity(module.functions.len());
    for function in module.functions.values() {
        functions.push(snapshot_function(function)?);
    }
    Ok(ModuleSnapshotV1 {
        name: module.name.clone(),
        source_file: module.metadata.source_file.clone(),
        functions,
    })
}

fn snapshot_function(function: &MirFunction) -> Result<FunctionSnapshotV1, SnapshotErrorV1> {
    let order = block_order(function);
    let block_indexes: BTreeMap<BasicBlockId, usize> = order
        .iter()
        .copied()
        .enumerate()
        .map(|(index, block)| (block, index))
        .collect();
    let mut values = HashMap::new();
    for (index, value) in function.params.iter().copied().enumerate() {
        values.insert(value, index as u32);
    }
    let mut next_value = function.params.len() as u32;
    let mut blocks = Vec::with_capacity(order.len());
    for (index, block_id) in order.iter().copied().enumerate() {
        let block = function.blocks.get(&block_id).ok_or_else(|| {
            SnapshotErrorV1::UnknownBlock { function: function.signature.name.clone(), block: block_id.0 }
        })?;
        let successors = block
            .successors
            .iter()
            .map(|target| {
                block_indexes.get(target).copied().ok_or_else(|| {
                    SnapshotErrorV1::UnknownBlock { function: function.signature.name.clone(), block: target.0 }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let instructions = block
            .instructions
            .iter()
            .map(|instruction| {
                snapshot_instruction(
                    instruction,
                    &function.signature.name,
                    block_id,
                    &block_indexes,
                    &mut values,
                    &mut next_value,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let terminator = block
            .terminator
            .as_ref()
            .map(|instruction| {
                snapshot_instruction(
                    instruction,
                    &function.signature.name,
                    block_id,
                    &block_indexes,
                    &mut values,
                    &mut next_value,
                )
            })
            .transpose()?;
        blocks.push(BlockSnapshotV1 { index, successors, instructions, terminator });
    }
    Ok(FunctionSnapshotV1 {
        name: function.signature.name.clone(),
        params: function.signature.params.iter().map(type_snapshot).collect(),
        return_type: type_snapshot(&function.signature.return_type),
        effects: function.signature.effects.bits(),
        locals: function.locals.iter().map(type_snapshot).collect(),
        blocks,
    })
}

fn block_order(function: &MirFunction) -> Vec<BasicBlockId> {
    let mut order = Vec::new();
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::from([function.entry_block]);
    while let Some(block) = queue.pop_front() {
        if !seen.insert(block) {
            continue;
        }
        order.push(block);
        if let Some(block_data) = function.blocks.get(&block) {
            queue.extend(block_data.successors.iter().copied());
        }
    }
    for block in function.blocks.keys().copied().collect::<BTreeSet<_>>() {
        if seen.insert(block) {
            order.push(block);
        }
    }
    order
}

fn snapshot_instruction(
    instruction: &MirInstruction,
    function: &str,
    block: BasicBlockId,
    block_indexes: &BTreeMap<BasicBlockId, usize>,
    values: &mut HashMap<ValueId, u32>,
    next_value: &mut u32,
) -> Result<InstructionSnapshotV1, SnapshotErrorV1> {
    let input = |value: ValueId, values: &HashMap<ValueId, u32>| {
        values.get(&value).copied().ok_or_else(|| SnapshotErrorV1::UnknownValue {
            function: function.to_owned(),
            value: value.0,
        })
    };
    let output = |value: ValueId, values: &mut HashMap<ValueId, u32>, next: &mut u32| {
        let canonical = *next;
        *next += 1;
        values.insert(value, canonical);
        canonical
    };
    let edge = |target: BasicBlockId| {
        block_indexes.get(&target).copied().ok_or_else(|| SnapshotErrorV1::UnknownBlock {
            function: function.to_owned(),
            block: target.0,
        })
    };
    match instruction {
        MirInstruction::Const { dst, value } => Ok(InstructionSnapshotV1::Const {
            dst: output(*dst, values, next_value),
            value: const_snapshot(value),
        }),
        MirInstruction::BinOp { dst, op, lhs, rhs } => Ok(InstructionSnapshotV1::BinOp {
            dst: output(*dst, values, next_value),
            op: binary_name(*op),
            lhs: input(*lhs, values)?,
            rhs: input(*rhs, values)?,
        }),
        MirInstruction::UnaryOp { dst, op, operand } => Ok(InstructionSnapshotV1::UnaryOp {
            dst: output(*dst, values, next_value),
            op: unary_name(*op),
            operand: input(*operand, values)?,
        }),
        MirInstruction::Compare { dst, op, lhs, rhs } => Ok(InstructionSnapshotV1::Compare {
            dst: output(*dst, values, next_value),
            op: compare_name(*op),
            lhs: input(*lhs, values)?,
            rhs: input(*rhs, values)?,
        }),
        MirInstruction::Copy { dst, src } => Ok(InstructionSnapshotV1::Copy {
            dst: output(*dst, values, next_value),
            src: input(*src, values)?,
        }),
        MirInstruction::Phi { dst, inputs, .. } => Ok(InstructionSnapshotV1::Phi {
            dst: output(*dst, values, next_value),
            inputs: inputs
                .iter()
                .map(|(from, value)| Ok((edge(*from)?, input(*value, values)?)))
                .collect::<Result<Vec<_>, SnapshotErrorV1>>()?,
        }),
        MirInstruction::Jump { target, edge_args } => Ok(InstructionSnapshotV1::Jump {
            target: edge(*target)?,
            args: edge_args
                .as_ref()
                .map(|args| args.values.iter().map(|value| input(*value, values)).collect())
                .transpose()?
                .unwrap_or_default(),
        }),
        MirInstruction::Branch { condition, then_bb, else_bb, .. } => Ok(InstructionSnapshotV1::Branch {
            condition: input(*condition, values)?,
            then_bb: edge(*then_bb)?,
            else_bb: edge(*else_bb)?,
        }),
        MirInstruction::Return { value } => Ok(InstructionSnapshotV1::Return {
            value: value.map(|value| input(value, values)).transpose()?,
        }),
        _ => Err(SnapshotErrorV1::UnsupportedInstruction {
            function: function.to_owned(),
            block: block.0,
        }),
    }
}

fn type_snapshot(ty: &MirType) -> TypeSnapshotV1 {
    match ty {
        MirType::Integer => TypeSnapshotV1::Integer,
        MirType::Float => TypeSnapshotV1::Float,
        MirType::Bool => TypeSnapshotV1::Bool,
        MirType::String => TypeSnapshotV1::String,
        MirType::Box(name) => TypeSnapshotV1::Box(name.clone()),
        MirType::Array(inner) => TypeSnapshotV1::Array(Box::new(type_snapshot(inner))),
        MirType::Future(inner) => TypeSnapshotV1::Future(Box::new(type_snapshot(inner))),
        MirType::WeakRef => TypeSnapshotV1::WeakRef,
        MirType::Void => TypeSnapshotV1::Void,
        MirType::Unknown => TypeSnapshotV1::Unknown,
    }
}

fn const_snapshot(value: &ConstValue) -> ConstSnapshotV1 {
    match value {
        ConstValue::Integer(value) => ConstSnapshotV1::Integer(*value),
        ConstValue::Float(value) => ConstSnapshotV1::FloatBits(value.to_bits()),
        ConstValue::Bool(value) => ConstSnapshotV1::Bool(*value),
        ConstValue::String(value) => ConstSnapshotV1::String(value.clone()),
        ConstValue::Null => ConstSnapshotV1::Null,
        ConstValue::Void => ConstSnapshotV1::Void,
    }
}

fn binary_name(op: crate::mir::BinaryOp) -> &'static str {
    match op {
        crate::mir::BinaryOp::Add => "add",
        crate::mir::BinaryOp::Sub => "sub",
        crate::mir::BinaryOp::Mul => "mul",
        crate::mir::BinaryOp::Div => "div",
        crate::mir::BinaryOp::Mod => "mod",
        crate::mir::BinaryOp::BitAnd => "bit_and",
        crate::mir::BinaryOp::BitOr => "bit_or",
        crate::mir::BinaryOp::BitXor => "bit_xor",
        crate::mir::BinaryOp::Shl => "shl",
        crate::mir::BinaryOp::Shr => "shr",
        crate::mir::BinaryOp::And => "and",
        crate::mir::BinaryOp::Or => "or",
    }
}

fn unary_name(op: crate::mir::UnaryOp) -> &'static str {
    match op {
        crate::mir::UnaryOp::Neg => "neg",
        crate::mir::UnaryOp::Not => "not",
        crate::mir::UnaryOp::BitNot => "bit_not",
    }
}

fn compare_name(op: crate::mir::CompareOp) -> &'static str {
    match op {
        crate::mir::CompareOp::Eq => "eq",
        crate::mir::CompareOp::Ne => "ne",
        crate::mir::CompareOp::Lt => "lt",
        crate::mir::CompareOp::Le => "le",
        crate::mir::CompareOp::Gt => "gt",
        crate::mir::CompareOp::Ge => "ge",
    }
}
