//! ControlTree / StepTree (structure-only SSOT)
//!
//! Policy:
//! - This module must NOT reference MIR/JoinIR values (`ValueId`, `BlockId`, `PHI`, ...).
//! - It only describes AST control structure and derived features/capabilities.
//!
//! Phase 120: Facts→Decision→Emit architecture
//! - step_tree_facts: Raw data collection (exits/writes/caps/cond_sig)
//! - step_tree_contract_box: Facts → Contract transformation
//! - step_tree: Structure + integration
//!
//! Phase 121: StepTree → Normalized shadow lowering (dev-only)
//! - normalized_shadow: StepTree → JoinModule conversion for if-only patterns

mod step_tree;
mod step_tree_contract_box;
mod step_tree_facts;

// Phase 121: Normalized shadow lowering (dev-only)
pub mod normalized_shadow;

pub use step_tree::{
    AstSummary, ExitKind, StepCapability, StepNode, StepStmtKind, StepTree, StepTreeBuilderBox,
    StepTreeFeatures, StepTreeSignature,
};
pub use step_tree_contract_box::{StepTreeContract, StepTreeContractBox};
pub use step_tree_facts::StepTreeFacts;

