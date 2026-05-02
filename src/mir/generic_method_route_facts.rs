/*!
 * Generic-method route facts.
 *
 * This module owns small, reusable facts consumed by generic-method route
 * planners. It does not choose backend helpers or promote CoreMethodOps.
 */

use super::value_origin::{resolve_value_origin, ValueDefMap};
use super::{ConstValue, MirFunction, MirInstruction, MirType, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodKeyRoute {
    I64Const,
    UnknownAny,
}

impl GenericMethodKeyRoute {
    pub fn as_metadata_name(self) -> &'static str {
        match self {
            Self::I64Const => "i64_const",
            Self::UnknownAny => "unknown_any",
        }
    }
}

impl std::fmt::Display for GenericMethodKeyRoute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_metadata_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodValueDemand {
    ReadRef,
    RuntimeI64OrHandle,
    ScalarI64,
    WriteAny,
}

impl GenericMethodValueDemand {
    pub fn as_metadata_name(self) -> &'static str {
        match self {
            Self::ReadRef => "read_ref",
            Self::RuntimeI64OrHandle => "runtime_i64_or_handle",
            Self::ScalarI64 => "scalar_i64",
            Self::WriteAny => "write_any",
        }
    }
}

impl std::fmt::Display for GenericMethodValueDemand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_metadata_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodReturnShape {
    MixedRuntimeI64OrHandle,
    ScalarI64OrMissingZero,
    ScalarI64,
}

impl GenericMethodReturnShape {
    pub fn as_metadata_name(self) -> &'static str {
        match self {
            Self::MixedRuntimeI64OrHandle => "mixed_runtime_i64_or_handle",
            Self::ScalarI64OrMissingZero => "scalar_i64_or_missing_zero",
            Self::ScalarI64 => "scalar_i64",
        }
    }
}

impl std::fmt::Display for GenericMethodReturnShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_metadata_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericMethodPublicationPolicy {
    NoPublication,
    RuntimeDataFacade,
}

impl GenericMethodPublicationPolicy {
    pub fn as_metadata_name(self) -> &'static str {
        match self {
            Self::NoPublication => "no_publication",
            Self::RuntimeDataFacade => "runtime_data_facade",
        }
    }
}

impl std::fmt::Display for GenericMethodPublicationPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_metadata_name())
    }
}

pub(crate) fn receiver_origin_box_name(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, receiver);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index)? {
        MirInstruction::NewBox { box_type, .. } => Some(box_type.clone()),
        MirInstruction::Phi { type_hint, .. } => type_hint
            .as_ref()
            .and_then(box_name_from_mir_type)
            .map(str::to_string)
            .or_else(|| {
                function
                    .metadata
                    .value_types
                    .get(&origin)
                    .and_then(box_name_from_mir_type)
                    .map(str::to_string)
            }),
        _ => function
            .metadata
            .value_types
            .get(&origin)
            .and_then(box_name_from_mir_type)
            .map(str::to_string),
    }
}

fn box_name_from_mir_type(ty: &MirType) -> Option<&str> {
    match ty {
        MirType::Box(name) => Some(name.as_str()),
        _ => None,
    }
}

pub(crate) fn classify_key_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    key: ValueId,
) -> GenericMethodKeyRoute {
    match const_i64_value(function, def_map, key) {
        Some(_) => GenericMethodKeyRoute::I64Const,
        None => GenericMethodKeyRoute::UnknownAny,
    }
}

pub(crate) fn const_i64_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<i64> {
    let origin = resolve_value_origin(function, def_map, value);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index) {
        Some(MirInstruction::Const {
            value: ConstValue::Integer(value),
            ..
        }) => Some(*value),
        _ => None,
    }
}

pub(crate) fn const_string_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<String> {
    let origin = resolve_value_origin(function, def_map, value);
    let (block_id, instruction_index) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(instruction_index) {
        Some(MirInstruction::Const {
            value: ConstValue::String(value),
            ..
        }) => Some(value.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirType};

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "facts".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn classifies_copy_chain_integer_const_key() {
        let mut function = make_function();
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Integer(-1),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        });
        function.add_block(block);

        let def_map = crate::mir::value_origin::build_value_def_map(&function);
        assert_eq!(
            classify_key_route(&function, &def_map, ValueId::new(2)),
            GenericMethodKeyRoute::I64Const
        );
    }

    #[test]
    fn classifies_unknown_key_when_root_is_not_integer_const() {
        let mut function = make_function();
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "StringBox".to_string(),
            args: vec![],
        });
        function.add_block(block);

        let def_map = crate::mir::value_origin::build_value_def_map(&function);
        assert_eq!(
            classify_key_route(&function, &def_map, ValueId::new(1)),
            GenericMethodKeyRoute::UnknownAny
        );
    }
}
