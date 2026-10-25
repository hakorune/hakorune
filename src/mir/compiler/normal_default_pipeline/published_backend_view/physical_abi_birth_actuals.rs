//! Final Birth argument lane projection from the same sealed source proofs.
use super::*;

#[derive(Debug, Clone, Copy)]
pub(in crate::mir) enum ObjectBirthActualTransportV1 { Null, Handle, NullableHandle, MixedHandle }
impl ObjectBirthActualTransportV1 {
    pub(in crate::mir) fn wire_kind(self) -> serde_json::Value {
        match self {
            Self::Null => serde_json::json!(0),
            Self::Handle => serde_json::json!(3),
            Self::NullableHandle | Self::MixedHandle => serde_json::json!("nullable_typed_object"),
        }
    }
}

impl PublishedMirBackendView<'_> {
    /// The finalized draft admits a dominated `new`-argument lane per
    /// exact (site, ordinal): that emitted `Handle{binding}` actual is the
    /// same lent Integer view and is transported as `"tagged"`. An
    /// `ObjectFieldStores` formal instead admits its nullable pair lane —
    /// `Null` as (0,0) or an owned `Handle` actual as the (3,payload)
    /// lease move. Every other actual must still prove its scalar lane
    /// here — non-admitted actual kinds fail closed.
    pub(super) fn issue_tagged_birth_actuals(
        &self,
        entry: &CompiledEntryContractV1<'_>,
    ) -> Result<(
        BTreeSet<(OwnedExprSiteV1, u32)>,
        BTreeMap<(OwnedExprSiteV1, u32), ObjectBirthActualTransportV1>,
    ), String> {
        use crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1 as Kind;
        let mut tagged = BTreeSet::new();
        let mut object = BTreeMap::new();
        for call in entry.birth_calls() {
            let caller = entry
                .program()
                .functions()
                .get(call.caller_function_index() as usize)
                .ok_or_else(|| fault("birth-caller-missing"))?;
            let callee = entry.program().functions().get(call.function_index() as usize)
                .ok_or_else(|| fault("birth-actual-formal-missing"))?;
            let super::super::physical_program::PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } = callee.role()
                else { return Err(fault("birth-actual-formal-missing")) };
            if let Some(source) = entry.program().handoff().root_source() {
                let function = self.module().functions.get(caller.name()).ok_or_else(|| fault("birth-caller-draft-missing"))?;
                source.validate_birth_text_actuals_v1(abi, call.actual(), function)?;
            }
            let uses = entry
                .program()
                .handoff()
                .root_source()
                .map(|source| {
                    // Callers outside the closed borrowed-entry profile
                    // cannot carry admitted `new`-argument uses; asking for
                    // their projection would demand entry values that were
                    // never recorded.
                    if !source.has_borrowed_ordinary_entry_v1(call.actual().owner()) {
                        return Ok(Vec::new().into_boxed_slice());
                    }
                    let caller_function = self
                        .module()
                        .functions
                        .get(caller.name())
                        .ok_or_else(|| fault("birth-caller-draft-missing"))?;
                    source.borrowed_ordinary_new_argument_uses_v1(
                        call.actual().owner(),
                        caller_function,
                    )
                })
                .transpose()?
                .unwrap_or_default();
            for actual in call.actual().arguments() {
                if matches!(actual.payload(), crate::mir::instruction::BirthArgumentV1::Text(_)) {
                    if abi.text_formal_source_v1(actual.source().ordinal())?.is_none()
                        || entry.program().handoff().root_source().is_none() {
                        return Err(fault("actual-kind-unavailable"));
                    }
                    continue;
                }
                let value = actual.value_only_v1()?;
                // The bound formal's contract is the sole capability
                // authority: an object-field formal admits the nullable
                // pair lane — `Null` or an owned `Handle` actual whose
                // lease moves at the call edge; a scalar or unused formal
                // admits the scalar kinds only.
                let birth_abi = match callee.role() {
                    super::super::physical_program::PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } => abi,
                    _ => return Err(fault("birth-actual-formal-missing")),
                };
                let object_formal = entry
                    .births()
                    .iter()
                    .find(|birth| birth.function_index() == call.function_index())
                    .and_then(|birth| {
                        birth.formals().iter().find(|formal| {
                            formal.source_ordinal() == Some(actual.source().ordinal())
                        })
                    })
                    .ok_or_else(|| fault("birth-actual-formal-missing"))?
                    .contract()
                    .is_some_and(|contract| {
                        matches!(
                            contract.uses(),
                            crate::mir::normal_callable_semantic_package
                                ::BirthFormalUseCoverageV1::ObjectFieldStores { .. }
                        )
                    }) || birth_abi.dynamic_object_formal_source_v1(actual.source().ordinal())?.is_some();
                let finite = if let Some(source) = entry.program().handoff().root_source() {
                    let function = self.module().functions.get(caller.name()).ok_or_else(|| fault("birth-caller-draft-missing"))?;
                    source.finite_entry_birth_argument_v1(call.actual(), actual.source().ordinal(), function)?.is_some()
                } else { false };
                let admitted = finite || uses.iter().any(|(_, formal, site, ordinal)| {
                    *ordinal == actual.source().ordinal() && site == actual.source().new_site()
                        && matches!(actual.source().kind(), Kind::Handle { binding } if binding == formal)
                });
                if admitted && !object_formal {
                    if !tagged.insert((actual.source().new_site().clone(), actual.source().ordinal())) {
                        return Err(fault("birth-actual-tagged-duplicate"));
                    }
                    continue;
                }
                if birth_abi.dynamic_i64_formal_source_v1(actual.source().ordinal())? {
                    if object_formal || scalar_actual_kind(
                        actual.source().kind(), value, caller.value_types(),
                    )? != 1 {
                        return Err(fault("actual-kind-unavailable"));
                    }
                    continue;
                }
                let transport = match actual.source().kind() {
                    Kind::Null => Some(ObjectBirthActualTransportV1::Null),
                    Kind::Handle { .. } => Some(ObjectBirthActualTransportV1::Handle),
                    Kind::BoundValue { .. } if object_formal => {
                        let source = entry.program().handoff().root_source()
                            .ok_or_else(|| fault("received-birth-source-missing"))?;
                        let function = self.module().functions.get(caller.name())
                            .ok_or_else(|| fault("received-birth-caller-missing"))?;
                        match source.received_birth_actual_result_v1(call.actual(), actual.source().ordinal(), function)? {
                            Some(crate::mir::instruction::InvokeCallResultKind::Handle) =>
                                Some(ObjectBirthActualTransportV1::Handle),
                            Some(crate::mir::instruction::InvokeCallResultKind::NullableHandle) =>
                                Some(ObjectBirthActualTransportV1::NullableHandle),
                            Some(crate::mir::instruction::InvokeCallResultKind::MixedHandle { .. }) =>
                                Some(ObjectBirthActualTransportV1::MixedHandle),
                            _ => return Err(fault("received-birth-source-unavailable")),
                        }
                    }
                    _ => None,
                };
                let object_actual = transport.is_some();
                let contextualize = |error: String, branch: &str| format!(
                    "{error} branch={branch} caller={} caller_index={} callee_index={} site={:?} ordinal={} kind={:?} value={:?} type={:?} object_formal={object_formal} object_actual={object_actual} admitted={admitted}",
                    caller.name(), call.caller_function_index(), call.function_index(),
                    actual.source().new_site(), actual.source().ordinal(),
                    actual.source().kind(), value, caller.value_types().get(&value),
                );
                if object_formal != object_actual {
                    return Err(contextualize(fault("actual-kind-unavailable"), "object-formal-mismatch"));
                }
                if object_formal {
                    // The wire kind is selected at transport time from the
                    // sealed source kind — `Null` (0,0) or `Handle` (3,
                    // payload). Lease ownership stays unproven here: the
                    // flow layer admits only a live owned handle for tag 3.
                    if object.insert((
                        actual.source().new_site().clone(), actual.source().ordinal(),
                    ), transport.ok_or_else(|| fault("object-transport-missing"))?).is_some() {
                        return Err(fault("birth-actual-tagged-duplicate"));
                    }
                    continue;
                }
                scalar_actual_kind(actual.source().kind(), value, caller.value_types())
                    .map_err(|error| contextualize(error, "scalar-kind"))?;
            }
        }
        Ok((tagged, object))
    }
}
