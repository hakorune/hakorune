//! StepTreeFacts - raw structural facts extraction (Phase 120, Phase 124)
//!
//! Responsibility:
//! - Collect raw structural facts from StepNode tree (exits/writes/reads/required_caps/cond_sig)
//! - NO formatting, NO decision-making, NO signature generation
//! - Pure "facts only" - data collection without interpretation
//!
//! Design:
//! - Facts are collected during tree traversal
//! - BTreeSet for deterministic iteration (order stability)
//! - No dependency on contract or signature logic
//!
//! Phase 124 Changes:
//! - Added reads: BTreeSet<String> for variable references
//! - reads tracks Variable(name) occurrences in expressions, conditions, and assignments

use crate::mir::control_tree::{ExitKind, StepCapability};
use std::collections::BTreeSet;

/// Raw structural facts extracted from StepTree (facts only, no interpretation)
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StepTreeFacts {
    /// Exit kinds found in the tree (return/break/continue)
    pub exits: BTreeSet<ExitKind>,
    /// Variable writes (Local declarations + Assignment targets)
    pub writes: BTreeSet<String>,
    /// Variable reads (Variable references in expressions, conditions, assignments)
    /// Phase 124: Tracks all Variable(name) occurrences for Return(Variable) support
    pub reads: BTreeSet<String>,
    /// Required capabilities (structural features like NestedLoop, TryCatch, etc.)
    pub required_caps: BTreeSet<StepCapability>,
    /// Condition signatures (compact string representations of if/loop conditions)
    /// - Collected in traversal order for signature stability
    pub cond_sig: Vec<String>,
}

impl StepTreeFacts {
    /// Create empty facts
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an exit kind
    pub fn add_exit(&mut self, kind: ExitKind) {
        self.exits.insert(kind);
    }

    /// Add a variable write
    pub fn add_write(&mut self, var: String) {
        self.writes.insert(var);
    }

    /// Add a variable read (Phase 124)
    pub fn add_read(&mut self, var: String) {
        self.reads.insert(var);
    }

    /// Add a required capability
    pub fn add_capability(&mut self, cap: StepCapability) {
        self.required_caps.insert(cap);
    }

    /// Add a condition signature
    pub fn add_cond_sig(&mut self, sig: String) {
        self.cond_sig.push(sig);
    }

    /// Merge another facts into this one
    pub fn merge(&mut self, other: StepTreeFacts) {
        self.exits.extend(other.exits);
        self.writes.extend(other.writes);
        self.reads.extend(other.reads); // Phase 124: merge reads
        self.required_caps.extend(other.required_caps);
        self.cond_sig.extend(other.cond_sig);
    }
}
