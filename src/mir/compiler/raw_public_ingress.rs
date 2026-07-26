//! PUBLIC-INGRESS0: the explicit, narrow Raw compiler entry.
//!
//! This is deliberately separate from `compile_with_source`.  It consumes the
//! complete Raw owner chain once, maps typed rejection owners to the existing
//! public String transport, and never retries through the legacy Builder path.

use super::MirCompileResult;
use crate::ast::ASTNode;
use crate::mir::RawPublishedCompileRequestV1;

#[allow(dead_code)]
pub(in crate::mir) enum RawPublicIngressPolicyV1 {
    NarrowV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawPublicImportDispositionV1 {
    None,
}

impl super::MirCompiler {
    /// Compile a narrow Raw source unit without touching the legacy ingress.
    ///
    /// The caller cannot select callable-Main policy, retry, fallback, JSON,
    /// or REPL behavior.  Every fallible stage is mapped before the final
    /// published-owner compatibility projection.
    pub fn compile_raw_with_source(
        &mut self,
        ast: ASTNode,
        source_file: Option<&str>,
    ) -> Result<MirCompileResult, String> {
        if self.builder.repl_mode {
            return Err("[raw-public/source-binding/repl-unsupported] NarrowV1".to_owned());
        }

        let published = self
            .compile_raw_published_v1(RawPublishedCompileRequestV1::narrow_v1(ast, source_file))
            .map_err(|rejected| rejected.into_public_string())?;
        Ok(published.into_compatibility_envelope().into_compatibility())
    }
}
