//! Recheck original support substitution and retained result roots.
use super::*;

impl OrdinaryNewResultClassClaimsV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn check_borrowed_nullable_roots_v1(
        &self,
        loan: &BorrowedNullableLocalSourceLoanV1,
        roots: &[&Rc<ResultOriginWitnessV1>],
    ) -> Result<(), &'static str> {
        if !loan.declared_result_matches_v1(self.borrowed_result_declaration_v1(loan.key()))
            || roots.is_empty() || loan.witnesses.len() != roots.len() {
            return Err("borrowed-nullable/root-coverage");
        }
        if let BorrowedLocalSupportV1::MapSlot(map) = &loan.support {
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
        if let BorrowedLocalSupportV1::OwnedReceiver { read, source } = &loan.support {
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
        if let BorrowedLocalSupportV1::OwnedLocal { callee_ordinal, actual, source } = &loan.support {
            use crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1 as Support;
            let declared = loan.declared_read_result_v1().ok_or("borrowed-nullable/source-identity")?;
            let Support::Formal { binding, ordinal } = declared.support() else {
                return Err("borrowed-nullable/root-origin");
            };
            let mut path = loan.call().site().node().segments().to_vec();
            path.push(crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(ordinal));
            if ordinal != *callee_ordinal || declared.owner() != loan.callee_owner()
                || actual.binding != Some(source.binding) || source.binding.owner() != loan.call().owner()
                || actual.site.owner() != loan.call().owner() || actual.site.site().node().segments() != path
                || source.construction.class() != declared.declared_type()
                || roots.iter().zip(loan.witnesses()).any(|(root, kept)| !Rc::ptr_eq(root, kept)
                    || root.site().owner() != loan.callee_owner()
                    || !(root.is_literal_null_v1() || (root.formal_ordinal() == Some(ordinal)
                        && root.supports_formal_v1(binding, ordinal)))) {
                return Err("borrowed-nullable/witness-identity");
            }
            return Ok(());
        }
        if let BorrowedLocalSupportV1::EntryForward { callee: crate::mir::instruction::BorrowedResultSupportV1::Receiver,
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
        let BorrowedLocalSupportV1::EntryForward {
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
                    let mut path = loan.call.site().node().segments().to_vec();
                    path.push(
                        crate::mir::resolved_semantics::SourcePathSegmentV1::Argument(ordinal),
                    );
                    if actual.argument_site.owner() != loan.call.owner()
                        || actual.argument_site.site().node().segments() != path
                    {
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
