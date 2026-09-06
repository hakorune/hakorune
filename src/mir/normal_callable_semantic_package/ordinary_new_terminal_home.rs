//! Borrow-only source join for terminal Home field reads.
//! The parent owns candidate selection; no definition or field facts escape.
use super::*;

pub(super) fn field_is_initialized_integer(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &[(OwnedExprSiteV1, Box<str>, usize, BindingRefV1, SourceBindingSiteV1, bool)],
    selected: &BTreeMap<OwnedExprSiteV1, BindingRefV1>,
    home: BindingRefV1,
    field: &str,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let mut matching = candidates.iter().filter(|(site, _, _, binding, _, _)|
        *binding == home && selected.contains_key(site));
    let Some((site, class, arity, _, _, overrides)) = matching.next()
        else { return Ok(false); };
    if matching.next().is_some() {
        return Err(OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site: site.clone() });
    }
    if *overrides { return Ok(false); }
    let source = batch.ordinary_box_coverage().row_for(class.as_ref())
        .map_err(|_| OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageDuplicate {
            site: site.clone(), class: class.clone(),
        })?.ok_or_else(|| OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageMissing {
            site: site.clone(), class: class.clone(),
        })?;
    let lookup_error = |error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
        site: site.clone(), class: class.clone(), error,
    };
    // A declared type alone does not prove initialization on New's Normal edge.
    let Ok(plan) = constructors.construction_for(source, *arity)
        .map_err(lookup_error)? else { return Ok(false); };
    constructors.with_source_object_definition(source, |object, definition| {
        if plan.object() != object {
            return Err(lookup_error(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch));
        }
        let mut fields = definition.fields().iter().filter(|row| row.name == field);
        let Some(declaration) = fields.next() else { return Ok(false); };
        Ok(fields.next().is_none() && !declaration.is_weak
            && declaration.declared_type_name.as_deref() == Some("i64"))
    }).map_err(lookup_error)?
}
