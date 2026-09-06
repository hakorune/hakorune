//! Existing root Home progress and physical emission validation.
//!
//! `RootHomeExitProgress` is the sole owner of the retained Home binding,
//! completion exit, object, and physical value until selected emission has
//! recorded and final validation has checked the release operation.

use super::*;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) enum RootHomeExitProgress {
    Unprepared,
    Unavailable,
    Prepared(Vec<RootHomeReleaseOriginV1>),
    Emitting,
    Emitted {
        origins: Vec<RootHomeReleaseEmissionV1>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    },
}

/// One source-issued root Home obligation after its existing local value has
/// been physically bound. The source binding and explicit return stay intact;
/// neither block identity nor an emitted instruction issues this origin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RootHomeReleaseOriginV1 {
    binding: BindingRefV1,
    exit: crate::mir::resolved_semantics::SourceStmtSiteV1,
    object: CanonicalObjectIdV1,
    value: ValueId,
}

impl RootHomeReleaseOriginV1 {
    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn exit(&self) -> &crate::mir::resolved_semantics::SourceStmtSiteV1 {
        &self.exit
    }

    pub(crate) const fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }

    pub(crate) const fn value(&self) -> ValueId {
        self.value
    }
}

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct RootHomeReleaseEmissionV1 {
    origin: RootHomeReleaseOriginV1,
    block: BasicBlockId,
    instruction: MirInstruction,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn root_home_exit_is_complete(
        &self,
    ) -> bool {
        let Some(Ok(completion)) = &self.root_completion else {
            return true;
        };
        !matches!(completion.cleanup().terminal_homes(), Some(Ok(_)))
            || matches!(
                *self.root_exit.borrow(),
                RootHomeExitProgress::Unavailable | RootHomeExitProgress::Emitted { .. }
            )
    }

    pub(crate) fn prepare_root_home_exit(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<bool, String> {
        let Some(Ok(completion)) = &self.root_completion else {
            return Ok(false);
        };
        if completion.owner() != owner {
            return Ok(false);
        }
        let Some(Ok(homes)) = completion.cleanup().terminal_homes() else {
            return Ok(false);
        };
        if !completion
            .explicit_site()
            .is_some_and(|expected| expected.node() == site)
        {
            return Err(freeze("root-exit-site-mismatch"));
        }
        let mut progress = self.root_exit.borrow_mut();
        if !matches!(*progress, RootHomeExitProgress::Unprepared) {
            return Err(freeze("duplicate-root-exit-prepare"));
        }
        let rows = self.local_commits.borrow();
        let exit = completion
            .explicit_site()
            .expect("checked explicit root exit")
            .clone();
        let mut origins = Vec::new();
        let mut available = true;
        for binding in homes {
            let mut candidates = rows.values().filter(|row| row.installs(*binding));
            let row = candidates
                .next()
                .ok_or_else(|| freeze("root-home-not-installed"))?;
            if candidates.next().is_some() {
                return Err(freeze("duplicate-root-home"));
            }
            available &= row.destruction
                == crate::mir::function::ObjectDestructionDispositionV1::PlainI64NoHook
                && matches!(row.emission, NewEmissionProgress::Emitted { .. });
            origins.push(RootHomeReleaseOriginV1 {
                binding: *binding,
                exit: exit.clone(),
                object: row.object,
                value: row.local.expect("installed Home"),
            });
        }
        *progress = if available {
            RootHomeExitProgress::Prepared(origins)
        } else {
            RootHomeExitProgress::Unavailable
        };
        Ok(available)
    }

    pub(crate) fn begin_root_home_exit(&self) -> Result<Vec<RootHomeReleaseOriginV1>, String> {
        let mut progress = self.root_exit.borrow_mut();
        if !matches!(*progress, RootHomeExitProgress::Prepared(_)) {
            return Err(freeze("root-exit-not-prepared"));
        }
        let RootHomeExitProgress::Prepared(operands) =
            std::mem::replace(&mut *progress, RootHomeExitProgress::Emitting)
        else {
            unreachable!()
        };
        Ok(operands)
    }

    pub(crate) fn record_root_home_exit(
        &self,
        origins: Vec<(RootHomeReleaseOriginV1, BasicBlockId, MirInstruction)>,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut progress = self.root_exit.borrow_mut();
        if !matches!(*progress, RootHomeExitProgress::Emitting) || bindings.is_empty() {
            return Err(freeze("root-exit-record-without-emission"));
        }
        let origins = origins
            .into_iter()
            .map(|(origin, block, instruction)| RootHomeReleaseEmissionV1 {
                origin,
                block,
                instruction,
            })
            .collect();
        *progress = RootHomeExitProgress::Emitted { origins, bindings };
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn validate_root_home_exit(
        &self,
        function: &MirFunction,
    ) -> Result<(), String> {
        let Some(Ok(completion)) = &self.root_completion else {
            return Ok(());
        };
        if !matches!(completion.cleanup().terminal_homes(), Some(Ok(_))) {
            return Ok(());
        }
        let expected_exit = completion
            .explicit_site()
            .ok_or_else(|| freeze("root-exit-source-missing"))?;
        let Some(Ok(expected_homes)) = completion.cleanup().terminal_homes() else {
            return Ok(());
        };
        match &*self.root_exit.borrow() {
            RootHomeExitProgress::Unavailable => Ok(()),
            RootHomeExitProgress::Emitted { origins, bindings } => {
                if origins.len() != expected_homes.len() {
                    return Err(freeze("root-exit-origin-count"));
                }
                for (emitted, expected_binding) in origins.iter().zip(expected_homes) {
                    if emitted.origin.binding() != *expected_binding
                        || emitted.origin.exit() != expected_exit
                    {
                        return Err(freeze("root-exit-origin-drift"));
                    }
                    if !matches!(
                        emitted.instruction,
                        MirInstruction::Invoke {
                            operation: crate::mir::instruction::InvokeOperation::HomeRelease {
                                object,
                                value,
                            },
                            ..
                        } if object == emitted.origin.object() && value == emitted.origin.value()
                    ) {
                        return Err(freeze("root-exit-operation-drift"));
                    }
                    if !function.blocks.get(&emitted.block).is_some_and(|block| {
                        block.all_instructions().any(|actual| {
                            matches!(
                                actual,
                                MirInstruction::Invoke {
                                    operation: crate::mir::instruction::InvokeOperation::HomeRelease {
                                        object,
                                        value,
                                    },
                                    ..
                                } if *object == emitted.origin.object() && *value == emitted.origin.value()
                            )
                        })
                    }) {
                        return Err(freeze("root-exit-operation-drift"));
                    }
                    if !function.blocks.get(&emitted.block).is_some_and(|block| {
                        block
                            .all_instructions()
                            .any(|actual| actual == &emitted.instruction)
                    }) {
                        return Err(freeze("root-exit-origin-binding-drift"));
                    }
                }
                for (id, expected) in bindings {
                    if !function.blocks.get(id).is_some_and(|block| {
                        block.all_instructions().any(|actual| actual == expected)
                    }) {
                        return Err(freeze("root-exit-binding-drift"));
                    }
                }
                Ok(())
            }
            _ => Err(freeze("root-exit-unconsumed")),
        }
    }
}
