use std::collections::BTreeSet;

use super::CallableResultCatalogErrorV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;

pub(super) type RequirementSetV1 = BTreeSet<u32>;

pub(super) fn seal_requirements(
    key: &CanonicalSameModuleCallableKeyV1,
    requirements: impl IntoIterator<Item = u32>,
) -> Result<Box<[u32]>, CallableResultCatalogErrorV1> {
    let requirements = requirements.into_iter().collect::<BTreeSet<_>>();
    if let Some(ordinal) = requirements
        .iter()
        .copied()
        .find(|ordinal| *ordinal >= key.arity())
    {
        return Err(
            CallableResultCatalogErrorV1::RequiredArgumentOrdinalOutOfRange {
                key: key.clone(),
                ordinal,
                arity: key.arity(),
            },
        );
    }
    Ok(requirements
        .into_iter()
        .collect::<Vec<_>>()
        .into_boxed_slice())
}

pub(super) fn union_requirements(
    left: &RequirementSetV1,
    right: &RequirementSetV1,
) -> RequirementSetV1 {
    left.union(right).copied().collect()
}
