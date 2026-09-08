use crate::runner::NyashRunner;
use std::collections::HashMap;

pub(crate) struct PreparedSourceWithImports {
    pub(crate) code: String,
    pub(crate) imports: HashMap<String, String>,
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
    prepare_source_with_imports_impl(runner, filename, code, false)
}

/// Selected normal callers preserve declarations for the source-backed parser.
/// Compatibility normalization is never used to repair this source.
pub(crate) fn prepare_normal_source_with_imports(
    runner: &NyashRunner,
    filename: &str,
    code: &str,
) -> Result<PreparedSourceWithImports, String> {
    prepare_source_with_imports_impl(runner, filename, code, true)
}

fn prepare_source_with_imports_impl(
    runner: &NyashRunner,
    filename: &str,
    code: &str,
    selected_normal: bool,
) -> Result<PreparedSourceWithImports, String> {
    let mut imports = HashMap::new();
    let mut prepared = if crate::config::env::enable_using() {
        use crate::runner::modes::common_util::resolve;
        let discover = if selected_normal {
            resolve::resolve_normal_prelude_paths_profiled
        } else {
            resolve::resolve_prelude_paths_profiled
        };
        match discover(runner, code, filename) {
            Ok((_, prelude_paths)) => {
                if !prelude_paths.is_empty() {
                    use crate::runner::modes::common_util::resolve;
                    let merge = if selected_normal {
                        resolve::merge_normal_prelude_text_with_imports
                    } else {
                        resolve::merge_prelude_text_with_imports
                    };
                    let (merged, merged_imports) = merge(runner, code, filename)?;
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

    prepared = if selected_normal {
        crate::runner::modes::common_util::resolve::preexpand_at_local(&prepared)
    } else {
        normalize_source_for_parser(&prepared, filename)
    };

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

#[cfg(test)]
#[path = "source_hint_normal_tests.rs"]
mod normal_tests;
