mod kind;
mod local_storage;
mod map_repr;

pub use kind::MapReprKind;
pub use local_storage::{
    LocalI64MapDirectStoragePlan, LocalI64MapEntryValueTrackingPlan, LocalMapStorageRealizationPlan,
};
pub use map_repr::MapReprPlan;

#[cfg(test)]
mod tests {
    use super::*;
    use hakorune_mir_core::{BasicBlockId, ValueId};

    #[test]
    fn map_repr_plan_keeps_metadata_tags_without_refresh_logic() {
        let plan = MapReprPlan::new(
            BasicBlockId::new(3),
            7,
            "map_repr.generic_hash_runtime",
            MapReprKind::GenericHashRuntime,
            "generic_method.get",
            "map_load_scalar_i64",
            "nyash.map.scalar_load_hi",
            "MapBox".to_string(),
            Some("MapBox".to_string()),
            "get".to_string(),
            ValueId::new(10),
            Some(ValueId::new(11)),
            Some(ValueId::new(12)),
            Some("i64_const"),
            Some("scalar_i64_or_missing_zero"),
            "scalar_i64",
            Some("no_publication"),
            "scalar_i64_get",
            Some("exact"),
        );

        assert_eq!(plan.block(), BasicBlockId::new(3));
        assert_eq!(plan.instruction_index(), 7);
        assert_eq!(plan.repr_kind_tag(), "generic_hash_runtime");
        assert_eq!(plan.source_route_kind(), "map_load_scalar_i64");
        assert_eq!(plan.receiver_origin_box(), Some("MapBox"));
        assert_eq!(plan.key_route_tag(), Some("i64_const"));
        assert_eq!(plan.return_shape_tag(), Some("scalar_i64_or_missing_zero"));
        assert_eq!(plan.publication_policy_tag(), Some("no_publication"));
    }

    #[test]
    fn local_i64_map_storage_plans_keep_execution_disabled() {
        let storage = LocalMapStorageRealizationPlan::local_i64_key_map(ValueId::new(1), 2, 3);
        assert_eq!(storage.receiver_value(), ValueId::new(1));
        assert_eq!(storage.representation(), "local_i64_key_map");
        assert_eq!(storage.candidate_set_count(), 2);
        assert_eq!(storage.candidate_scalar_get_count(), 3);
        assert!(!storage.backend_lowering_enabled());
        assert!(!storage.runtime_helper_enabled());

        let direct = LocalI64MapDirectStoragePlan::closed_world_i64_key_value_table(
            ValueId::new(1),
            2,
            3,
        );
        assert_eq!(direct.representation(), "closed_world_i64_key_value_table");
        assert!(!direct.entry_value_tracking_enabled());
        assert!(!direct.backend_lowering_enabled());
        assert!(!direct.runtime_helper_enabled());
    }
}
