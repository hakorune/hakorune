use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{Callee, CompareOp, ConstValue, MirFunction, MirInstruction, MirType, ValueId};

pub(super) fn is_program_json_emit_body_function(function: &MirFunction) -> bool {
    if function.params.len() != 1 || function.signature.params.len() != 1 {
        return false;
    }
    if !program_json_emit_source_type_is_handle_compatible(&function.signature.params[0])
        || !program_json_emit_return_type_is_handle_compatible(&function.signature.return_type)
    {
        return false;
    }

    let mut state = ProgramJsonEmitBodyState::new(function.params[0]);
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());
    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            return false;
        };
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            if !state.observe(instruction) {
                return false;
            }
        }
    }
    state.is_complete()
}

fn program_json_emit_source_type_is_handle_compatible(ty: &MirType) -> bool {
    matches!(ty, MirType::Integer | MirType::String | MirType::Unknown)
        || matches!(ty, MirType::Box(name) if name == "StringBox")
}

fn program_json_emit_return_type_is_handle_compatible(ty: &MirType) -> bool {
    matches!(ty, MirType::Integer | MirType::String | MirType::Unknown)
        || matches!(ty, MirType::Box(name) if name == "StringBox")
}

#[derive(Debug)]
struct ProgramJsonEmitBodyState {
    source_param: ValueId,
    aliases: BTreeMap<ValueId, ValueId>,
    const_i64_values: BTreeMap<ValueId, i64>,
    parse_result: Option<ValueId>,
    freeze_i64: Option<ValueId>,
    freeze_bool_values: BTreeSet<ValueId>,
    enrich_result: Option<ValueId>,
    saw_freeze_branch: bool,
    saw_freeze_return: bool,
    saw_enrich_return: bool,
}

impl ProgramJsonEmitBodyState {
    fn new(source_param: ValueId) -> Self {
        Self {
            source_param,
            aliases: BTreeMap::new(),
            const_i64_values: BTreeMap::new(),
            parse_result: None,
            freeze_i64: None,
            freeze_bool_values: BTreeSet::new(),
            enrich_result: None,
            saw_freeze_branch: false,
            saw_freeze_return: false,
            saw_enrich_return: false,
        }
    }

    fn observe(&mut self, instruction: &MirInstruction) -> bool {
        match instruction {
            MirInstruction::Copy { dst, src } => {
                let resolved = self.resolve(*src);
                self.aliases.insert(*dst, resolved);
                if self.freeze_bool_values.contains(&resolved) {
                    self.freeze_bool_values.insert(*dst);
                }
                true
            }
            MirInstruction::Phi { dst, inputs, .. } => self.observe_phi(*dst, inputs),
            MirInstruction::Const {
                dst,
                value: ConstValue::Integer(value),
            } => {
                self.const_i64_values.insert(*dst, *value);
                true
            }
            MirInstruction::Compare { dst, op, lhs, rhs } => {
                self.observe_freeze_compare(*dst, *op, *lhs, *rhs)
            }
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Global(name)),
                args,
                ..
            } => self.observe_global_call(name, *dst, args),
            MirInstruction::Branch { condition, .. } => {
                let condition = self.resolve(*condition);
                if !self.freeze_bool_values.contains(&condition) {
                    return false;
                }
                self.saw_freeze_branch = true;
                true
            }
            MirInstruction::Jump { .. }
            | MirInstruction::KeepAlive { .. }
            | MirInstruction::ReleaseStrong { .. } => true,
            MirInstruction::Return { value } => self.observe_return(*value),
            _ => false,
        }
    }

    fn observe_phi(
        &mut self,
        dst: ValueId,
        inputs: &[(crate::mir::BasicBlockId, ValueId)],
    ) -> bool {
        let Some((_, first)) = inputs.first() else {
            return false;
        };
        let first = self.resolve(*first);
        if inputs
            .iter()
            .any(|(_, value)| self.resolve(*value) != first)
        {
            return false;
        }
        self.aliases.insert(dst, first);
        if self.freeze_bool_values.contains(&first) {
            self.freeze_bool_values.insert(dst);
        }
        true
    }

    fn observe_freeze_compare(
        &mut self,
        dst: ValueId,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> bool {
        if op != CompareOp::Eq {
            return false;
        }
        let lhs = self.resolve(lhs);
        let rhs = self.resolve(rhs);
        let Some(freeze_i64) = self.freeze_i64 else {
            return false;
        };
        let lhs_matches = lhs == freeze_i64 && self.const_i64_values.get(&rhs) == Some(&1);
        let rhs_matches = rhs == freeze_i64 && self.const_i64_values.get(&lhs) == Some(&1);
        if !(lhs_matches || rhs_matches) {
            return false;
        }
        self.freeze_bool_values.insert(dst);
        true
    }

    fn observe_global_call(&mut self, name: &str, dst: Option<ValueId>, args: &[ValueId]) -> bool {
        match name {
            "BuildBox._parse_program_json_from_scan_src/1" => {
                if self.parse_result.is_some()
                    || args.len() != 1
                    || self.resolve(args[0]) != self.source_param
                {
                    return false;
                }
                let Some(dst) = dst else {
                    return false;
                };
                self.parse_result = Some(dst);
                true
            }
            "BuildBox._is_freeze_tag/1" => {
                if self.freeze_i64.is_some()
                    || args.len() != 1
                    || Some(self.resolve(args[0])) != self.parse_result
                {
                    return false;
                }
                let Some(dst) = dst else {
                    return false;
                };
                self.freeze_i64 = Some(dst);
                true
            }
            "BuildProgramFragmentBox.enrich/2" => {
                if self.enrich_result.is_some()
                    || args.len() != 2
                    || Some(self.resolve(args[0])) != self.parse_result
                    || self.resolve(args[1]) != self.source_param
                {
                    return false;
                }
                let Some(dst) = dst else {
                    return false;
                };
                self.enrich_result = Some(dst);
                true
            }
            _ => false,
        }
    }

    fn observe_return(&mut self, value: Option<ValueId>) -> bool {
        let Some(value) = value else {
            return false;
        };
        let value = self.resolve(value);
        if Some(value) == self.parse_result {
            self.saw_freeze_return = true;
            true
        } else if Some(value) == self.enrich_result {
            self.saw_enrich_return = true;
            true
        } else {
            false
        }
    }

    fn is_complete(&self) -> bool {
        self.parse_result.is_some()
            && self.freeze_i64.is_some()
            && self.enrich_result.is_some()
            && self.saw_freeze_branch
            && self.saw_freeze_return
            && self.saw_enrich_return
    }

    fn resolve(&self, mut value: ValueId) -> ValueId {
        for _ in 0..32 {
            let Some(next) = self.aliases.get(&value).copied() else {
                break;
            };
            if next == value {
                break;
            }
            value = next;
        }
        value
    }
}
