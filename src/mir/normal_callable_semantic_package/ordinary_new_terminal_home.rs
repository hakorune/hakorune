//! Borrow-only source join for terminal Home field reads.
//! The parent owns candidate selection; no definition or field facts escape.
use super::*;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use hakorune_mir_defs::SameModuleCallableNamespaceV1;

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

/// Entry-loan receiver proof shared by the readiness probe and the
/// verified walk: the declaration's own box source row, resolved through
/// the selected callable's ordinary-box coverage. `None` when the owner
/// is not an instance-box method or carries no entry loan.
pub(super) fn entry_receiver_box_proof<'a>(
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    batch: &'a crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    entry_home: Option<&crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    batch_slot: u32,
) -> Option<(BindingRefV1, &'a crate::parser::ParserOrdinaryBoxSourceRowV1)> {
    let owner_box = selected
        .keys()
        .filter_map(|selected_key| {
            let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                return None;
            };
            (selected.batch_slot(selected_key) == Some(batch_slot)
                && key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod)
            .then(|| key.owner())
        })
        .next()?;
    entry_home.and_then(|loan| {
        batch
            .ordinary_box_coverage()
            .row_for(owner_box)
            .ok()
            .flatten()
            .map(|row| (loan.receiver(), row))
    })
}

/// Argument-position `receiver.field` proof for one selected `new` site.
///
/// Two disjoint families admit a read:
/// - the receiver's handle root is a claim-local selected `new` Home —
///   the existing `initialized_integer_field` provenance decides;
/// - the root is the sole entry loan's receiver binding — the field is
///   proven `i64` on this declaration's own box source definition.
///
/// Anything else returns `None`; the scanner names that row
/// `ArgumentNotTrivial`. No MIR type or runtime layout participates.
pub(super) fn argument_integer_field(
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &[OrdinaryNewCandidate],
    site: &OwnedExprSiteV1,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    home: BindingRefV1,
    field: &str,
) -> Result<Option<hakorune_mir_defs::CanonicalFieldRefV1>, OrdinaryNewCoSealIssueV1> {
    if let Some(field) = initialized_integer_field(constructors, candidates, home, field)? {
        return Ok(Some(field));
    }
    let Some((receiver, box_source)) = receiver else {
        return Ok(None);
    };
    // The receiver arm admits only the entry loan's exact receiver root:
    // the sole Home ABI issuer bound `me` to this declaration's box.
    if home != receiver || receiver.owner() != site.owner() {
        return Ok(None);
    }
    let class: Box<str> = box_source.name().into();
    let lookup_error = |error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
        site: site.clone(),
        class: class.clone(),
        error,
    };
    constructors
        .with_source_object_definition(box_source, |object, definition| {
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
