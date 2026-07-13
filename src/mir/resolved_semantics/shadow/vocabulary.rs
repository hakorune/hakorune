//! Exhaustive SA1 accepted syntax inventory.
//!
//! Anything outside these lists must produce a typed Unsupported outcome.

pub(super) const SHADOW_ACCEPTED_STATEMENTS_V0: &[&str] = &[
    "Local",
    "Outbox",
    "Assignment",
    "ScopeBox",
    "If",
    "Loop",
    "Break",
    "Continue",
    "Return",
    "ClosedExpressionStatement",
];

pub(super) const SHADOW_ACCEPTED_EXPRESSIONS_V0: &[&str] = &[
    "Literal",
    "Variable",
    "Me",
    "This",
    "UnaryOp",
    "BinaryOp",
    "MethodCall",
    "FieldAccess",
    "Index",
    "FunctionCall",
    "New",
];

pub(super) const SHADOW_ACCEPTED_ASSIGNMENT_TARGETS_V0: &[&str] =
    &["Variable", "FieldAccess", "Index"];
