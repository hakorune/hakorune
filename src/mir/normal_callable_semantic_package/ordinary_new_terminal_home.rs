//! Borrow-only source join for terminal Home field reads.
//! The parent owns candidate selection; no definition or field facts escape.
use super::*;

pub(super) fn initialized_integer_field(
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &[OrdinaryNewCandidate],
    home: BindingRefV1,
    field: &str,
) -> Result<Option<hakorune_mir_defs::CanonicalFieldRefV1>, OrdinaryNewCoSealIssueV1> {
    let mut matching = candidates
        .iter()
        .filter(|candidate| candidate.destination == home);
    let Some(candidate) = matching.next() else {
        return Ok(None);
    };
    if matching.next().is_some() {
        return Err(OrdinaryNewCoSealIssueV1::InitializerBindingMismatch {
            site: candidate.site.clone(),
        });
    }
    // Unavailable construction (including overrides) remains a retained descriptor.
    let Ok(plan) = &candidate.construction else {
        return Ok(None);
    };
    let lookup_error = |error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
        site: candidate.site.clone(),
        class: candidate.class.clone(),
        error,
    };
    constructors
        .with_source_object_definition(&candidate.box_source, |object, definition| {
            if plan.object() != object {
                return Err(lookup_error(
                    InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch,
                ));
            }
            let mut fields = definition
                .fields()
                .iter()
                .enumerate()
                .filter(|(_, row)| row.name == field);
            let Some((ordinal, declaration)) = fields.next() else {
                return Ok(None);
            };
            if fields.next().is_some()
                || declaration.is_weak
                || declaration.declared_type_name.as_deref() != Some("i64")
            {
                return Ok(None);
            }
            hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, ordinal)
                .map(Some)
                .ok_or_else(|| {
                    lookup_error(InstanceConstructorBirthLookupErrorV1::ObjectDefinitionMissing)
                })
        })
        .map_err(lookup_error)?
}
