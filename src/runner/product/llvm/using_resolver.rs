//! Using/prelude resolution for LLVM mode
//!
//! Handles `using` statement resolution and prelude merge.

use nyash_rust::ast::ASTNode;

/// Using resolver Box
///
/// **Responsibility**: Resolve `using` statements and merge preludes
/// **Input**: runner, code, filename
/// **Output**: Result<(String, Vec<ASTNode>), String>
pub struct UsingResolverBox;

impl UsingResolverBox {
    /// Resolve `using` statements and merge preludes
    ///
    /// This function:
    /// 1. Resolves prelude paths from `using` statements
    /// 2. Parses preludes to ASTs
    /// 3. Returns cleaned code and prelude ASTs
    ///
    /// Returns (cleaned_code, prelude_asts) on success.
    pub fn resolve(
        runner: &crate::runner::NyashRunner,
        code: &str,
        filename: &str,
    ) -> Result<(String, Vec<ASTNode>), String> {
        let use_ast = crate::config::env::using_ast_enabled();
        let mut code_ref: &str = code;
        let cleaned_code_owned;
        let mut prelude_asts: Vec<ASTNode> = Vec::new();

        if crate::config::env::enable_using() {
            match crate::runner::modes::common_util::resolve::resolve_prelude_paths_profiled(
                runner, code, filename,
            ) {
                Ok((clean, paths)) => {
                    cleaned_code_owned = clean;
                    code_ref = &cleaned_code_owned;
                    if !paths.is_empty() && !use_ast {
                        return Err(
                            "using: AST prelude merge is disabled in this profile. Enable NYASH_USING_AST=1 or remove 'using' lines.".to_string()
                        );
                    }
                    if use_ast && !paths.is_empty() {
                        match crate::runner::modes::common_util::resolve::parse_preludes_to_asts(
                            runner, &paths,
                        ) {
                            Ok(v) => prelude_asts = v,
                            Err(e) => {
                                return Err(format!("{}", e));
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(format!("{}", e));
                }
            }
        }

        // Pre-expand '@name[:T] = expr' sugar at line-head (same as common path)
        let preexpanded_owned =
            crate::runner::modes::common_util::resolve::preexpand_at_local(code_ref);
        code_ref = &preexpanded_owned;

        Ok((code_ref.to_string(), prelude_asts))
    }
}
