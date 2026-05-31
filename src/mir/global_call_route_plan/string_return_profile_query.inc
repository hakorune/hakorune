#[cfg(test)]
pub(super) fn generic_string_void_sentinel_return_global_blocker(
    function: &MirFunction,
    targets: &BTreeMap<String, GlobalCallTargetFacts>,
) -> Option<GenericPureStringReject> {
    let profile = build_generic_string_return_profile(function, targets);
    generic_string_void_sentinel_return_global_blocker_from_profile(function, &profile)
}

pub(super) fn generic_string_void_sentinel_return_global_blocker_from_profile(
    function: &MirFunction,
    profile: &GenericStringReturnProfile,
) -> Option<GenericPureStringReject> {
    let mut saw_void = false;
    let mut return_blocker = None;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Return { value: Some(value) } => {
                    let class = generic_string_return_value_class(&profile.values, *value);
                    if let Some(blocker) = profile.blockers.get(value).cloned() {
                        if return_blocker.is_none() {
                            return_blocker = Some(blocker);
                        }
                    }
                    if class.is_void_like() {
                        saw_void = true;
                    }
                }
                MirInstruction::Return { value: None } => saw_void = true,
                _ => {}
            }
        }
    }

    if saw_void {
        return_blocker.map(|blocker| {
            GenericPureStringReject::with_shape_blocker(
                GlobalCallTargetShapeReason::GenericStringGlobalTargetShapeUnknown,
                blocker,
            )
        })
    } else {
        None
    }
}

pub(super) fn generic_string_return_object_boundary_candidate_from_profile(
    function: &MirFunction,
    profile: &GenericStringReturnProfile,
) -> bool {
    function.blocks.values().any(|block| {
        block
            .instructions
            .iter()
            .chain(block.terminator.iter())
            .any(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::Return { value: Some(value) }
                        if generic_string_return_value_class(&profile.values, *value)
                            == GenericStringReturnValueClass::Object
                )
            })
    })
}

pub(super) fn generic_string_void_sentinel_return_candidate_from_profile(
    function: &MirFunction,
    profile: &GenericStringReturnProfile,
) -> bool {
    let mut saw_string = false;
    let mut saw_concrete_string = false;
    let mut saw_string_or_void_child = false;
    let mut saw_unknown_param_passthrough = false;
    let mut saw_void = false;
    for block in function.blocks.values() {
        for instruction in block.instructions.iter().chain(block.terminator.iter()) {
            match instruction {
                MirInstruction::Return { value: Some(value) } => {
                    match generic_string_return_value_class(&profile.values, *value) {
                        GenericStringReturnValueClass::String => {
                            saw_string = true;
                            saw_concrete_string = true;
                        }
                        GenericStringReturnValueClass::StringOrVoid => {
                            saw_string = true;
                            saw_string_or_void_child = true;
                            saw_void = true;
                        }
                        GenericStringReturnValueClass::Void => saw_void = true,
                        GenericStringReturnValueClass::Unknown
                            if profile.passthrough.contains(value) =>
                        {
                            saw_string = true;
                            saw_unknown_param_passthrough = true;
                        }
                        GenericStringReturnValueClass::Unknown
                        | GenericStringReturnValueClass::Object
                        | GenericStringReturnValueClass::Other => {
                            return false;
                        }
                    }
                }
                MirInstruction::Return { value: None } => saw_void = true,
                _ => {}
            }
        }
    }
    let requires_concrete_string = function.signature.return_type == MirType::Unknown
        || (saw_unknown_param_passthrough
            && generic_string_return_type_requires_concrete_unknown_passthrough(
                &function.signature.return_type,
            ));
    saw_string
        && saw_void
        && (!requires_concrete_string
            || saw_concrete_string
            || (saw_string_or_void_child && !saw_unknown_param_passthrough))
}

fn generic_string_return_value_class(
    values: &BTreeMap<ValueId, GenericStringReturnValueClass>,
    value: ValueId,
) -> GenericStringReturnValueClass {
    values
        .get(&value)
        .copied()
        .unwrap_or(GenericStringReturnValueClass::Unknown)
}

fn merge_generic_string_return_value_class(
    lhs: GenericStringReturnValueClass,
    rhs: GenericStringReturnValueClass,
) -> GenericStringReturnValueClass {
    match (lhs, rhs) {
        (GenericStringReturnValueClass::Unknown, class)
        | (class, GenericStringReturnValueClass::Unknown) => class,
        (same_lhs, same_rhs) if same_lhs == same_rhs => same_lhs,
        (GenericStringReturnValueClass::Object, _) | (_, GenericStringReturnValueClass::Object) => {
            GenericStringReturnValueClass::Object
        }
        (
            GenericStringReturnValueClass::String,
            GenericStringReturnValueClass::Void | GenericStringReturnValueClass::StringOrVoid,
        )
        | (
            GenericStringReturnValueClass::Void | GenericStringReturnValueClass::StringOrVoid,
            GenericStringReturnValueClass::String,
        )
        | (GenericStringReturnValueClass::Void, GenericStringReturnValueClass::StringOrVoid)
        | (GenericStringReturnValueClass::StringOrVoid, GenericStringReturnValueClass::Void) => {
            GenericStringReturnValueClass::StringOrVoid
        }
        _ => GenericStringReturnValueClass::Other,
    }
}

fn set_generic_string_return_value_class(
    values: &mut BTreeMap<ValueId, GenericStringReturnValueClass>,
    value: ValueId,
    class: GenericStringReturnValueClass,
    changed: &mut bool,
) {
    if class == GenericStringReturnValueClass::Unknown {
        return;
    }
    match values.get(&value).copied() {
        Some(existing) if existing == class => {}
        Some(GenericStringReturnValueClass::Unknown) | None => {
            values.insert(value, class);
            *changed = true;
        }
        Some(existing) => {
            let merged = merge_generic_string_return_value_class(existing, class);
            if merged != existing {
                values.insert(value, merged);
                *changed = true;
            }
        }
    }
}

fn set_generic_string_return_string_handle_value_class(
    values: &mut BTreeMap<ValueId, GenericStringReturnValueClass>,
    value: ValueId,
    changed: &mut bool,
) {
    match values.get(&value).copied() {
        Some(GenericStringReturnValueClass::String) => {}
        Some(GenericStringReturnValueClass::Unknown | GenericStringReturnValueClass::Other)
        | None => {
            values.insert(value, GenericStringReturnValueClass::String);
            *changed = true;
        }
        Some(existing) => {
            let merged = merge_generic_string_return_value_class(
                existing,
                GenericStringReturnValueClass::String,
            );
            if merged != existing {
                values.insert(value, merged);
                *changed = true;
            }
        }
    }
}

impl GenericStringReturnValueClass {
    fn is_void_like(self) -> bool {
        matches!(
            self,
            GenericStringReturnValueClass::Void | GenericStringReturnValueClass::StringOrVoid
        )
    }
}
