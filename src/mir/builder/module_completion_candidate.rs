//! MODULETX0-S0 private vocabulary for an eventual module-completion candidate.
//!
//! This is intentionally not a Builder transaction yet. It owns only moved
//! candidate data and exposes no commit, repair, fact-projection, metadata
//! publication, or derived-refresh operation. REMATFACT0 owns the future
//! producer-branded fresh-value projection before any production consumer may
//! construct this from live Builder state.

use crate::ast::Span;
use crate::mir::value_kind::MirValueKind;
use crate::mir::{MirModule, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Candidate-local copy of every current TypeContext lane plus diagnostic rows.
///
/// The facts are observations in S0, not producer receipts. In particular,
/// they cannot authorize a fresh rematerialized ValueId publication.
#[derive(Debug, Default)]
pub(super) struct PendingModuleCompletionFactsV1 {
    value_types: BTreeMap<ValueId, MirType>,
    value_kinds: HashMap<ValueId, MirValueKind>,
    value_origin_newbox: BTreeMap<ValueId, String>,
    string_literals: BTreeMap<ValueId, String>,
    map_value_types: BTreeMap<ValueId, MirType>,
    map_literal_value_types: BTreeMap<(ValueId, String), MirType>,
    diagnostic_origin_spans: BTreeMap<ValueId, Span>,
    diagnostic_origin_callers: BTreeMap<ValueId, String>,
}

/// Derived module products invalidated by future candidate MIR replacement.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum ModuleCompletionDerivedArtifactV1 {
    RecordAndPackedLayout,
    TypedObjectPlan,
    DirectStatePlan,
}

/// Owned invalidation ledger; S0 never refreshes or publishes these artifacts.
#[derive(Debug, Default)]
struct ModuleCompletionDerivedArtifactInvalidationV1 {
    pending: BTreeSet<ModuleCompletionDerivedArtifactV1>,
}

impl ModuleCompletionDerivedArtifactInvalidationV1 {
    fn all_current_module_products() -> Self {
        Self {
            pending: BTreeSet::from([
                ModuleCompletionDerivedArtifactV1::RecordAndPackedLayout,
                ModuleCompletionDerivedArtifactV1::TypedObjectPlan,
                ModuleCompletionDerivedArtifactV1::DirectStatePlan,
            ]),
        }
    }
}

/// Non-Clone, single-use ownership boundary for a future module transaction.
///
/// The module already owns every assembled function. The fact/session and
/// invalidation ledgers move with it, so a later I0 cannot pair a repaired
/// module with facts or derived-artifact work from a different session.
#[derive(Debug)]
pub(super) struct PreparedModuleCompletionCandidateV1 {
    module: MirModule,
    facts: PendingModuleCompletionFactsV1,
    invalidated_artifacts: ModuleCompletionDerivedArtifactInvalidationV1,
    _seal: ModuleCompletionCandidateSealV1,
}

#[derive(Debug)]
struct ModuleCompletionCandidateSealV1;

/// Moves already-owned candidate data into the S0 boundary.
///
/// No production caller exists. Future construction from `MirBuilder` is
/// gated by REMATFACT0 and MODULETX0-P0.
#[allow(dead_code)]
pub(super) fn prepare_module_completion_candidate_v1(
    module: MirModule,
    facts: PendingModuleCompletionFactsV1,
) -> PreparedModuleCompletionCandidateV1 {
    PreparedModuleCompletionCandidateV1 {
        module,
        facts,
        invalidated_artifacts:
            ModuleCompletionDerivedArtifactInvalidationV1::all_current_module_products(),
        _seal: ModuleCompletionCandidateSealV1,
    }
}

#[cfg(test)]
impl PendingModuleCompletionFactsV1 {
    #[allow(clippy::too_many_arguments)]
    fn from_test_parts(
        value_types: BTreeMap<ValueId, MirType>,
        value_kinds: HashMap<ValueId, MirValueKind>,
        value_origin_newbox: BTreeMap<ValueId, String>,
        string_literals: BTreeMap<ValueId, String>,
        map_value_types: BTreeMap<ValueId, MirType>,
        map_literal_value_types: BTreeMap<(ValueId, String), MirType>,
        diagnostic_origin_spans: BTreeMap<ValueId, Span>,
        diagnostic_origin_callers: BTreeMap<ValueId, String>,
    ) -> Self {
        Self {
            value_types,
            value_kinds,
            value_origin_newbox,
            string_literals,
            map_value_types,
            map_literal_value_types,
            diagnostic_origin_spans,
            diagnostic_origin_callers,
        }
    }
}

#[cfg(test)]
impl PreparedModuleCompletionCandidateV1 {
    fn function_names(&self) -> Vec<&str> {
        self.module.functions.keys().map(String::as_str).collect()
    }

    fn fact_lane_counts(&self) -> [usize; 8] {
        [
            self.facts.value_types.len(),
            self.facts.value_kinds.len(),
            self.facts.value_origin_newbox.len(),
            self.facts.string_literals.len(),
            self.facts.map_value_types.len(),
            self.facts.map_literal_value_types.len(),
            self.facts.diagnostic_origin_spans.len(),
            self.facts.diagnostic_origin_callers.len(),
        ]
    }

    fn invalidated_artifacts(&self) -> Vec<ModuleCompletionDerivedArtifactV1> {
        self.invalidated_artifacts.pending.iter().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_module_completion_candidate_v1, ModuleCompletionDerivedArtifactV1,
        PendingModuleCompletionFactsV1,
    };
    use crate::ast::Span;
    use crate::mir::value_kind::MirValueKind;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType, ValueId,
    };
    use std::collections::{BTreeMap, HashMap};

    fn function(name: &str) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: name.to_string(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn candidate_keeps_module_and_all_transient_observation_lanes_together() {
        let value = ValueId::new(7);
        let mut module = MirModule::new("module-candidate".to_string());
        module.add_function(function("z/0"));
        module.add_function(function("a/0"));
        let facts = PendingModuleCompletionFactsV1::from_test_parts(
            BTreeMap::from([(value, MirType::Integer)]),
            HashMap::from([(value, MirValueKind::Temporary)]),
            BTreeMap::from([(value, "Owner".to_string())]),
            BTreeMap::from([(value, "literal".to_string())]),
            BTreeMap::from([(value, MirType::Integer)]),
            BTreeMap::from([((value, "key".to_string()), MirType::Integer)]),
            BTreeMap::from([(value, Span::unknown())]),
            BTreeMap::from([(value, "caller".to_string())]),
        );

        let candidate = prepare_module_completion_candidate_v1(module, facts);
        assert_eq!(candidate.function_names(), vec!["a/0", "z/0"]);
        assert_eq!(candidate.fact_lane_counts(), [1; 8]);
        assert_eq!(
            candidate.invalidated_artifacts(),
            vec![
                ModuleCompletionDerivedArtifactV1::RecordAndPackedLayout,
                ModuleCompletionDerivedArtifactV1::TypedObjectPlan,
                ModuleCompletionDerivedArtifactV1::DirectStatePlan,
            ]
        );
    }
}
