use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::PreparedNormalDefaultProgramRootV1;
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveScriptOutcomeV1, ScriptRootResolvedDemandV1,
    ScriptRootReturnExitAdmissionV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptSyntaxViewV1, SourcePathSegmentV1, SourcePathV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};
use crate::mir::{MirCompiler, MirPrinter, NormalCompileRequestV1};

fn final_return_entry() -> VerifiedScriptRootDemandEntryV1 {
    VerifiedScriptRootDemandEntryV1::new(
        SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .stmt(),
        ScriptRootSemanticDispositionV1::Resolved(ScriptRootResolvedDemandV1::ReturnExit(
            ScriptRootReturnExitAdmissionV1::new(),
        )),
        ScriptRootRuntimeDispositionV1::RetainedExistingTerminal,
    )
}

#[test]
fn final_root_return_seals_script_owner_and_matches_existing_lowering() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(7),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program.clone()).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![final_return_entry()], 1)
        .expect("final Return window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    let outcome = FunctionSemanticResolverSessionV1::new(0)
        .expect("resolver")
        .resolve_script(view, &window)
        .expect("final Return resolve");
    assert!(matches!(outcome, ResolveScriptOutcomeV1::Complete(_)));

    let mut legacy = MirCompiler::with_options(false);
    let legacy = legacy
        .compile_with_source(program.clone(), Some("script-final-return.hako"))
        .expect("legacy Return");
    let normal = MirCompiler::with_options(false)
        .compile_normal(
            NormalCompileRequestV1::for_mir_mode(
                program,
                Some("script-final-return.hako"),
                std::collections::HashMap::new(),
            )
            .expect("normal request"),
        )
        .expect("selected Return");
    assert_eq!(
        MirPrinter::new().print_module(&normal.module),
        MirPrinter::new().print_module(&legacy.module),
    );
    assert_eq!(normal.verification_result, legacy.verification_result);
}

#[test]
fn final_root_void_return_is_complete_without_a_value_demand() {
    let program = ASTNode::Program {
        statements: vec![ASTNode::Return {
            value: None,
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };
    let source = PreparedNormalDefaultProgramRootV1::seal(program).expect("Program source");
    let window = VerifiedScriptRootDemandWindowV1::seal(vec![final_return_entry()], 1)
        .expect("final Return window");
    let view = ScriptSyntaxViewV1::from_program(source.source_ast()).expect("Script view");
    assert!(matches!(
        FunctionSemanticResolverSessionV1::new(0)
            .expect("resolver")
            .resolve_script(view, &window)
            .expect("void Return resolve"),
        ResolveScriptOutcomeV1::Complete(_),
    ));
}


#[test]
fn normal_script_commits_one_return_of_the_selected_operand() {
    use crate::mir::{ConstValue, MirInstruction, MirType};
    use crate::parser::NyashParser;

    for (source, expected_type, expected_value) in [
        ("return 7", MirType::Integer, ConstValue::Integer(7)),
        ("return", MirType::Void, ConstValue::Void),
        ("", MirType::Void, ConstValue::Void),
    ] {
        let program = NyashParser::parse_from_string(source).expect("Script source");
        let result = MirCompiler::with_options(false)
            .compile_normal(
                NormalCompileRequestV1::for_mir_mode(
                    program,
                    Some("normal-script-return.hako"),
                    Default::default(),
                )
                .expect("normal Script request"),
            )
            .expect("normal Script publishes only a finished module");
        assert!(result.verification_result.is_ok(), "{source:?}");
        let main = &result.module.functions["main"];
        assert_eq!(main.signature.return_type, expected_type, "{source:?}");
        let mut returns = main.blocks.values().filter_map(|block| match block.terminator.as_ref() {
            Some(MirInstruction::Return { value }) => Some(value),
            _ => None,
        });
        let returned = returns.next().copied().flatten().expect("Script return operand");
        assert!(returns.next().is_none(), "{source:?}");
        assert!(main.blocks.values().flat_map(|block| &block.instructions).any(|inst| {
            matches!(inst, MirInstruction::Const { dst, value } if *dst == returned && value == &expected_value)
        }), "{source:?}");
    }
}
