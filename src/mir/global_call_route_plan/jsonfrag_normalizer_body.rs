use crate::mir::{Callee, ConstValue, MirFunction, MirInstruction};

use super::generic_string_abi::generic_pure_string_abi_type_is_handle_compatible;

pub(super) fn is_jsonfrag_instruction_array_normalizer_body_function(
    function: &MirFunction,
) -> bool {
    if function.params.len() != function.signature.params.len() {
        return false;
    }
    if !generic_pure_string_abi_type_is_handle_compatible(&function.signature.return_type) {
        return false;
    }
    if function.signature.params.len() != 1
        || !function
            .signature
            .params
            .iter()
            .all(generic_pure_string_abi_type_is_handle_compatible)
    {
        return false;
    }

    let mut facts = JsonFragNormalizerFacts::default();
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            facts.observe(instruction);
        }
    }
    facts.is_instruction_array_normalizer_shape()
}

#[derive(Default)]
struct JsonFragNormalizerFacts {
    array_births: usize,
    map_births: usize,
    array_pushes: usize,
    map_reads: usize,
    map_writes: usize,
    string_surface: bool,
    void_sentinel_const: bool,
    returns_value: bool,
}

impl JsonFragNormalizerFacts {
    fn observe(&mut self, instruction: &MirInstruction) {
        match instruction {
            MirInstruction::Const { value, .. } => match value {
                ConstValue::String(_) => self.string_surface = true,
                ConstValue::Null | ConstValue::Void => self.void_sentinel_const = true,
                _ => {}
            },
            MirInstruction::NewBox { box_type, args, .. } if args.is_empty() => {
                if box_type == "ArrayBox" {
                    self.array_births += 1;
                } else if box_type == "MapBox" {
                    self.map_births += 1;
                }
            }
            MirInstruction::Call {
                callee:
                    Some(Callee::Method {
                        box_name, method, ..
                    }),
                ..
            } => match (box_name.as_str(), method.as_str()) {
                ("ArrayBox" | "RuntimeDataBox", "push") => self.array_pushes += 1,
                ("MapBox" | "RuntimeDataBox", "get") => self.map_reads += 1,
                ("MapBox" | "RuntimeDataBox", "set") => self.map_writes += 1,
                ("StringBox" | "RuntimeDataBox", "length" | "substring" | "indexOf") => {
                    self.string_surface = true;
                }
                _ => {}
            },
            MirInstruction::Return { value: Some(_), .. } => self.returns_value = true,
            _ => {}
        }
    }

    fn is_instruction_array_normalizer_shape(&self) -> bool {
        self.returns_value
            && self.string_surface
            && self.void_sentinel_const
            && self.array_births >= 2
            && self.map_births >= 1
            && self.array_pushes >= 1
            && self.map_reads >= 1
            && self.map_writes >= 1
    }
}
