//! Disconnected completed-draft typed-value definition products.
//!
//! FINALIZE0-VERIFY-SPLIT0-S0 separates the read-only completed-draft check
//! from the legacy helper that also removes transient stale rows.  This module
//! deliberately owns neither `MirBuilder` nor a MIR mutation path.

use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::builder::type_context::TypeContext;
use crate::mir::verification::utils::compute_def_blocks;
use crate::mir::{MirFunction, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

/// One typed transient row observed against a completed MIR function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TypedValueDefinitionRowV1 {
    value: ValueId,
    value_type: MirType,
}

impl TypedValueDefinitionRowV1 {
    #[cfg(test)]
    fn value(&self) -> ValueId {
        self.value
    }
}

/// Read-only inventory of typed values without an in-function definition.
///
/// `ValueId::INVALID` is a sentinel rather than a value and is therefore not a
/// row.  The rows are sorted by ValueId so diagnostics and prepared products
/// do not depend on the transient map's iteration order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct TypedValueDefinitionRowsV1 {
    missing: Box<[TypedValueDefinitionRowV1]>,
}

impl TypedValueDefinitionRowsV1 {
    pub(super) fn collect(
        function: &MirFunction,
        value_types: &BTreeMap<ValueId, MirType>,
    ) -> Self {
        let def_blocks = compute_def_blocks(function);
        let mut missing: Vec<_> = value_types
            .iter()
            .filter(|(value, _)| {
                **value != ValueId::INVALID
                    && !def_blocks.contains_key(*value)
                    && !function.params.contains(value)
            })
            .map(|(value, value_type)| TypedValueDefinitionRowV1 {
                value: *value,
                value_type: value_type.clone(),
            })
            .collect();
        missing.sort_by_key(|row| row.value.0);
        Self {
            missing: missing.into_boxed_slice(),
        }
    }

    /// Verifies a completed draft after every transient stale row has already
    /// been handled by its separate lifecycle owner.
    pub(super) fn verify_completed_draft(
        &self,
    ) -> Result<(), CompletedDraftTypedValueDefinitionErrorV1> {
        let Some(first) = self.missing.first() else {
            return Ok(());
        };
        Err(
            CompletedDraftTypedValueDefinitionErrorV1::MissingDefinition {
                value: first.value,
                value_type: first.value_type.clone(),
                missing_count: self.missing.len(),
            },
        )
    }

    /// Prepares only rows whose missing definition has no retained transient
    /// use.  A future lifecycle normalizer owns the actual map removals.
    pub(super) fn prepare_transient_stale_rows(
        &self,
        referenced: &BTreeSet<ValueId>,
        pending_phi_destinations: &BTreeSet<ValueId>,
        pinned_values: &BTreeSet<ValueId>,
    ) -> Result<PreparedTransientStaleValueFactsV1, TransientStaleValueFactErrorV1> {
        let mut stale = Vec::new();
        for row in self.missing.iter() {
            let retained_by = if referenced.contains(&row.value) {
                Some(TransientTypedValueRetentionV1::Referenced)
            } else if pending_phi_destinations.contains(&row.value) {
                Some(TransientTypedValueRetentionV1::PendingPhi)
            } else if pinned_values.contains(&row.value) {
                Some(TransientTypedValueRetentionV1::Pinned)
            } else {
                None
            };

            if let Some(retention) = retained_by {
                return Err(TransientStaleValueFactErrorV1::RetainedMissingDefinition {
                    value: row.value,
                    value_type: row.value_type.clone(),
                    retention,
                });
            }
            stale.push(row.value);
        }
        Ok(PreparedTransientStaleValueFactsV1 {
            values: stale.into_boxed_slice(),
            _seal: TransientStaleValueFactsSealV1,
        })
    }

    #[cfg(test)]
    fn missing(&self) -> &[TypedValueDefinitionRowV1] {
        &self.missing
    }
}

/// Collects the current function's reachable instruction and parameter uses.
///
/// The transient normalizer receives this exact set as an input.  Definitions
/// remain all-function facts, including unreachable blocks; only use retention
/// follows the existing reachable-use contract.
pub(in crate::mir::builder) fn collect_referenced_typed_values_v1(
    function: &MirFunction,
) -> BTreeSet<ValueId> {
    let remapper = JoinIrIdRemapper::new();
    let reachable = crate::mir::verification::utils::compute_reachable_blocks(function);
    let mut values = BTreeSet::new();
    for (block_id, block) in &function.blocks {
        if reachable.contains(block_id) {
            values.extend(remapper.collect_values_in_block(block));
        }
    }
    values.extend(function.params.iter().copied());
    values
}

/// Prepares the one transient stale-row normalization candidate for a function.
///
/// This remains independent of `MirBuilder`: caller-owned pending-PHI and pin
/// sets are explicit inputs, while MIR and type facts are borrowed only for
/// classification.
pub(in crate::mir::builder) fn prepare_transient_stale_value_facts_v1(
    function: &MirFunction,
    value_types: &BTreeMap<ValueId, MirType>,
    pending_phi_destinations: &BTreeSet<ValueId>,
    pinned_values: &BTreeSet<ValueId>,
) -> Result<PreparedTransientStaleValueFactsV1, TransientStaleValueFactErrorV1> {
    TypedValueDefinitionRowsV1::collect(function, value_types).prepare_transient_stale_rows(
        &collect_referenced_typed_values_v1(function),
        pending_phi_destinations,
        pinned_values,
    )
}

/// Verifies that a completed draft has no residual typed-without-definition row.
pub(in crate::mir::builder) fn verify_completed_draft_typed_value_definitions_v1(
    function: &MirFunction,
    value_types: &BTreeMap<ValueId, MirType>,
) -> Result<(), CompletedDraftTypedValueDefinitionErrorV1> {
    TypedValueDefinitionRowsV1::collect(function, value_types).verify_completed_draft()
}

/// A completed-draft failure: no repair or source-semantic inference follows.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum CompletedDraftTypedValueDefinitionErrorV1 {
    MissingDefinition {
        value: ValueId,
        value_type: MirType,
        missing_count: usize,
    },
}

impl std::fmt::Display for CompletedDraftTypedValueDefinitionErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDefinition {
                value,
                value_type,
                missing_count,
            } => write!(
                formatter,
                "[freeze:contract][value_lifecycle/completed_draft_typed_without_def] missing_count={missing_count} missing0=%{} missing0_ty={value_type:?}",
                value.0
            ),
        }
    }
}

impl std::error::Error for CompletedDraftTypedValueDefinitionErrorV1 {}

/// Why a missing typed value cannot be normalized as a stale transient row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum TransientTypedValueRetentionV1 {
    Referenced,
    PendingPhi,
    Pinned,
}

/// A typed failure before any transient fact-map mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum TransientStaleValueFactErrorV1 {
    RetainedMissingDefinition {
        value: ValueId,
        value_type: MirType,
        retention: TransientTypedValueRetentionV1,
    },
}

impl std::fmt::Display for TransientStaleValueFactErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RetainedMissingDefinition {
                value,
                value_type,
                retention,
            } => write!(
                formatter,
                "[freeze:contract][value_lifecycle/transient_stale_row_retained] value=%{} type={value_type:?} retention={retention:?}",
                value.0
            ),
        }
    }
}

impl std::error::Error for TransientStaleValueFactErrorV1 {}

/// Non-Clone, single-use candidate for the later transient normalizer.
///
/// It carries only ValueIds; the normalizer will own the three coordinated
/// transient-map removals and their lifecycle timing in I0.
#[derive(Debug, Eq, PartialEq)]
pub(in crate::mir::builder) struct PreparedTransientStaleValueFactsV1 {
    values: Box<[ValueId]>,
    _seal: TransientStaleValueFactsSealV1,
}

#[derive(Debug, Eq, PartialEq)]
struct TransientStaleValueFactsSealV1;

impl PreparedTransientStaleValueFactsV1 {
    /// Commits only the three existing transient stale-row removals.
    ///
    /// The candidate is prepared before metadata publication and consumed once
    /// after all retention checks have succeeded.  It cannot mutate MIR or
    /// allocate a ValueId.
    pub(in crate::mir::builder) fn commit(self, type_ctx: &mut TypeContext) {
        for value in self.values {
            type_ctx.value_types.remove(&value);
            type_ctx.value_kinds.remove(&value);
            type_ctx.value_origin_newbox.remove(&value);
        }
    }

    #[cfg(test)]
    fn values(&self) -> &[ValueId] {
        &self.values
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CompletedDraftTypedValueDefinitionErrorV1, TransientStaleValueFactErrorV1,
        TransientTypedValueRetentionV1, TypedValueDefinitionRowsV1,
    };
    use crate::mir::builder::type_context::TypeContext;
    use crate::mir::{
        BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirType, ValueId,
    };
    use hakorune_mir_core::MirValueKind;
    use std::collections::{BTreeMap, BTreeSet};

    fn function_with_parameter_and_const() -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "value-lifecycle-definition/1".to_string(),
                params: vec![MirType::Integer],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block")
            .add_instruction(MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::Integer(7),
            });
        function
    }

    #[test]
    fn defined_parameter_and_instruction_rows_verify_without_mutation() {
        let function = function_with_parameter_and_const();
        let types = BTreeMap::from([
            (ValueId::new(0), MirType::Integer),
            (ValueId::new(1), MirType::Integer),
        ]);

        let rows = TypedValueDefinitionRowsV1::collect(&function, &types);
        assert!(rows.missing().is_empty());
        assert_eq!(rows.verify_completed_draft(), Ok(()));
    }

    #[test]
    fn invalid_sentinel_is_not_a_completed_draft_row() {
        let function = function_with_parameter_and_const();
        let types = BTreeMap::from([(ValueId::INVALID, MirType::Unknown)]);

        let rows = TypedValueDefinitionRowsV1::collect(&function, &types);
        assert!(rows.missing().is_empty());
        assert_eq!(rows.verify_completed_draft(), Ok(()));
    }

    #[test]
    fn all_function_definitions_include_unreachable_instruction_definitions() {
        let mut function = function_with_parameter_and_const();
        let mut unreachable = BasicBlock::new(BasicBlockId::new(7));
        unreachable.add_instruction(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Integer(11),
        });
        function.add_block(unreachable);
        let types = BTreeMap::from([(ValueId::new(2), MirType::Integer)]);

        let rows = TypedValueDefinitionRowsV1::collect(&function, &types);
        assert!(rows.missing().is_empty());
        assert_eq!(rows.verify_completed_draft(), Ok(()));
    }

    #[test]
    fn completed_draft_rejects_each_remaining_missing_typed_value() {
        let function = function_with_parameter_and_const();
        let types = BTreeMap::from([
            (ValueId::new(9), MirType::String),
            (ValueId::new(5), MirType::Integer),
        ]);

        let rows = TypedValueDefinitionRowsV1::collect(&function, &types);
        assert_eq!(
            rows.missing()
                .iter()
                .map(|row| row.value())
                .collect::<Vec<_>>(),
            vec![ValueId::new(5), ValueId::new(9)]
        );
        assert_eq!(
            rows.verify_completed_draft(),
            Err(
                CompletedDraftTypedValueDefinitionErrorV1::MissingDefinition {
                    value: ValueId::new(5),
                    value_type: MirType::Integer,
                    missing_count: 2,
                }
            )
        );
    }

    #[test]
    fn completed_draft_rejects_stale_candidates_until_a_normalizer_consumes_them() {
        let function = function_with_parameter_and_const();
        let types = BTreeMap::from([(ValueId::new(9), MirType::Void)]);
        let rows = TypedValueDefinitionRowsV1::collect(&function, &types);

        let prepared = rows
            .prepare_transient_stale_rows(&BTreeSet::new(), &BTreeSet::new(), &BTreeSet::new())
            .expect("the row is a normalizer candidate");
        assert_eq!(prepared.values(), &[ValueId::new(9)]);
        assert_eq!(
            rows.verify_completed_draft(),
            Err(
                CompletedDraftTypedValueDefinitionErrorV1::MissingDefinition {
                    value: ValueId::new(9),
                    value_type: MirType::Void,
                    missing_count: 1,
                }
            )
        );
    }

    #[test]
    fn only_unretained_missing_rows_prepare_as_transient_stale_candidates() {
        let function = function_with_parameter_and_const();
        let types = BTreeMap::from([
            (ValueId::INVALID, MirType::Unknown),
            (ValueId::new(9), MirType::Integer),
            (ValueId::new(5), MirType::Unknown),
        ]);
        let rows = TypedValueDefinitionRowsV1::collect(&function, &types);

        let prepared = rows
            .prepare_transient_stale_rows(
                &BTreeSet::from([ValueId::INVALID]),
                &BTreeSet::from([ValueId::INVALID]),
                &BTreeSet::from([ValueId::INVALID]),
            )
            .expect("unretained row is stale");
        assert_eq!(prepared.values(), &[ValueId::new(5), ValueId::new(9)]);
    }

    #[test]
    fn referenced_pending_and_pinned_rows_are_not_stale_candidates() {
        let function = function_with_parameter_and_const();
        let types = BTreeMap::from([(ValueId::new(9), MirType::Integer)]);
        let rows = TypedValueDefinitionRowsV1::collect(&function, &types);

        for (referenced, pending, pinned, retention) in [
            (
                BTreeSet::from([ValueId::new(9)]),
                BTreeSet::new(),
                BTreeSet::new(),
                TransientTypedValueRetentionV1::Referenced,
            ),
            (
                BTreeSet::new(),
                BTreeSet::from([ValueId::new(9)]),
                BTreeSet::new(),
                TransientTypedValueRetentionV1::PendingPhi,
            ),
            (
                BTreeSet::new(),
                BTreeSet::new(),
                BTreeSet::from([ValueId::new(9)]),
                TransientTypedValueRetentionV1::Pinned,
            ),
        ] {
            assert_eq!(
                rows.prepare_transient_stale_rows(&referenced, &pending, &pinned),
                Err(TransientStaleValueFactErrorV1::RetainedMissingDefinition {
                    value: ValueId::new(9),
                    value_type: MirType::Integer,
                    retention,
                })
            );
        }
    }

    #[test]
    fn retention_priority_is_deterministic_and_preparation_cannot_publish_a_partial_product() {
        let function = function_with_parameter_and_const();
        let types = BTreeMap::from([
            (ValueId::new(5), MirType::Integer),
            (ValueId::new(9), MirType::Unknown),
        ]);
        let rows = TypedValueDefinitionRowsV1::collect(&function, &types);

        let all_retained = rows.prepare_transient_stale_rows(
            &BTreeSet::from([ValueId::new(9)]),
            &BTreeSet::from([ValueId::new(9)]),
            &BTreeSet::from([ValueId::new(9)]),
        );
        assert_eq!(
            all_retained,
            Err(TransientStaleValueFactErrorV1::RetainedMissingDefinition {
                value: ValueId::new(9),
                value_type: MirType::Unknown,
                retention: TransientTypedValueRetentionV1::Referenced,
            })
        );

        let later_retained = rows.prepare_transient_stale_rows(
            &BTreeSet::from([ValueId::new(9)]),
            &BTreeSet::new(),
            &BTreeSet::new(),
        );
        assert_eq!(
            later_retained,
            Err(TransientStaleValueFactErrorV1::RetainedMissingDefinition {
                value: ValueId::new(9),
                value_type: MirType::Unknown,
                retention: TransientTypedValueRetentionV1::Referenced,
            })
        );
        assert_eq!(
            rows.missing()
                .iter()
                .map(|row| row.value())
                .collect::<Vec<_>>(),
            vec![ValueId::new(5), ValueId::new(9)]
        );
    }

    #[test]
    fn prepared_stale_commit_removes_exactly_the_three_transient_lanes() {
        let function = function_with_parameter_and_const();
        let stale = ValueId::new(9);
        let retained = ValueId::new(1);
        let rows = TypedValueDefinitionRowsV1::collect(
            &function,
            &BTreeMap::from([(stale, MirType::Integer), (retained, MirType::Integer)]),
        );
        let prepared = rows
            .prepare_transient_stale_rows(&BTreeSet::new(), &BTreeSet::new(), &BTreeSet::new())
            .expect("unretained row prepares");

        let mut type_ctx = TypeContext::default();
        type_ctx.value_types.insert(stale, MirType::Integer);
        type_ctx.value_kinds.insert(stale, MirValueKind::Temporary);
        type_ctx
            .value_origin_newbox
            .insert(stale, "StaleOrigin".to_string());
        type_ctx.value_types.insert(retained, MirType::Integer);
        type_ctx
            .value_kinds
            .insert(retained, MirValueKind::Parameter(0));
        type_ctx
            .value_origin_newbox
            .insert(retained, "DefinedOrigin".to_string());

        prepared.commit(&mut type_ctx);

        assert!(!type_ctx.value_types.contains_key(&stale));
        assert!(!type_ctx.value_kinds.contains_key(&stale));
        assert!(!type_ctx.value_origin_newbox.contains_key(&stale));
        assert_eq!(type_ctx.value_types.get(&retained), Some(&MirType::Integer));
        assert_eq!(
            type_ctx.value_kinds.get(&retained),
            Some(&MirValueKind::Parameter(0))
        );
        assert_eq!(
            type_ctx.value_origin_newbox.get(&retained),
            Some(&"DefinedOrigin".to_string())
        );
    }
}
