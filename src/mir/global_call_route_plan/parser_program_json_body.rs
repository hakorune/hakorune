use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{Callee, ConstValue, MirFunction, MirInstruction, MirType, ValueId};

pub(super) fn is_parser_program_json_body_function(function: &MirFunction) -> bool {
    if function.params.len() != 1 || function.signature.params.len() != 1 {
        return false;
    }
    if !parser_source_type_is_handle_compatible(&function.signature.params[0])
        || !parser_program_json_return_type_is_handle_compatible(&function.signature.return_type)
    {
        return false;
    }
    if function.blocks.len() != 1 {
        return false;
    }
    let Some(block) = function.blocks.get(&function.entry_block) else {
        return false;
    };

    let mut state = ParserProgramJsonBodyState::new(function.params[0]);
    for instruction in block.instructions.iter().chain(block.terminator.iter()) {
        if !state.observe(instruction) {
            return false;
        }
    }
    state.is_complete()
}

fn parser_source_type_is_handle_compatible(ty: &MirType) -> bool {
    matches!(ty, MirType::Integer | MirType::String | MirType::Unknown)
        || matches!(ty, MirType::Box(name) if name == "StringBox")
}

fn parser_program_json_return_type_is_handle_compatible(ty: &MirType) -> bool {
    matches!(ty, MirType::Integer | MirType::String | MirType::Unknown)
        || matches!(ty, MirType::Box(name) if name == "StringBox")
}

#[derive(Debug)]
struct ParserProgramJsonBodyState {
    source_param: ValueId,
    aliases: BTreeMap<ValueId, ValueId>,
    parser_roots: BTreeSet<ValueId>,
    const_i64_values: BTreeMap<ValueId, i64>,
    parser_root: Option<ValueId>,
    parse_result: Option<ValueId>,
    saw_birth: bool,
    saw_stage3_enable: bool,
    saw_return: bool,
}

impl ParserProgramJsonBodyState {
    fn new(source_param: ValueId) -> Self {
        Self {
            source_param,
            aliases: BTreeMap::new(),
            parser_roots: BTreeSet::new(),
            const_i64_values: BTreeMap::new(),
            parser_root: None,
            parse_result: None,
            saw_birth: false,
            saw_stage3_enable: false,
            saw_return: false,
        }
    }

    fn observe(&mut self, instruction: &MirInstruction) -> bool {
        match instruction {
            MirInstruction::NewBox {
                dst,
                box_type,
                args,
            } => self.observe_newbox(*dst, box_type, args),
            MirInstruction::Copy { dst, src } => {
                let resolved = self.resolve(*src);
                self.aliases.insert(*dst, resolved);
                true
            }
            MirInstruction::Const {
                dst,
                value: ConstValue::Integer(value),
            } => {
                self.const_i64_values.insert(*dst, *value);
                true
            }
            MirInstruction::Call {
                dst,
                callee:
                    Some(Callee::Method {
                        box_name,
                        method,
                        receiver,
                        ..
                    }),
                args,
                ..
            } => self.observe_parser_method_call(*dst, box_name, method, *receiver, args),
            MirInstruction::Return { value } => self.observe_return(*value),
            _ => false,
        }
    }

    fn observe_newbox(&mut self, dst: ValueId, box_type: &str, args: &[ValueId]) -> bool {
        if box_type != "ParserBox" || !args.is_empty() || self.parser_root.is_some() {
            return false;
        }
        self.parser_roots.insert(dst);
        self.parser_root = Some(dst);
        true
    }

    fn observe_parser_method_call(
        &mut self,
        dst: Option<ValueId>,
        box_name: &str,
        method: &str,
        receiver: Option<ValueId>,
        args: &[ValueId],
    ) -> bool {
        if box_name != "ParserBox" || !self.receiver_is_parser_root(receiver) {
            return false;
        }
        match method {
            "birth" => {
                if self.saw_birth
                    || !args.is_empty()
                    || self.saw_stage3_enable
                    || self.parse_result.is_some()
                {
                    return false;
                }
                self.saw_birth = true;
                if let Some(dst) = dst {
                    self.const_i64_values.insert(dst, 0);
                }
                true
            }
            "stage3_enable" => {
                if !self.saw_birth || self.saw_stage3_enable || self.parse_result.is_some() {
                    return false;
                }
                if args.len() != 1 || self.const_i64(args[0]) != Some(1) {
                    return false;
                }
                self.saw_stage3_enable = true;
                if let Some(dst) = dst {
                    self.const_i64_values.insert(dst, 0);
                }
                true
            }
            "parse_program2" => {
                if !self.saw_stage3_enable || self.parse_result.is_some() || args.len() != 1 {
                    return false;
                }
                if self.resolve(args[0]) != self.resolve(self.source_param) {
                    return false;
                }
                let Some(dst) = dst else {
                    return false;
                };
                self.parse_result = Some(dst);
                true
            }
            _ => false,
        }
    }

    fn observe_return(&mut self, value: Option<ValueId>) -> bool {
        if self.saw_return {
            return false;
        }
        let (Some(value), Some(parse_result)) = (value, self.parse_result) else {
            return false;
        };
        if self.resolve(value) != self.resolve(parse_result) {
            return false;
        }
        self.saw_return = true;
        true
    }

    fn receiver_is_parser_root(&mut self, receiver: Option<ValueId>) -> bool {
        let Some(receiver) = receiver else {
            return false;
        };
        let resolved = self.resolve(receiver);
        self.parser_roots.contains(&resolved)
    }

    fn const_i64(&mut self, value: ValueId) -> Option<i64> {
        let resolved = self.resolve(value);
        self.const_i64_values.get(&resolved).copied()
    }

    fn resolve(&mut self, value: ValueId) -> ValueId {
        let mut current = value;
        for _ in 0..32 {
            let Some(next) = self.aliases.get(&current).copied() else {
                break;
            };
            if next == current {
                break;
            }
            current = next;
        }
        if current != value {
            self.aliases.insert(value, current);
        }
        current
    }

    fn is_complete(&self) -> bool {
        self.parser_root.is_some()
            && self.saw_birth
            && self.saw_stage3_enable
            && self.parse_result.is_some()
            && self.saw_return
    }
}
