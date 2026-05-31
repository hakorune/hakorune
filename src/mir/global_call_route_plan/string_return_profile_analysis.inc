pub(super) fn build_generic_string_return_profile(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> GenericStringReturnProfile {
    let mut values = BTreeMap::<ValueId, GenericStringReturnValueClass>::new();
    let mut blockers = BTreeMap::<ValueId, GlobalCallShapeBlocker>::new();
    seed_generic_string_return_values(function, &mut values);

    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());
    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                refine_generic_string_return_value_class(
                    instruction,
                    targets,
                    &mut values,
                    &mut changed,
                );
                refine_generic_string_return_blocker(
                    instruction,
                    targets,
                    &mut blockers,
                    &mut changed,
                );
            }
        }
        if !changed {
            break;
        }
    }

    GenericStringReturnProfile {
        values,
        passthrough: unknown_param_passthrough_values(function),
        blockers,
    }
}

pub(super) fn generic_string_void_sentinel_profile_may_apply(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    if matches!(
        &function.signature.return_type,
        MirType::Unknown | MirType::Void | MirType::String
    ) || matches!(&function.signature.return_type, MirType::Box(name) if name == "StringBox")
    {
        return true;
    }
    if function.signature.params.iter().any(|ty| {
        matches!(ty, MirType::Unknown | MirType::String)
            || matches!(ty, MirType::Box(name) if name == "StringBox")
    }) {
        return true;
    }
    if function
        .metadata
        .string_corridor_facts
        .values()
        .any(|fact| {
            matches!(
                fact.op,
                StringCorridorOp::StrLen | StringCorridorOp::StrSlice | StringCorridorOp::FreezeStr
            )
        })
    {
        return true;
    }
    function.blocks.values().any(|block| {
        block
            .instructions
            .iter()
            .chain(block.terminator.iter())
            .any(|instruction| {
                generic_string_void_sentinel_instruction_may_apply(instruction, targets)
            })
    })
}

fn generic_string_void_sentinel_instruction_may_apply(
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> bool {
    match instruction {
        MirInstruction::Const {
            value: ConstValue::String(_) | ConstValue::Null | ConstValue::Void,
            ..
        } => true,
        MirInstruction::NewBox { box_type, .. } if box_type == "StringBox" => true,
        MirInstruction::Return { value: None } => true,
        MirInstruction::Call {
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if matches!(
            classify_extern_call_route(name, args.len()),
            Some(
                ExternCallRouteKind::EnvGet
                    | ExternCallRouteKind::Stage1EmitProgramJson
                    | ExternCallRouteKind::Stage1EmitMirFromSource
                    | ExternCallRouteKind::Stage1EmitMirFromProgramJson
                    | ExternCallRouteKind::HostBridgeExternInvoke
            )
        ) =>
        {
            true
        }
        MirInstruction::Call {
            callee: Some(Callee::Method {
                box_name, method, ..
            }),
            ..
        } if matches!(box_name.as_str(), "RuntimeDataBox" | "StringBox")
            && method == "substring" =>
        {
            true
        }
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            args,
            ..
        } if is_hostbridge_extern_invoke_symbol(name, args.len()) => true,
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } if !supported_backend_global(name) => lookup_global_call_target(name, targets)
            .map(|target| {
                matches!(
                    target.return_contract(),
                    Some(
                        GlobalCallReturnContract::StringHandle
                            | GlobalCallReturnContract::StringHandleOrNull
                            | GlobalCallReturnContract::VoidSentinelI64Zero
                    ) | None
                )
            })
            .unwrap_or(true),
        _ => false,
    }
}

fn seed_generic_string_return_values(
    function: &MirFunction,
    values: &mut BTreeMap<ValueId, GenericStringReturnValueClass>,
) {
    let mut changed = false;
    for (index, param) in function.params.iter().enumerate() {
        if let Some(class) = function
            .signature
            .params
            .get(index)
            .and_then(generic_string_return_value_class_from_type)
        {
            set_generic_string_return_value_class(values, *param, class, &mut changed);
        }
    }
    for (value, ty) in &function.metadata.value_types {
        if let Some(class) = generic_string_return_metadata_value_class_from_type(ty) {
            set_generic_string_return_value_class(values, *value, class, &mut changed);
        }
    }
    seed_generic_string_return_corridor_values(function, values, &mut changed);
}

fn seed_generic_string_return_corridor_values(
    function: &MirFunction,
    values: &mut BTreeMap<ValueId, GenericStringReturnValueClass>,
    changed: &mut bool,
) {
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            let MirInstruction::Call {
                dst: Some(dst),
                callee:
                    Some(Callee::Method {
                        receiver: Some(receiver),
                        ..
                    }),
                args,
                ..
            } = instruction
            else {
                continue;
            };
            let Some(fact) = function.metadata.string_corridor_facts.get(dst) else {
                continue;
            };
            match fact.op {
                StringCorridorOp::StrLen if args.is_empty() => {
                    set_generic_string_return_string_handle_value_class(values, *receiver, changed);
                    set_generic_string_return_value_class(
                        values,
                        *dst,
                        GenericStringReturnValueClass::Other,
                        changed,
                    );
                }
                StringCorridorOp::StrSlice if matches!(args.len(), 1 | 2) => {
                    set_generic_string_return_string_handle_value_class(values, *receiver, changed);
                    for arg in args {
                        set_generic_string_return_value_class(
                            values,
                            *arg,
                            GenericStringReturnValueClass::Other,
                            changed,
                        );
                    }
                    set_generic_string_return_string_handle_value_class(values, *dst, changed);
                }
                StringCorridorOp::FreezeStr => {}
                _ => {}
            }
        }
    }
}

fn generic_string_return_value_class_from_type(
    ty: &MirType,
) -> Option<GenericStringReturnValueClass> {
    match ty {
        MirType::String => Some(GenericStringReturnValueClass::String),
        MirType::Box(name) if name == "StringBox" => Some(GenericStringReturnValueClass::String),
        MirType::Box(_) => Some(GenericStringReturnValueClass::Object),
        MirType::Integer | MirType::Bool | MirType::Float => {
            Some(GenericStringReturnValueClass::Other)
        }
        MirType::Void => Some(GenericStringReturnValueClass::Void),
        _ => None,
    }
}

fn generic_string_return_metadata_value_class_from_type(
    ty: &MirType,
) -> Option<GenericStringReturnValueClass> {
    match ty {
        MirType::String => Some(GenericStringReturnValueClass::String),
        MirType::Box(name) if name == "StringBox" => Some(GenericStringReturnValueClass::String),
        MirType::Box(_) => Some(GenericStringReturnValueClass::Object),
        MirType::Void => Some(GenericStringReturnValueClass::Void),
        _ => None,
    }
}
fn unknown_param_passthrough_values(function: &MirFunction) -> BTreeSet<ValueId> {
    let mut values = BTreeSet::<ValueId>::new();
    if !generic_string_return_type_allows_unknown_param_passthrough(&function.signature.return_type)
    {
        return values;
    }
    for (index, param) in function.params.iter().enumerate() {
        if function.signature.params.get(index) == Some(&MirType::Unknown) {
            values.insert(*param);
        }
    }
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());
    for _ in 0..16 {
        let mut changed = false;
        for block_id in &block_ids {
            let Some(block) = function.blocks.get(block_id) else {
                continue;
            };
            for instruction in block.instructions.iter().chain(block.terminator.iter()) {
                match instruction {
                    MirInstruction::Copy { dst, src } if values.contains(src) => {
                        changed |= values.insert(*dst);
                    }
                    MirInstruction::Phi { dst, inputs, .. }
                        if inputs.iter().any(|(_, value)| values.contains(value)) =>
                    {
                        changed |= values.insert(*dst);
                    }
                    _ => {}
                }
            }
        }
        if !changed {
            break;
        }
    }
    values
}

fn generic_string_return_type_allows_unknown_param_passthrough(ty: &MirType) -> bool {
    matches!(ty, MirType::Void | MirType::Unknown | MirType::String)
        || matches!(ty, MirType::Box(name) if name == "StringBox")
}

fn generic_string_return_type_requires_concrete_unknown_passthrough(ty: &MirType) -> bool {
    matches!(ty, MirType::Unknown | MirType::String)
        || matches!(ty, MirType::Box(name) if name == "StringBox")
}

fn refine_generic_string_return_value_class(
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    values: &mut BTreeMap<ValueId, GenericStringReturnValueClass>,
    changed: &mut bool,
) {
    match instruction {
        MirInstruction::Const { dst, value } => {
            let class = match value {
                ConstValue::String(_) => GenericStringReturnValueClass::String,
                ConstValue::Null | ConstValue::Void => GenericStringReturnValueClass::Void,
                ConstValue::Integer(_) | ConstValue::Bool(_) => {
                    GenericStringReturnValueClass::Other
                }
                _ => GenericStringReturnValueClass::Unknown,
            };
            set_generic_string_return_value_class(values, *dst, class, changed);
        }
        MirInstruction::NewBox { dst, box_type, .. } => {
            let class = if box_type == "StringBox" {
                GenericStringReturnValueClass::String
            } else {
                GenericStringReturnValueClass::Object
            };
            set_generic_string_return_value_class(values, *dst, class, changed);
        }
        MirInstruction::Copy { dst, src } => {
            let class = generic_string_return_value_class(values, *src);
            if class != GenericStringReturnValueClass::Unknown {
                set_generic_string_return_value_class(values, *dst, class, changed);
            } else {
                let dst_class = generic_string_return_value_class(values, *dst);
                if dst_class != GenericStringReturnValueClass::Unknown {
                    set_generic_string_return_value_class(values, *src, dst_class, changed);
                }
            }
        }
        MirInstruction::Phi {
            dst,
            inputs,
            type_hint,
        } => {
            let mut class = GenericStringReturnValueClass::Unknown;
            let mut saw_unknown = false;
            let mut saw_string_like = false;
            let mut saw_other = false;
            let mut saw_void = false;
            let mut saw_non_string = false;
            let type_hint_class = type_hint
                .as_ref()
                .and_then(generic_string_return_value_class_from_type);
            for (_, value) in inputs {
                let input_class = generic_string_return_value_class(values, *value);
                if input_class == GenericStringReturnValueClass::Unknown {
                    saw_unknown = true;
                    continue;
                }
                saw_string_like |= matches!(
                    input_class,
                    GenericStringReturnValueClass::String
                        | GenericStringReturnValueClass::StringOrVoid
                );
                saw_non_string |= matches!(
                    input_class,
                    GenericStringReturnValueClass::Void | GenericStringReturnValueClass::Other
                );
                saw_other |= input_class == GenericStringReturnValueClass::Other;
                saw_void |= input_class == GenericStringReturnValueClass::Void;
                class = merge_generic_string_return_value_class(class, input_class);
            }
            if saw_unknown && saw_string_like && !saw_non_string {
                // Loop-carried string builders often carry raw i64 metadata for
                // handle values. Preserve the observed string base instead of
                // collapsing the return profile to an ABI-only blocker.
                set_generic_string_return_value_class(
                    values,
                    *dst,
                    GenericStringReturnValueClass::String,
                    changed,
                );
            } else if type_hint_class == Some(GenericStringReturnValueClass::Other)
                && !saw_string_like
                && !saw_void
            {
                set_generic_string_return_value_class(
                    values,
                    *dst,
                    GenericStringReturnValueClass::Other,
                    changed,
                );
            } else if saw_unknown && saw_other && !saw_string_like && !saw_void {
                // Loop-carried scalar indices commonly feed substring bounds in
                // string-or-void scanners. Keep them scalar in the return profile
                // instead of blocking the string return candidate.
                set_generic_string_return_value_class(
                    values,
                    *dst,
                    GenericStringReturnValueClass::Other,
                    changed,
                );
            } else if !saw_unknown {
                set_generic_string_return_value_class(values, *dst, class, changed);
            }
        }
        MirInstruction::BinOp {
            dst, op, lhs, rhs, ..
        } => {
            let lhs_class = generic_string_return_value_class(values, *lhs);
            let rhs_class = generic_string_return_value_class(values, *rhs);
            let class = if *op == BinaryOp::Add
                && (lhs_class == GenericStringReturnValueClass::String
                    || rhs_class == GenericStringReturnValueClass::String)
            {
                GenericStringReturnValueClass::String
            } else if lhs_class == GenericStringReturnValueClass::Unknown
                || rhs_class == GenericStringReturnValueClass::Unknown
            {
                GenericStringReturnValueClass::Unknown
            } else {
                GenericStringReturnValueClass::Other
            };
            if class == GenericStringReturnValueClass::String {
                set_generic_string_return_string_handle_value_class(values, *dst, changed);
            } else {
                set_generic_string_return_value_class(values, *dst, class, changed);
            }
        }
        MirInstruction::Compare {
            dst, op, lhs, rhs, ..
        } => {
            if generic_string_return_compare_proves_scalar(*op) {
                let lhs_class = generic_string_return_value_class(values, *lhs);
                let rhs_class = generic_string_return_value_class(values, *rhs);
                if lhs_class == GenericStringReturnValueClass::Unknown
                    && rhs_class == GenericStringReturnValueClass::Other
                {
                    set_generic_string_return_value_class(
                        values,
                        *lhs,
                        GenericStringReturnValueClass::Other,
                        changed,
                    );
                }
                if rhs_class == GenericStringReturnValueClass::Unknown
                    && lhs_class == GenericStringReturnValueClass::Other
                {
                    set_generic_string_return_value_class(
                        values,
                        *rhs,
                        GenericStringReturnValueClass::Other,
                        changed,
                    );
                }
            }
            set_generic_string_return_value_class(
                values,
                *dst,
                GenericStringReturnValueClass::Other,
                changed,
            );
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if matches!(
            classify_extern_call_route(name, args.len()),
            Some(
                ExternCallRouteKind::EnvGet
                    | ExternCallRouteKind::Stage1EmitProgramJson
                    | ExternCallRouteKind::Stage1EmitMirFromSource
                    | ExternCallRouteKind::Stage1EmitMirFromProgramJson
            )
        ) =>
        {
            if let Some(dst) = dst {
                set_generic_string_return_value_class(
                    values,
                    *dst,
                    GenericStringReturnValueClass::String,
                    changed,
                );
            }
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } if classify_extern_call_route(name, args.len())
            == Some(ExternCallRouteKind::HostBridgeExternInvoke) =>
        {
            if let Some(dst) = dst {
                set_generic_string_return_value_class(
                    values,
                    *dst,
                    GenericStringReturnValueClass::String,
                    changed,
                );
            }
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
            if let Some(dst) = dst {
                let receiver_class = generic_string_return_value_class(values, *receiver);
                if generic_string_return_accepts_substring_method(
                    box_name,
                    method,
                    args,
                    receiver_class,
                    values,
                ) {
                    set_generic_string_return_value_class(
                        values,
                        *dst,
                        GenericStringReturnValueClass::String,
                        changed,
                    );
                }
            }
        }
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Global(name)),
            args,
            ..
        } => {
            let class = if is_hostbridge_extern_invoke_symbol(name, args.len()) {
                GenericStringReturnValueClass::String
            } else {
                lookup_global_call_target(name, targets)
                    .map(|target| match target.return_contract() {
                        Some(GlobalCallReturnContract::StringHandle) => {
                            GenericStringReturnValueClass::String
                        }
                        Some(
                            GlobalCallReturnContract::ArrayHandle
                            | GlobalCallReturnContract::MapHandle
                            | GlobalCallReturnContract::ObjectHandle,
                        ) => GenericStringReturnValueClass::Object,
                        Some(GlobalCallReturnContract::StringHandleOrNull) => {
                            GenericStringReturnValueClass::StringOrVoid
                        }
                        Some(GlobalCallReturnContract::VoidSentinelI64Zero) => {
                            GenericStringReturnValueClass::Void
                        }
                        Some(
                            GlobalCallReturnContract::ScalarI64
                            | GlobalCallReturnContract::MixedRuntimeI64OrHandle,
                        ) => GenericStringReturnValueClass::Other,
                        None => GenericStringReturnValueClass::Unknown,
                    })
                    .unwrap_or(GenericStringReturnValueClass::Unknown)
            };
            if let Some(dst) = dst {
                set_generic_string_return_value_class(values, *dst, class, changed);
            }
        }
        _ => {}
    }
}

fn generic_string_return_accepts_substring_method(
    box_name: &str,
    method: &str,
    args: &[ValueId],
    receiver_class: GenericStringReturnValueClass,
    values: &BTreeMap<ValueId, GenericStringReturnValueClass>,
) -> bool {
    matches!(box_name, "RuntimeDataBox" | "StringBox")
        && method == "substring"
        && matches!(args.len(), 1 | 2)
        && receiver_class == GenericStringReturnValueClass::String
        && args.iter().all(|arg| {
            generic_string_return_value_class(values, *arg) == GenericStringReturnValueClass::Other
        })
}

fn generic_string_return_compare_proves_scalar(op: crate::mir::CompareOp) -> bool {
    !matches!(op, crate::mir::CompareOp::Eq | crate::mir::CompareOp::Ne)
}

fn refine_generic_string_return_blocker(
    instruction: &MirInstruction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
    blockers: &mut BTreeMap<ValueId, GlobalCallShapeBlocker>,
    changed: &mut bool,
) {
    match instruction {
        MirInstruction::Call {
            dst: Some(dst),
            callee: Some(Callee::Global(name)),
            ..
        } if !supported_backend_global(name) => {
            let blocker = match lookup_global_call_target(name, targets) {
                Some(target)
                    if target.shape() == GlobalCallTargetShape::Unknown
                        && target.return_contract().is_none() =>
                {
                    Some(propagated_unknown_global_target_blocker(name, target))
                }
                Some(_) => None,
                None => Some(GlobalCallShapeBlocker {
                    symbol: crate::mir::naming::normalize_static_global_name(name),
                    reason: Some(GlobalCallTargetShapeReason::GenericStringGlobalTargetMissing),
                }),
            };
            if let Some(blocker) = blocker {
                set_generic_string_return_blocker(blockers, *dst, blocker, changed);
            }
        }
        MirInstruction::Copy { dst, src } => {
            if let Some(blocker) = blockers.get(src).cloned() {
                set_generic_string_return_blocker(blockers, *dst, blocker, changed);
            }
        }
        MirInstruction::Phi { dst, inputs, .. } => {
            if let Some(blocker) = inputs
                .iter()
                .filter_map(|(_, value)| blockers.get(value).cloned())
                .next()
            {
                set_generic_string_return_blocker(blockers, *dst, blocker, changed);
            }
        }
        _ => {}
    }
}

fn set_generic_string_return_blocker(
    blockers: &mut BTreeMap<ValueId, GlobalCallShapeBlocker>,
    value: ValueId,
    blocker: GlobalCallShapeBlocker,
    changed: &mut bool,
) {
    match blockers.get(&value) {
        Some(existing) if *existing == blocker => {}
        Some(_) => {}
        None => {
            blockers.insert(value, blocker);
            *changed = true;
        }
    }
}
