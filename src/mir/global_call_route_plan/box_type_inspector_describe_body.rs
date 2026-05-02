use std::collections::BTreeMap;

use crate::mir::extern_call_route_plan::{classify_extern_call_route, ExternCallRouteKind};
use crate::mir::{Callee, ConstValue, MirFunction, MirInstruction, MirType, ValueId};

use super::generic_string_reject::GenericPureStringReject;
use super::model::GlobalCallTargetShapeReason;

pub(super) fn box_type_inspector_describe_body_reject_reason(
    function: &MirFunction,
) -> Option<GenericPureStringReject> {
    if function.params.len() != function.signature.params.len() || function.params.len() != 1 {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::ParamBindingMismatch,
        ));
    }
    if !box_type_inspector_return_type_candidate(&function.signature.return_type) {
        return Some(GenericPureStringReject::new(
            GlobalCallTargetShapeReason::GenericStringReturnObjectAbiNotHandleCompatible,
        ));
    }

    let mut facts = BoxTypeInspectorDescribeFacts::default();
    facts.seed_value_types(&function.metadata.value_types);
    for param in &function.params {
        facts.set_value(*param, InspectorValueClass::UnknownHandle, &mut false);
    }

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
                if let Some(reject) = facts.observe(instruction, &mut changed) {
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

pub(super) fn is_box_type_inspector_describe_body_candidate(function: &MirFunction) -> bool {
    if function.params.len() != function.signature.params.len()
        || function.params.len() != 1
        || !box_type_inspector_return_type_candidate(&function.signature.return_type)
    {
        return false;
    }

    let mut markers = BoxTypeInspectorDescribeMarkers::default();
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            markers.observe(instruction);
        }
    }
    markers.is_candidate()
}

fn box_type_inspector_return_type_candidate(return_type: &MirType) -> bool {
    matches!(return_type, MirType::Unknown)
        || matches!(return_type, MirType::Box(name) if name == "MapBox")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InspectorValueClass {
    UnknownHandle,
    Map,
    String,
    Scalar,
    Bool,
    Void,
}

#[derive(Default)]
struct BoxTypeInspectorDescribeFacts {
    values: BTreeMap<ValueId, InspectorValueClass>,
    saw_map_birth: bool,
    saw_indexof_probe: bool,
    saw_kind_key: bool,
    saw_is_map_key: bool,
    saw_is_array_key: bool,
    saw_box_probe_const: bool,
    saw_map_set: bool,
    returned_map: bool,
}

#[derive(Default)]
struct BoxTypeInspectorDescribeMarkers {
    saw_map_birth: bool,
    saw_kind_key: bool,
    saw_is_map_key: bool,
    saw_is_array_key: bool,
    saw_box_probe_const: bool,
    saw_indexof: bool,
    saw_map_set: bool,
}

impl BoxTypeInspectorDescribeMarkers {
    fn observe(&mut self, instruction: &MirInstruction) {
        match instruction {
            MirInstruction::NewBox { box_type, args, .. }
                if args.is_empty() && box_type == "MapBox" =>
            {
                self.saw_map_birth = true;
            }
            MirInstruction::Const {
                value: ConstValue::String(text),
                ..
            } => match text.as_str() {
                "kind" => self.saw_kind_key = true,
                "is_map" => self.saw_is_map_key = true,
                "is_array" => self.saw_is_array_key = true,
                "MapBox(" | "ArrayBox(" | "StringBox(" | "IntegerBox(" | "{" | "[" => {
                    self.saw_box_probe_const = true;
                }
                _ => {}
            },
            MirInstruction::Call {
                callee:
                    Some(Callee::Method {
                        box_name, method, ..
                    }),
                ..
            } => {
                if matches!(box_name.as_str(), "RuntimeDataBox" | "StringBox")
                    && method == "indexOf"
                {
                    self.saw_indexof = true;
                }
                if matches!(box_name.as_str(), "RuntimeDataBox" | "MapBox") && method == "set" {
                    self.saw_map_set = true;
                }
            }
            _ => {}
        }
    }

    fn is_candidate(&self) -> bool {
        self.saw_map_birth
            && self.saw_kind_key
            && self.saw_is_map_key
            && self.saw_is_array_key
            && self.saw_box_probe_const
            && self.saw_indexof
            && self.saw_map_set
    }
}

impl BoxTypeInspectorDescribeFacts {
    fn seed_value_types(&mut self, value_types: &BTreeMap<ValueId, MirType>) {
        for (value, ty) in value_types {
            if let Some(class) = inspector_value_class_from_type_hint(ty) {
                self.values.insert(*value, class);
            }
        }
    }

    fn observe(
        &mut self,
        instruction: &MirInstruction,
        changed: &mut bool,
    ) -> Option<GenericPureStringReject> {
        match instruction {
            MirInstruction::Const { dst, value } => {
                let class = match value {
                    ConstValue::String(text) => {
                        self.observe_string_const(text);
                        InspectorValueClass::String
                    }
                    ConstValue::Integer(_) | ConstValue::Bool(_) => InspectorValueClass::Scalar,
                    ConstValue::Null | ConstValue::Void => InspectorValueClass::Void,
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
            } if args.is_empty() && box_type == "MapBox" => {
                self.saw_map_birth = true;
                self.set_value(*dst, InspectorValueClass::Map, changed);
                None
            }
            MirInstruction::Copy { dst, src } => {
                if let Some(class) = self.values.get(src).copied() {
                    self.set_value(*dst, class, changed);
                }
                None
            }
            MirInstruction::Phi {
                dst,
                inputs,
                type_hint,
            } => {
                if let Some(class) = type_hint
                    .as_ref()
                    .and_then(inspector_value_class_from_type_hint)
                {
                    self.set_value(*dst, class, changed);
                    return None;
                }
                let mut class = None;
                let mut saw_unknown = false;
                for (_, value) in inputs {
                    match self.values.get(value).copied() {
                        Some(input_class) => {
                            class = Some(match class {
                                None => input_class,
                                Some(existing) => {
                                    merge_inspector_value_class(existing, input_class)
                                }
                            });
                        }
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
            MirInstruction::BinOp { dst, lhs, rhs, .. } => {
                let lhs_class = self.value_class(*lhs);
                let rhs_class = self.value_class(*rhs);
                if lhs_class == InspectorValueClass::String
                    || rhs_class == InspectorValueClass::String
                {
                    self.set_value(*dst, InspectorValueClass::String, changed);
                } else {
                    self.set_value(*dst, InspectorValueClass::Scalar, changed);
                }
                None
            }
            MirInstruction::Compare { dst, .. } => {
                self.set_value(*dst, InspectorValueClass::Bool, changed);
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
                if self.accepts_indexof_method(box_name, method, *receiver, args) {
                    if let Some(dst) = dst {
                        self.set_value(*dst, InspectorValueClass::Scalar, changed);
                    }
                    self.saw_indexof_probe = true;
                    return None;
                }
                if self.accepts_map_set_method(box_name, method, *receiver, args) {
                    if let Some(dst) = dst {
                        self.set_value(*dst, InspectorValueClass::Scalar, changed);
                    }
                    self.saw_map_set = true;
                    return None;
                }
                if *changed {
                    None
                } else {
                    Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedMethodCall,
                    ))
                }
            }
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Extern(name)),
                args,
                ..
            } => {
                if classify_extern_call_route(name, args.len()) == Some(ExternCallRouteKind::EnvGet)
                    && args
                        .first()
                        .is_some_and(|arg| self.value_class(*arg) == InspectorValueClass::String)
                {
                    if let Some(dst) = dst {
                        self.set_value(*dst, InspectorValueClass::String, changed);
                    }
                    return None;
                }
                if *changed {
                    None
                } else {
                    Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedExternCall,
                    ))
                }
            }
            MirInstruction::Call {
                dst,
                callee: Some(Callee::Global(name)),
                args,
                ..
            } if name == "print" && args.len() == 1 => {
                if let Some(dst) = dst {
                    self.set_value(*dst, InspectorValueClass::Scalar, changed);
                }
                None
            }
            MirInstruction::Return { value: Some(value) } => {
                if self.value_class(*value) == InspectorValueClass::Map {
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
            MirInstruction::Return { value: None } => Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringReturnNotString,
            )),
            MirInstruction::Branch { condition, .. } => {
                if matches!(
                    self.value_class(*condition),
                    InspectorValueClass::Bool | InspectorValueClass::Scalar
                ) || *changed
                {
                    None
                } else {
                    Some(GenericPureStringReject::new(
                        GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
                    ))
                }
            }
            MirInstruction::Jump { .. }
            | MirInstruction::KeepAlive { .. }
            | MirInstruction::ReleaseStrong { .. } => None,
            _ => Some(GenericPureStringReject::new(
                GlobalCallTargetShapeReason::GenericStringUnsupportedInstruction,
            )),
        }
    }

    fn accepts(&self) -> bool {
        self.saw_map_birth
            && self.saw_kind_key
            && self.saw_is_map_key
            && self.saw_is_array_key
            && self.saw_box_probe_const
            && self.saw_indexof_probe
            && self.saw_map_set
            && self.returned_map
    }

    fn observe_string_const(&mut self, text: &str) {
        match text {
            "kind" => self.saw_kind_key = true,
            "is_map" => self.saw_is_map_key = true,
            "is_array" => self.saw_is_array_key = true,
            "MapBox(" | "ArrayBox(" | "StringBox(" | "IntegerBox(" | "{" | "[" => {
                self.saw_box_probe_const = true;
            }
            _ => {}
        }
    }

    fn accepts_indexof_method(
        &self,
        box_name: &str,
        method: &str,
        receiver: ValueId,
        args: &[ValueId],
    ) -> bool {
        if !matches!(box_name, "RuntimeDataBox" | "StringBox")
            || method != "indexOf"
            || !matches!(args.len(), 1 | 2)
        {
            return false;
        }
        if self.value_class(receiver) != InspectorValueClass::String {
            return false;
        }
        self.value_class(args[0]) == InspectorValueClass::String
            && (args.len() == 1 || self.value_class(args[1]) == InspectorValueClass::Scalar)
    }

    fn accepts_map_set_method(
        &self,
        box_name: &str,
        method: &str,
        receiver: ValueId,
        args: &[ValueId],
    ) -> bool {
        if !matches!(box_name, "RuntimeDataBox" | "MapBox") || method != "set" {
            return false;
        }
        if self.value_class(receiver) != InspectorValueClass::Map {
            return false;
        }
        let (key, value) = match args {
            [key, value] => (*key, *value),
            [receiver_arg, key, value]
                if self.value_class(*receiver_arg) == InspectorValueClass::Map =>
            {
                (*key, *value)
            }
            _ => return false,
        };
        self.value_class(key) == InspectorValueClass::String
            && matches!(
                self.value_class(value),
                InspectorValueClass::String
                    | InspectorValueClass::Scalar
                    | InspectorValueClass::Bool
                    | InspectorValueClass::UnknownHandle
                    | InspectorValueClass::Void
            )
    }

    fn value_class(&self, value: ValueId) -> InspectorValueClass {
        self.values
            .get(&value)
            .copied()
            .unwrap_or(InspectorValueClass::UnknownHandle)
    }

    fn set_value(&mut self, value: ValueId, class: InspectorValueClass, changed: &mut bool) {
        let class = merge_inspector_value_class(self.value_class(value), class);
        if self.values.get(&value) != Some(&class) {
            self.values.insert(value, class);
            *changed = true;
        }
    }
}

fn merge_inspector_value_class(
    lhs: InspectorValueClass,
    rhs: InspectorValueClass,
) -> InspectorValueClass {
    match (lhs, rhs) {
        (InspectorValueClass::UnknownHandle, class)
        | (class, InspectorValueClass::UnknownHandle) => class,
        (InspectorValueClass::Bool, InspectorValueClass::Scalar)
        | (InspectorValueClass::Scalar, InspectorValueClass::Bool) => InspectorValueClass::Scalar,
        (left, right) if left == right => left,
        _ => InspectorValueClass::UnknownHandle,
    }
}

fn inspector_value_class_from_type_hint(ty: &MirType) -> Option<InspectorValueClass> {
    match ty {
        MirType::Integer => Some(InspectorValueClass::Scalar),
        MirType::Bool => Some(InspectorValueClass::Bool),
        MirType::String => Some(InspectorValueClass::String),
        MirType::Void => Some(InspectorValueClass::Void),
        MirType::Box(name) if name == "MapBox" => Some(InspectorValueClass::Map),
        MirType::Box(name) if name == "StringBox" => Some(InspectorValueClass::String),
        _ => None,
    }
}
