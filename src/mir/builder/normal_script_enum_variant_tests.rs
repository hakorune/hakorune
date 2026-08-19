use crate::ast::{ASTNode, Span};
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootResolvedDemandV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1,
    SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
use crate::parser::NyashParser;
use std::collections::HashMap;

use super::super::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use super::PreparedNormalDefaultProgramRootV1;

fn source() -> ASTNode {
    NyashParser::parse_from_string("enum Flag { On(i64) }\nFlag::On(7)")
        .expect("enum variant source")
}

fn window() -> VerifiedScriptRootDemandWindowV1 {
    VerifiedScriptRootDemandWindowV1::seal(
        vec![
            VerifiedScriptRootDemandEntryV1::new(
                SourcePathV1::program_body()
                    .child(SourcePathSegmentV1::ProgramBody(0))
                    .stmt(),
                ScriptRootSemanticDispositionV1::Transferred(
                    crate::mir::resolved_semantics::ScriptTransferredBoundaryV1::ProgramEnumDeclaration,
                ),
                ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
            ),
            VerifiedScriptRootDemandEntryV1::new(
                SourcePathV1::program_body()
                    .child(SourcePathSegmentV1::ProgramBody(1))
                    .stmt(),
                ScriptRootSemanticDispositionV1::Resolved(
                    ScriptRootResolvedDemandV1::LexicalCore,
                ),
                ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
            ),
        ],
        2,
    )
    .expect("enum variant window")
}

#[test]
fn enum_variant_receipt_is_complete_and_projects_its_argument() {
    let program = source();
    let prepared = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let view = ScriptSyntaxViewV1::from_program(prepared.source_ast()).expect("Script view");
    let facts = PreparedNormalProgramDeclarationFactsV1::collect(prepared.source_ast())
        .expect("declaration facts");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let outcome = facts.with_record_schema_demand_view(|records| {
        facts.with_enum_variant_demand_view(|variants| {
            facts.with_enum_match_demand_view(|matches| {
                resolver.resolve_script_with_declaration_views(
                    view,
                    &window(),
                    records,
                    variants,
                    matches,
                )
            })
        })
    });
    let ResolveScriptOutcomeV1::Complete(product) = outcome.expect("enum resolve") else {
        panic!("direct enum variant must complete")
    };
    let sealed = super::VerifiedScriptSemanticSourceV1::seal(&prepared, product, &window())
        .expect("enum semantic source");
    let demands = sealed.enum_variant_demands().collect::<Vec<_>>();
    assert_eq!(demands.len(), 1);
    assert_eq!(demands[0].1.argument_count(), 1);
    assert_eq!(
        demands[0].0.node().segments(),
        &[
            SourcePathSegmentV1::ProgramBodyRoot,
            SourcePathSegmentV1::ProgramBody(1),
        ]
    );
    assert!(matches!(
        crate::mir::resolved_semantics::project_source_node_v1(
            prepared.source_ast(),
            SourcePathV1::from_node(demands[0].0.node())
                .child(SourcePathSegmentV1::Argument(0))
                .expr()
                .node()
        ),
        Some(crate::mir::resolved_semantics::ProjectedSourceNodeV1::Node(
            ASTNode::Literal { .. }
        ))
    ));
}

#[test]
fn enum_variant_selected_pipeline_matches_legacy() {
    let program = source();
    let legacy = MirCompiler::with_options(false)
        .compile_with_source(program.clone(), Some("script-enum-variant.hako"))
        .expect("legacy enum variant");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-enum-variant.hako"),
                HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("selected enum variant");
    let printer = MirPrinter::new();
    let rendered = printer.print_module(&normal.module);
    assert!(rendered.contains("variant.make"), "{rendered}");
    assert_eq!(rendered, printer.print_module(&legacy.module));
    assert_eq!(normal.verification_result, legacy.verification_result);
}

#[test]
fn ordinary_from_call_stays_deferred() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::FromCall {
            parent: "NotAnEnum".to_owned(),
            method: "build".to_owned(),
            arguments: Vec::new(),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let prepared = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let view = ScriptSyntaxViewV1::from_program(prepared.source_ast()).expect("Script view");
    let entry = VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    );
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![entry], 1).expect("window");
    let facts = PreparedNormalProgramDeclarationFactsV1::collect(prepared.source_ast())
        .expect("declaration facts");
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let outcome = facts.with_record_schema_demand_view(|records| {
        facts.with_enum_variant_demand_view(|variants| {
            facts.with_enum_match_demand_view(|matches| {
                resolver.resolve_script_with_declaration_views(
                    view, &window, records, variants, matches,
                )
            })
        })
    });
    assert!(matches!(
        outcome.expect("ordinary FromCall selection"),
        ResolveScriptOutcomeV1::Deferred
    ));
}
