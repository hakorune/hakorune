use crate::mir::{ConstValue, MirInstruction, MirModule};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NativeShape {
    MainReturnI32Const,
    MainReturnI32ConstViaCopy,
}

impl NativeShape {
    pub(crate) fn id(self) -> &'static str {
        match self {
            NativeShape::MainReturnI32Const => "wsm.p4.main_return_i32_const.v0",
            NativeShape::MainReturnI32ConstViaCopy => "wsm.p5.main_return_i32_const_via_copy.v0",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativeMatch {
    pub(crate) shape: NativeShape,
    pub(crate) value: i32,
}

type ShapeMatcher = fn(&MirModule) -> Option<NativeMatch>;

const NATIVE_SHAPE_TABLE: &[ShapeMatcher] = &[
    match_main_return_i32_const,
    match_main_return_i32_const_via_copy,
];

pub(crate) fn match_native_shape(mir_module: &MirModule) -> Option<NativeMatch> {
    for matcher in NATIVE_SHAPE_TABLE {
        if let Some(found) = matcher(mir_module) {
            return Some(found);
        }
    }
    None
}

fn match_main_return_i32_const(mir_module: &MirModule) -> Option<NativeMatch> {
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
    Some(NativeMatch {
        shape: NativeShape::MainReturnI32Const,
        value,
    })
}

fn match_main_return_i32_const_via_copy(mir_module: &MirModule) -> Option<NativeMatch> {
    let main = mir_module.get_function("main")?;
    if main.blocks.len() != 1 {
        return None;
    }

    let entry = main.blocks.get(&main.entry_block)?;
    if entry.instructions.len() != 2 {
        return None;
    }

    let MirInstruction::Const { dst, value } = &entry.instructions[0] else {
        return None;
    };
    let MirInstruction::Copy { dst: copy_dst, src } = &entry.instructions[1] else {
        return None;
    };
    if src != dst {
        return None;
    }

    let MirInstruction::Return {
        value: Some(ret_val),
    } = entry.terminator.as_ref()?
    else {
        return None;
    };
    if ret_val != copy_dst {
        return None;
    }

    let ConstValue::Integer(n) = value else {
        return None;
    };
    let value = i32::try_from(*n).ok()?;
    Some(NativeMatch {
        shape: NativeShape::MainReturnI32ConstViaCopy,
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
        let found = match_native_shape(&module).expect("shape table should match");
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
            match_native_shape(&module).is_none(),
            "shape table must fail-fast outside strict pilot shape"
        );
    }

    #[test]
    fn wasm_shape_table_matches_const_copy_return_contract() {
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
        let block = function
            .get_block_mut(entry)
            .expect("entry block must exist");
        let const_dst = ValueId(1);
        let copy_dst = ValueId(2);
        block.add_instruction(MirInstruction::Const {
            dst: const_dst,
            value: ConstValue::Integer(8),
        });
        block.add_instruction(MirInstruction::Copy {
            dst: copy_dst,
            src: const_dst,
        });
        block.add_instruction(MirInstruction::Return {
            value: Some(copy_dst),
        });
        module.add_function(function);

        let found = match_native_shape(&module).expect("const-copy-return shape should match");
        assert_eq!(found.shape.id(), "wsm.p5.main_return_i32_const_via_copy.v0");
        assert_eq!(found.value, 8);
    }
}
