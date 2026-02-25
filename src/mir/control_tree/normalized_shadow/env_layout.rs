//! Env layout helper (writes + inputs) for normalized shadow lowering

use crate::mir::control_tree::step_tree::StepTree;
use crate::mir::control_tree::step_tree_contract_box::StepTreeContract;
use crate::mir::ValueId;
use std::collections::BTreeMap;

/// Phase 125: Normalized env layout (writes + inputs)
///
/// ## SSOT
///
/// - writes: From `StepTreeContract.writes`
/// - inputs: From `(StepTreeContract.reads ∩ available_inputs)`
#[derive(Debug, Clone)]
pub struct EnvLayout {
    /// Variables written (generate ValueId for these)
    pub writes: Vec<String>,
    /// Variables read from outer scope (reference ValueId from available_inputs)
    pub inputs: Vec<String>,
}

impl EnvLayout {
    /// Create env layout from contract and available_inputs (Phase 125)
    pub fn from_contract(
        contract: &StepTreeContract,
        available_inputs: &BTreeMap<String, ValueId>,
    ) -> Self {
        // Phase 125 P2: writes from contract
        let writes: Vec<String> = contract.writes.iter().cloned().collect();
        let writes_set: std::collections::BTreeSet<&String> = contract.writes.iter().collect();

        // Phase 125 P2: inputs = (reads ∩ available_inputs) \ writes
        // inputs are read-only by definition; if a variable is written, it must not be treated as an input.
        let inputs: Vec<String> = contract
            .reads
            .iter()
            .filter(|name| available_inputs.contains_key(*name))
            .filter(|name| !writes_set.contains(name))
            .cloned()
            .collect();

        EnvLayout { writes, inputs }
    }

    /// Flatten writes+inputs to a single field list (deterministic)
    pub fn env_fields(&self) -> Vec<String> {
        self.writes
            .iter()
            .chain(self.inputs.iter())
            .cloned()
            .collect()
    }
}

/// Phase 129-B: Expected env field count (writes + inputs)
pub fn expected_env_field_count(
    step_tree: &StepTree,
    available_inputs: &BTreeMap<String, ValueId>,
) -> usize {
    let env_layout = EnvLayout::from_contract(&step_tree.contract, available_inputs);
    env_layout.writes.len() + env_layout.inputs.len()
}
