//! Recheck original support substitution and retained result roots.
use super::*;

/// Common caller lifetime identity only. Original reads and payload class
/// stay on their own source loans; no leaf read stands for a joined value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BorrowedLifetimeAnchorV1 {
    Entry { root: BindingRefV1, caller_support: crate::mir::instruction::BorrowedResultSupportV1 },
    Owned { root: BindingRefV1, acquisition: OwnedExprSiteV1 },
}

impl OrdinaryNewResultClassClaimsV1 {
    /// Source class folds retain only the exact Call and original declaration;
    /// Pending field roots, Home/nonnull and live-use proof are not consulted.
    pub(in crate::mir::normal_callable_semantic_package) fn declared_read_source_at_v1(&self,
        batch: &VerifiedResolvedCallableSemanticBatchV1, selected: &VerifiedSelectedCallableBatchMapV1,
        fields: &super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1, app_main: Option<AppMainSourceRefV1<'_>>,
        caller: Option<&CanonicalSameModuleCallableKeyV1>,
    ) -> Result<Option<crate::mir::resolved_semantics::home_new_prefix::DeclaredCallReadSourceV1>, &'static str> {
        use crate::mir::resolved_semantics::home_new_prefix::DeclaredCallReadSourceV1;
        if !batch.declarations().any(|row| row.owner() == input.owner()) { return Ok(None); }
        DeclaredCallReadSourceV1::issue_source_v1(input, site, |producer, destination| {
            let Some(original) = self.original_local_call_identity_v1(batch, selected, fields, producer, destination, parameters, app_main, caller)? else { return Ok(None); };
            let Some(callee) = batch.unique_declaration_for_owner(original.callee_owner).ok().flatten()
                else { return Err("borrowed-nullable/callee-owner"); };
            Ok(callee.borrowed_result().map(std::sync::Arc::clone))
        })
    }

    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_support_read_at_v1(&self,
        batch: &VerifiedResolvedCallableSemanticBatchV1, selected: &VerifiedSelectedCallableBatchMapV1,
        fields: &super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1, app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<Option<BorrowedSupportReadV1>, &'static str> {
        let Some(source) = self.declared_read_source_at_v1(batch, selected, fields, parameters, input, site, app_main, None)? else { return Ok(None); };
        let mut loans = Vec::new();
        for (call, destination, _) in source.original_calls_v1() {
            loans.push(self.borrowed_nullable_local_source_v1(batch, selected, fields, parameters, call, destination, app_main)?
                .ok_or("borrowed-nullable/source-missing")?);
        }
        BorrowedSupportReadV1::issue(source, loans).map(Some)
    }

    pub(super) fn original_borrowed_read_v1(&self,
        batch: &VerifiedResolvedCallableSemanticBatchV1, selected: &VerifiedSelectedCallableBatchMapV1,
        fields: &super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        outer: &call_witness::OriginalCallSourceV1, actual: &ResultActualSourceV1,
        app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<Option<BorrowedSupportReadV1>, &'static str> {
        batch.with_lowering_input(outer.caller_slot, |input|
            self.borrowed_support_read_at_v1(batch, selected, fields, parameters, input, actual.site.site(), app_main))
            .map_err(|_| "borrowed-nullable/caller-input")?
    }

    pub(in crate::mir::normal_callable_semantic_package) fn check_borrowed_nullable_roots_v1(
        &self,
        loan: &BorrowedCallResultSourceLoanV1,
        roots: &[&Rc<ResultOriginWitnessV1>],
    ) -> Result<(), &'static str> {
        if !loan.declared_result_matches_v1(self.borrowed_result_declaration_v1(loan.key()))
            || roots.is_empty() || loan.witnesses.len() != roots.len() {
            return Err("borrowed-nullable/root-coverage");
        }
        if let BorrowedCallSupportV1::Reborrow { callee, actual } = &loan.support {
            let declaration = loan.declared_read_result_v1().ok_or("borrowed-nullable/source-identity")?;
            for inner in actual.loans() {
                let inner_roots = self.checked_original_source_roots_v1(inner.key(), inner.callee_owner())?;
                self.check_borrowed_nullable_roots_v1(inner, &inner_roots)?;
                if inner.call() == loan.call() || !inner.is_declared_read_v1() { return Err("borrowed-nullable/source-identity"); }
            }
            use crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1 as Support;
            let matches = match declaration.support() {
                Support::Formal { binding, ordinal } => {
                    *callee == crate::mir::instruction::BorrowedResultSupportV1::Formal(ordinal)
                        && actual.read().site().is_argument_of(loan.call().site(), ordinal)
                        && roots.iter().all(|root| root.is_literal_null_v1() || root.supports_formal_v1(binding, ordinal))
                }
                Support::Receiver { binding } => *callee == crate::mir::instruction::BorrowedResultSupportV1::Receiver
                    && actual.read().site() == loan.receiver()
                    && actual.declared_type() == loan.key().owner()
                    && roots.iter().all(|root| root.supports_receiver_v1(binding, declaration.declared_type())
                        && (root.is_literal_null_v1() || root.recheck_receiver_payload_v1(self))),
            };
            if !matches || declaration.owner() != loan.callee_owner()
                || actual.read().owner() != loan.call().owner()
                || roots.iter().zip(loan.witnesses()).any(|(root, kept)| !Rc::ptr_eq(root, kept)) {
                return Err("borrowed-nullable/source-identity");
            }
            return Ok(());
        }
        if let BorrowedCallSupportV1::MapSlot(map) = &loan.support {
            if map.receiver.site() != loan.receiver() || map.receiver.owner() != loan.call().owner()
                || map.holder.owner() != loan.call().owner() { return Err("borrowed-nullable/source-identity"); }
            for (witness, root) in loan.witnesses.iter().zip(roots) {
                if witness.site() != loan.call() || witness.origin() != &map.origin()
                    || root.origin() != &map.source.origin()
                    || !map.source.returns.contains_key(root.site())
                    || !matches!(root.step(), ResultWitnessStepV1::MapStored { source } if Rc::ptr_eq(source, &map.source))
                    || !matches!(witness.step(), ResultWitnessStepV1::Call { site, key, callee, substitution }
                        if site == loan.call() && key == loan.key() && Rc::ptr_eq(callee, root) && substitution.is_none())
                { return Err("borrowed-nullable/witness-identity"); }
            }
            return Ok(());
        }
        if let BorrowedCallSupportV1::OwnedReceiver { read, source } = &loan.support {
            let declared = loan.declared_read_result_v1().ok_or("borrowed-nullable/source-identity")?;
            let crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Receiver { binding } = declared.support()
                else { return Err("borrowed-nullable/root-origin"); };
            if declared.owner() != loan.callee_owner() || read.owner() != loan.call().owner() || read.site() != loan.receiver()
                || source.binding.owner() != loan.call().owner() || source.construction.class() != loan.key().owner()
                || roots.iter().zip(loan.witnesses()).any(|(root, kept)| !Rc::ptr_eq(root, kept)
                    || !root.supports_receiver_v1(binding, declared.declared_type())
                    || !(root.is_literal_null_v1() || root.recheck_receiver_payload_v1(self))) {
                return Err("borrowed-nullable/witness-identity");
            }
            return Ok(());
        }
        if let BorrowedCallSupportV1::OwnedLocal { callee_ordinal, actual, source } = &loan.support {
            use crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1 as Support;
            let declared = loan.declared_read_result_v1().ok_or("borrowed-nullable/source-identity")?;
            let Support::Formal { binding, ordinal } = declared.support() else {
                return Err("borrowed-nullable/root-origin");
            };
            if ordinal != *callee_ordinal || declared.owner() != loan.callee_owner()
                || actual.binding != Some(source.binding) || source.binding.owner() != loan.call().owner()
                || !actual.site.is_argument_of(loan.call(), ordinal)
                || source.construction.class() != declared.declared_type()
                || roots.iter().zip(loan.witnesses()).any(|(root, kept)| !Rc::ptr_eq(root, kept)
                    || root.site().owner() != loan.callee_owner()
                    || !(root.is_literal_null_v1() || (root.formal_ordinal() == Some(ordinal)
                        && root.supports_formal_v1(binding, ordinal)))) {
                return Err("borrowed-nullable/witness-identity");
            }
            return Ok(());
        }
        if let BorrowedCallSupportV1::EntryForward { callee: crate::mir::instruction::BorrowedResultSupportV1::Receiver,
            caller: crate::mir::instruction::BorrowedResultSupportV1::Receiver, root: binding, actual } = &loan.support {
            use crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1 as Support;
            let declared = loan.declared_read_result_v1().ok_or("borrowed-nullable/source-identity")?;
            let Support::Receiver { binding: callee_binding } = declared.support() else { return Err("borrowed-nullable/root-origin"); };
            if actual.site() != loan.receiver() || actual.owner() != loan.call().owner() || binding.owner() != loan.call().owner()
                || declared.owner() != loan.callee_owner() || roots.iter().zip(loan.witnesses()).any(|(root, kept)| {
                    !root.supports_receiver_v1(callee_binding, declared.declared_type())
                    || !(root.is_literal_null_v1() || root.recheck_receiver_payload_v1(self))
                    || !matches!(kept.step(), ResultWitnessStepV1::Call { site, key, callee, substitution }
                        if site == loan.call() && key == loan.key() && Rc::ptr_eq(callee, root)
                        && match substitution {
                            Some(ResultSupportSubstitutionV1::Receiver { read, binding: original }) => read == actual && original == binding,
                            None => root.is_literal_null_v1(), _ => false })
                }) { return Err("borrowed-nullable/witness-identity"); }
            return Ok(());
        }
        let BorrowedCallSupportV1::EntryForward {
            callee: crate::mir::instruction::BorrowedResultSupportV1::Formal(callee_ordinal),
            caller: crate::mir::instruction::BorrowedResultSupportV1::Formal(caller_ordinal), root: formal_root, .. } = loan.support
            else { return Err("borrowed-nullable/root-origin"); };
        let mut forwarded = false;
        for (witness, root) in loan.witnesses.iter().zip(roots) {
            let ResultWitnessStepV1::Call {
                site,
                key,
                callee,
                substitution,
            } = witness.step()
            else {
                return Err("borrowed-nullable/witness-role");
            };
            if witness.site() != loan.call()
                || site != loan.call()
                || key != loan.key()
                || !Rc::ptr_eq(callee, root)
            {
                return Err("borrowed-nullable/witness-identity");
            }
            // A formal's Null alternative keeps its substitution too. Literal
            // Null has no formal ancestry; the two must never be conflated.
            match (root.formal_ordinal(), substitution) {
                (None, None) => {}
                (Some(ordinal), Some(ResultSupportSubstitutionV1::Formal(actual)))
                    if ordinal == callee_ordinal
                        && actual.binding == formal_root
                        && actual.callee_ordinal == ordinal
                        && actual.caller_ordinal == caller_ordinal =>
                {
                    if !actual.argument_site.is_argument_of(&loan.call, ordinal) {
                        return Err("borrowed-nullable/argument-identity");
                    }
                }
                _ => return Err("borrowed-nullable/formal-substitution"),
            }
            match (root.origin(), witness.origin()) {
                (ResultValueOriginV1::Null, ResultValueOriginV1::Null) => {}
                (
                    ResultValueOriginV1::ForwardFormal { ordinal },
                    ResultValueOriginV1::ForwardFormal { ordinal: caller },
                ) if *ordinal == callee_ordinal
                    && *caller == caller_ordinal
                    && root.formal_ordinal() == Some(*ordinal) =>
                {
                    forwarded = true;
                }
                _ => return Err("borrowed-nullable/root-origin"),
            }
        }
        if !forwarded {
            return Err("borrowed-nullable/forwarded-root-missing");
        }
        Ok(())
    }
}
