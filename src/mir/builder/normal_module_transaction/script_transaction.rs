//! One-row canonical Script module transaction.
//!
//! This transaction retains the completed Script exit receipt rather than
//! reconstructing source-result evidence from a finished MIR module.

use crate::mir::builder::module_lowering_shell::{
    ModuleLoweringShellDrainInventoryV1, ModuleLoweringShellErrorV1, ModuleLoweringShellV1,
    PreparedModuleLoweringShellDrainV1,
};
use crate::mir::builder::script_physical_exit::{
    CompletedScriptPhysicalExitCoreV1, ScriptPhysicalResultV1, ScriptSourceCompletionV1,
};
use crate::mir::builder::{CanonicalNormalMainEntryTargetV1, CompletedScriptPhysicalFunctionV1};
use crate::mir::compiler::normal_source_plan::{
    CompletedScriptPhysicalExitV1, RetainedNormalScriptSourceV1,
};
use crate::mir::raw_root_body_recipe::RawScriptUnitOriginV1;
use crate::mir::verification::MirVerifier;
use crate::mir::verification_types::VerificationError;
use crate::mir::{MirModule, MirType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalScriptModuleTransactionStageV1 {
    Schema,
    Verification,
    Evidence,
    Shell,
}

#[derive(Debug)]
pub(in crate::mir) enum NormalScriptModuleTransactionErrorV1 {
    Schema(NormalScriptModuleSchemaErrorV1),
    Verification(Box<[VerificationError]>),
    Evidence(NormalScriptCandidateEvidenceErrorV1),
    Shell(ModuleLoweringShellErrorV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum NormalScriptModuleSchemaErrorV1 {
    PhysicalEntry { symbol: Box<str>, arity: usize },
    TargetMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum NormalScriptCandidateEvidenceErrorV1 {
    UnitPhysicalResultWasNotVoid,
    ValuePhysicalResultWasNotScalar,
    SignatureMismatch { expected: MirType, actual: MirType },
}

/// The Script candidate has exactly one physical entry and no source-Main row.
#[derive(Debug)]
pub(in crate::mir) struct NormalScriptModuleTransactionSchemaV1 {
    expected_symbol: &'static str,
    expected_arity: usize,
    expected_row_count: usize,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedNormalScriptModuleTransactionV1 {
    source: RetainedNormalScriptSourceV1,
    draft: CompletedScriptPhysicalFunctionV1,
    stage: NormalScriptModuleTransactionStageV1,
    cause: NormalScriptModuleTransactionErrorV1,
}

pub(in crate::mir) struct PreparedNormalScriptModuleTransactionV1 {
    source: RetainedNormalScriptSourceV1,
    draft: CompletedScriptPhysicalFunctionV1,
    result: VerifiedScriptEntryResultContractV1,
    shell: PreparedModuleLoweringShellDrainV1,
    schema: NormalScriptModuleTransactionSchemaV1,
    verification: NormalScriptCandidateVerificationReceiptV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CompletedNormalScriptModuleEvidenceV1 {
    schema: NormalScriptModuleTransactionSchemaV1,
    target: CanonicalNormalMainEntryTargetV1,
    exit: CompletedScriptPhysicalExitCoreV1,
    result: VerifiedScriptEntryResultContractV1,
    source: RetainedNormalScriptSourceV1,
}

#[derive(Debug)]
pub(in crate::mir) struct CompletedNormalScriptModuleCandidateV1 {
    module: MirModule,
    evidence: CompletedNormalScriptModuleEvidenceV1,
    verification: NormalScriptCandidateVerificationReceiptV1,
}

#[derive(Debug)]
pub(in crate::mir) struct NormalScriptCandidateVerificationReceiptV1 {
    function_count: usize,
    _seal: NormalScriptCandidateVerificationReceiptSealV1,
}

#[derive(Debug)]
struct NormalScriptCandidateVerificationReceiptSealV1;

/// The source-result vocabulary carried into publication. It is sealed from
/// the completed exit receipt and never inferred from AST, module inventory,
/// or a Return scan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum VerifiedScriptEntryResultContractV1 {
    Unit {
        origin: RawScriptUnitOriginV1,
        physical: VerifiedScriptUnitPhysicalV1,
    },
    Integer,
    Bool,
    Float,
    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum VerifiedScriptUnitPhysicalV1 {
    ExactVoid,
}

impl PreparedNormalScriptModuleTransactionV1 {
    pub(in crate::mir) fn prepare(
        exit: CompletedScriptPhysicalExitV1,
    ) -> Result<Self, RejectedNormalScriptModuleTransactionV1> {
        let (source, draft) = exit.into_parts();
        Self::prepare_parts(source, draft)
    }

    fn prepare_parts(
        source: RetainedNormalScriptSourceV1,
        draft: CompletedScriptPhysicalFunctionV1,
    ) -> Result<Self, RejectedNormalScriptModuleTransactionV1> {
        let schema = NormalScriptModuleTransactionSchemaV1::physical_entry_only();
        if let Err(error) = schema.validate_draft(&draft) {
            return Err(Self::reject(
                source,
                draft,
                NormalScriptModuleTransactionStageV1::Schema,
                NormalScriptModuleTransactionErrorV1::Schema(error),
            ));
        }
        if let Err(errors) = MirVerifier::new().verify_function(draft.draft()) {
            return Err(Self::reject(
                source,
                draft,
                NormalScriptModuleTransactionStageV1::Verification,
                NormalScriptModuleTransactionErrorV1::Verification(errors.into_boxed_slice()),
            ));
        }
        let result = match VerifiedScriptEntryResultContractV1::seal(&draft) {
            Ok(result) => result,
            Err(error) => {
                return Err(Self::reject(
                    source,
                    draft,
                    NormalScriptModuleTransactionStageV1::Evidence,
                    NormalScriptModuleTransactionErrorV1::Evidence(error),
                ))
            }
        };
        let inventory =
            match ModuleLoweringShellDrainInventoryV1::from_symbols(vec!["main".to_owned()]) {
                Ok(inventory) => inventory,
                Err(error) => {
                    return Err(Self::reject(
                        source,
                        draft,
                        NormalScriptModuleTransactionStageV1::Shell,
                        NormalScriptModuleTransactionErrorV1::Shell(error),
                    ))
                }
            };
        let shell =
            match ModuleLoweringShellV1::from_empty_module(MirModule::new("script".to_owned())) {
                Ok(shell) => shell.prepare_drain(inventory),
                Err(error) => {
                    return Err(Self::reject(
                        source,
                        draft,
                        NormalScriptModuleTransactionStageV1::Shell,
                        NormalScriptModuleTransactionErrorV1::Shell(error),
                    ))
                }
            };
        Ok(Self {
            source,
            draft,
            result,
            shell,
            schema,
            verification: NormalScriptCandidateVerificationReceiptV1 {
                function_count: 1,
                _seal: NormalScriptCandidateVerificationReceiptSealV1,
            },
        })
    }

    fn reject(
        source: RetainedNormalScriptSourceV1,
        draft: CompletedScriptPhysicalFunctionV1,
        stage: NormalScriptModuleTransactionStageV1,
        cause: NormalScriptModuleTransactionErrorV1,
    ) -> RejectedNormalScriptModuleTransactionV1 {
        RejectedNormalScriptModuleTransactionV1 {
            source,
            draft,
            stage,
            cause,
        }
    }

    pub(in crate::mir) fn commit(self) -> CompletedNormalScriptModuleCandidateV1 {
        let Self {
            source,
            draft,
            result,
            shell,
            schema,
            verification,
        } = self;
        let (function, target, exit) = draft.into_parts();
        CompletedNormalScriptModuleCandidateV1 {
            module: shell.commit_preflighted(vec![function]),
            evidence: CompletedNormalScriptModuleEvidenceV1 {
                schema,
                target,
                exit,
                result,
                source,
            },
            verification,
        }
    }
}

impl NormalScriptModuleTransactionSchemaV1 {
    fn physical_entry_only() -> Self {
        Self {
            expected_symbol: "main",
            expected_arity: 0,
            expected_row_count: 1,
        }
    }

    fn validate_draft(
        &self,
        draft: &CompletedScriptPhysicalFunctionV1,
    ) -> Result<(), NormalScriptModuleSchemaErrorV1> {
        let function = draft.draft();
        if function.signature.name != self.expected_symbol
            || function.signature.params.len() != self.expected_arity
        {
            return Err(NormalScriptModuleSchemaErrorV1::PhysicalEntry {
                symbol: function.signature.name.clone().into_boxed_str(),
                arity: function.signature.params.len(),
            });
        }
        if !draft.target().is_main()
            || draft.target().symbol() != function.signature.name
            || draft.target().arity() != function.signature.params.len()
        {
            return Err(NormalScriptModuleSchemaErrorV1::TargetMismatch);
        }
        Ok(())
    }
}

impl VerifiedScriptEntryResultContractV1 {
    fn seal(
        draft: &CompletedScriptPhysicalFunctionV1,
    ) -> Result<Self, NormalScriptCandidateEvidenceErrorV1> {
        let exit = draft.exit();
        let result = match (exit.source(), exit.physical()) {
            (
                ScriptSourceCompletionV1::Unit { origin },
                ScriptPhysicalResultV1::ExistingOperand {
                    ty: MirType::Void, ..
                },
            )
            | (
                ScriptSourceCompletionV1::Unit { origin },
                ScriptPhysicalResultV1::SyntheticVoid { .. },
            ) => Self::Unit {
                origin,
                physical: VerifiedScriptUnitPhysicalV1::ExactVoid,
            },
            (ScriptSourceCompletionV1::Unit { .. }, _) => {
                return Err(NormalScriptCandidateEvidenceErrorV1::UnitPhysicalResultWasNotVoid)
            }
            (
                ScriptSourceCompletionV1::Value,
                ScriptPhysicalResultV1::ExistingOperand {
                    ty: MirType::Integer,
                    ..
                },
            ) => Self::Integer,
            (
                ScriptSourceCompletionV1::Value,
                ScriptPhysicalResultV1::ExistingOperand {
                    ty: MirType::Bool, ..
                },
            ) => Self::Bool,
            (
                ScriptSourceCompletionV1::Value,
                ScriptPhysicalResultV1::ExistingOperand {
                    ty: MirType::Float, ..
                },
            ) => Self::Float,
            (
                ScriptSourceCompletionV1::Value,
                ScriptPhysicalResultV1::ExistingOperand {
                    ty: MirType::String,
                    ..
                },
            ) => Self::String,
            (ScriptSourceCompletionV1::Value, _) => {
                return Err(NormalScriptCandidateEvidenceErrorV1::ValuePhysicalResultWasNotScalar)
            }
        };
        let expected = result.physical_return_type();
        let actual = draft.draft().signature.return_type.clone();
        if actual != expected {
            return Err(NormalScriptCandidateEvidenceErrorV1::SignatureMismatch {
                expected,
                actual,
            });
        }
        Ok(result)
    }

    fn physical_return_type(&self) -> MirType {
        match self {
            Self::Unit { .. } => MirType::Void,
            Self::Integer => MirType::Integer,
            Self::Bool => MirType::Bool,
            Self::Float => MirType::Float,
            Self::String => MirType::String,
        }
    }
}

impl RejectedNormalScriptModuleTransactionV1 {
    pub(in crate::mir) const fn stage(&self) -> NormalScriptModuleTransactionStageV1 {
        self.stage
    }

    pub(in crate::mir) fn cause(&self) -> &NormalScriptModuleTransactionErrorV1 {
        &self.cause
    }

    pub(in crate::mir) fn discard(self) {
        drop(self);
    }
}

impl CompletedNormalScriptModuleCandidateV1 {
    pub(in crate::mir) fn module(&self) -> &MirModule {
        &self.module
    }

    pub(in crate::mir) fn evidence(&self) -> &CompletedNormalScriptModuleEvidenceV1 {
        &self.evidence
    }

    pub(in crate::mir) fn verification(&self) -> &NormalScriptCandidateVerificationReceiptV1 {
        &self.verification
    }
}

impl CompletedNormalScriptModuleEvidenceV1 {
    pub(in crate::mir) fn target(&self) -> &CanonicalNormalMainEntryTargetV1 {
        &self.target
    }

    pub(in crate::mir) fn exit(&self) -> &CompletedScriptPhysicalExitCoreV1 {
        &self.exit
    }

    pub(in crate::mir) fn result(&self) -> &VerifiedScriptEntryResultContractV1 {
        &self.result
    }

    pub(in crate::mir) fn source_identity(&self) -> &str {
        self.source.source_identity()
    }

    pub(in crate::mir) const fn schema_row_count(&self) -> usize {
        self.schema.expected_row_count
    }
}

impl NormalScriptCandidateVerificationReceiptV1 {
    pub(in crate::mir) const fn function_count(&self) -> usize {
        self.function_count
    }
}
