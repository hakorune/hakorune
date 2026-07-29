/*!
 * Phase-1 compatibility bridge binary-only direct route - MIR compilation helper.
 */

use crate::cli::CliGroups;
use crate::config::env::stage1;
use crate::mir::{
    MirCompileResult, MirCompiler, MirModule, MirPrinter, NormalCompileRequestV1,
    RejectedPostMacroWholeFileProgramV1, VerifiedPostMacroWholeFileProgramV1,
};
use crate::runner::NyashRunner;

fn reject_non_program_macro_output(rejected: RejectedPostMacroWholeFileProgramV1) -> String {
    let message = rejected.error().to_string();
    rejected.discard();
    message
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
    let program =
        VerifiedPostMacroWholeFileProgramV1::seal(ast).map_err(reject_non_program_macro_output)?;
    let mut compiler = MirCompiler::with_options(optimize);
    compiler.compile_normal(NormalCompileRequestV1::for_stage1_direct_post_macro(
        program,
        Some(source),
    ))
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
    fn non_program_macro_output_fails_before_stage1_compiler_admission() {
        let ast = ASTNode::Literal {
            value: LiteralValue::Integer(3),
            span: Span::unknown(),
        };
        let error = compile_post_macro_root(ast, "stage1-residual.hako", true)
            .expect_err("whole-file non-Program output must reject");
        assert_eq!(
            error,
            "[macro/whole-file-root] expected Program output from whole-file macro expansion"
        );
    }
}
