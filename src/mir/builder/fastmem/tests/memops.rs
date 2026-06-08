use super::*;
use crate::mir::function::{FastMemBlockNextProofKind, FastMemRemoteOwnerProofKind};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::instruction::MemOpKind;
use crate::mir::MirInstruction;

fn emitted_memop_kinds(function: &crate::mir::MirFunction) -> Vec<MemOpKind> {
    function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect()
}

fn emitted_binop_kinds(function: &crate::mir::MirFunction) -> Vec<crate::mir::BinaryOp> {
    function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::BinOp { op, .. } => Some(*op),
            _ => None,
        })
        .collect()
}

#[test]
fn fastmem_source_emits_local_free_list_memops() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_local_free_list/0".to_string());
    let body = vec![
        local("page_table", int_lit(8192)),
        local("key", int_lit(3)),
        local("block", int_lit(12288)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("page", index(var("page_table"), var("key"))),
                ASTNode::FunctionCall {
                    name: "mem.localFreePush".to_string(),
                    arguments: vec![var("page"), var("block")],
                    span: span(),
                },
                local(
                    "popped",
                    ASTNode::FunctionCall {
                        name: "mem.localFreePop".to_string(),
                        arguments: vec![var("page")],
                        span: span(),
                    },
                ),
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();

    assert_eq!(
        emitted_memop_kinds(function),
        vec![
            MemOpKind::TableIndex,
            MemOpKind::LocalFreePush,
            MemOpKind::LocalFreePop,
        ]
    );
}

#[test]
fn fastmem_source_emits_free_head_pop_memop() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_free_head_pop/0".to_string());
    let body = vec![
        local("page_table", int_lit(8192)),
        local("key", int_lit(3)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("page", index(var("page_table"), var("key"))),
                local(
                    "popped",
                    ASTNode::FunctionCall {
                        name: "mem.freeHeadPop".to_string(),
                        arguments: vec![var("page")],
                        span: span(),
                    },
                ),
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();

    assert_eq!(
        emitted_memop_kinds(function),
        vec![MemOpKind::TableIndex, MemOpKind::FreeHeadPop]
    );
}

#[test]
fn fastmem_source_emits_free_head_push_memop() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_free_head_push/0".to_string());
    let body = vec![
        local("page_table", int_lit(8192)),
        local("key", int_lit(3)),
        local("block", int_lit(12288)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("page", index(var("page_table"), var("key"))),
                ASTNode::FunctionCall {
                    name: "mem.freeHeadPush".to_string(),
                    arguments: vec![var("page"), var("block")],
                    span: span(),
                },
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();

    assert_eq!(
        emitted_memop_kinds(function),
        vec![MemOpKind::TableIndex, MemOpKind::FreeHeadPush]
    );
}

#[test]
fn fastmem_source_emits_atomic_remote_head_push_memop() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_atomic_remote_head_push/0".to_string());
    let body = vec![
        local("page_table", int_lit(8192)),
        local("key", int_lit(3)),
        local("block", int_lit(12288)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("page", index(var("page_table"), var("key"))),
                ASTNode::FunctionCall {
                    name: "mem.atomicRemoteHeadPush".to_string(),
                    arguments: vec![var("page"), var("block")],
                    span: span(),
                },
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();

    assert_eq!(
        emitted_memop_kinds(function),
        vec![MemOpKind::TableIndex, MemOpKind::AtomicRemoteHeadPush]
    );
}

#[test]
fn fastmem_source_emits_atomic_remote_head_drain_memop() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_atomic_remote_head_drain/0".to_string());
    let body = vec![
        local("page_table", int_lit(8192)),
        local("key", int_lit(3)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("page", index(var("page_table"), var("key"))),
                local(
                    "drained",
                    ASTNode::FunctionCall {
                        name: "mem.atomicRemoteHeadDrain".to_string(),
                        arguments: vec![var("page")],
                        span: span(),
                    },
                ),
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();

    assert_eq!(
        emitted_memop_kinds(function),
        vec![MemOpKind::TableIndex, MemOpKind::AtomicRemoteHeadDrain]
    );
}

#[test]
fn fastmem_source_emits_numeric_binary_memops() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_numeric_binary_ops/0".to_string());
    let body = vec![
        local("lhs", int_lit(16)),
        local("rhs", int_lit(3)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("sum", bin(BinaryOperator::Add, var("lhs"), var("rhs"))),
                local(
                    "diff",
                    bin(BinaryOperator::Subtract, var("lhs"), var("rhs")),
                ),
                local("shr", bin(BinaryOperator::Shr, var("lhs"), var("rhs"))),
                local("and", bin(BinaryOperator::BitAnd, var("lhs"), var("rhs"))),
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();

    assert!(emitted_memop_kinds(function).is_empty());
    assert_eq!(
        emitted_binop_kinds(function),
        vec![
            crate::mir::BinaryOp::Add,
            crate::mir::BinaryOp::Sub,
            crate::mir::BinaryOp::Shr,
            crate::mir::BinaryOp::BitAnd,
        ]
    );
}

#[test]
fn fastmem_source_emits_drain_remote_list_to_local_memop() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_drain_remote_list_to_local/0".to_string());
    let body = vec![
        local("page_table", int_lit(8192)),
        local("key", int_lit(3)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("page", index(var("page_table"), var("key"))),
                local(
                    "drained",
                    ASTNode::FunctionCall {
                        name: "mem.atomicRemoteHeadDrain".to_string(),
                        arguments: vec![var("page")],
                        span: span(),
                    },
                ),
                ASTNode::FunctionCall {
                    name: "mem.drainRemoteListToLocal".to_string(),
                    arguments: vec![var("page"), var("drained")],
                    span: span(),
                },
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();

    assert_eq!(
        emitted_memop_kinds(function),
        vec![
            MemOpKind::TableIndex,
            MemOpKind::AtomicRemoteHeadDrain,
            MemOpKind::DrainRemoteListToLocal,
        ]
    );
}

#[test]
fn fastmem_source_records_local_free_precondition_facts() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_local_free_preconditions/0".to_string());
    let body = vec![
        local("page", int_lit(8192)),
        local("block", int_lit(12288)),
        local("same_owner", int_lit(1)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                ASTNode::FunctionCall {
                    name: "mem.assumeSameOwner".to_string(),
                    arguments: vec![var("page"), var("same_owner")],
                    span: span(),
                },
                ASTNode::FunctionCall {
                    name: "mem.assumeLocalFreeBlockNext".to_string(),
                    arguments: vec![var("block")],
                    span: span(),
                },
                ASTNode::FunctionCall {
                    name: "mem.assumeFreeHeadBlockNext".to_string(),
                    arguments: vec![var("block")],
                    span: span(),
                },
                ASTNode::FunctionCall {
                    name: "mem.assumeRemoteOwner".to_string(),
                    arguments: vec![var("page")],
                    span: span(),
                },
                ASTNode::FunctionCall {
                    name: "mem.assumeRemoteFreeBlockNext".to_string(),
                    arguments: vec![var("block")],
                    span: span(),
                },
                ASTNode::FunctionCall {
                    name: "mem.assumeLocalFreeNonEmpty".to_string(),
                    arguments: vec![var("page")],
                    span: span(),
                },
                ASTNode::FunctionCall {
                    name: "mem.assumeFreeHeadNonEmpty".to_string(),
                    arguments: vec![var("page")],
                    span: span(),
                },
                ASTNode::FunctionCall {
                    name: "mem.localFreePush".to_string(),
                    arguments: vec![var("page"), var("block")],
                    span: span(),
                },
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    assert_eq!(function.metadata.fastmem_same_owner_facts.len(), 1);
    assert_eq!(function.metadata.fastmem_remote_owner_facts.len(), 1);
    assert_eq!(function.metadata.fastmem_block_next_facts.len(), 3);
    assert_eq!(
        function.metadata.fastmem_local_free_non_empty_facts.len(),
        1
    );
    assert_eq!(function.metadata.fastmem_free_head_non_empty_facts.len(), 1);
    assert_eq!(
        function.metadata.fastmem_same_owner_facts[0].region,
        FastMemRegionId::new(0)
    );
    assert_eq!(
        function.metadata.fastmem_block_next_facts[0].next_field_id,
        "next"
    );
    assert_eq!(
        function.metadata.fastmem_block_next_facts[0].proof_kind,
        FastMemBlockNextProofKind::SourceAssumeLocalFreeBlockNext
    );
    assert_eq!(
        function.metadata.fastmem_block_next_facts[1].proof_kind,
        FastMemBlockNextProofKind::SourceAssumeFreeHeadBlockNext
    );
    assert_eq!(
        function.metadata.fastmem_remote_owner_facts[0].proof_kind,
        FastMemRemoteOwnerProofKind::SourceAssumeRemoteOwner
    );
    assert_eq!(
        function.metadata.fastmem_block_next_facts[2].proof_kind,
        FastMemBlockNextProofKind::SourceAssumeRemoteFreeBlockNext
    );
    assert!(function.metadata.fastmem_local_free_non_empty_facts[0].non_empty);
    assert!(function.metadata.fastmem_free_head_non_empty_facts[0].non_empty);
}

#[test]
fn fastmem_source_records_access_sites() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_access_sites/0".to_string());
    let body = vec![
        local("page_table", int_lit(8192)),
        local("key", int_lit(3)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("page", index(var("page_table"), var("key"))),
                local("owner", field(var("page"), "owner_worker_id")),
                assign(field(var("page"), "used"), int_lit(1)),
            ],
            span: span(),
        },
    ];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();

    assert_eq!(function.metadata.fastmem_field_access_sites.len(), 2);
    assert_eq!(function.metadata.fastmem_index_access_sites.len(), 1);
    assert!(function
        .metadata
        .fastmem_field_access_sites
        .iter()
        .all(|site| site.region.is_some()));
    assert!(function
        .metadata
        .fastmem_index_access_sites
        .iter()
        .all(|site| site.region.is_some()));
    assert_eq!(
        function.metadata.fastmem_field_access_sites[0].required_route,
        "verified_layout_field"
    );
    assert_eq!(
        function.metadata.fastmem_index_access_sites[0].required_route,
        "verified_table_index"
    );
    assert_eq!(
        function.metadata.fastmem_field_access_sites[0].fallback_policy,
        "forbidden"
    );
    assert_eq!(
        function.metadata.fastmem_index_access_sites[0].fallback_policy,
        "forbidden"
    );

    let instructions: Vec<_> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .collect();
    assert!(instructions.iter().any(|inst| matches!(
        inst,
        MirInstruction::FieldGet { field, .. } if field == "owner_worker_id"
    )));
    assert!(instructions.iter().any(|inst| matches!(
        inst,
        MirInstruction::FieldSet { field, .. } if field == "used"
    )));
    assert!(!instructions.iter().any(|inst| matches!(
        inst,
        MirInstruction::MemOp {
            kind: MemOpKind::FieldLoad | MemOpKind::FieldStore,
            ..
        }
    )));
}

#[test]
fn fastmem_source_shared_shell_accepts_local_without_initializer_and_return() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_shared_shell/0".to_string());
    let body = vec![ASTNode::FastMemRegion {
        contract: "PageMapV0".to_string(),
        body: vec![
            local_no_init("tmp"),
            ASTNode::Return {
                value: Some(Box::new(var("tmp"))),
                span: span(),
            },
        ],
        span: span(),
    }];

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();

    assert!(function
        .blocks
        .values()
        .any(|block| matches!(block.terminator, Some(MirInstruction::Return { .. }))));
}
