use std::collections::{BTreeMap, BTreeSet};

use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::ValueId;

/// Temporary pre-SSA reaching-value owner.
///
/// SSA-S2 preserves current behavior behind this one box. It has no source,
/// name, scope, control-flow, or identity-claim responsibility and is replaced
/// atomically by the function-owned SSA box at SSA-I1.
#[derive(Debug)]
pub(super) struct PreSsaValueEnvironmentV1 {
    values: BTreeMap<BindingRefV1, ValueId>,
}

impl PreSsaValueEnvironmentV1 {
    pub(super) fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    pub(super) fn publish(&mut self, binding: BindingRefV1, value: ValueId) -> Result<(), String> {
        if self.values.insert(binding, value).is_some() {
            return Err(format!(
                "[freeze:contract][canonical_identity/value_republished] binding={binding:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn value(&self, binding: BindingRefV1) -> Result<ValueId, String> {
        self.values.get(&binding).copied().ok_or_else(|| {
            format!(
                "[freeze:contract][canonical_identity/value_unmaterialized] binding={binding:?}"
            )
        })
    }

    pub(super) fn rebind(
        &mut self,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Result<ValueId, String> {
        let previous = self.values.get_mut(&binding).ok_or_else(|| {
            format!(
                "[freeze:contract][canonical_identity/rebind_unmaterialized] binding={binding:?}"
            )
        })?;
        let old = *previous;
        *previous = value;
        Ok(old)
    }

    pub(super) fn contains(&self, binding: BindingRefV1) -> bool {
        self.values.contains_key(&binding)
    }

    pub(super) fn remove(&mut self, binding: BindingRefV1) -> Option<ValueId> {
        self.values.remove(&binding)
    }

    pub(super) fn bindings(&self) -> BTreeSet<BindingRefV1> {
        self.values.keys().copied().collect()
    }
}
