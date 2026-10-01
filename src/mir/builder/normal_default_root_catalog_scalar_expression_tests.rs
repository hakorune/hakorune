//! Source-backed pure scalar roots consume exact field ports in existing operators.
use super::normal_default_root_catalog_lifecycle_tests::{callable_source, session};
use crate::mir::builder::{CallableMainMaterializationPolicyV1, NormalRuntimeInputSnapshotV1};
use crate::mir::{MirInstruction, MirType};
use crate::parser::ParserBuildConfig;

fn lower(body: &str) -> crate::mir::MirModule {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source = callable_source(
        &format!(
            "box Pool {{ size: i64 = 0 birth() {{ }} }}
        static box Main {{ main() {{ local pool = new Pool() {body} }} }}"
        ),
        ParserBuildConfig::default(),
    );
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            CallableMainMaterializationPolicyV1::Omitted,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("source-backed scalar root must lower");
    completed.into_parts().1
}

#[test]
fn scalar_expression_emits_typed_field_reads_and_arithmetic() {
    let module = lower("local n = pool.size + 1 return n");
    let main = module
        .functions
        .values()
        .find(|f| f.signature.name == "main")
        .expect("main");
    let reads: Vec<_> = main
        .blocks
        .values()
        .flat_map(|b| b.all_instructions())
        .filter_map(|i| match i {
            MirInstruction::ObjectFieldGet { dst, field, .. } => Some((*dst, *field)),
            _ => None,
        })
        .collect();
    assert_eq!(reads.len(), 1);
    assert_eq!(reads[0].1.declaration_ordinal(), 0);
    assert_eq!(
        main.metadata.value_types.get(&reads[0].0),
        Some(&MirType::Integer)
    );
    let instructions: Vec<_> = main
        .blocks
        .values()
        .flat_map(|b| b.all_instructions())
        .collect();
    let copy_origin = |mut value| {
        for _ in 0..=instructions.len() {
            let Some(source) = instructions.iter().find_map(|i| match i {
                MirInstruction::Copy { dst, src } if *dst == value => Some(*src),
                _ => None,
            }) else {
                return value;
            };
            value = source;
        }
        panic!("copy cycle in source-backed scalar arithmetic");
    };
    assert!(
        instructions.iter().any(|i| matches!(i,
            MirInstruction::BinOp { op: crate::mir::BinaryOp::Add, lhs, rhs, .. }
                if copy_origin(*lhs) == reads[0].0 || copy_origin(*rhs) == reads[0].0
        )),
        "Add must consume the exact read, possibly through source copies: {instructions:#?}"
    );
    assert!(!main
        .blocks
        .values()
        .flat_map(|b| b.all_instructions())
        .any(|i| matches!(i, MirInstruction::FieldGet { .. })));
}

#[test]
fn scalar_expression_short_circuit_rhs_read_stays_in_deferred_block() {
    let module = lower("local b = pool.size == 0 && pool.size != 1 if b { return 1 } return 0");
    let main = module
        .functions
        .values()
        .find(|f| f.signature.name == "main")
        .expect("main");
    let blocks: Vec<_> = main
        .blocks
        .iter()
        .flat_map(|(id, b)| {
            b.all_instructions().filter_map(move |i| {
                matches!(i, MirInstruction::ObjectFieldGet { .. }).then_some(*id)
            })
        })
        .collect();
    assert_eq!(blocks.len(), 2);
    assert_ne!(
        blocks[0], blocks[1],
        "RHS must not be hoisted into LHS block"
    );
    assert!(!main
        .blocks
        .values()
        .flat_map(|b| b.all_instructions())
        .any(|i| matches!(i, MirInstruction::FieldGet { .. })));
}
