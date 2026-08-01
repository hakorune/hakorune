use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootResolvedDemandV1,
    ScriptRootRuntimeDispositionV1, ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1,
    ScriptTransferredBoundaryV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};
use crate::parser::NyashParser;

use super::super::program_declaration_facts::PreparedNormalProgramDeclarationFactsV1;
use super::PreparedNormalDefaultProgramRootV1;

fn source() -> crate::ast::ASTNode {
    NyashParser::parse_from_string(
        "enum Flag { On(i64)\nOff }\nlocal value = Flag::On(1)\nmatch value { On(value) => value\nOff => null }",
    )
    .expect("direct enum match source")
}

fn window() -> VerifiedScriptRootDemandWindowV1 {
    let entry = |index, semantic| {
        VerifiedScriptRootDemandEntryV1::new(
            SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(index))
                .stmt(),
            semantic,
            ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
        )
    };
    VerifiedScriptRootDemandWindowV1::seal(
        vec![
            entry(
                0,
                ScriptRootSemanticDispositionV1::Transferred(
                    ScriptTransferredBoundaryV1::ProgramEnumDeclaration,
                ),
            ),
            entry(
                1,
                ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
            ),
            entry(
                2,
                ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::LexicalCore),
            ),
        ],
        3,
    )
    .expect("enum match window")
}

#[test]
fn direct_enum_match_seals_only_its_scrutinee_receipt() {
    let program = source();
    let prepared = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let facts = PreparedNormalProgramDeclarationFactsV1::collect(prepared.source_ast());
    let view = ScriptSyntaxViewV1::from_program(prepared.source_ast()).expect("Script view");
    let window = window();
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
    let ResolveScriptOutcomeV1::Complete(product) = outcome.expect("direct EnumMatch resolve") else {
        panic!("direct EnumMatch must complete");
    };
    let sealed = super::VerifiedScriptSemanticSourceV1::seal(&prepared, product, &window)
        .expect("EnumMatch source seal");
    let demands = sealed.enum_match_demands().collect::<Vec<_>>();
    assert_eq!(demands.len(), 1);
    assert_eq!(
        demands[0].node().segments(),
        &[
            SourcePathSegmentV1::ProgramBodyRoot,
            SourcePathSegmentV1::ProgramBody(2),
        ]
    );
}

#[test]
fn direct_enum_match_selected_pipeline_matches_legacy() {
    let program = source();
    let legacy = MirCompiler::with_options(false)
        .compile_with_source(program.clone(), Some("script-enum-match.hako"))
        .expect("legacy direct EnumMatch");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-enum-match.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("selected direct EnumMatch");
    let printer = MirPrinter::new();
    assert_eq!(printer.print_module(&normal.module), printer.print_module(&legacy.module));
    assert_eq!(normal.verification_result, legacy.verification_result);
}
