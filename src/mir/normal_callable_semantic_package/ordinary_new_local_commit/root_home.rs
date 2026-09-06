//! Existing root Home progress and physical emission validation.
//!
//! This is a layout-only split. The progress payload remains the existing
//! object/value handoff until the separately selected origin-retention row.

use super::*;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) enum RootHomeExitProgress {
    Unprepared,
    Unavailable,
    Prepared(Vec<(CanonicalObjectIdV1, ValueId)>),
    Emitting,
    Emitted(Vec<(BasicBlockId, MirInstruction)>),
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
                RootHomeExitProgress::Unavailable | RootHomeExitProgress::Emitted(_)
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
        let mut operands = Vec::new();
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
            operands.push((row.object, row.local.expect("installed Home")));
        }
        *progress = if available {
            RootHomeExitProgress::Prepared(operands)
        } else {
            RootHomeExitProgress::Unavailable
        };
        Ok(available)
    }

    pub(crate) fn begin_root_home_exit(
        &self,
    ) -> Result<Vec<(CanonicalObjectIdV1, ValueId)>, String> {
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
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut progress = self.root_exit.borrow_mut();
        if !matches!(*progress, RootHomeExitProgress::Emitting) || bindings.is_empty() {
            return Err(freeze("root-exit-record-without-emission"));
        }
        *progress = RootHomeExitProgress::Emitted(bindings);
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
        match &*self.root_exit.borrow() {
            RootHomeExitProgress::Unavailable => Ok(()),
            RootHomeExitProgress::Emitted(bindings) => {
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
