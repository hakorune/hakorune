//! Main0 derived-predicate physical lowerer canary.
//!
//! Test-only harness proving the one-way bridge from the installed App Main
//! source loan through selection, full operation demand, the shared segment
//! allocator/dispatcher, After, tail Completion, and DraftSeal — all inside
//! the one-shot `with_app_main_root_lowering_input` scope.

#![cfg(test)]

use super::main0_derived_predicate_lowerer::lower_main0_derived_predicate_function_draft_v1;
use crate::mir::builder::normal_main0_derived_predicate_prepared_operation::PreparedMain0DerivedPredicateLoopIngressV1;
use crate::mir::builder::CompilationContext;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::main0_derived_predicate_root_selection::{
    select_main0_derived_predicate_source_v1, Main0DerivedPredicateSourceSelectionV1,
};
use crate::mir::normal_callable_semantic_package::{
    issue_normal_callable_semantic_package_v1, InstalledNormalCallableSemanticPackageV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

const DERIVED_PREDICATE_SOURCE: &str = r#"
static box Main {
    main() {
        local j = 0
        local m = 0
        local n = 3
        loop(j + m <= n) {
            j = j + 1
        }
        return j
    }
}
"#;

fn final_source(source: &str) -> VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("handoff source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("source-backed transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

fn installed_package(source: &str) -> (InstalledNormalCallableSemanticPackageV1, CompilationContext) {
    let mut resolver = FunctionSemanticResolverSessionV1::new(7001).expect("resolver");
    let package = issue_normal_callable_semantic_package_v1(&mut resolver, final_source(source))
        .expect("source-backed package");
    let mut context = CompilationContext::new();
    let installed = package
        .prepare_install(&mut context)
        .expect("vacant catalog")
        .commit();
    (installed, context)
}

fn app_main_relation(
    context: &CompilationContext,
) -> (
    crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    crate::parser::CallableDeclarationIdentityV1,
) {
    let catalog = context
        .callable_declaration_catalog()
        .expect("installed catalog");
    let app_main = catalog
        .source_backed_app_main()
        .expect("source-backed App Main");
    (
        app_main.catalog_key().clone(),
        app_main.parser_identity().clone(),
    )
}

/// Observation records extracted while the draft is still open, plus the
/// committed function for the projected Return check.
#[derive(Debug)]
struct LoweredDraftObservationV1 {
    block_count: usize,
    add_count: usize,
    phi_backedge_from_add: bool,
    terminal_returns_value: bool,
    predecessor_edges: usize,
}

fn observe_current_function(builder: &MirBuilder) -> (usize, usize, bool, usize) {
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .expect("selected function still open");
    let mut predecessor_edges = 0usize;
    // The profile emits two Adds: the derived predicate add and the carrier
    // step add. Only the step add feeds the header PHI backedge, so all
    // Add results are collected and any PHI input match proves the loop
    // backedge materialized.
    let mut add_values = Vec::new();
    for (block, block_data) in &function.blocks {
        predecessor_edges += block_data.predecessors.len();
        for instruction in &block_data.instructions {
            if let crate::mir::MirInstruction::BinOp {
                op: crate::mir::BinaryOp::Add,
                dst,
                ..
            } = instruction
            {
                add_values.push((*block, *dst));
            }
        }
    }
    let phi_backedge_from_add = function.blocks.values().any(|block_data| {
        block_data.instructions.iter().any(|instruction| {
            matches!(
                instruction,
                crate::mir::MirInstruction::Phi { inputs, .. }
                    if inputs
                        .iter()
                        .any(|(_, value)| add_values.iter().any(|(_, dst)| dst == value))
            )
        })
    });
    (
        function.blocks.len(),
        add_values.len(),
        phi_backedge_from_add,
        predecessor_edges,
    )
}

#[test]
fn main0_derived_predicate_lowers_to_one_draft_seal() {
    let (installed, context) = installed_package(DERIVED_PREDICATE_SOURCE);
    let mut port = installed.begin_lowering(&context).expect("same catalog");
    let (key, identity) = app_main_relation(&context);

    let selection = select_main0_derived_predicate_source_v1(&port, &key, &identity)
        .expect("selection observes the installed source");
    let Main0DerivedPredicateSourceSelectionV1::Selected(product) = selection else {
        panic!("exact Main0 derived-predicate source must be selected")
    };

    let mut builder = MirBuilder::new();
    let observation = port
        .with_app_main_root_lowering_input(&key, &identity, |input, _identity| {
            let ingress = PreparedMain0DerivedPredicateLoopIngressV1::issue(input, product)
                .expect("ingress pairs the lent input with the product");
            let program = ingress.prepare_full_demand().expect("full demand");
            let mut outer =
                builder.open_resolved_function_draft_seal_session_v1("Main.main/0");
            let ready = lower_main0_derived_predicate_function_draft_v1(
                &mut outer,
                program,
                "Main.main/0".to_owned(),
            )
            .expect("Main0 derived-predicate draft");
            let (block_count, add_count, phi_backedge_from_add, predecessor_edges) =
                observe_current_function(outer.builder_view());
            let prepared = match ready.open(outer).prepare() {
                Ok(prepared) => prepared,
                Err(rejected) => {
                    panic!("draft seal prepare rejected: {}", rejected.into_discarded_error())
                }
            };
            // DraftSeal is the only Return projection owner: the committed
            // function now carries the projected tail return.
            let function = prepared.commit().consume_non_authority_evidence();
            let terminal_returns_value = function.blocks.values().any(|block_data| {
                matches!(
                    block_data.terminator,
                    Some(crate::mir::MirInstruction::Return { value: Some(_) })
                )
            });
            LoweredDraftObservationV1 {
                block_count,
                add_count,
                phi_backedge_from_add,
                terminal_returns_value,
                predecessor_edges,
            }
        })
        .expect("loan consumed once by the selected branch");

    assert!(observation.block_count >= 4, "entry+header+body+after");
    assert_eq!(
        observation.add_count, 2,
        "predicate add + carrier step add"
    );
    assert!(
        observation.phi_backedge_from_add,
        "header PHI must receive the carrier Add backedge"
    );
    assert!(
        observation.terminal_returns_value,
        "tail return carries the carrier value"
    );
    assert!(observation.predecessor_edges >= 2, "real CFG edges exist");
}
