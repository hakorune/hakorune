//! Aggregate representation planning vocabulary.
//!
//! This module is intentionally passive. It names storage outcomes for
//! identity-free aggregate values such as records, enum payloads, tuple
//! payloads, and closure environments. It does not lower records, does not
//! mutate MIR, and does not collapse the source-level `record` / `box`
//! distinction.

use crate::object_storage_plan::{FieldScalarPlan, LayoutId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggregateSubjectKind {
    Record,
    EnumPayload,
    TuplePayload,
    ClosureEnv,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggregateFallbackReason {
    MissingLayoutProof,
    VisibleMaterializationRequired,
    BackendRouteUnsupported,
    UnsupportedEscape,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregateStoragePlan {
    GenericAggregate {
        subject: AggregateSubjectKind,
        reason: AggregateFallbackReason,
    },
    StackAggregate {
        subject: AggregateSubjectKind,
        layout_id: LayoutId,
    },
    ExactNativeStruct {
        subject: AggregateSubjectKind,
        layout_id: LayoutId,
    },
    Scalarized {
        subject: AggregateSubjectKind,
        fields: Vec<FieldScalarPlan>,
    },
}

impl AggregateStoragePlan {
    #[inline]
    pub fn subject(&self) -> AggregateSubjectKind {
        match self {
            Self::GenericAggregate { subject, .. }
            | Self::StackAggregate { subject, .. }
            | Self::ExactNativeStruct { subject, .. }
            | Self::Scalarized { subject, .. } => *subject,
        }
    }

    #[inline]
    pub fn is_exact_candidate(&self) -> bool {
        matches!(
            self,
            Self::StackAggregate { .. } | Self::ExactNativeStruct { .. } | Self::Scalarized { .. }
        )
    }

    #[inline]
    pub fn is_generic_fallback(&self) -> bool {
        matches!(self, Self::GenericAggregate { .. })
    }
}

pub fn aggregate_storage_plan_report_fields() -> &'static [(&'static str, &'static str)] {
    &[
        ("output_contract", "hako-aggregate-storage-plan-v0"),
        ("record_box_surface_model", "two_surface_one_substrate"),
        ("record_identity_free_value_surface", "1"),
        ("box_identity_behavior_lifecycle_surface", "1"),
        ("source_surface_collapsed_to_box", "0"),
        ("aggregate_storage_plan_vocabulary_defined", "1"),
        ("aggregate_storage_plan_execution_enabled", "0"),
        ("object_storage_plan_shared_substrate", "1"),
        ("mirbuilder_representation_owner", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object_storage_plan::{FieldId, ScalarStorageType};

    #[test]
    fn exact_aggregate_candidates_are_separate_from_fallback() {
        assert!(AggregateStoragePlan::StackAggregate {
            subject: AggregateSubjectKind::Record,
            layout_id: LayoutId(1),
        }
        .is_exact_candidate());
        assert!(AggregateStoragePlan::ExactNativeStruct {
            subject: AggregateSubjectKind::EnumPayload,
            layout_id: LayoutId(2),
        }
        .is_exact_candidate());
        assert!(AggregateStoragePlan::Scalarized {
            subject: AggregateSubjectKind::ClosureEnv,
            fields: vec![FieldScalarPlan {
                field_id: FieldId(1),
                layout_id: LayoutId(3),
                scalar_type: ScalarStorageType::I64,
            }],
        }
        .is_exact_candidate());

        let fallback = AggregateStoragePlan::GenericAggregate {
            subject: AggregateSubjectKind::TuplePayload,
            reason: AggregateFallbackReason::VisibleMaterializationRequired,
        };
        assert!(fallback.is_generic_fallback());
        assert_eq!(fallback.subject(), AggregateSubjectKind::TuplePayload);
    }

    #[test]
    fn report_fields_keep_surface_split_and_execution_disabled() {
        let fields = aggregate_storage_plan_report_fields();
        assert!(fields.contains(&("record_box_surface_model", "two_surface_one_substrate")));
        assert!(fields.contains(&("source_surface_collapsed_to_box", "0")));
        assert!(fields.contains(&("aggregate_storage_plan_vocabulary_defined", "1")));
        assert!(fields.contains(&("aggregate_storage_plan_execution_enabled", "0")));
        assert!(fields.contains(&("object_storage_plan_shared_substrate", "1")));
        assert!(fields.contains(&("mirbuilder_representation_owner", "0")));
    }
}
