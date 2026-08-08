//! Passive frontend AST crate scaffold.
//!
//! This crate is the future owner for parser-independent AST data. It must not
//! depend on parser, runtime, backend, MIR, or Box implementations.
//!
//! Current row is scaffold-only: the main crate still owns the active AST
//! implementation until a later compatibility facade wiring row.

pub const FRONTEND_AST_CRATE_READY: bool = true;

mod ast_node;
mod attrs;
mod box_method_inventory;
mod build_predicate;
mod decls;
mod literal;
mod node_wrappers;
mod operators;
pub mod result_option_prelude;
pub mod rune_profile;
mod span;
mod utils;
pub use ast_node::{
    ASTNode, CatchClause, CheckItem, ContractClause, EnumMatchArm, EnumVariantDecl, FieldDecl,
};
pub use attrs::{DeclarationAttrs, RuneAttr};
pub use box_method_inventory::{
    BoxMemberGateSiteV1, BoxMethodCompatibilityOriginV1, BoxMethodDeclarationTransformErrorV1,
    BoxMethodEntryV1, BoxMethodGateSelectionV1, BoxMethodGeneratedProvenanceV1,
    BoxMethodInventoryErrorV1, BoxMethodInventoryOrdinalV1, BoxMethodInventoryPlacementReceiptV1,
    BoxMethodInventoryRoundtripRowV2, BoxMethodInventoryV1, BoxMethodProvenanceV1,
    BoxMethodSourceSelectionV1, PreparedBoxMethodInventoryAppendV1,
    PreparedBoxMethodInventoryRoundtripV2, PreparedGeneratedBoxMethodBatchV1,
    PreparedGeneratedBoxMethodV1,
};
pub use build_predicate::BuildPredicate;
pub use decls::{
    ContractKind, DelegateDecl, DelegateDeclarationProvenanceV1, DelegateExposeDecl,
    DelegateSelectionErrorV1, ParamDecl, TransitionDecl,
};
pub use literal::LiteralValue;
pub use node_wrappers::*;
pub use operators::{BinaryOperator, UnaryOperator};
pub use span::Span;

/// Passive boundary marker for the first frontend AST split.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrontendAstBoundary;

impl FrontendAstBoundary {
    pub const fn name(self) -> &'static str {
        "hakorune-frontend-ast"
    }
}
