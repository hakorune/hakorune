//! One-row canonical Script module transaction.

use crate::mir::builder::module_lowering_shell::{
    ModuleLoweringShellDrainInventoryV1, ModuleLoweringShellErrorV1, ModuleLoweringShellV1,
    PreparedModuleLoweringShellDrainV1,
};
use crate::mir::compiler::normal_source_plan::CompletedScriptPhysicalExitV1;
use crate::mir::verification::MirVerifier;
use crate::mir::verification_types::VerificationError;
use crate::mir::{MirFunction, MirModule};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalScriptModuleTransactionStageV1 {
    Schema,
    Verification,
    Shell,
}

#[derive(Debug)]
pub(in crate::mir) enum NormalScriptModuleTransactionErrorV1 {
    Schema { symbol: Box<str>, arity: usize },
    Verification(Box<[VerificationError]>),
    Shell(ModuleLoweringShellErrorV1),
}

pub(in crate::mir) struct RejectedNormalScriptModuleTransactionV1 {
    draft: MirFunction,
    stage: NormalScriptModuleTransactionStageV1,
    cause: NormalScriptModuleTransactionErrorV1,
}

pub(in crate::mir) struct PreparedNormalScriptModuleTransactionV1 {
    draft: MirFunction,
    shell: PreparedModuleLoweringShellDrainV1,
}

pub(in crate::mir) struct CompletedNormalScriptModuleCandidateV1 {
    module: MirModule,
}

impl PreparedNormalScriptModuleTransactionV1 {
    pub(in crate::mir) fn prepare(
        exit: CompletedScriptPhysicalExitV1,
    ) -> Result<Self, RejectedNormalScriptModuleTransactionV1> {
        Self::prepare_draft(exit.into_draft())
    }

    fn prepare_draft(draft: MirFunction) -> Result<Self, RejectedNormalScriptModuleTransactionV1> {
        if draft.signature.name != "main" || !draft.signature.params.is_empty() {
            let symbol = draft.signature.name.clone().into_boxed_str();
            let arity = draft.signature.params.len();
            return Err(RejectedNormalScriptModuleTransactionV1 {
                draft,
                stage: NormalScriptModuleTransactionStageV1::Schema,
                cause: NormalScriptModuleTransactionErrorV1::Schema { symbol, arity },
            });
        }
        if let Err(errors) = MirVerifier::new().verify_function(&draft) {
            return Err(RejectedNormalScriptModuleTransactionV1 {
                draft,
                stage: NormalScriptModuleTransactionStageV1::Verification,
                cause: NormalScriptModuleTransactionErrorV1::Verification(
                    errors.into_boxed_slice(),
                ),
            });
        }
        let inventory =
            match ModuleLoweringShellDrainInventoryV1::from_symbols(vec!["main".to_owned()]) {
                Ok(inventory) => inventory,
                Err(error) => {
                    return Err(RejectedNormalScriptModuleTransactionV1 {
                        draft,
                        stage: NormalScriptModuleTransactionStageV1::Shell,
                        cause: NormalScriptModuleTransactionErrorV1::Shell(error),
                    })
                }
            };
        let shell =
            match ModuleLoweringShellV1::from_empty_module(MirModule::new("script".to_owned())) {
                Ok(shell) => shell.prepare_drain(inventory),
                Err(error) => {
                    return Err(RejectedNormalScriptModuleTransactionV1 {
                        draft,
                        stage: NormalScriptModuleTransactionStageV1::Shell,
                        cause: NormalScriptModuleTransactionErrorV1::Shell(error),
                    })
                }
            };
        Ok(Self { draft, shell })
    }

    pub(in crate::mir) fn commit(self) -> CompletedNormalScriptModuleCandidateV1 {
        CompletedNormalScriptModuleCandidateV1 {
            module: self.shell.commit_preflighted(vec![self.draft]),
        }
    }

    #[cfg(test)]
    fn prepare_draft_for_test(
        draft: MirFunction,
    ) -> Result<Self, RejectedNormalScriptModuleTransactionV1> {
        Self::prepare_draft(draft)
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
    pub(in crate::mir) fn into_module(self) -> MirModule {
        self.module
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirType, ValueId,
    };

    #[test]
    fn normal_script_tx_commits_exactly_one_physical_entry() {
        let entry = BasicBlockId::new(0);
        let mut draft = MirFunction::new(
            FunctionSignature {
                name: "main".to_owned(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            entry,
        );
        let value = ValueId::new(0);
        draft.metadata.value_types.insert(value, MirType::Void);
        let block = draft.get_block_mut(entry).expect("entry block");
        block.add_instruction(MirInstruction::Const {
            dst: value,
            value: ConstValue::Void,
        });
        block.set_terminator(MirInstruction::Return { value: Some(value) });

        let prepared = match PreparedNormalScriptModuleTransactionV1::prepare_draft_for_test(draft)
        {
            Ok(prepared) => prepared,
            Err(rejected) => panic!("prepare one-row Script transaction: {:?}", rejected.cause()),
        };
        let candidate = prepared.commit();
        assert_eq!(candidate.module().functions.len(), 1);
        assert!(candidate.module().functions.contains_key("main"));
    }
}
