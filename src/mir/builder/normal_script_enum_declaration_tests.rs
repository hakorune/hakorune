use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1, ScriptTransferredBoundaryV1,
    SourcePathSegmentV1, SourcePathV1, VerifiedScriptRootDemandEntryV1,
    VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, NormalCompileRequestV1};
use crate::parser::NyashParser;

use super::PreparedNormalDefaultProgramRootV1;

#[test]
fn enum_declaration_is_a_complete_script_transfer_boundary() {
    let program =
        NyashParser::parse_from_string("enum Choice { No, Yes }").expect("enum declaration source");
    let source = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let entry = VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .stmt(),
        ScriptRootSemanticDispositionV1::Transferred(
            ScriptTransferredBoundaryV1::ProgramEnumDeclaration,
        ),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    );
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![entry], 1).expect("enum window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        FunctionSemanticResolverSessionV1::new(0)
            .expect("resolver")
            .resolve_script(view, &window)
            .expect("enum transfer resolve"),
        ResolveScriptOutcomeV1::Complete(_)
    ));
}

#[test]
fn enum_declaration_completes_the_selected_script_with_void() {
    MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                NyashParser::parse_from_string("enum Choice { No, Yes }")
                    .expect("enum declaration source"),
                Some("script-enum-declaration-completion.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("enum declaration must complete as selected Script Void");
}
