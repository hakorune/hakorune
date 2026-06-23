/*!
 * MIR-owned constructor call route plans.
 *
 * This module owns the narrow runtime collection constructor facts consumed by
 * LLVM/AOT declaration prepass. Backends must consume these rows instead of
 * reclassifying constructor callee spelling.
 */

use crate::mir::definitions::Callee;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstructorCallRouteKind {
    MapBoxBirth,
    ArrayBoxBirth,
}

impl ConstructorCallRouteKind {
    pub fn from_box_type(box_type: &str) -> Option<Self> {
        match box_type {
            "MapBox" => Some(Self::MapBoxBirth),
            "ArrayBox" => Some(Self::ArrayBoxBirth),
            _ => None,
        }
    }

    pub fn route_id(self) -> &'static str {
        match self {
            Self::MapBoxBirth => "constructor.map_birth",
            Self::ArrayBoxBirth => "constructor.array_birth",
        }
    }

    pub fn core_op(self) -> &'static str {
        match self {
            Self::MapBoxBirth => "MapBirth",
            Self::ArrayBoxBirth => "ArrayBirth",
        }
    }

    pub fn route_kind(self) -> &'static str {
        self.route_id()
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::MapBoxBirth => "nyash.map.birth_h",
            Self::ArrayBoxBirth => "nyash.array.birth_h",
        }
    }

    pub fn result_origin(self) -> &'static str {
        match self {
            Self::MapBoxBirth => "map_birth",
            Self::ArrayBoxBirth => "array_birth",
        }
    }

    pub fn need_kind(self) -> &'static str {
        match self {
            Self::MapBoxBirth => "map_birth",
            Self::ArrayBoxBirth => "array_birth",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstructorCallRoute {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub box_type: String,
    pub result_value: ValueId,
    pub kind: ConstructorCallRouteKind,
}

impl ConstructorCallRoute {
    fn new(
        block: BasicBlockId,
        instruction_index: usize,
        box_type: &str,
        result_value: ValueId,
        kind: ConstructorCallRouteKind,
    ) -> Self {
        Self {
            block,
            instruction_index,
            box_type: box_type.to_string(),
            result_value,
            kind,
        }
    }

    pub fn route_id(&self) -> &'static str {
        self.kind.route_id()
    }

    pub fn core_op(&self) -> &'static str {
        self.kind.core_op()
    }

    pub fn route_kind(&self) -> &'static str {
        self.kind.route_kind()
    }

    pub fn symbol(&self) -> &'static str {
        self.kind.symbol()
    }

    pub fn result_origin(&self) -> &'static str {
        self.kind.result_origin()
    }

    pub fn need_kind(&self) -> &'static str {
        self.kind.need_kind()
    }
}

pub fn refresh_function_constructor_call_routes(function: &mut MirFunction) {
    let mut routes = Vec::new();

    for (block, bb) in function.blocks.iter() {
        for (instruction_index, instruction) in bb.instructions.iter().enumerate() {
            let MirInstruction::Call {
                dst: Some(result_value),
                callee: Some(Callee::Constructor { box_type }),
                ..
            } = instruction
            else {
                continue;
            };
            let Some(kind) = ConstructorCallRouteKind::from_box_type(box_type) else {
                continue;
            };
            routes.push(ConstructorCallRoute::new(
                *block,
                instruction_index,
                box_type,
                *result_value,
                kind,
            ));
        }
    }

    routes.sort_by_key(|route| (route.block.as_u32(), route.instruction_index));
    function.metadata.constructor_call_routes = routes;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::basic_block::BasicBlock;
    use crate::mir::definitions::Callee;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirInstruction, MirType};

    #[test]
    fn refresh_function_constructor_call_routes_records_collection_births() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.add_instruction(MirInstruction::Call {
            dst: Some(ValueId::new(1)),
            func: ValueId::INVALID,
            callee: Some(Callee::Constructor {
                box_type: "MapBox".to_string(),
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        function.blocks.insert(BasicBlockId::new(0), block);

        refresh_function_constructor_call_routes(&mut function);

        assert_eq!(function.metadata.constructor_call_routes.len(), 1);
        let route = &function.metadata.constructor_call_routes[0];
        assert_eq!(route.route_id(), "constructor.map_birth");
        assert_eq!(route.result_origin(), "map_birth");
        assert_eq!(route.result_value, ValueId::new(1));
    }
}
