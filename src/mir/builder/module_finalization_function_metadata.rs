//! Prepared function metadata for shared module finalization.
//!
//! This owner snapshots final value types and merges current diagnostic origin
//! callers. It does not infer types, refresh module metadata, or publish a
//! function.

use super::{MirFunction, MirType, ValueId};
use std::collections::BTreeMap;

pub(super) struct PreparedModuleFinalizationFunctionMetadataV1 {
    value_types: BTreeMap<ValueId, MirType>,
    value_origin_callers: BTreeMap<ValueId, String>,
}

impl PreparedModuleFinalizationFunctionMetadataV1 {
    pub(super) fn prepare(
        function: &MirFunction,
        value_types: &BTreeMap<ValueId, MirType>,
        current_origin_caller_rows: Vec<(ValueId, String)>,
    ) -> Self {
        let mut value_origin_callers = function.metadata.value_origin_callers.clone();
        for (value, caller) in current_origin_caller_rows {
            value_origin_callers.insert(value, caller);
        }

        Self {
            value_types: value_types.clone(),
            value_origin_callers,
        }
    }

    pub(super) fn commit_into(self, function: &mut MirFunction) {
        function.metadata.value_types = self.value_types;
        function.metadata.value_origin_callers = self.value_origin_callers;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::MirBuilder;

    #[test]
    fn prepares_exact_type_snapshot_and_current_origin_caller_wins_on_commit() {
        let mut builder = MirBuilder::new();
        builder.prepare_module().expect("module shell");
        let mut function = builder
            .function_state
            .current_function
            .take()
            .expect("entry function");
        let overlap = ValueId::new(17);
        let retained = ValueId::new(23);
        function
            .metadata
            .value_origin_callers
            .insert(overlap, "old-caller".to_owned());
        function
            .metadata
            .value_origin_callers
            .insert(retained, "retained-caller".to_owned());
        let value_types = BTreeMap::from([(overlap, MirType::Integer)]);
        let prior_metadata = function.metadata.clone();

        let prepared = PreparedModuleFinalizationFunctionMetadataV1::prepare(
            &function,
            &value_types,
            vec![(overlap, "current-caller".to_owned())],
        );

        assert_eq!(function.metadata.value_types, prior_metadata.value_types);
        assert_eq!(
            function.metadata.value_origin_callers,
            prior_metadata.value_origin_callers
        );

        prepared.commit_into(&mut function);

        assert_eq!(function.metadata.value_types, value_types);
        assert_eq!(
            function.metadata.value_origin_callers.get(&overlap),
            Some(&"current-caller".to_owned())
        );
        assert_eq!(
            function.metadata.value_origin_callers.get(&retained),
            Some(&"retained-caller".to_owned())
        );
    }
}
