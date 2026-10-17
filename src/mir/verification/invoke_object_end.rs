//! Structural validation of the retained End body; source correspondence and
//! permission remain with the original lifecycle owner and finished bindings.
use crate::mir::function::ObjectDestructionDispositionV1 as Destruction;
use crate::mir::instruction::ObjectEndStepV1 as Step;
use crate::mir::MirModule;
use hakorune_mir_defs::CanonicalObjectIdV1;
use std::collections::BTreeSet;

fn releasable(module: &MirModule, object: CanonicalObjectIdV1) -> bool {
    module
        .canonical_object_definition(object)
        .is_some_and(|row| {
            matches!(
                row.destruction_disposition(),
                Destruction::PlainI64NoHook
                    | Destruction::OwnedArrayFieldsNoHook
                    | Destruction::OwnedObjectFieldsNoHook
            )
        })
}

pub(super) fn check(module: &MirModule, object: CanonicalObjectIdV1, end: &[Step]) -> bool {
    if !releasable(module, object) || !matches!(end.last(), Some(Step::ReleaseOuter)) {
        return false;
    }
    let disposition = module
        .canonical_object_definition(object)
        .unwrap()
        .destruction_disposition();
    let mut seen = BTreeSet::new();
    let mut previous = None;
    for step in &end[..end.len() - 1] {
        let field = match step {
            Step::ArrayField { field } | Step::ObjectField { field, .. } => *field,
            Step::ReleaseOuter => return false,
        };
        if field.object() != object || !seen.insert(field) {
            return false;
        }
        if previous.is_some_and(|ordinal| ordinal <= field.declaration_ordinal()) {
            return false;
        }
        previous = Some(field.declaration_ordinal());
        let Some(definition) = module.canonical_field_definition(field) else {
            return false;
        };
        if definition.is_weak {
            return false;
        }
        match step {
            Step::ArrayField { .. } => {
                if definition.declared_type_name.as_deref() != Some("ArrayBox")
                    || !matches!(
                        disposition,
                        Destruction::OwnedArrayFieldsNoHook | Destruction::OwnedObjectFieldsNoHook
                    )
                {
                    return false;
                }
            }
            Step::ObjectField { child, .. } => {
                let declared = definition.declared_type_name.as_deref().and_then(|name| {
                    module
                        .metadata
                        .canonical_object_membership
                        .as_ref()
                        .and_then(|members| members.get(name))
                });
                if declared != Some(child)
                    || !releasable(module, *child)
                    || disposition != Destruction::OwnedObjectFieldsNoHook
                {
                    return false;
                }
            }
            Step::ReleaseOuter => unreachable!("only the last step releases the outer object"),
        }
    }
    true
}
