//! PUBLIC-INGRESS0: the explicit, narrow Raw compiler entry.
//!
//! This is deliberately separate from `compile_with_source`.  It consumes the
//! complete Raw owner chain once, maps typed rejection owners to the existing
//! public String transport, and never retries through the legacy Builder path.

use super::lowering_input::LegacyModuleLoweringInputV1;
use super::module_postprocess::ModulePostprocessOwnerV1;
use super::raw_source_binding::{RawCallableMainSelectionV1, RejectedRawSourceBindingV1};
use super::raw_root_helper_coverage::RawPublicEligibilityProfileV1;
use super::MirCompileResult;
use crate::ast::ASTNode;
use std::fmt::Debug;

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

        let import_disposition = RawPublicImportDispositionV1::None;
        let package = self
            .bind_raw_source_for_public(
                LegacyModuleLoweringInputV1::bare_ast(ast),
                source_file,
                "main",
                RawCallableMainSelectionV1::Omitted,
                import_disposition,
            )
            .map_err(|rejected| {
                reject(
                    "source-binding",
                    rejected,
                    RejectedRawSourceBindingV1::discard,
                )
            })?;
        let root_package = package
            .into_root_package()
            .map_err(|rejected| reject("root-package", rejected, |owner| owner.discard()))?;
        let eligible = root_package
            .prepare_public_eligibility(RawPublicEligibilityProfileV1::narrow_v1())
            .map_err(|rejected| reject("eligibility", rejected, |owner| owner.discard()))?;
        let opened = eligible
            .open_physical(&self.builder)
            .map_err(|rejected| reject("physical-open", rejected, |owner| owner.discard()))?;
        let pending = opened
            .prepare_children()
            .map_err(|rejected| reject("children", rejected, |owner| owner.discard()))?;
        let complete = pending
            .complete_all()
            .map_err(|rejected| reject("children", rejected, |owner| owner.discard()))?;
        let ready = complete
            .finish_callable_main()
            .map_err(|rejected| reject("callable-main", rejected, |owner| owner.discard()))?;
        let declared = ready
            .declare_environment()
            .map_err(|rejected| reject("declaration-access", rejected, |owner| owner.discard()))?;
        let body = declared
            .begin_body()
            .map_err(|rejected| reject("body", rejected, |owner| owner.discard()))?;
        let batch = body
            .prepare_root_batch()
            .map_err(|rejected| reject("root-batch", rejected, |owner| owner.discard()))?;
        let drained = batch
            .prepare_drain()
            .map_err(|rejected| reject("drain", rejected, |owner| owner.discard()))?
            .drain();
        let finalized = drained
            .prepare_finalization()
            .map_err(|rejected| reject("finalization", rejected, |owner| owner.discard()))?;
        let ready = finalized.prepare_postprocess();
        let postprocessed = ModulePostprocessOwnerV1::new(&mut self.verifier, self.optimize)
            .run_raw_ready(ready)
            .map_err(|rejected| reject("postprocess", rejected, |owner| owner.discard()))?;
        let prepared = postprocessed
            .prepare_external_commit()
            .map_err(|rejected| {
                reject("external-commit-preparation", rejected, |owner| {
                    owner.discard()
                })
            })?;
        let published = self
            .publish_raw_direct(prepared)
            .map_err(|rejected| reject("publication", rejected, |owner| owner.discard()))?;
        Ok(published.into_compatibility_envelope().into_compatibility())
    }
}

fn reject<T>(stage: &'static str, rejection: T, discard: impl FnOnce(T)) -> String
where
    T: Debug,
{
    let detail = format!("{rejection:?}");
    discard(rejection);
    format!("[raw-public/{stage}/rejected] {detail}")
}
