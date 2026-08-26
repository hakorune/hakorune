use super::VmHakoErr;
use crate::mir::{
    MirCompileResult, MirCompiler, NormalCompileRequestV1, RejectedPostMacroWholeFileProgramV1,
    VerifiedPostMacroWholeFileProgramV1,
};
use crate::runner::NyashRunner;
use serde_json::Value;
use std::collections::HashMap;

pub(super) fn compile_source_to_canonical_v1(
    runner: &NyashRunner,
    filename: &str,
    code: &str,
) -> Result<Value, VmHakoErr> {
    let (prepared_source, using_imports) =
        prepare_vm_hako_source_and_imports(runner, filename, code)?;

    let ast = match runner.parse_source(&prepared_source) {
        Ok(ast) => ast,
        Err(e) => {
            crate::runner::modes::common_util::diag::print_parse_error_with_context(
                filename,
                &prepared_source,
                &e,
            );
            return Err(("parse-error", e.to_string()));
        }
    };
    let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);
    let compile_result =
        compile_post_macro_program(ast, filename, using_imports, !runner.config.no_optimize)
            .map_err(|error| ("compile-error", error))?;
    crate::runner::modes::common_util::verifier_gate::enforce_vm_verify_gate_or_exit(
        &compile_result.module,
        "vm-hako",
    );
    crate::runner::modes::common_util::safety_gate::enforce_vm_lifecycle_safety_or_exit(
        &compile_result.module,
        "vm-hako",
    );
    crate::runner::mir_json_emit::emit_canonical_v1_value_for_reference(&compile_result.module)
        .map_err(|e| ("emit-error", e))
}

fn compile_post_macro_program(
    ast: crate::ast::ASTNode,
    filename: &str,
    imports: HashMap<String, String>,
    optimize: bool,
) -> Result<MirCompileResult, String> {
    let program = VerifiedPostMacroWholeFileProgramV1::seal(ast).map_err(
        |rejected: RejectedPostMacroWholeFileProgramV1| {
            let message = rejected.error().to_string();
            rejected.discard();
            message
        },
    )?;
    let mut compiler = MirCompiler::with_options(optimize);
    compiler.compile_normal(NormalCompileRequestV1::for_vm_hako_post_macro(
        program, filename, imports,
    ))
}

fn prepare_vm_hako_source_and_imports(
    runner: &NyashRunner,
    filename: &str,
    code: &str,
) -> Result<(String, HashMap<String, String>), VmHakoErr> {
    let prepared = match crate::runner::modes::common_util::source_hint::prepare_source_with_imports(
        runner, filename, code,
    ) {
        Ok(prepared) => prepared,
        Err(e) => return Err(("resolve-error", e)),
    };

    crate::runner::modes::common_util::safety_gate::enforce_vm_source_safety_or_exit(
        &prepared.code,
        "vm-hako",
    );

    Ok((prepared.code, prepared.imports))
}

#[cfg(test)]
mod tests {
    use super::compile_post_macro_program;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use std::collections::HashMap;

    #[test]
    fn nonprogram_macro_output_rejects_before_vm_hako_compiler_admission() {
        let error = compile_post_macro_program(
            ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            },
            "vm-hako.hako",
            HashMap::new(),
            true,
        )
        .expect_err("whole-file non-Program output must reject");
        assert!(error.starts_with("[macro/whole-file-root] expected Program"));
    }
}
