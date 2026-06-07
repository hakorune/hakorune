use super::*;
use crate::ast::{BinaryOperator, LiteralValue};

fn span() -> Span {
    Span::unknown()
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: span(),
    }
}

fn int_lit(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

fn mem_addr(arg: ASTNode) -> ASTNode {
    ASTNode::FunctionCall {
        name: "mem.addr".to_string(),
        arguments: vec![arg],
        span: span(),
    }
}

fn bin(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
        span: span(),
    }
}

fn index(target: ASTNode, idx: ASTNode) -> ASTNode {
    ASTNode::Index {
        target: Box::new(target),
        index: Box::new(idx),
        span: span(),
    }
}

fn field(object: ASTNode, name: &str) -> ASTNode {
    ASTNode::FieldAccess {
        object: Box::new(object),
        field: name.to_string(),
        span: span(),
    }
}

fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(target),
        value: Box::new(value),
        span: span(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: Vec::new(),
        span: span(),
    }
}

fn bool_lit(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: span(),
    }
}

#[test]
fn fastmem_source_lowers_to_region_metadata_and_memops() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_test/0".to_string());
    let body = vec![
        local("ptr", int_lit(4096)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("addr", mem_addr(var("ptr"))),
                local(
                    "key",
                    bin(
                        BinaryOperator::BitAnd,
                        bin(BinaryOperator::Shr, var("addr"), int_lit(12)),
                        int_lit(255),
                    ),
                ),
            ],
            span: span(),
        },
    ];

    super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    assert_eq!(function.metadata.fastmem_regions.len(), 1);
    let region = &function.metadata.fastmem_regions[0];
    assert_eq!(region.contract, "PageMapV0");
    assert_eq!(region.body_statement_count, 2);
    assert_eq!(region.emitted_memop_count, 3);

    let kinds: Vec<MemOpKind> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect();
    assert_eq!(
        kinds,
        vec![MemOpKind::AddrOf, MemOpKind::LogicalShr, MemOpKind::BitAnd]
    );
}

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

    let err = super::super::stmts::block_stmt::build_block(&mut builder, body)
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
                local("current", ASTNode::FunctionCall {
                    name: "mem.currentAllocOwnerId".to_string(),
                    arguments: Vec::new(),
                    span: span(),
                }),
                local("same_owner", ASTNode::FunctionCall {
                    name: "mem.ownerEq".to_string(),
                    arguments: vec![field(var("page"), "owner_worker_id"), var("current")],
                    span: span(),
                }),
                ASTNode::If {
                    condition: Box::new(var("same_owner")),
                    then_body: vec![assign(field(var("page"), "used"), int_lit(1))],
                    else_body: Some(vec![
                        local("drained", ASTNode::FunctionCall {
                            name: "mem.atomicRemoteHeadDrain".to_string(),
                            arguments: vec![var("page")],
                            span: span(),
                        }),
                        ASTNode::FunctionCall {
                            name: "mem.drainRemoteListToLocal".to_string(),
                            arguments: vec![var("page"), var("drained")],
                            span: span(),
                        },
                    ]),
                    span: span(),
                },
            ],
            span: span(),
        },
    ];

    super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
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
        MemOpKind::FieldLoad,
        MemOpKind::OwnerEq,
        MemOpKind::FieldStore,
        MemOpKind::AtomicRemoteHeadDrain,
        MemOpKind::DrainRemoteListToLocal,
    ] {
        assert_eq!(
            kinds.iter().filter(|actual| **actual == kind).count(),
            1,
            "kind={:?} all={:?}",
            kind,
            kinds
        );
    }
    let branch_count = function
        .blocks
        .values()
        .filter(|block| matches!(block.terminator, Some(MirInstruction::Branch { .. })))
        .count();
    assert_eq!(branch_count, 1);
}

#[test]
fn fastmem_layout_table_source_preserves_symbolic_access_ids() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_access/0".to_string());
    let body = vec![
        local("page_table", int_lit(8192)),
        local("key", int_lit(3)),
        local("ptr", int_lit(12288)),
        ASTNode::FastMemRegion {
            contract: "PageMapV0".to_string(),
            body: vec![
                local("page", index(var("page_table"), var("key"))),
                local("owner", field(var("page"), "owner_id")),
                assign(field(var("page"), "local_free_head"), var("ptr")),
            ],
            span: span(),
        },
    ];

    super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let access_entries: Vec<(MemOpKind, Option<String>, Option<String>)> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, access, .. } => Some((
                *kind,
                access.as_ref().and_then(|access| access.table_id.clone()),
                access.as_ref().and_then(|access| access.field_id.clone()),
            )),
            _ => None,
        })
        .collect();

    assert_eq!(
        access_entries,
        vec![
            (MemOpKind::TableIndex, Some("page_table".to_string()), None,),
            (MemOpKind::FieldLoad, None, Some("owner_id".to_string())),
            (
                MemOpKind::FieldStore,
                None,
                Some("local_free_head".to_string()),
            ),
        ]
    );
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

    super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let kinds: Vec<MemOpKind> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect();

    assert_eq!(
        kinds,
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

    super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let kinds: Vec<MemOpKind> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect();

    assert_eq!(kinds, vec![MemOpKind::TableIndex, MemOpKind::FreeHeadPop]);
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

    super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let kinds: Vec<MemOpKind> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect();

    assert_eq!(kinds, vec![MemOpKind::TableIndex, MemOpKind::FreeHeadPush]);
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

    super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let kinds: Vec<MemOpKind> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect();

    assert_eq!(
        kinds,
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

    super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let kinds: Vec<MemOpKind> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect();

    assert_eq!(
        kinds,
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

    super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
    let kinds: Vec<MemOpKind> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect();

    assert_eq!(
        kinds,
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

    super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
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
