//! Borrow the sealed Direct source packet; no physical or cleanup capability.
use super::super::{
    lexical_instance_call::{
        LexicalInstanceCallDispositionRowV1, LexicalInstanceCallSourceTargetV1,
    },
    OrdinaryNewClaimLedgerV1,
};
use super::normal_return::NormalReturnDispositionV1;
use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::normal_callable_semantic_package::OrdinaryNewResultClassV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallArgumentV1, ObjectCallSourceSupportV1, ObjectReturnAcquisitionV1, TerminalRelationV1,
    TerminalReturnedSourceV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1, SourceStmtSiteV1};

#[derive(Debug)]
pub(crate) struct VerifiedDirectObjectReturnSourceV1<'a> {
    proof: &'a std::rc::Rc<super::return_handoff::VerifiedObjectReturnHandoffV1>,
    target: &'a LexicalInstanceCallSourceTargetV1,
    arguments: &'a [LocalCallArgumentV1],
    result: InvokeCallResultKind,
    class: &'a str,
    teardown: &'a super::return_handoff::ObjectReturnTeardownAvailabilityV1,
}
impl VerifiedDirectObjectReturnSourceV1<'_> {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn cleanup_source(
        &self,
    ) -> Option<super::DirectRootCleanupSourceV1> {
        self.teardown()?;
        Some(super::DirectRootCleanupSourceV1::from_checked_view(
            std::rc::Rc::clone(self.proof),
            self.result,
        ))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn teardown(
        &self,
    ) -> Option<(&super::return_leaf::ObjectReturnTeardownDescriptorV1, bool)> {
        self.teardown.descriptor()
    }

    pub(crate) fn target(&self) -> &LexicalInstanceCallSourceTargetV1 {
        self.target
    }
    pub(crate) fn arguments(&self) -> &[LocalCallArgumentV1] {
        self.arguments
    }
    pub(crate) fn result(&self) -> InvokeCallResultKind {
        self.result
    }
    pub(crate) fn class(&self) -> &str {
        self.class
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Corroborate the already-taken row before any receiver/argument consumption.
    pub(crate) fn checked_direct_object_return_row_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
        row: &LexicalInstanceCallDispositionRowV1,
    ) -> Result<Option<VerifiedDirectObjectReturnSourceV1<'_>>, String> {
        match self.verified_direct_object_return_source_v1(owner, exit)? {
            Some(view) => {
                if row.source_target() != view.target() || row.result() != Some(view.result()) {
                    return Err(freeze("taken-row-drift"));
                }
                Ok(Some(view))
            }
            None if row.source_target().has_object_source_requirement()
                || matches!(
                    row.result(),
                    Some(InvokeCallResultKind::Handle | InvokeCallResultKind::NullableHandle)
                ) =>
            {
                Err(freeze("taken-row-proof-missing"))
            }
            None => Ok(None),
        }
    }

    pub(crate) fn verified_direct_object_return_source_v1(
        &self,
        owner: FunctionOwnerIdV1,
        exit: &SourceStmtSiteV1,
    ) -> Result<Option<VerifiedDirectObjectReturnSourceV1<'_>>, String> {
        let Some(index) = self.terminal_relation_index.get(&owner) else {
            return Ok(None);
        };
        let Some(TerminalRelationV1::Value(value)) = index.get(exit) else {
            return Ok(None);
        };
        let TerminalReturnedSourceV1::OwnedCall(original) = value.returned() else {
            return Ok(None);
        };
        let ObjectReturnAcquisitionV1::Direct {
            argument_sites,
            fault_homes,
        } = original.acquisition()
        else {
            return Ok(None);
        };
        // Same projection corroborates original Completion/terminal/proof identity.
        let Some(projection) = self.normal_exit_projection_v1(owner, exit)? else {
            return Ok(None);
        };
        let Some(NormalReturnDispositionV1::Verified { proof }) = self
            .normal_return_dispositions
            .as_ref()
            .and_then(|rows| rows.get(&(owner, exit.clone())))
        else {
            return Ok(None);
        };
        let loan = original.qualification();
        if value.owner() != owner
            || value.return_site() != exit
            || loan.value() != &OwnedExprSiteV1::new(owner, value.value_site().clone())
            || loan.call() != loan.value()
            || original.return_site() != exit
            || proof.acquisition().original() != original.as_ref()
            || fault_homes.as_ref() != projection.fault_homes()
            || projection.homes() != projection.fault_homes()
            || self
                .callable_result_classes
                .object_return_qualification(loan.value())
                .as_ref()
                != Some(loan)
        {
            return Err(freeze("identity"));
        }
        let Some(source) = &self.borrowed_formal_source else {
            return Ok(None);
        };
        let source = source.as_ref().map_err(Clone::clone)?;
        let Some(target) = source.object_source_target_at_v1(loan.call())? else {
            return Ok(None);
        };
        let arguments = proof.acquisition().arguments();
        if target.call_site() != loan.call()
            || target.target() != loan.key()
            || !target
                .object_return_sources()
                .is_some_and(|rows| rows.contains(loan))
            || target.argument_sites().len() != arguments.len()
            || target.argument_sites().len() != argument_sites.len()
            || argument_sites
                .iter()
                .zip(target.argument_sites())
                .any(|(original, site)| original.owner() != owner || original.site() != site)
        {
            return Err(freeze("target-or-arguments"));
        }
        match original.arguments() {
            ObjectCallSourceSupportV1::Unavailable => return Ok(None),
            ObjectCallSourceSupportV1::SourceOnly(snapshot)
            | ObjectCallSourceSupportV1::Observed(snapshot)
                if snapshot.as_ref() == arguments => {}
            _ => return Err(freeze("argument-snapshot")),
        }
        let (result, class) = match loan.class() {
            OrdinaryNewResultClassV1::Object(class) => {
                (InvokeCallResultKind::Handle, class.as_ref())
            }
            OrdinaryNewResultClassV1::NullableObject(class) => {
                (InvokeCallResultKind::NullableHandle, class.as_ref())
            }
            _ => return Err(freeze("result-class")),
        };
        Ok(Some(VerifiedDirectObjectReturnSourceV1 {
            proof,
            target,
            arguments,
            result,
            class,
            teardown: proof.teardown(),
        }))
    }
}
fn freeze(reason: &str) -> String {
    format!("[freeze:contract][ordinary-new/direct-return-source-{reason}]")
}

#[cfg(test)]
#[path = "ordinary_new_direct_return_source_tests.rs"]
mod tests;
