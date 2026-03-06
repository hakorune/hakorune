//! Skeleton Type Definitions
//!
//! Core data structures for the Loop Canonicalizer.
//! These types represent the normalized "skeleton" of a loop structure
//! without any JoinIR-specific information (BlockId, ValueId, etc.).

use crate::ast::{ASTNode, Span};

// ============================================================================
// Core Skeleton Types
// ============================================================================

/// Loop skeleton - The canonical representation of a loop structure
///
/// This is the single output type of the Canonicalizer.
/// It represents the essential structure of a loop without any
/// JoinIR-specific information.
#[derive(Debug, Clone)]
pub struct LoopSkeleton {
    /// Sequence of steps (HeaderCond, BodyInit, BreakCheck, Updates, Tail)
    pub steps: Vec<SkeletonStep>,

    /// Carriers (loop variables with update rules and boundary crossing contracts)
    pub carriers: Vec<CarrierSlot>,

    /// Exit contract (presence and payload of break/continue/return)
    pub exits: ExitContract,

    /// Captured variables from outer scope (optional)
    pub captured: Option<Vec<CapturedSlot>>,

    /// Source location for debugging
    pub span: Span,
}

/// Skeleton step - Minimal step kinds for loop structure
///
/// Each step represents a fundamental operation in the loop lifecycle.
#[derive(Debug, Clone)]
pub enum SkeletonStep {
    /// Loop continuation condition (the `cond` in `loop(cond)`)
    HeaderCond { expr: Box<ASTNode> },

    /// Early exit check (`if cond { break }`)
    BreakCheck { cond: Box<ASTNode>, has_value: bool },

    /// Skip check (`if cond { continue }`)
    ContinueCheck { cond: Box<ASTNode> },

    /// Carrier update (`i = i + 1`, etc.)
    Update {
        carrier_name: String,
        update_kind: UpdateKind,
    },

    /// Loop body (all other statements)
    Body { stmts: Vec<ASTNode> },
}

/// Update kind - How a carrier variable is updated
///
/// This categorization helps determine which pattern can handle the loop.
#[derive(Debug, Clone)]
pub enum UpdateKind {
    /// Constant step (`i = i + const`)
    ConstStep { delta: i64 },

    /// Conditional step with numeric deltas
    ///
    /// # Pattern
    ///
    /// ```text
    /// if escape_cond { carrier = carrier + then_delta }
    /// else { carrier = carrier + else_delta }
    /// ```
    ///
    /// # Contract (SSOT - Phase 92 P0)
    ///
    /// ## Invariants (MUST hold, Fail-Fast otherwise):
    /// 1. **Single Update**: Updates the same carrier exactly once per iteration
    /// 2. **Constant Deltas**: Both `then_delta` and `else_delta` are compile-time constants
    /// 3. **Pure Condition**: The escape condition has no side effects
    /// 4. **No Reassignment**: Carrier is not reassigned elsewhere in the loop body
    /// 5. **Deterministic**: Update path is determined solely by escape condition
    ///
    /// ## Supported Use Cases:
    /// - Escape sequence handling (e.g., `if ch == '\\' { i += 2 } else { i += 1 }`)
    /// - Conditional skip patterns (e.g., `if skip { pos += len } else { pos += 1 }`)
    ///
    /// ## Fail-Fast Conditions:
    /// - Multiple updates to the same carrier in one iteration → Error
    /// - Non-constant deltas (e.g., `i += f()`) → Error
    /// - Condition with side effects (e.g., `if mutate() { ... }`) → Error
    /// - Carrier reassignment in body (e.g., `carrier = 0`) → Error
    ///
    /// # Phase 92 P0: Lowering Strategy
    ///
    /// LoopBreak handles ConditionalStep by generating:
    /// ```text
    /// if escape_cond {
    ///     carrier_new = carrier + then_delta
    /// } else {
    ///     carrier_new = carrier + else_delta
    /// }
    /// // PHI merge at loop header: carrier_phi = phi [init, carrier_new]
    /// ```
    ///
    /// Phase 91 P5b: Used for escape sequence handling and similar conditional increments
    /// Phase 92 P0-3: Added condition expression for JoinIR Select generation
    ConditionalStep {
        /// The condition expression (e.g., `ch == '\\'`)
        cond: Box<ASTNode>,
        /// Delta for then branch (when condition is true)
        then_delta: i64,
        /// Delta for else branch (when condition is false)
        else_delta: i64,
    },

    /// Conditional update with AST expressions (`if cond { x = a } else { x = b }`)
    Conditional {
        then_value: Box<ASTNode>,
        else_value: Box<ASTNode>,
    },

    /// Arbitrary update (everything else)
    Arbitrary,
}

/// Carrier slot - A loop variable with its role and update rule
///
/// Carriers are variables that are updated in each iteration
/// and need to cross loop boundaries (via PHI nodes in MIR).
#[derive(Debug, Clone)]
pub struct CarrierSlot {
    pub name: String,
    pub role: CarrierRole,
    pub update_kind: UpdateKind,
}

/// Carrier role - The semantic role of a carrier variable
///
/// This helps determine the appropriate pattern and PHI structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarrierRole {
    /// Loop counter (the `i` in `i < n`)
    Counter,

    /// Accumulator (the `sum` in `sum += x`)
    Accumulator,

    /// Condition variable (the `is_valid` in `while(is_valid)`)
    ConditionVar,

    /// Derived value (e.g., `digit_pos` computed from other carriers)
    Derived,
}

/// Captured slot - An outer variable used within the loop
///
/// These are read-only references to variables defined outside the loop.
/// (Write access would make them carriers instead.)
#[derive(Debug, Clone)]
pub struct CapturedSlot {
    pub name: String,
    pub is_mutable: bool,
}

// ============================================================================
// Exit Contract
// ============================================================================

/// Exit contract - What kinds of exits the loop has
///
/// This determines the exit line architecture needed.
#[derive(Debug, Clone)]
pub struct ExitContract {
    pub has_break: bool,
    pub has_continue: bool,
    pub has_return: bool,
    pub break_has_value: bool,
}

// ============================================================================
// Implementation Helpers
// ============================================================================

impl LoopSkeleton {
    /// Create a new empty skeleton
    pub fn new(span: Span) -> Self {
        Self {
            steps: Vec::new(),
            carriers: Vec::new(),
            exits: ExitContract::default(),
            captured: None,
            span,
        }
    }

    /// Count the number of break checks in this skeleton
    pub fn count_break_checks(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| matches!(s, SkeletonStep::BreakCheck { .. }))
            .count()
    }

    /// Count the number of continue checks in this skeleton
    pub fn count_continue_checks(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| matches!(s, SkeletonStep::ContinueCheck { .. }))
            .count()
    }

    /// Get all carrier names
    pub fn carrier_names(&self) -> Vec<&str> {
        self.carriers.iter().map(|c| c.name.as_str()).collect()
    }
}

impl ExitContract {
    /// Create a contract with no exits
    pub fn none() -> Self {
        Self {
            has_break: false,
            has_continue: false,
            has_return: false,
            break_has_value: false,
        }
    }

    /// Check if any exit exists
    pub fn has_any_exit(&self) -> bool {
        self.has_break || self.has_continue || self.has_return
    }
}

impl Default for ExitContract {
    fn default() -> Self {
        Self::none()
    }
}

impl std::fmt::Display for CarrierRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CarrierRole::Counter => write!(f, "Counter"),
            CarrierRole::Accumulator => write!(f, "Accumulator"),
            CarrierRole::ConditionVar => write!(f, "ConditionVar"),
            CarrierRole::Derived => write!(f, "Derived"),
        }
    }
}
