//! Compatibility re-export for passive AST declaration metadata.
//!
//! Recursive declaration nodes remain in the main crate with `ASTNode`; simple
//! metadata without ASTNode fields lives in `hakorune-frontend-ast`.

pub use hakorune_frontend_ast::{
    ContractKind, DelegateDecl, DelegateExposeDecl, ParamDecl, TransitionDecl,
};
