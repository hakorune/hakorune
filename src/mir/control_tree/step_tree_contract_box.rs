//! StepTreeContractBox - facts → contract transformation (Phase 120, Phase 124)
//!
//! Responsibility:
//! - Transform StepTreeFacts into StepTreeContract
//! - Format facts into contract structure
//! - NO decision-making - just data transformation
//! - Maintain determinism (BTreeSet/BTreeMap order)
//!
//! Design:
//! - Pure transformation: facts → contract (idempotent)
//! - No AST traversal, no interpretation
//! - Contract is the formatted representation of facts
//!
//! Phase 124 Changes:
//! - Added reads to StepTreeContract
//! - reads included in signature_basis_string

use crate::mir::control_tree::step_tree_facts::StepTreeFacts;
use crate::mir::control_tree::{ExitKind, StepCapability};
use std::collections::BTreeSet;

/// Structured contract derived from facts (formatted, stable representation)
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StepTreeContract {
    pub exits: BTreeSet<ExitKind>,
    pub writes: BTreeSet<String>,
    /// Phase 124: Variable reads (for Return(Variable) support)
    pub reads: BTreeSet<String>,
    pub required_caps: BTreeSet<StepCapability>,
    pub cond_sig: Vec<String>,
}

impl StepTreeContract {
    /// Generate signature basis string (stable representation for hashing)
    ///
    /// This is used for StepTreeSignature computation.
    /// Format: "kinds=...;exits=...;writes=...;reads=...;caps=...;conds=..."
    ///
    /// Invariants:
    /// - Span is NOT included (determinism)
    /// - BTreeSet iteration order is stable
    /// - cond_sig order is preserved from traversal
    ///
    /// Phase 124: reads added to signature
    pub fn signature_basis_string(&self, node_kinds: &str) -> String {
        let exits = self
            .exits
            .iter()
            .map(|e| match e {
                ExitKind::Return => "return",
                ExitKind::Break => "break",
                ExitKind::Continue => "continue",
            })
            .collect::<Vec<_>>()
            .join(",");
        let writes = self.writes.iter().cloned().collect::<Vec<_>>().join(",");
        let reads = self.reads.iter().cloned().collect::<Vec<_>>().join(",");
        let caps = self
            .required_caps
            .iter()
            .map(|c| match c {
                StepCapability::If => "If",
                StepCapability::Loop => "Loop",
                StepCapability::NestedIf => "NestedIf",
                StepCapability::NestedLoop => "NestedLoop",
                StepCapability::Return => "Return",
                StepCapability::Break => "Break",
                StepCapability::Continue => "Continue",
                StepCapability::TryCatch => "TryCatch",
                StepCapability::Throw => "Throw",
                StepCapability::Lambda => "Lambda",
                StepCapability::While => "While",
                StepCapability::ForRange => "ForRange",
                StepCapability::Match => "Match",
                StepCapability::Arrow => "Arrow",
            })
            .collect::<Vec<_>>()
            .join(",");
        let cond_sig = self.cond_sig.join("|");

        format!(
            "kinds={};exits={};writes={};reads={};caps={};conds={}",
            node_kinds, exits, writes, reads, caps, cond_sig
        )
    }
}

/// StepTreeContractBox - pure transformation from facts to contract
pub struct StepTreeContractBox;

impl StepTreeContractBox {
    /// Transform facts into contract (idempotent, deterministic)
    ///
    /// This is a pure transformation:
    /// - No decision-making
    /// - No AST traversal
    /// - Same facts → same contract (idempotent)
    ///
    /// Phase 124: reads added to contract
    pub fn from_facts(facts: &StepTreeFacts) -> StepTreeContract {
        StepTreeContract {
            exits: facts.exits.clone(),
            writes: facts.writes.clone(),
            reads: facts.reads.clone(), // Phase 124
            required_caps: facts.required_caps.clone(),
            cond_sig: facts.cond_sig.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_facts_is_idempotent() {
        let mut facts = StepTreeFacts::new();
        facts.add_exit(ExitKind::Return);
        facts.add_write("x".to_string());
        facts.add_capability(StepCapability::If);
        facts.add_cond_sig("var:cond".to_string());

        let contract1 = StepTreeContractBox::from_facts(&facts);
        let contract2 = StepTreeContractBox::from_facts(&facts);

        assert_eq!(contract1, contract2);
    }

    #[test]
    fn signature_basis_string_is_deterministic() {
        let mut facts = StepTreeFacts::new();
        // Add in different order
        facts.add_write("z".to_string());
        facts.add_write("a".to_string());
        facts.add_write("m".to_string());

        let contract = StepTreeContractBox::from_facts(&facts);
        let basis1 = contract.signature_basis_string("Block");
        let basis2 = contract.signature_basis_string("Block");

        assert_eq!(basis1, basis2);
        // BTreeSet should give stable order: a, m, z
        assert!(basis1.contains("writes=a,m,z"));
    }

    #[test]
    fn facts_are_order_independent() {
        // Phase 120: Contract requirement - facts collection order doesn't affect contract
        let mut facts1 = StepTreeFacts::new();
        facts1.add_exit(ExitKind::Break);
        facts1.add_exit(ExitKind::Return);
        facts1.add_write("x".to_string());
        facts1.add_write("y".to_string());
        facts1.add_capability(StepCapability::Loop);
        facts1.add_capability(StepCapability::If);

        let mut facts2 = StepTreeFacts::new();
        // Different order
        facts2.add_capability(StepCapability::If);
        facts2.add_write("y".to_string());
        facts2.add_exit(ExitKind::Return);
        facts2.add_capability(StepCapability::Loop);
        facts2.add_write("x".to_string());
        facts2.add_exit(ExitKind::Break);

        let contract1 = StepTreeContractBox::from_facts(&facts1);
        let contract2 = StepTreeContractBox::from_facts(&facts2);

        // Contracts should be identical (BTreeSet guarantees stable order)
        assert_eq!(contract1.exits, contract2.exits);
        assert_eq!(contract1.writes, contract2.writes);
        assert_eq!(contract1.required_caps, contract2.required_caps);

        // Signature basis should match
        let basis1 = contract1.signature_basis_string("Block");
        let basis2 = contract2.signature_basis_string("Block");
        assert_eq!(basis1, basis2);
    }
}
