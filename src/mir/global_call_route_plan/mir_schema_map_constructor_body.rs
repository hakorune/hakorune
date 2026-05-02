use std::collections::BTreeMap;

use crate::mir::{Callee, ConstValue, MirFunction, MirInstruction, MirType, ValueId};

use super::generic_string_reject::GenericPureStringReject;
use super::model::{
    GlobalCallShapeBlocker, GlobalCallTargetFacts, GlobalCallTargetShape,
    GlobalCallTargetShapeReason,
};
use super::shape_blocker::propagated_unknown_global_target_blocker;

pub(super) fn mir_schema_map_constructor_body_reject_reason(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    if function.params.len() != function.signature.params.len() {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::ParamBindingMismatch,
        ));
    }
    if !mir_schema_map_constructor_return_type_candidate(&function.signature.return_type) {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringReturnObjectAbiNotHandleCompatible,
        ));
    }

    let mut facts = MirSchemaMapConstructorFacts::default();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());
    for _ in 0..function
        .blocks
        .values()
        .map(|block| block.instructions.len() + usize::from(block.terminator.is_some()))
        .sum::<usize>()
        .saturating_add(1)
    {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                if let Some(reject) = facts.observe(instruction, targets, &mut changed) {
                    return Some(reject);
                }
            }
        }
        if !changed {
            break;
        }
    }

    if facts.accepts() {
        None
    } else {
        Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
        ))
    }
}

fn mir_schema_map_constructor_return_type_candidate(ty: &MirType) -> bool {
    matches!(ty, MirType::Box(name) if name == "MapBox")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirSchemaValueClass {
    Array,
    Map,
    Scalar,
    String,
}

#[derive(Default)]
struct MirSchemaMapConstructorFacts {
    values: BTreeMap<ValueId, MirSchemaValueClass>,
    saw_map_birth: bool,
    saw_map_set: bool,
    returned_map: bool,
}

impl MirSchemaMapConstructorFacts {
    fn observe(
        &mut self,
        instruction: &MirInstruction,
        targets: &BTreeMap<String, GlobalCallTargetFacts>,
        changed: &mut bool,
    ) -> Option<GenericPureStringReject> {
        match instruction {
            MirInstruction::Const { dst, value } => {
                let class = match value {
                    ConstValue::String(_) => MirSchemaValueClass::String,
                    ConstValue::Integer(_)
                    | ConstValue::Bool(_)
                    | ConstValue::Null
                    | ConstValue::Void => MirSchemaValueClass::Scalar,
                    _ => {
                        return Some(GenericPureStringReject::new(
                            GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                        ));
                    }
                };
                self.set_value(*dst, class, changed);
                None
            }
            MirInstruction::NewBox {
                dst,
                box_type,
                args,
            } if args.is_empty() => {
                let class = match box_type.as_str() {
                    "ArrayBox" => MirSchemaValueClass::Array,
                    "MapBox" => {
                        self.saw_map_birth = true;
                        MirSchemaValueClass::Map
                    }
                    _ => {
                        return Some(GenericPureStringReject::new(
                            GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                        ));
                    }
                };
                self.set_value(*dst, class, changed);
                None
            }
            MirInstruction::Copy { dst, src } => {
                if let Some(class) = self.values.get(src).copied() {
                    self.set_value(*dst, class, changed);
                }
                None
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
                if matches!(box_name.as_str(), "MapBox" | "RuntimeDataBox") && method == "set" {
                    if self.values.get(receiver) != Some(&MirSchemaValueClass::Map)
                        || !self.accepts_map_set_args(args)
                    {
                        return if *changed {
                            None
                        } else {
                            Some(GenericPureStringReject::new(
                                GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                            ))
                        };
                    }
                    if let Some(dst) = dst {
                        self.set_value(*dst, MirSchemaValueClass::Scalar, changed);
                    }
                    self.saw_map_set = true;
                    return None;
                }
                if matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox") && method == "push" {
                    if self.values.get(receiver) != Some(&MirSchemaValueClass::Array)
                        || !self.accepts_array_push_args(args)
                    {
                        return if *changed {
                            None
                        } else {
                            Some(GenericPureStringReject::new(
                                GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                            ))
                        };
                    }
                    if let Some(dst) = dst {
                        self.set_value(*dst, MirSchemaValueClass::Scalar, changed);
                    }
                    return None;
                }
                Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                ))
            }
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Global(name)),
                ..
            } => {
                let Some(class) = self.global_call_value_class(name, targets) else {
                    let blocker = self.global_call_blocker(name, targets);
                    return Some(GenericPureStringReject::with_shape_blocker(
                        GlobalCallTargetShapeReason::GenericStringGlobalTargetShapeUnknown,
                        blocker,
                    ));
                };
                if let Some(dst) = dst {
                    self.set_value(*dst, class, changed);
                }
                None
            }
            MirInstruction::Return { value: Some(value) } => {
                if self.values.get(value) == Some(&MirSchemaValueClass::Map) {
                    self.returned_map = true;
                    None
                } else if *changed {
                    None
                } else {
                    Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringReturnNotString,
                    ))
                }
            }
            MirInstruction::Return { value: None }
            | MirInstruction::Branch { .. }
            | MirInstruction::Jump { .. }
            | MirInstruction::KeepAlive { .. }
            | MirInstruction::ReleaseStrong { .. } => None,
            MirInstruction::Phi { dst, inputs, .. } => {
                let mut class = None;
                let mut saw_unknown = false;
                for (_, value) in inputs {
                    match self.values.get(value).copied() {
                        Some(input_class) => match class {
                            None => class = Some(input_class),
                            Some(existing) if existing == input_class => {}
                            Some(_) => {
                                return Some(GenericPureStringReject::new(
                                    GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                                ));
                            }
                        },
                        None => saw_unknown = true,
                    }
                }
                if !saw_unknown {
                    if let Some(class) = class {
                        self.set_value(*dst, class, changed);
                    }
                }
                None
            }
            _ => Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
            )),
        }
    }

    fn accepts(&self) -> bool {
        self.saw_map_birth && self.saw_map_set && self.returned_map
    }

    fn accepts_map_set_args(&self, args: &[ValueId]) -> bool {
        match args {
            [key, _value] => self.values.get(key) == Some(&MirSchemaValueClass::String),
            [receiver_arg, key, _value] => {
                self.values.get(receiver_arg) == Some(&MirSchemaValueClass::Map)
                    && self.values.get(key) == Some(&MirSchemaValueClass::String)
            }
            _ => false,
        }
    }

    fn accepts_array_push_args(&self, args: &[ValueId]) -> bool {
        match args {
            [_value] => true,
            [receiver_arg, _value] => {
                self.values.get(receiver_arg) == Some(&MirSchemaValueClass::Array)
            }
            _ => false,
        }
    }

    fn global_call_value_class(
        &self,
        name: &str,
        targets: &BTreeMap<String, GlobalCallTargetFacts>,
    ) -> Option<MirSchemaValueClass> {
        let target = super::lookup_global_call_target(name, targets)?;
        match target.shape() {
            GlobalCallTargetShape::MirSchemaMapConstructorBody => Some(MirSchemaValueClass::Map),
            GlobalCallTargetShape::StaticStringArrayBody => Some(MirSchemaValueClass::Array),
            GlobalCallTargetShape::GenericPureStringBody
            | GlobalCallTargetShape::GenericStringOrVoidSentinelBody
            | GlobalCallTargetShape::ParserProgramJsonBody
            | GlobalCallTargetShape::ProgramJsonEmitBody
            | GlobalCallTargetShape::JsonFragInstructionArrayNormalizerBody => {
                Some(MirSchemaValueClass::String)
            }
            GlobalCallTargetShape::NumericI64Leaf
            | GlobalCallTargetShape::GenericStringVoidLoggingBody
            | GlobalCallTargetShape::GenericI64Body => Some(MirSchemaValueClass::Scalar),
            GlobalCallTargetShape::Unknown | GlobalCallTargetShape::BuilderRegistryDispatchBody => {
                None
            }
        }
    }

    fn global_call_blocker(
        &self,
        name: &str,
        targets: &BTreeMap<String, GlobalCallTargetFacts>,
    ) -> GlobalCallShapeBlocker {
        match super::lookup_global_call_target(name, targets) {
            Some(target) if target.shape() == GlobalCallTargetShape::Unknown => {
                propagated_unknown_global_target_blocker(name, target)
            }
            Some(_) => GlobalCallShapeBlocker {
                symbol: crate::mir::naming::normalize_static_global_name(name),
                reason: Some(GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction),
            },
            None => GlobalCallShapeBlocker {
                symbol: crate::mir::naming::normalize_static_global_name(name),
                reason: Some(GlobalCallTargetShapeReason::GenericStringGlobalTargetMissing),
            },
        }
    }

    fn set_value(&mut self, value: ValueId, class: MirSchemaValueClass, changed: &mut bool) {
        match self.values.get(&value).copied() {
            Some(existing) if existing == class => {}
            Some(_) => {}
            None => {
                self.values.insert(value, class);
                *changed = true;
            }
        }
    }
}
