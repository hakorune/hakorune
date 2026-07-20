//! Disconnected completed-draft typed-value definition products.
//!
//! FINALIZE0-VERIFY-SPLIT0-S0 separates the read-only completed-draft check
//! from the legacy helper that also removes transient stale rows.  This module
//! deliberately owns neither `MirBuilder` nor a fact-map commit path.

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

/// A completed-draft failure: no repair or source-semantic inference follows.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum CompletedDraftTypedValueDefinitionErrorV1 {
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
pub(super) enum TransientTypedValueRetentionV1 {
    Referenced,
    PendingPhi,
    Pinned,
}

/// A typed failure before any transient fact-map mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum TransientStaleValueFactErrorV1 {
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
pub(super) struct PreparedTransientStaleValueFactsV1 {
    values: Box<[ValueId]>,
    _seal: TransientStaleValueFactsSealV1,
}

#[derive(Debug, Eq, PartialEq)]
struct TransientStaleValueFactsSealV1;

impl PreparedTransientStaleValueFactsV1 {
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
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirType, ValueId,
    };
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
    fn only_unretained_missing_rows_prepare_as_transient_stale_candidates() {
        let function = function_with_parameter_and_const();
        let types = BTreeMap::from([(ValueId::new(9), MirType::Integer)]);
        let rows = TypedValueDefinitionRowsV1::collect(&function, &types);

        let prepared = rows
            .prepare_transient_stale_rows(&BTreeSet::new(), &BTreeSet::new(), &BTreeSet::new())
            .expect("unretained row is stale");
        assert_eq!(prepared.values(), &[ValueId::new(9)]);
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
}
