use std::collections::BTreeMap;

use super::{
    BasicBlockId, MirType, PhiConcreteTypeConflictV1, PhiTransientTypeDecisionV1,
    PhiTypeFactSiteV1, PhiTypeNoPublicationReasonV1, PreparedPhiTypePublicationV1, ValueId,
};

struct ExactFactRefV1<'a> {
    site: PhiTypeFactSiteV1,
    ty: &'a MirType,
}

/// Decide one destination-type publication from logical PHI inputs.
///
/// Missing entries and `Unknown` are non-facts. Every other `MirType`,
/// including `Void`, is exact. Existing destination and explicit hint facts
/// constrain known incoming facts but never manufacture a candidate.
impl PhiTransientTypeDecisionV1 {
    pub(in crate::mir::builder) fn prepare(
        dst: ValueId,
        logical_inputs: &[(BasicBlockId, ValueId)],
        value_types: &BTreeMap<ValueId, MirType>,
        existing_destination: Option<&MirType>,
        type_hint: Option<&MirType>,
    ) -> Result<PreparedPhiTypePublicationV1, PhiConcreteTypeConflictV1> {
        decide_phi_transient_type_v1(
            dst,
            logical_inputs,
            value_types,
            existing_destination,
            type_hint,
        )
    }
}

fn decide_phi_transient_type_v1(
    dst: ValueId,
    logical_inputs: &[(BasicBlockId, ValueId)],
    value_types: &BTreeMap<ValueId, MirType>,
    existing_destination: Option<&MirType>,
    type_hint: Option<&MirType>,
) -> Result<PreparedPhiTypePublicationV1, PhiConcreteTypeConflictV1> {
    let existing = exact_fact(PhiTypeFactSiteV1::ExistingDestination, existing_destination);
    let hint = exact_fact(PhiTypeFactSiteV1::ExplicitTypeHint, type_hint);

    if let (Some(first), Some(second)) = (&existing, &hint) {
        require_equal(dst, first, second)?;
    }

    let mut ordered_inputs = logical_inputs.to_vec();
    ordered_inputs.sort_by_key(|(predecessor, value)| (*predecessor, *value));

    for (predecessor, value) in &ordered_inputs {
        let Some(incoming_type) = value_types.get(value) else {
            continue;
        };
        let Some(incoming) = exact_fact(
            PhiTypeFactSiteV1::Incoming {
                predecessor: *predecessor,
                value: *value,
            },
            Some(incoming_type),
        ) else {
            continue;
        };
        if let Some(constraint) = &existing {
            require_equal(dst, constraint, &incoming)?;
        }
        if let Some(constraint) = &hint {
            require_equal(dst, constraint, &incoming)?;
        }
    }

    if let Some(reason) = classify_no_publication(&ordered_inputs, value_types) {
        return Ok(match existing {
            Some(existing) => PreparedPhiTypePublicationV1::PreserveExisting {
                existing: existing.ty.clone(),
                reason,
            },
            None => PreparedPhiTypePublicationV1::NoPublication(reason),
        });
    }

    let candidate = value_types
        .get(&ordered_inputs[0].1)
        .expect("complete PHI input classification must retain the first type")
        .clone();

    Ok(match existing {
        Some(existing) => PreparedPhiTypePublicationV1::Idempotent(existing.ty.clone()),
        None => PreparedPhiTypePublicationV1::Publish(candidate),
    })
}

fn exact_fact<'a>(site: PhiTypeFactSiteV1, ty: Option<&'a MirType>) -> Option<ExactFactRefV1<'a>> {
    match ty {
        Some(MirType::Unknown) | None => None,
        Some(ty) => Some(ExactFactRefV1 { site, ty }),
    }
}

fn require_equal(
    dst: ValueId,
    first: &ExactFactRefV1<'_>,
    second: &ExactFactRefV1<'_>,
) -> Result<(), PhiConcreteTypeConflictV1> {
    if first.ty == second.ty {
        return Ok(());
    }
    Err(PhiConcreteTypeConflictV1 {
        dst,
        first_site: first.site.clone(),
        first_type: first.ty.clone(),
        second_site: second.site.clone(),
        second_type: second.ty.clone(),
    })
}

fn classify_no_publication(
    ordered_inputs: &[(BasicBlockId, ValueId)],
    value_types: &BTreeMap<ValueId, MirType>,
) -> Option<PhiTypeNoPublicationReasonV1> {
    if ordered_inputs.is_empty() {
        return Some(PhiTypeNoPublicationReasonV1::EmptyInputs);
    }

    if let Some((predecessor, value)) = ordered_inputs
        .iter()
        .find(|(_, value)| !value_types.contains_key(value))
    {
        return Some(PhiTypeNoPublicationReasonV1::MissingInputType {
            predecessor: *predecessor,
            value: *value,
        });
    }

    if let Some((predecessor, value)) = ordered_inputs
        .iter()
        .find(|(_, value)| value_types.get(value) == Some(&MirType::Unknown))
    {
        return Some(PhiTypeNoPublicationReasonV1::UnknownInputType {
            predecessor: *predecessor,
            value: *value,
        });
    }

    let first = value_types
        .get(&ordered_inputs[0].1)
        .expect("complete PHI input classification must retain the first type");
    if ordered_inputs
        .iter()
        .skip(1)
        .any(|(_, value)| value_types.get(value) != Some(first))
    {
        return Some(PhiTypeNoPublicationReasonV1::HeterogeneousInputTypes);
    }

    None
}
