//! Structural validation of the SAME complete End body; source correspondence
//! and permission remain with the lifecycle owner and finished bindings.
use crate::mir::instruction::{ObjectEndStepV1 as Step, FieldResidenceEndModeV1 as Mode};
use crate::mir::MirModule;
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};

fn releasable(module: &MirModule, object: CanonicalObjectIdV1) -> bool {
    module.canonical_object_definition(object)
        .is_some_and(|row| row.destruction_disposition().has_no_hook())
}

pub(super) fn check(module: &MirModule, object: CanonicalObjectIdV1, end: &[Step]) -> bool {
    if !releasable(module, object) { return false; }
    let definition = module.canonical_object_definition(object).expect("checked above");
    let mut required = Vec::new();
    for (ordinal, row) in definition.fields().iter().enumerate() {
        if row.is_weak { continue; }
        let Some(field) = CanonicalFieldRefV1::from_declaration_ordinal(object, ordinal) else { return false; };
        let step = if row.declared_type_name.is_none() {
            if !super::dynamic_field(module, field) { return false; }
            Step::FieldResidence { field, mode: Mode::DynamicResidence }
        } else if module.canonical_array_field(field) {
            Step::FieldResidence { field, mode: Mode::ArrayResidence }
        } else if row.declared_type_name.as_deref().and_then(
            crate::mir::declared_type_storage::exact_numeric_storage_for_declared_type).is_some() {
            continue;
        } else {
            let child = row.declared_type_name.as_deref().and_then(|name|
                module.metadata.canonical_object_membership.as_ref().and_then(|members| members.get(name)));
            let Some(child) = child.copied().filter(|child| releasable(module, *child)) else { return false; };
            Step::ObjectField { field, child }
        };
        required.push(step);
    }
    required.reverse();
    required.push(Step::ReleaseOuter);
    // A valid subset is not a complete End. Equality also retains the original
    // canonical fields, modes, reverse order, cardinality and outer-last law.
    end == required
}
