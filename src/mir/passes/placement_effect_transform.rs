/*!
 * Generic placement/effect transform owner seam.
 *
 * This module is the first top-level MIR transform owner for the generic
 * placement/effect lane. In this cut it owns module iteration and pre/post-DCE
 * scheduling, while delegating the landed string family transform to the
 * string-corridor sink module. This keeps family logic local while removing
 * optimizer-level family-specific wiring.
 */

use crate::mir::MirModule;

pub fn apply_pre_dce_transforms(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    for function in module.functions.values_mut() {
        rewritten +=
            crate::mir::passes::string_corridor_sink::apply_string_corridor_pre_dce_transforms(
                function,
            );
    }
    rewritten
}

pub fn apply_post_dce_transforms(module: &mut MirModule) -> usize {
    let mut rewritten = 0usize;
    for function in module.functions.values_mut() {
        rewritten +=
            crate::mir::passes::string_corridor_sink::apply_string_corridor_post_dce_transforms(
                function,
            );
    }
    rewritten
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
        MirType, ValueId,
    };

    #[test]
    fn pre_dce_transform_owner_runs_landed_string_corridor_sink() {
        let mut module = MirModule::new("placement_effect_transform".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("StringBox".to_string()),
                MirType::Integer,
                MirType::Integer,
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(3)),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Method {
                box_name: "StringBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId(0)),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId(1), ValueId(2)],
            effects: EffectMask::PURE,
        });
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(4)),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Method {
                box_name: "StringBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId(3)),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });
        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box("StringBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Integer);
        module.add_function(function);

        let rewritten = apply_pre_dce_transforms(&mut module);
        assert_eq!(rewritten, 1);
    }

    #[test]
    fn post_dce_transform_owner_runs_landed_string_corridor_sink() {
        let mut module = MirModule::new("placement_effect_transform_post".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box("StringBox".to_string()),
                MirType::Integer,
                MirType::Integer,
            ],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(3)),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Method {
                box_name: "StringBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId(0)),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId(1), ValueId(2)],
            effects: EffectMask::PURE,
        });
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId(4)),
            func: ValueId::INVALID,
            callee: Some(crate::mir::Callee::Method {
                box_name: "StringBox".to_string(),
                method: "length".to_string(),
                receiver: Some(ValueId(3)),
                certainty: crate::mir::definitions::call_unified::TypeCertainty::Known,
                box_kind: crate::mir::definitions::call_unified::CalleeBoxKind::RuntimeData,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(4)),
        });
        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box("StringBox".to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Integer);
        module.add_function(function);

        let rewritten = apply_post_dce_transforms(&mut module);
        assert_eq!(rewritten, 1);
    }
}
