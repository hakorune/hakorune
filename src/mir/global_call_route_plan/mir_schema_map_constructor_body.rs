use std::collections::{BTreeMap, BTreeSet};

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
    matches!(ty, MirType::Unknown) || matches!(ty, MirType::Box(name) if name == "MapBox")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirSchemaValueClass {
    Array,
    Map,
    Scalar,
    String,
}

pub(super) fn is_mir_schema_map_constructor_body_candidate(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    if function.params.len() != function.signature.params.len() {
        return false;
    }
    if matches!(&function.signature.return_type, MirType::Box(name) if name == "MapBox") {
        return true;
    }
    if function.signature.return_type != MirType::Unknown {
        return false;
    }

    let mut facts = MirSchemaMapWrapperCandidateFacts::default();
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
                facts.observe(instruction, targets, &mut changed);
            }
        }
        if !changed {
            break;
        }
    }
    facts.accepts()
}

#[derive(Default)]
struct MirSchemaMapWrapperCandidateFacts {
    arrays: BTreeSet<ValueId>,
    map_values: BTreeSet<ValueId>,
    array_wrapped_map_values: BTreeSet<ValueId>,
    saw_array_birth: bool,
    saw_array_push: bool,
    returned_array_wrapped_map: bool,
}

impl MirSchemaMapWrapperCandidateFacts {
    fn observe(
        &mut self,
        instruction: &MirInstruction,
        targets: &BTreeMap<String, GlobalCallTargetFacts>,
        changed: &mut bool,
    ) {
        match instruction {
            MirInstruction::NewBox {
                dst,
                box_type,
                args,
            } if args.is_empty() && box_type == "ArrayBox" => {
                self.saw_array_birth = true;
                self.insert_array(*dst, changed);
            }
            MirInstruction::Copy { dst, src } => {
                if self.arrays.contains(src) {
                    self.insert_array(*dst, changed);
                }
                if self.map_values.contains(src) {
                    self.insert_map(*dst, changed);
                }
                if self.array_wrapped_map_values.contains(src) {
                    self.insert_array_wrapped_map(*dst, changed);
                }
            }
            MirInstruction::Call {
                callee:
                    Some(Callee::Method {
                        box_name,
                        method,
                        receiver: Some(receiver),
                        ..
                    }),
                args,
                ..
            } if matches!(box_name.as_str(), "ArrayBox" | "RuntimeDataBox")
                && method == "push"
                && self.arrays.contains(receiver)
                && self.array_pushes_map(args) =>
            {
                self.saw_array_push = true;
            }
            MirInstruction::Call {
                dst: Some(dst),
                callee: Some(Callee::Global(name)),
                args,
                ..
            } if target_may_return_mir_schema_map(name, targets) => {
                self.insert_map(*dst, changed);
                if args.iter().any(|arg| self.arrays.contains(arg)) {
                    self.insert_array_wrapped_map(*dst, changed);
                }
            }
            MirInstruction::Return { value: Some(value) } => {
                if self.array_wrapped_map_values.contains(value) {
                    self.returned_array_wrapped_map = true;
                }
            }
            _ => {}
        }
    }

    fn accepts(&self) -> bool {
        self.saw_array_birth && self.saw_array_push && self.returned_array_wrapped_map
    }

    fn array_pushes_map(&self, args: &[ValueId]) -> bool {
        match args {
            [value] => self.map_values.contains(value),
            [receiver_arg, value] => {
                self.arrays.contains(receiver_arg) && self.map_values.contains(value)
            }
            _ => false,
        }
    }

    fn insert_array(&mut self, value: ValueId, changed: &mut bool) {
        if self.arrays.insert(value) {
            *changed = true;
        }
    }

    fn insert_map(&mut self, value: ValueId, changed: &mut bool) {
        if self.map_values.insert(value) {
            *changed = true;
        }
    }

    fn insert_array_wrapped_map(&mut self, value: ValueId, changed: &mut bool) {
        if self.array_wrapped_map_values.insert(value) {
            *changed = true;
        }
    }
}

fn target_may_return_mir_schema_map(
    name: &str,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    let Some(target) = super::lookup_global_call_target(name, targets) else {
        return false;
    };
    target.shape() == GlobalCallTargetShape::MirSchemaMapConstructorBody
        || matches!(target.return_type(), Some(MirType::Box(box_name)) if box_name == "MapBox")
}

#[derive(Default)]
struct MirSchemaMapConstructorFacts {
    values: BTreeMap<ValueId, MirSchemaValueClass>,
    array_wrapped_map_values: BTreeSet<ValueId>,
    saw_array_birth: bool,
    saw_map_birth: bool,
    saw_map_set: bool,
    saw_array_push: bool,
    returned_map: bool,
    returned_array_wrapped_map: bool,
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
                    "ArrayBox" => {
                        self.saw_array_birth = true;
                        MirSchemaValueClass::Array
                    }
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
                if self.array_wrapped_map_values.contains(src) {
                    self.set_array_wrapped_map(*dst, changed);
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
                if method == "birth" && args.is_empty() {
                    let expected = match box_name.as_str() {
                        "ArrayBox" | "RuntimeDataBox"
                            if self.values.get(receiver) == Some(&MirSchemaValueClass::Array) =>
                        {
                            Some(MirSchemaValueClass::Scalar)
                        }
                        "MapBox" | "RuntimeDataBox"
                            if self.values.get(receiver) == Some(&MirSchemaValueClass::Map) =>
                        {
                            Some(MirSchemaValueClass::Scalar)
                        }
                        _ => None,
                    };
                    let Some(class) = expected else {
                        return if *changed {
                            None
                        } else {
                            Some(GenericPureStringReject::new(
                                GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                            ))
                        };
                    };
                    if let Some(dst) = dst {
                        self.set_value(*dst, class, changed);
                    }
                    return None;
                }
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
                    self.saw_array_push = true;
                    return None;
                }
                Some(GenericPureStringReject::new(
                    GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                ))
            }
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Global(name)),
                args,
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
                    if class == MirSchemaValueClass::Map
                        && args
                            .iter()
                            .any(|arg| self.values.get(arg) == Some(&MirSchemaValueClass::Array))
                    {
                        self.set_array_wrapped_map(*dst, changed);
                    }
                }
                None
            }
            MirInstruction::Return { value: Some(value) } => {
                if self.values.get(value) == Some(&MirSchemaValueClass::Map) {
                    self.returned_map = true;
                    if self.array_wrapped_map_values.contains(value) {
                        self.returned_array_wrapped_map = true;
                    }
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
                    if inputs
                        .iter()
                        .any(|(_, value)| self.array_wrapped_map_values.contains(value))
                    {
                        self.set_array_wrapped_map(*dst, changed);
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
        let local_map_constructor = self.saw_map_birth && self.saw_map_set && self.returned_map;
        let array_wrapped_map_constructor =
            self.saw_array_birth && self.saw_array_push && self.returned_array_wrapped_map;
        local_map_constructor || array_wrapped_map_constructor
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
            [value] => self.values.get(value) == Some(&MirSchemaValueClass::Map),
            [receiver_arg, value] => {
                self.values.get(receiver_arg) == Some(&MirSchemaValueClass::Array)
                    && self.values.get(value) == Some(&MirSchemaValueClass::Map)
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
            GlobalCallTargetShape::MirSchemaMapConstructorBody
            | GlobalCallTargetShape::BoxTypeInspectorDescribeBody => Some(MirSchemaValueClass::Map),
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
            | GlobalCallTargetShape::GenericI64Body
            | GlobalCallTargetShape::PatternUtilLocalValueProbeBody => {
                Some(MirSchemaValueClass::Scalar)
            }
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

    fn set_array_wrapped_map(&mut self, value: ValueId, changed: &mut bool) {
        if self.array_wrapped_map_values.insert(value) {
            *changed = true;
        }
    }
}
