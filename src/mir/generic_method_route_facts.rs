/*!
 * Generic-method route facts.
 *
 * This module owns small, reusable facts consumed by generic-method route
 * planners. It does not choose backend helpers or promote CoreMethodOps.
 */

use super::{resolve_value_origin, ConstValue, MirFunction, MirInstruction, ValueDefMap, ValueId};

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
}

impl GenericMethodValueDemand {
    pub fn as_metadata_name(self) -> &'static str {
        match self {
            Self::ReadRef => "read_ref",
        }
    }
}

impl std::fmt::Display for GenericMethodValueDemand {
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
        _ => None,
    }
}

pub(crate) fn classify_key_route(
    function: &MirFunction,
    def_map: &ValueDefMap,
    key: ValueId,
) -> GenericMethodKeyRoute {
    let origin = resolve_value_origin(function, def_map, key);
    let Some((block_id, instruction_index)) = def_map.get(&origin).copied() else {
        return GenericMethodKeyRoute::UnknownAny;
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return GenericMethodKeyRoute::UnknownAny;
    };
    match block.instructions.get(instruction_index) {
        Some(MirInstruction::Const {
            value: ConstValue::Integer(_),
            ..
        }) => GenericMethodKeyRoute::I64Const,
        _ => GenericMethodKeyRoute::UnknownAny,
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

        let def_map = crate::mir::build_value_def_map(&function);
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

        let def_map = crate::mir::build_value_def_map(&function);
        assert_eq!(
            classify_key_route(&function, &def_map, ValueId::new(1)),
            GenericMethodKeyRoute::UnknownAny
        );
    }
}
