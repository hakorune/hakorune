//! Candidate-only physical `main/0` session for canonical Script.
//!
//! This is intentionally separate from Raw root lifecycle and from the
//! compiler's replaceable module session. It owns a fresh Builder and can only
//! yield one detached completed function or be dropped.

use crate::mir::builder::normal_module_transaction::CanonicalNormalMainEntryTargetV1;
use crate::mir::raw_root_body_recipe::RawScriptBodyRecipeV1;
use crate::mir::verification::MirVerifier;
use crate::mir::verification_types::VerificationError;
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirBuilder, MirFunction, MirType};

use super::{
    CompletedScriptPhysicalExitCoreV1, PreparedScriptPhysicalExitCoreV1,
    ScriptPhysicalExitCommitV1, ScriptPhysicalExitErrorV1, ScriptPhysicalExitOpenContractV1,
};

#[derive(Debug)]
pub(in crate::mir) enum ScriptPhysicalEntrySessionErrorV1 {
    TargetIsNotMain,
    TargetArityMismatch { actual: usize },
    CandidateFunctionAlreadyOpen,
    MissingCompletedFunction,
    Lowering(String),
    Exit(ScriptPhysicalExitErrorV1),
    Verification(Box<[VerificationError]>),
}

pub(in crate::mir) struct OpenScriptPhysicalEntrySessionV1 {
    candidate: MirBuilder,
    target: CanonicalNormalMainEntryTargetV1,
    entry_block: BasicBlockId,
    _seal: OpenScriptPhysicalEntrySessionSealV1,
}

#[derive(Debug)]
struct OpenScriptPhysicalEntrySessionSealV1;

#[derive(Debug)]
pub(in crate::mir) struct CompletedScriptPhysicalFunctionV1 {
    draft: MirFunction,
    target: CanonicalNormalMainEntryTargetV1,
    exit: CompletedScriptPhysicalExitCoreV1,
    _seal: CompletedScriptPhysicalFunctionSealV1,
}

#[derive(Debug)]
struct CompletedScriptPhysicalFunctionSealV1;

impl OpenScriptPhysicalEntrySessionV1 {
    pub(in crate::mir) fn open(
        current: &MirBuilder,
        target: CanonicalNormalMainEntryTargetV1,
    ) -> Result<Self, ScriptPhysicalEntrySessionErrorV1> {
        if !target.is_main() {
            return Err(ScriptPhysicalEntrySessionErrorV1::TargetIsNotMain);
        }
        if target.arity() != 0 {
            return Err(ScriptPhysicalEntrySessionErrorV1::TargetArityMismatch {
                actual: target.arity(),
            });
        }
        let mut candidate = MirBuilder::new();
        candidate.comp_ctx.quiet_internal_logs = current.comp_ctx.quiet_internal_logs;
        if candidate.function_state.current_function.is_some()
            || candidate.function_state.current_block.is_some()
        {
            return Err(ScriptPhysicalEntrySessionErrorV1::CandidateFunctionAlreadyOpen);
        }
        let entry_block = candidate.next_block_id();
        let signature = FunctionSignature {
            name: target.symbol().to_owned(),
            params: Vec::new(),
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        };
        candidate.function_state.current_function =
            Some(candidate.new_function_with_metadata(signature, entry_block));
        candidate.function_state.current_block = Some(entry_block);
        candidate.function_state.frag_emit_session.reset();
        candidate.comp_ctx.current_slot_registry =
            Some(crate::mir::region::function_slot_registry::FunctionSlotRegistry::new());
        Ok(Self {
            candidate,
            target,
            entry_block,
            _seal: OpenScriptPhysicalEntrySessionSealV1,
        })
    }

    pub(in crate::mir) fn builder(&self) -> &MirBuilder {
        &self.candidate
    }

    pub(in crate::mir) fn builder_mut(&mut self) -> &mut MirBuilder {
        &mut self.candidate
    }

    pub(in crate::mir) const fn entry_block(&self) -> BasicBlockId {
        self.entry_block
    }

    /// Complete one Script recipe inside the detached candidate. No Raw
    /// lifecycle owner or live compiler Builder participates in this path.
    pub(in crate::mir) fn lower_and_complete(
        mut self,
        recipe: &RawScriptBodyRecipeV1,
    ) -> Result<CompletedScriptPhysicalFunctionV1, (Self, ScriptPhysicalEntrySessionErrorV1)> {
        let terminal = {
            let scope =
                super::super::vars::lexical_scope::LexicalScopeGuard::new(&mut self.candidate);
            let lowered = self
                .candidate
                .lower_script_body_recipe_v1(recipe)
                .map_err(|error| ScriptPhysicalEntrySessionErrorV1::Lowering(error.to_string()));
            drop(scope);
            match lowered {
                Ok(terminal) => terminal,
                Err(error) => return Err((self, error)),
            }
        };
        let prepared = match PreparedScriptPhysicalExitCoreV1::prepare(
            &self.candidate,
            terminal,
            ScriptPhysicalExitOpenContractV1::ProvisionalUnknown,
        ) {
            Ok(prepared) => prepared,
            Err(error) => return Err((self, ScriptPhysicalEntrySessionErrorV1::Exit(error))),
        };
        let completed = ScriptPhysicalExitCommitV1::commit_projected(&mut self.candidate, prepared);
        let verification = {
            let function = self
                .candidate
                .function_state
                .current_function
                .as_ref()
                .expect("Script physical session keeps its candidate function open");
            MirVerifier::new().verify_function(function)
        };
        if let Err(errors) = verification {
            return Err((
                self,
                ScriptPhysicalEntrySessionErrorV1::Verification(errors.into_boxed_slice()),
            ));
        }
        self.finish(completed).map_err(|session| {
            (
                session,
                ScriptPhysicalEntrySessionErrorV1::MissingCompletedFunction,
            )
        })
    }

    fn finish(
        self,
        exit: CompletedScriptPhysicalExitCoreV1,
    ) -> Result<CompletedScriptPhysicalFunctionV1, Self> {
        let Self {
            mut candidate,
            target,
            entry_block,
            _seal: _,
        } = self;
        let Some(draft) = candidate.function_state.current_function.take() else {
            return Err(Self {
                candidate,
                target,
                entry_block,
                _seal: OpenScriptPhysicalEntrySessionSealV1,
            });
        };
        Ok(CompletedScriptPhysicalFunctionV1 {
            draft,
            target,
            exit,
            _seal: CompletedScriptPhysicalFunctionSealV1,
        })
    }
}

impl CompletedScriptPhysicalFunctionV1 {
    pub(in crate::mir) fn draft(&self) -> &MirFunction {
        &self.draft
    }

    pub(in crate::mir) fn target(&self) -> &CanonicalNormalMainEntryTargetV1 {
        &self.target
    }

    pub(in crate::mir) fn exit(&self) -> &CompletedScriptPhysicalExitCoreV1 {
        &self.exit
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        MirFunction,
        CanonicalNormalMainEntryTargetV1,
        CompletedScriptPhysicalExitCoreV1,
    ) {
        (self.draft, self.target, self.exit)
    }
}
