use super::*;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
use crate::mir::instruction::MemOpKind;
use crate::mir::MirInstruction;

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

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    assert_eq!(function.metadata.fastmem_regions.len(), 1);
    let region = &function.metadata.fastmem_regions[0];
    assert_eq!(region.contract, "PageMapV0");
    assert_eq!(region.body_statement_count, 2);
    assert_eq!(region.emitted_memop_count, 1);

    let memop_kinds: Vec<MemOpKind> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::MemOp { kind, .. } => Some(*kind),
            _ => None,
        })
        .collect();
    assert_eq!(memop_kinds, vec![MemOpKind::AddrOf]);

    let binop_kinds: Vec<crate::mir::BinaryOp> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|inst| match inst {
            MirInstruction::BinOp { op, .. } => Some(*op),
            _ => None,
        })
        .collect();
    assert_eq!(
        binop_kinds,
        vec![
            crate::mir::BinaryOp::Shr,
            crate::mir::BinaryOp::Shr,
            crate::mir::BinaryOp::BitAnd,
        ]
    );
}

#[test]
fn port_aware_fastmem_body_error_restores_the_outer_region() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("fastmem_error_cleanup/0".to_string());
    let outer = builder
        .register_fastmem_region("OuterV0".to_owned(), span(), 0)
        .expect("outer region");
    builder.push_fastmem_region(outer);
    let mut port = RawLegacyChildLoweringPortV1;

    let error = build_fastmem_region_with_port_v1(
        &mut builder,
        &mut port,
        "InnerV0".to_owned(),
        vec![var("missing_fastmem_child")],
        span(),
    )
    .expect_err("missing FastMem child must reject");

    assert!(
        error.contains("Undefined variable: missing_fastmem_child"),
        "{error}"
    );
    assert_eq!(builder.current_fastmem_region(), Some(outer));
    assert_eq!(
        builder
            .function_state
            .current_function
            .as_ref()
            .expect("function")
            .metadata
            .fastmem_regions
            .len(),
        2,
        "typed failure retains candidate-local metadata until candidate discard"
    );
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

    super::super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
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
            (MemOpKind::TableIndex, Some("page_table".to_string()), None),
            (MemOpKind::FieldLoad, None, Some("owner_id".to_string())),
            (
                MemOpKind::FieldStore,
                None,
                Some("local_free_head".to_string()),
            ),
        ]
    );
    assert_eq!(function.metadata.fastmem_field_access_sites.len(), 2);
    assert!(function
        .metadata
        .fastmem_field_access_sites
        .iter()
        .all(|site| site.region.is_some()));
    assert_eq!(
        function.metadata.fastmem_field_access_sites[0].field_id,
        "owner_id"
    );
    assert_eq!(
        function.metadata.fastmem_field_access_sites[1].field_id,
        "local_free_head"
    );
    let legacy_field_access = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .any(|inst| match inst {
            MirInstruction::FieldGet { field, .. } => field == "owner_id",
            MirInstruction::FieldSet { field, .. } => field == "local_free_head",
            _ => false,
        });
    assert!(
        !legacy_field_access,
        "FastMem field access must stay on the MemOp surface"
    );
}
