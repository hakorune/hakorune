//! Escape Analysis (VM-only footing)
//! Conservative analysis to elide write/read barriers for definitely non-escaping boxes
//! and their Copy aliases.
//! Enabled for VM backend as a staging step before LLVM.

use crate::mir::phi_query::collect_passthrough_phi_parents;
use crate::mir::{
    classify_escape_uses, resolve_value_origin_from_parent_map, MirFunction, MirInstruction,
    MirModule, ParentMap, ValueId,
};
use std::collections::{HashMap, HashSet};

/// Run a conservative escape analysis and remove Barrier(Read/Write) for non-escaping boxes.
/// Returns the number of barriers removed.
pub fn escape_elide_barriers_vm(module: &mut MirModule) -> usize {
    let mut removed = 0usize;
    let mut analysis: HashMap<String, EscapeInfo> = HashMap::new();

    // 1) Analyze each function
    for (name, func) in module.functions.iter() {
        analysis.insert(name.clone(), analyze_function(func));
    }

    // 2) Apply in-place edits per function
    for (name, info) in analysis.into_iter() {
        if let Some(func) = module.functions.get_mut(&name) {
            removed += elide_barriers_in_function(func, &info);
        }
    }
    removed
}

#[derive(Default)]
struct EscapeInfo {
    local_boxes: HashSet<ValueId>,
    escaping: HashSet<ValueId>,
    alias_parents: ParentMap,
}

impl EscapeInfo {
    fn is_non_escaping(&self, v: &ValueId) -> bool {
        let root = resolve_value_origin_from_parent_map(*v, &self.alias_parents);
        self.local_boxes.contains(&root) && !self.escaping.contains(&root)
    }
}

fn analyze_function(func: &MirFunction) -> EscapeInfo {
    let mut info = EscapeInfo::default();
    // Collect local boxes: results of NewBox in this function
    for block in func.blocks.values() {
        for sp in block.iter_spanned() {
            if let MirInstruction::NewBox { dst, .. } = sp.inst {
                info.local_boxes.insert(*dst);
            }
        }
        if let Some(term) = &block.terminator {
            if let MirInstruction::NewBox { dst, .. } = term {
                info.local_boxes.insert(*dst);
            }
        }
    }
    // Collect alias chains for Copy results and one-input passthrough PHIs.
    // Barrier elimination should follow local-box aliases so a `Copy`/carry-PHI
    // fed barrier can still disappear when the underlying box stays local.
    for block in func.blocks.values() {
        for sp in block.iter_spanned() {
            if let MirInstruction::Copy { dst, src } = sp.inst {
                info.alias_parents.insert(*dst, *src);
            }
        }
        if let Some(term) = &block.terminator {
            if let MirInstruction::Copy { dst, src } = term {
                info.alias_parents.insert(*dst, *src);
            }
        }
    }
    info.alias_parents
        .extend(collect_passthrough_phi_parents(func));
    // Conservative escape marking through operand-role barriers
    for block in func.blocks.values() {
        for sp in block.all_spanned_instructions() {
            for use_site in classify_escape_uses(sp.inst) {
                let root =
                    resolve_value_origin_from_parent_map(use_site.value, &info.alias_parents);
                if info.local_boxes.contains(&root) {
                    info.escaping.insert(root);
                }
            }
        }
    }
    info
}

fn elide_barriers_in_function(func: &mut MirFunction, info: &EscapeInfo) -> usize {
    let mut removed = 0usize;
    for block in func.blocks.values_mut() {
        block.instructions.retain(|ins| match ins {
            MirInstruction::Barrier { ptr, .. } if info.is_non_escaping(ptr) => {
                removed += 1;
                false
            }
            _ => true,
        });

        if let Some(MirInstruction::Barrier { ptr, .. }) = block.terminator.as_ref() {
            if info.is_non_escaping(ptr) {
                block.terminator = None;
                block.terminator_span = None;
                removed += 1;
            }
        }
    }
    if removed > 0 {
        func.update_cfg();
    }
    removed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{BasicBlock, BasicBlockId, Callee, EffectMask, FunctionSignature, MirType};

    fn build_alias_escape_module(return_alias: bool) -> MirModule {
        let mut module = MirModule::new("escape_alias_test".to_string());
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: if return_alias {
                MirType::Box("Point".to_string())
            } else {
                MirType::Void
            },
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);
        let mut block = BasicBlock::new(entry);

        let local_box = ValueId::new(1);
        let alias = ValueId::new(2);

        block.add_instruction(MirInstruction::NewBox {
            dst: local_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: alias,
            src: local_box,
        });
        block.add_instruction(MirInstruction::Barrier {
            op: crate::mir::BarrierOp::Write,
            ptr: alias,
        });
        block.set_terminator(MirInstruction::Return {
            value: if return_alias { Some(alias) } else { None },
        });

        func.add_block(block);
        module.add_function(func);
        module
    }

    fn build_method_receiver_escape_module() -> MirModule {
        let mut module = MirModule::new("escape_call_receiver_test".to_string());
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);
        let mut block = BasicBlock::new(entry);

        let local_box = ValueId::new(1);
        let alias = ValueId::new(2);

        block.add_instruction(MirInstruction::NewBox {
            dst: local_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: alias,
            src: local_box,
        });
        block.add_instruction(MirInstruction::Barrier {
            op: crate::mir::BarrierOp::Write,
            ptr: alias,
        });
        block.add_instruction(MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "Point".to_string(),
                method: "sum".to_string(),
                receiver: Some(alias),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.set_terminator(MirInstruction::Return { value: None });

        func.add_block(block);
        module.add_function(func);
        module
    }

    fn build_fieldset_base_only_module() -> MirModule {
        let mut module = MirModule::new("escape_fieldset_base_only_test".to_string());
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);
        let mut block = BasicBlock::new(entry);

        let local_box = ValueId::new(1);
        let alias = ValueId::new(2);
        let value = ValueId::new(3);

        block.add_instruction(MirInstruction::NewBox {
            dst: local_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        block.add_instruction(MirInstruction::Copy {
            dst: alias,
            src: local_box,
        });
        block.add_instruction(MirInstruction::Const {
            dst: value,
            value: crate::mir::ConstValue::Integer(7),
        });
        block.add_instruction(MirInstruction::Barrier {
            op: crate::mir::BarrierOp::Write,
            ptr: alias,
        });
        block.add_instruction(MirInstruction::FieldSet {
            base: alias,
            field: "child".to_string(),
            value,
            declared_type: Some(MirType::Integer),
        });
        block.set_terminator(MirInstruction::Return { value: None });

        func.add_block(block);
        module.add_function(func);
        module
    }

    fn build_single_input_phi_alias_module(return_phi: bool) -> MirModule {
        let mut module = MirModule::new("escape_single_input_phi_alias_test".to_string());
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: if return_phi {
                MirType::Box("Point".to_string())
            } else {
                MirType::Void
            },
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let carry = BasicBlockId::new(1);
        let mut func = MirFunction::new(sig, entry);
        func.add_block(BasicBlock::new(carry));

        let local_box = ValueId::new(1);
        let alias = ValueId::new(2);
        let phi_alias = ValueId::new(3);

        let entry_block = func.blocks.get_mut(&entry).expect("entry");
        entry_block.add_instruction(MirInstruction::NewBox {
            dst: local_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        entry_block.add_instruction(MirInstruction::Copy {
            dst: alias,
            src: local_box,
        });
        entry_block.set_terminator(MirInstruction::Jump {
            target: carry,
            edge_args: None,
        });

        let carry_block = func.blocks.get_mut(&carry).expect("carry");
        carry_block.add_instruction(MirInstruction::Phi {
            dst: phi_alias,
            inputs: vec![(entry, alias)],
            type_hint: Some(MirType::Box("Point".to_string())),
        });
        carry_block.add_instruction(MirInstruction::Barrier {
            op: crate::mir::BarrierOp::Write,
            ptr: phi_alias,
        });
        carry_block.set_terminator(MirInstruction::Return {
            value: return_phi.then_some(phi_alias),
        });

        module.add_function(func);
        module
    }

    fn build_multi_input_phi_merge_module() -> MirModule {
        let mut module = MirModule::new("escape_multi_input_phi_merge_test".to_string());
        let sig = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let entry = BasicBlockId::new(0);
        let alt = BasicBlockId::new(1);
        let merge = BasicBlockId::new(2);
        let mut func = MirFunction::new(sig, entry);
        func.add_block(BasicBlock::new(alt));
        func.add_block(BasicBlock::new(merge));

        let local_box = ValueId::new(1);
        let alias = ValueId::new(2);
        let phi_alias = ValueId::new(3);

        let entry_block = func.blocks.get_mut(&entry).expect("entry");
        entry_block.add_instruction(MirInstruction::NewBox {
            dst: local_box,
            box_type: "Point".to_string(),
            args: vec![],
        });
        entry_block.set_terminator(MirInstruction::Jump {
            target: merge,
            edge_args: None,
        });

        let alt_block = func.blocks.get_mut(&alt).expect("alt");
        alt_block.add_instruction(MirInstruction::Copy {
            dst: alias,
            src: local_box,
        });
        alt_block.set_terminator(MirInstruction::Jump {
            target: merge,
            edge_args: None,
        });

        let merge_block = func.blocks.get_mut(&merge).expect("merge");
        merge_block.add_instruction(MirInstruction::Phi {
            dst: phi_alias,
            inputs: vec![(entry, local_box), (alt, alias)],
            type_hint: Some(MirType::Box("Point".to_string())),
        });
        merge_block.add_instruction(MirInstruction::Barrier {
            op: crate::mir::BarrierOp::Write,
            ptr: phi_alias,
        });
        merge_block.set_terminator(MirInstruction::Return { value: None });

        module.add_function(func);
        module
    }

    #[test]
    fn test_escape_elides_barrier_through_copy_alias() {
        let mut module = build_alias_escape_module(false);

        let removed = escape_elide_barriers_vm(&mut module);
        assert_eq!(removed, 1);

        let func = module.get_function("main").unwrap();
        let block = func.blocks.get(&BasicBlockId::new(0)).unwrap();
        assert!(!block
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Barrier { .. })));
    }

    #[test]
    fn test_escape_keeps_barrier_when_copy_alias_returns() {
        let mut module = build_alias_escape_module(true);

        let removed = escape_elide_barriers_vm(&mut module);
        assert_eq!(removed, 0);

        let func = module.get_function("main").unwrap();
        let block = func.blocks.get(&BasicBlockId::new(0)).unwrap();
        assert!(block
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Barrier { .. })));
    }

    #[test]
    fn test_escape_keeps_barrier_when_copy_alias_is_method_receiver() {
        let mut module = build_method_receiver_escape_module();

        let removed = escape_elide_barriers_vm(&mut module);
        assert_eq!(removed, 0);

        let func = module.get_function("main").unwrap();
        let block = func.blocks.get(&BasicBlockId::new(0)).unwrap();
        assert!(block
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Barrier { .. })));
    }

    #[test]
    fn test_escape_elides_barrier_when_alias_only_appears_as_fieldset_base() {
        let mut module = build_fieldset_base_only_module();

        let removed = escape_elide_barriers_vm(&mut module);
        assert_eq!(removed, 1);

        let func = module.get_function("main").unwrap();
        let block = func.blocks.get(&BasicBlockId::new(0)).unwrap();
        assert!(!block
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Barrier { .. })));
    }

    #[test]
    fn test_escape_elides_barrier_through_single_input_phi_alias() {
        let mut module = build_single_input_phi_alias_module(false);

        let removed = escape_elide_barriers_vm(&mut module);
        assert_eq!(removed, 1);

        let func = module.get_function("main").unwrap();
        let block = func.blocks.get(&BasicBlockId::new(1)).unwrap();
        assert!(!block
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Barrier { .. })));
    }

    #[test]
    fn test_escape_keeps_barrier_when_single_input_phi_alias_returns() {
        let mut module = build_single_input_phi_alias_module(true);

        let removed = escape_elide_barriers_vm(&mut module);
        assert_eq!(removed, 0);

        let func = module.get_function("main").unwrap();
        let block = func.blocks.get(&BasicBlockId::new(1)).unwrap();
        assert!(block
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Barrier { .. })));
    }

    #[test]
    fn test_escape_keeps_barrier_across_multi_input_phi_merge() {
        let mut module = build_multi_input_phi_merge_module();

        let removed = escape_elide_barriers_vm(&mut module);
        assert_eq!(removed, 0);

        let func = module.get_function("main").unwrap();
        let block = func.blocks.get(&BasicBlockId::new(2)).unwrap();
        assert!(block
            .instructions
            .iter()
            .any(|inst| matches!(inst, MirInstruction::Barrier { .. })));
    }
}
