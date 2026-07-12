//! A2-C2-P0 inventory for the existing Recipe verifier.
//!
//! These tests intentionally describe gaps.  Block-contract verification
//! owns shape, reference bounds, and ports; it does not currently prove that
//! every statement in a `RecipeBody` is referenced exactly once.

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::recipe_tree::{
    verify_block_contract_with_pre, BlockContractKind, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::builder::control_flow::recipes::RecipeBody;

fn int_stmt(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn existing_verifier_accepts(body: Vec<ASTNode>, refs: &[usize]) -> bool {
    let mut arena = RecipeBodies::new();
    let body_id = arena.register(RecipeBody::new(body));
    let block = RecipeBlock::new(
        body_id,
        refs.iter()
            .copied()
            .map(StmtRef::new)
            .map(RecipeItem::Stmt)
            .collect(),
    );
    verify_block_contract_with_pre(
        &arena,
        &block,
        BlockContractKind::StmtOnly,
        "a2_c2_p0_coverage_inventory",
        None,
    )
    .is_ok()
}

#[test]
fn existing_verifier_does_not_reject_an_omitted_statement() {
    assert!(existing_verifier_accepts(
        vec![int_stmt(1), int_stmt(2)],
        &[0]
    ));
}

#[test]
fn existing_verifier_does_not_reject_a_duplicate_statement_reference() {
    assert!(existing_verifier_accepts(
        vec![int_stmt(1), int_stmt(2)],
        &[0, 0]
    ));
}

#[test]
fn existing_verifier_still_rejects_an_out_of_bounds_reference() {
    assert!(!existing_verifier_accepts(vec![int_stmt(1)], &[1]));
}
