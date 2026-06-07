use super::*;
use crate::mir::function::{FastMemBlockNextProofKind, FastMemRemoteOwnerProofKind};
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
