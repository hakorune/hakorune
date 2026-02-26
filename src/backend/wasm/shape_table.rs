use crate::mir::{ConstValue, MirInstruction, MirModule};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PilotShape {
    MainReturnI32Const,
}

impl PilotShape {
    #[cfg(test)]
    pub(crate) fn id(self) -> &'static str {
        match self {
            PilotShape::MainReturnI32Const => "wsm.p4.main_return_i32_const.v0",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PilotMatch {
    pub(crate) shape: PilotShape,
    pub(crate) value: i32,
}

type ShapeMatcher = fn(&MirModule) -> Option<PilotMatch>;

const PILOT_SHAPE_TABLE: &[ShapeMatcher] = &[match_main_return_i32_const];

pub(crate) fn match_pilot_shape(mir_module: &MirModule) -> Option<PilotMatch> {
    for matcher in PILOT_SHAPE_TABLE {
        if let Some(found) = matcher(mir_module) {
            return Some(found);
        }
    }
    None
}

fn match_main_return_i32_const(mir_module: &MirModule) -> Option<PilotMatch> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() != 1 {
        return None;
    }

    let entry = main.blocks.get(&main.entry_block)?;
    if entry.instructions.len() != 1 {
        return None;
    }

    let MirInstruction::Const { dst, value } = &entry.instructions[0] else {
        return None;
    };
    let MirInstruction::Return {
        value: Some(ret_val),
    } = entry.terminator.as_ref()?
    else {
        return None;
    };
    if ret_val != dst {
        return None;
    }

    let ConstValue::Integer(n) = value else {
        return None;
    };
    let value = i32::try_from(*n).ok()?;
    Some(PilotMatch {
        shape: PilotShape::MainReturnI32Const,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType, ValueId,
    };

    fn make_module_with_single_const_return(value: i64) -> MirModule {
        let mut module = MirModule::new("test".to_string());
        let entry = BasicBlockId(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            entry,
        );
        let dst = ValueId(1);
        let block = function
            .get_block_mut(entry)
            .expect("entry block must exist");
        block.add_instruction(MirInstruction::Const {
            dst,
            value: ConstValue::Integer(value),
        });
        block.add_instruction(MirInstruction::Return { value: Some(dst) });
        module.add_function(function);
        module
    }

    #[test]
    fn wasm_shape_table_matches_min_const_return_contract() {
        let module = make_module_with_single_const_return(-1);
        let found = match_pilot_shape(&module).expect("shape table should match");
        assert_eq!(found.shape.id(), "wsm.p4.main_return_i32_const.v0");
        assert_eq!(found.value, -1);
    }

    #[test]
    fn wasm_shape_table_rejects_non_const_return_contract() {
        let mut module = make_module_with_single_const_return(7);
        let entry = module
            .get_function_mut("main")
            .expect("main should exist")
            .get_block_mut(BasicBlockId(0))
            .expect("entry block should exist");
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(9),
        });

        assert!(
            match_pilot_shape(&module).is_none(),
            "shape table must fail-fast outside strict pilot shape"
        );
    }
}
