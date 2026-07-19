use super::super::verify_with_normalized_test_view_at;
use crate::ast::FieldDecl;
use crate::mir::builder::MirBuilder;
use crate::mir::verification::MirVerifier;
use crate::mir::{MirCompiler, MirFunction, MirInstruction, MirModule, MirType};
use crate::parser::NyashParser;
use hakorune_mir_core::MirValueKind;
use std::sync::Once;

const SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/apps/current-receiver-declared-field-proof/main.hako"
));

const CASES: [(&str, &str); 4] = [
    (
        "DeclaredFieldOwnerV1.declfield_probe_v1_direct_array/1",
        "R",
    ),
    (
        "DeclaredFieldOwnerV1.declfield_probe_v1_after_validation/2",
        "P[R,R]",
    ),
    (
        "DeclaredFieldOwnerV1.declfield_probe_v1_after_nested_validation/2",
        "R",
    ),
    (
        "DeclaredFieldOwnerV1.declfield_probe_v1_through_receiver_alias/1",
        "R",
    ),
];

#[test]
fn p0_real_declfield_fixture_has_exact_same_root_shapes() {
    ensure_ring0_initialized();
    let ast = NyashParser::parse_from_string(SOURCE).expect("parse DECLFIELD0 fixture");
    let module = MirCompiler::new()
        .compile(ast)
        .expect("compile DECLFIELD0 fixture")
        .module;

    for (function_name, expected) in CASES {
        let function = module
            .functions
            .get(function_name)
            .unwrap_or_else(|| panic!("missing fixture function {function_name}"));
        MirVerifier::new()
            .verify_function(function)
            .unwrap_or_else(|errors| {
                panic!("final fixture function must verify: {function_name}: {errors:?}")
            });

        let shapes = item_field_base_shapes(&module, function);
        assert_eq!(
            shapes.len(),
            2,
            "{function_name} must retain push/length FieldGet evidence"
        );
        assert!(
            shapes.iter().all(|shape| shape == expected),
            "{function_name} expected {expected}, actual {shapes:?}"
        );
    }
}

fn item_field_base_shapes(module: &MirModule, function: &MirFunction) -> Vec<String> {
    let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
    block_ids.sort();
    let mut shapes = Vec::new();
    for block_id in block_ids {
        let block = function.blocks.get(&block_id).expect("fixture block");
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            let MirInstruction::FieldGet { base, field, .. } = instruction else {
                continue;
            };
            if field != "items" {
                continue;
            }
            let builder = rehydrate_builder(module, function, block_id);
            let (_, shape) =
                verify_with_normalized_test_view_at(&builder, *base, block_id, instruction_index)
                    .unwrap_or_else(|error| {
                        panic!(
                            "same-root proof failed at {} block={} instruction={}: {:?}",
                            function.signature.name, block_id.0, instruction_index, error
                        )
                    });
            shapes.push(shape);
        }
    }
    shapes
}

fn rehydrate_builder(
    module: &MirModule,
    function: &MirFunction,
    use_block: crate::mir::BasicBlockId,
) -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.function_state.type_ctx.value_types = function.metadata.value_types.clone();
    for (index, parameter) in function.params.iter().copied().enumerate() {
        builder.register_value_kind(parameter, MirValueKind::Parameter(index as u32));
    }

    let receiver = *function.params.first().expect("receiver parameter");
    let owner = match function.signature.params.first() {
        Some(MirType::Box(owner)) => owner.clone(),
        other => panic!("fixture receiver must have exact Box owner: {other:?}"),
    };
    builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .insert(receiver, owner);

    for (box_name, declarations) in &module.metadata.user_box_field_decls {
        builder.comp_ctx.register_user_box_with_field_decls(
            box_name.clone(),
            declarations
                .iter()
                .map(|declaration| FieldDecl {
                    name: declaration.name.clone(),
                    declared_type_name: declaration.declared_type_name.clone(),
                    is_weak: declaration.is_weak,
                    default_value: None,
                })
                .collect(),
        );
    }
    builder.function_state.current_function = Some(function.clone());
    builder.function_state.current_block = Some(use_block);
    builder
}

fn ensure_ring0_initialized() {
    use crate::runtime::ring0::{default_ring0, init_global_ring0};
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        init_global_ring0(default_ring0());
    });
}
