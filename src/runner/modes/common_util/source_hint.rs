use crate::ast::ASTNode;
use crate::mir::{MirCompileResult, MirCompiler};
use crate::runner::NyashRunner;
use std::collections::HashMap;

pub(crate) struct PreparedSourceWithImports {
    pub(crate) code: String,
    pub(crate) imports: HashMap<String, String>,
}

/// Compile AST with a source filename hint, reducing call-site duplication.
/// Falls back to regular compile when filename is None or empty.
pub fn compile_with_source_hint(
    compiler: &mut MirCompiler,
    ast: ASTNode,
    filename: Option<&str>,
) -> Result<MirCompileResult, String> {
    if let Some(f) = filename {
        if !f.is_empty() {
            return compiler.compile_with_source(ast, Some(f));
        }
    }
    compiler.compile_with_source(ast, None)
}

/// Compile AST with a source hint plus explicit imported static-box bindings.
pub fn compile_with_source_hint_and_imports(
    compiler: &mut MirCompiler,
    ast: ASTNode,
    filename: Option<&str>,
    imports: HashMap<String, String>,
) -> Result<MirCompileResult, String> {
    if let Some(f) = filename {
        if !f.is_empty() {
            return compiler.compile_with_source_and_imports(ast, Some(f), imports);
        }
    }
    compiler.compile_with_source_and_imports(ast, None, imports)
}

fn normalize_source_for_parser(code: &str, filename: &str) -> String {
    let mut prepared = crate::runner::modes::common_util::resolve::preexpand_at_local(code);
    if crate::runner::modes::common_util::hako::looks_like_hako_code(&prepared)
        || filename.ends_with(".hako")
    {
        prepared = crate::runner::modes::common_util::hako::strip_local_decl(&prepared);
    }
    prepared
}

/// Prepare source text for perf-sensitive emit lanes.
///
/// This skips `using` prelude resolution entirely and keeps only the small
/// text normalizations needed by current `.hako` benchmark fixtures.
pub(crate) fn prepare_source_minimal(code: &str, filename: &str) -> Result<String, String> {
    if code
        .lines()
        .any(|line| line.trim_start().starts_with("using "))
    {
        return Err(
            "minimal MIR emit mode does not support using/prelude resolution; use --emit-mir-json for full route or remove using lines"
                .to_string(),
        );
    }
    Ok(normalize_source_for_parser(code, filename))
}

/// Prepare source text for direct source-based compile lanes.
///
/// This is the SSOT for:
/// - `using` text-merge
/// - imported static-box alias collection
/// - `@local` pre-expansion
/// - `.hako` local-decl normalization for the Rust parser
pub(crate) fn prepare_source_with_imports(
    runner: &NyashRunner,
    filename: &str,
    code: &str,
) -> Result<PreparedSourceWithImports, String> {
    let mut imports = HashMap::new();
    let mut prepared = if crate::config::env::enable_using() {
        match crate::runner::modes::common_util::resolve::resolve_prelude_paths_profiled(
            runner, code, filename,
        ) {
            Ok((_, prelude_paths)) => {
                if !prelude_paths.is_empty() {
                    let (merged, merged_imports) =
                        crate::runner::modes::common_util::resolve::merge_prelude_text_with_imports(
                            runner, code, filename,
                        )?;
                    imports = merged_imports;
                    merged
                } else {
                    code.to_string()
                }
            }
            Err(e) => return Err(e),
        }
    } else {
        if code.contains("\nusing ") || code.trim_start().starts_with("using ") {
            return Err(
                "using: prelude merge is disabled in this profile. Enable NYASH_USING_AST=1 or remove 'using' lines."
                    .to_string(),
            );
        }
        code.to_string()
    };

    prepared = normalize_source_for_parser(&prepared, filename);

    Ok(PreparedSourceWithImports {
        code: prepared,
        imports,
    })
}

#[cfg(test)]
mod tests {
    use super::prepare_source_minimal;

    #[test]
    fn minimal_prepare_strips_top_level_local() {
        let prepared = prepare_source_minimal("local foo = 1\n", "bench.hako").unwrap();
        assert_eq!(prepared, "foo = 1\n");
    }

    #[test]
    fn minimal_prepare_rejects_using_lines() {
        let err =
            prepare_source_minimal("using foo.bar\nlocal foo = 1\n", "bench.hako").unwrap_err();
        assert!(err.contains("using/prelude resolution"));
    }
}
