use super::*;
use crate::mir::function::FastMemBranchConditionProofKind;
use crate::mir::instruction::MemOpKind;
use crate::mir::MirInstruction;

#[test]
fn fastmem_source_rejects_non_owner_eq_branch_cfg_condition() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_branch_closed/0".to_string());
    let body = vec![ASTNode::FastMemRegion {
        contract: "PageMapV0".to_string(),
        body: vec![ASTNode::If {
            condition: Box::new(bool_lit(true)),
            then_body: vec![local("then_value", int_lit(1))],
            else_body: Some(vec![local("else_value", int_lit(0))]),
            span: span(),
        }],
        span: span(),
    }];

    let err = super::super::super::stmts::block_stmt::build_block(&mut builder, body)
        .expect_err("fastmem branch CFG must stay closed");
    assert!(
        err.contains("[freeze:contract][fastmem/branch_cfg_requires_owner_eq_condition]"),
        "unexpected error: {}",
        err
    );
}

#[test]
fn fastmem_source_lowers_owner_eq_branch_cfg_pilot() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_owner_eq_branch_cfg/0".to_string());
    let body = vec![
        local("page_table", int_lit(8192)),
        local("key", int_lit(3)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("page", index(var("page_table"), var("key"))),
                local(
                    "current",
                    ASTNode::FunctionCall {
                        name: "mem.currentAllocOwnerId".to_string(),
                        arguments: Vec::new(),
                        span: span(),
                    },
                ),
                local(
                    "same_owner",
                    ASTNode::FunctionCall {
                        name: "mem.ownerEq".to_string(),
                        arguments: vec![field(var("page"), "owner_worker_id"), var("current")],
                        span: span(),
                    },
                ),
                ASTNode::If {
                    condition: Box::new(var("same_owner")),
                    then_body: vec![assign(field(var("page"), "used"), int_lit(1))],
                    else_body: Some(vec![local("else_value", int_lit(0))]),
                    span: span(),
                },
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    assert_eq!(function.metadata.fastmem_branch_condition_facts.len(), 1);
    assert_eq!(
        function.metadata.fastmem_branch_condition_facts[0].proof_kind,
        FastMemBranchConditionProofKind::SourceAssumeOwnerEq
    );
    assert!(function.metadata.fastmem_branch_condition_facts[0].owner_eq_required);
    let kinds: Vec<MemOpKind> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect();
    for kind in [
        MemOpKind::TableIndex,
        MemOpKind::CurrentAllocOwnerId,
        MemOpKind::OwnerEq,
        MemOpKind::FieldLoad,
        MemOpKind::FieldStore,
    ] {
        assert_eq!(
            kinds.iter().filter(|actual| **actual == kind).count(),
            1,
            "kind={:?} all={:?}",
            kind,
            kinds
        );
    }
    let field_memops: Vec<(MemOpKind, String)> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, access, .. }
                if matches!(kind, MemOpKind::FieldLoad | MemOpKind::FieldStore) =>
            {
                access
                    .as_ref()
                    .and_then(|access| access.field_id.clone())
                    .map(|field| (*kind, field))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        field_memops,
        vec![
            (MemOpKind::FieldLoad, "owner_worker_id".to_string()),
            (MemOpKind::FieldStore, "used".to_string()),
        ]
    );
    let legacy_field_access = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .any(|inst| match inst {
            MirInstruction::FieldGet { field, .. } => field == "owner_worker_id",
            MirInstruction::FieldSet { field, .. } => field == "used",
            _ => false,
        });
    assert!(
        !legacy_field_access,
        "FastMem field access must stay on the MemOp surface"
    );
    let branch_count = function
        .blocks
        .values()
        .filter(|block| matches!(block.terminator, Some(MirInstruction::Branch { .. })))
        .count();
    assert_eq!(branch_count, 1);
}

#[test]
fn fastmem_source_lowers_direct_owner_eq_branch_cfg_pilot() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_direct_owner_eq_branch_cfg/0".to_string());
    let body = vec![
        local("page_table", int_lit(8192)),
        local("key", int_lit(3)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("page", index(var("page_table"), var("key"))),
                local(
                    "current",
                    ASTNode::FunctionCall {
                        name: "mem.currentAllocOwnerId".to_string(),
                        arguments: Vec::new(),
                        span: span(),
                    },
                ),
                ASTNode::If {
                    condition: Box::new(ASTNode::FunctionCall {
                        name: "mem.ownerEq".to_string(),
                        arguments: vec![field(var("page"), "owner_worker_id"), var("current")],
                        span: span(),
                    }),
                    then_body: vec![assign(field(var("page"), "used"), int_lit(1))],
                    else_body: Some(vec![local("else_value", int_lit(0))]),
                    span: span(),
                },
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    assert_eq!(function.metadata.fastmem_branch_condition_facts.len(), 1);
    assert_eq!(
        function.metadata.fastmem_branch_condition_facts[0].proof_kind,
        FastMemBranchConditionProofKind::SourceAssumeOwnerEq
    );
    assert!(function.metadata.fastmem_branch_condition_facts[0].owner_eq_required);
    let branch_count = function
        .blocks
        .values()
        .filter(|block| matches!(block.terminator, Some(MirInstruction::Branch { .. })))
        .count();
    assert_eq!(branch_count, 1);
}
