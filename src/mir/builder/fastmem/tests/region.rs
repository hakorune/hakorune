use super::*;
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
    let function = builder.scope_ctx.current_function.as_ref().unwrap();
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
        vec![(MemOpKind::TableIndex, Some("page_table".to_string()), None,)]
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
    let field_insts: Vec<&MirInstruction> = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter(|inst| {
            matches!(
                inst,
                MirInstruction::FieldGet { .. } | MirInstruction::FieldSet { .. }
            )
        })
        .collect();
    assert!(field_insts.iter().any(|inst| matches!(
        inst,
        MirInstruction::FieldGet { field, .. } if field == "owner_id"
    )));
    assert!(field_insts.iter().any(|inst| matches!(
        inst,
        MirInstruction::FieldSet { field, .. } if field == "local_free_head"
    )));
}
