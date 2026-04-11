//! Escape Analysis (VM-only footing)
//! Conservative analysis to elide write/read barriers for definitely non-escaping boxes
//! and their Copy aliases.
//! Enabled for VM backend as a staging step before LLVM.

use crate::mir::{
    classify_escape_uses, resolve_value_origin_from_copy_parents, MirFunction, MirInstruction,
    MirModule, ValueId,
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
    copy_parents: HashMap<ValueId, ValueId>,
}

impl EscapeInfo {
    fn is_non_escaping(&self, v: &ValueId) -> bool {
        let root = resolve_value_origin_from_copy_parents(*v, &self.copy_parents);
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
    // Collect alias chains for Copy results. Barrier elimination should follow
    // local-box aliases so a `Copy`-fed barrier can still disappear when the
    // underlying box stays local.
    for block in func.blocks.values() {
        for sp in block.iter_spanned() {
            if let MirInstruction::Copy { dst, src } = sp.inst {
                info.copy_parents.insert(*dst, *src);
            }
        }
        if let Some(term) = &block.terminator {
            if let MirInstruction::Copy { dst, src } = term {
                info.copy_parents.insert(*dst, *src);
            }
        }
    }
    // Conservative escape marking through operand-role barriers
    for block in func.blocks.values() {
        for sp in block.all_spanned_instructions() {
            for use_site in classify_escape_uses(sp.inst) {
                let root =
                    resolve_value_origin_from_copy_parents(use_site.value, &info.copy_parents);
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
}
