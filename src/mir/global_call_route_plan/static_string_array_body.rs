use std::collections::BTreeMap;

use crate::mir::{Callee, ConstValue, MirFunction, MirInstruction, MirType, ValueId};

pub(super) fn is_static_string_array_body_function(function: &MirFunction) -> bool {
    if function.params.len() != function.signature.params.len() || !function.params.is_empty() {
        return false;
    }
    if !static_string_array_return_type_candidate(&function.signature.return_type) {
        return false;
    }
    if function.blocks.len() != 1 {
        return false;
    }

    let mut facts = StaticStringArrayFacts::default();
    let Some(block) = function.blocks.get(&function.entry_block) else {
        return false;
    };
    for instruction in block.instructions.iter().chain(block.terminator.iter()) {
        if !facts.observe(instruction) {
            return false;
        }
    }
    facts.accepts()
}

fn static_string_array_return_type_candidate(return_type: &MirType) -> bool {
    matches!(return_type, MirType::Unknown | MirType::Void)
        || matches!(return_type, MirType::Box(name) if name == "ArrayBox")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StaticStringArrayValueClass {
    Array,
    String,
    Scalar,
}

#[derive(Default)]
struct StaticStringArrayFacts {
    values: BTreeMap<ValueId, StaticStringArrayValueClass>,
    array_births: usize,
    string_consts: usize,
    pushes: usize,
    returned_array: bool,
}

impl StaticStringArrayFacts {
    fn observe(&mut self, instruction: &MirInstruction) -> bool {
        match instruction {
            MirInstruction::Const { dst, value } => match value {
                ConstValue::String(_) => {
                    self.values
                        .insert(*dst, StaticStringArrayValueClass::String);
                    self.string_consts += 1;
                    true
                }
                _ => false,
            },
            MirInstruction::NewBox {
                dst,
                box_type,
                args,
            } => {
                if box_type != "ArrayBox" || !args.is_empty() {
                    return false;
                }
                self.array_births += 1;
                self.values.insert(*dst, StaticStringArrayValueClass::Array);
                true
            }
            MirInstruction::Copy { dst, src } => {
                let Some(class) = self.values.get(src).copied() else {
                    return false;
                };
                self.values.insert(*dst, class);
                true
            }
            MirInstruction::Call {
                dst,
                callee:
                    Some(Callee::Method {
                        box_name,
                        method,
                        receiver: Some(receiver),
                        ..
                    }),
                args,
                ..
            } => {
                if !matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox")
                    || method != "push"
                    || self.values.get(receiver) != Some(&StaticStringArrayValueClass::Array)
                    || self.push_string_arg(receiver, args).is_none()
                {
                    return false;
                }
                if let Some(dst) = dst {
                    self.values
                        .insert(*dst, StaticStringArrayValueClass::Scalar);
                }
                self.pushes += 1;
                true
            }
            MirInstruction::Return { value: Some(value) } => {
                if self.values.get(value) != Some(&StaticStringArrayValueClass::Array) {
                    return false;
                }
                self.returned_array = true;
                true
            }
            _ => false,
        }
    }

    fn accepts(&self) -> bool {
        self.array_births == 1 && self.string_consts >= 1 && self.pushes >= 1 && self.returned_array
    }

    fn push_string_arg(&self, receiver: &ValueId, args: &[ValueId]) -> Option<ValueId> {
        match args {
            [value] if self.values.get(value) == Some(&StaticStringArrayValueClass::String) => {
                Some(*value)
            }
            [receiver_arg, value]
                if self.values.get(receiver_arg) == Some(&StaticStringArrayValueClass::Array)
                    && self.values.get(receiver) == Some(&StaticStringArrayValueClass::Array)
                    && self.values.get(value) == Some(&StaticStringArrayValueClass::String) =>
            {
                Some(*value)
            }
            _ => None,
        }
    }
}
