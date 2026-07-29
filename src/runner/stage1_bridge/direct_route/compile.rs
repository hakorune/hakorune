/*!
 * Phase-1 compatibility bridge binary-only direct route - MIR compilation helper.
 */

use crate::cli::CliGroups;
use crate::config::env::stage1;
use crate::mir::{
    MirCompileResult, MirCompiler, MirModule, MirPrinter, NormalCompileRequestV1,
    PreparedPostMacroNormalNonProgramV1, PreparedPostMacroNormalRootV1,
};
use crate::runner::NyashRunner;

struct ExistingStage1DirectPostMacroCompatibilityV1;

impl ExistingStage1DirectPostMacroCompatibilityV1 {
    fn compile(
        compiler: &mut MirCompiler,
        source: PreparedPostMacroNormalNonProgramV1,
        filename: &str,
    ) -> Result<MirCompileResult, String> {
        crate::runner::modes::common_util::source_hint::compile_with_source_hint(
            compiler,
            source.into_ast(),
            Some(filename),
        )
    }
}

pub(super) fn compile_and_maybe_dump(
    runner: &NyashRunner,
    groups: &CliGroups,
) -> Result<MirModule, String> {
    let module = compile_mir_binary_only_direct(runner, groups)?;
    dump_mir_if_requested(groups, &module);
    Ok(module)
}

fn compile_mir_binary_only_direct(
    runner: &NyashRunner,
    groups: &CliGroups,
) -> Result<MirModule, String> {
    let source = stage1::input_path()
        .or_else(|| groups.input.file.as_ref().cloned())
        .ok_or_else(|| "input file is required".to_string())?;

    let code = std::fs::read_to_string(&source)
        .map_err(|error| format!("read error: {}: {}", source, error))?;

    let ast = runner
        .parse_source(&code)
        .map_err(|error| format!("parse error: {}", error))?;
    let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);

    let compile_result = compile_post_macro_root(ast, &source, !runner.config.no_optimize)
        .map_err(|error| format!("MIR compilation error: {}", error))?;

    Ok(compile_result.module)
}

fn compile_post_macro_root(
    ast: crate::ast::ASTNode,
    source: &str,
    optimize: bool,
) -> Result<MirCompileResult, String> {
    let root = PreparedPostMacroNormalRootV1::classify(ast);
    let mut compiler = MirCompiler::with_options(optimize);
    match root {
        PreparedPostMacroNormalRootV1::Program(program) => compiler.compile_normal(
            NormalCompileRequestV1::for_stage1_direct_post_macro(program, Some(source)),
        ),
        PreparedPostMacroNormalRootV1::NonProgram(non_program) => {
            ExistingStage1DirectPostMacroCompatibilityV1::compile(
                &mut compiler,
                non_program,
                source,
            )
        }
    }
}

fn dump_mir_if_requested(groups: &CliGroups, module: &MirModule) {
    if groups.debug.dump_mir {
        let mut printer = if groups.debug.mir_verbose {
            MirPrinter::verbose()
        } else {
            MirPrinter::new()
        };
        if groups.debug.mir_verbose_effects {
            printer.set_show_effects_inline(true);
        }
        println!("{}", printer.print_module(module));
    }
}

#[cfg(test)]
mod tests {
    use super::compile_post_macro_root;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::PreparedPostMacroNormalRootV1;
    use crate::mir::{MirCompiler, MirPrinter};
    use crate::parser::NyashParser;

    fn parse(source: &str) -> crate::ast::ASTNode {
        NyashParser::parse_from_string(source).expect("fixture must parse")
    }

    #[test]
    fn program_route_matches_legacy_stage1_result() {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let ast = parse("static box Main { main() { return 7 } }");
        let mut legacy = MirCompiler::with_options(true);
        let expected = legacy
            .compile_with_source(ast.clone(), Some("stage1-parity.hako"))
            .expect("legacy oracle");
        let actual =
            compile_post_macro_root(ast, "stage1-parity.hako", true).expect("typed Program route");

        assert_eq!(
            MirPrinter::new().print_module(&actual.module),
            MirPrinter::new().print_module(&expected.module)
        );
        assert_eq!(actual.verification_result, expected.verification_result);

        let failing = parse("static box Main { main() { return missing } }");
        let expected_error = MirCompiler::with_options(true)
            .compile_with_source(failing.clone(), Some("stage1-failure.hako"))
            .expect_err("legacy oracle must reject");
        let actual_error = compile_post_macro_root(failing, "stage1-failure.hako", true)
            .expect_err("typed Program route must reject without compatibility retry");
        assert_eq!(actual_error, expected_error);
    }

    #[test]
    fn non_program_route_preserves_stage1_compatibility() {
        let ast = ASTNode::Literal {
            value: LiteralValue::Integer(3),
            span: Span::unknown(),
        };
        assert!(matches!(
            PreparedPostMacroNormalRootV1::classify(ast.clone()),
            PreparedPostMacroNormalRootV1::NonProgram(_)
        ));
        let mut legacy = MirCompiler::with_options(true);
        let expected = legacy
            .compile_with_source(ast.clone(), Some("stage1-residual.hako"))
            .expect("legacy oracle");
        let actual = compile_post_macro_root(ast, "stage1-residual.hako", true)
            .expect("explicit residual route");

        assert_eq!(
            MirPrinter::new().print_module(&actual.module),
            MirPrinter::new().print_module(&expected.module)
        );
        assert_eq!(actual.verification_result, expected.verification_result);
    }
}
