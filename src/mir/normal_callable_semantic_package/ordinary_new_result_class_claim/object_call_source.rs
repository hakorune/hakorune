//! Exact nonterminal call-result source loan, issued by the existing result owner.
//! No synthetic Return, acquired Home, Completion or executable result is issued.
use super::super::lexical_instance_call::LexicalInstanceCallSourceTargetV1;
use super::*;
use crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct ObjectCallSourceLoanV1 {
    source: LexicalInstanceCallSourceTargetV1,
    result_class: OrdinaryNewResultClassV1,
    roots: Box<[Rc<ResultOriginWitnessV1>]>,
}
impl ObjectCallSourceLoanV1 {
    pub(crate) fn source(&self) -> &LexicalInstanceCallSourceTargetV1 {
        &self.source
    }
    pub(crate) fn class(&self) -> &str {
        match &self.result_class {
            OrdinaryNewResultClassV1::Object(class)
            | OrdinaryNewResultClassV1::NullableObject(class) => class,
            OrdinaryNewResultClassV1::NullableForwarded { .. } => {
                unreachable!("object source loan admits only fresh result kinds")
            }
        }
    }
    pub(crate) fn result_class(&self) -> &OrdinaryNewResultClassV1 {
        &self.result_class
    }
    pub(crate) fn roots(&self) -> &[Rc<ResultOriginWitnessV1>] {
        &self.roots
    }
}

impl OrdinaryNewResultClassClaimsV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn object_call_source_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        source: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<ObjectCallSourceLoanV1, &'static str> {
        self.check_object_call_identity_v1(batch, selected, source)?;
        let result_class = self
            .get(source.target())
            .ok_or("object-call-source/class-unavailable")?;
        let class = match Some(result_class) {
            Some(
                OrdinaryNewResultClassV1::Object(class)
                | OrdinaryNewResultClassV1::NullableObject(class),
            ) => class,
            _ => return Err("object-call-source/class-unavailable"),
        };
        let roots =
            self.checked_original_source_roots_v1(source.target(), source.callee_owner())?;
        if roots.is_empty()
            || roots.iter().any(|root| match root.origin() {
                ResultValueOriginV1::Null => false,
                ResultValueOriginV1::Fresh(name) => name != class,
                _ => true,
            })
        {
            return Err("object-call-source/original-roots");
        }
        Ok(ObjectCallSourceLoanV1 {
            source: source.clone(),
            result_class: result_class.clone(),
            roots: roots.into_iter().map(Rc::clone).collect(),
        })
    }

    /// Corroborate this LOCAL's own initializer and complete fresh-result source.
    /// The canonical prefix observer, not this source check, issues its Home.
    pub(in crate::mir::normal_callable_semantic_package) fn check_owning_local_object_source_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        source: &LexicalInstanceCallSourceTargetV1,
        destination: BindingRefV1,
    ) -> Result<(), &'static str> {
        let loan = self.object_call_source_v1(batch, selected, source)?;
        if !matches!(loan.result_class(), OrdinaryNewResultClassV1::Object(_)) {
            return Err("object-local-source/result-kind");
        }
        self.original_local_object_target_source_v1(batch, selected, source, destination, &loan)?;
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_object_call_source_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        source: &LexicalInstanceCallSourceTargetV1,
        loan: &ObjectCallSourceLoanV1,
    ) -> Result<(), &'static str> {
        self.check_object_call_identity_v1(batch, selected, source)?;
        self.recheck_object_call_roots_v1(source, loan)
    }

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_object_call_roots_v1(
        &self,
        source: &LexicalInstanceCallSourceTargetV1,
        loan: &ObjectCallSourceLoanV1,
    ) -> Result<(), &'static str> {
        if source != loan.source() || self.get(source.target()) != Some(loan.result_class()) {
            return Err("object-call-source/source-drift");
        }
        let roots =
            self.checked_original_source_roots_v1(source.target(), source.callee_owner())?;
        if roots.len() != loan.roots().len()
            || roots
                .iter()
                .zip(loan.roots())
                .any(|(a, b)| !Rc::ptr_eq(a, b))
        {
            return Err("object-call-source/roots-drift");
        }
        Ok(())
    }

    fn check_object_call_identity_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        source: &LexicalInstanceCallSourceTargetV1,
    ) -> Result<(), &'static str> {
        let mut callees = batch
            .declarations()
            .filter(|row| row.batch_slot() == source.target_batch_slot());
        let callee = callees.next().ok_or("object-call-source/callee-missing")?;
        if callees.next().is_some()
            || callee.owner() != source.callee_owner()
            || source.callee_owner().compilation_brand()
                != source.call_site().owner().compilation_brand()
            || !matches!(selected.key_for_batch_slot(source.target_batch_slot()),
                Some(SelectedNormalCallableKeyV1::Cataloged(key)) if key == source.target())
        {
            return Err("object-call-source/selected-callee");
        }
        let mut callers = batch
            .declarations()
            .filter(|row| row.owner() == source.call_site().owner());
        let caller = callers.next().ok_or("object-call-source/caller-missing")?;
        if callers.next().is_some() {
            return Err("object-call-source/caller-ambiguous");
        }
        let exact = batch
            .with_lowering_input(caller.batch_slot(), |input| {
                input
                    .function()
                    .method_call(source.call_site().site())
                    .is_some_and(|call| {
                        call.owner() == input.owner()
                            && call.site() == source.call_site().site()
                            && source.matches_receiver(input, call)
                            && call.selector() == source.target().name()
                            && call.arity() == source.target().arity()
                            && call.arguments().len() == source.argument_sites().len()
                            && call.arguments().iter().enumerate().all(|(ordinal, arg)| {
                                arg.ordinal() == ordinal as u32
                                    && arg.site() == &source.argument_sites()[ordinal]
                            })
                    })
            })
            .map_err(|_| "object-call-source/caller-loan")?;
        if !exact {
            return Err("object-call-source/original-call");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nonterminal_object_call_source_retains_every_original_root_and_refuses_drift() {
        let text = "box Token { value: i64 birth(v) { me.value = v } } box Door { make(flag) { if flag == 0 { return null } return new Token(1) } observe(flag) { me.make(flag) return 0 } } static box Main { main() { return 0 } }";
        super::super::witness_tests::with_source_claims_fixture(
            text,
            |batch, selected, constructors, parameters, main, writers| {
                let (fields, _, classes, _) =
                    super::super::super::coseal_issue::source_claims::prepare_source_claims(
                        batch,
                        selected,
                        main,
                        constructors,
                        parameters,
                        std::rc::Rc::clone(writers), None,
                    )
                    .unwrap();
                let names: Vec<Box<str>> = batch
                    .ordinary_box_coverage()
                    .rows()
                    .iter()
                    .map(|row| row.name().into())
                    .collect();
                let needs =
                    super::super::super::lexical_instance_call::prepare_lexical_source_targets_v1(
                        batch,
                        selected,
                        &BTreeMap::new(),
                        &names,
                        &fields,
                        &classes,
                        &mut |_, _, _| Ok(None),
                        None,
                    )
                    .unwrap();
                let source = needs.iter().filter_map(|row| row.as_ref().unwrap().as_ref()).find_map(|need| match need {
                super::super::super::lexical_instance_call::PreparedSourceCallNeedV1::Lexical(row)
                    if row.call_site().site().node().segments() == [crate::mir::resolved_semantics::SourcePathSegmentV1::Body(0)] => Some(row),
                _ => None,
            }).unwrap();
                let loan = classes
                    .object_call_source_v1(batch, selected, source)
                    .unwrap();
                assert_eq!(loan.class(), "Token");
                assert_eq!(loan.roots().len(), 2);
                assert!(
                    classes
                        .object_return_qualification(source.call_site())
                        .is_none(),
                    "no fabricated caller Return"
                );
                classes
                    .recheck_object_call_source_v1(batch, selected, source, &loan)
                    .unwrap();
                let roots = classes
                    .checked_original_source_roots_v1(source.target(), source.callee_owner())
                    .unwrap();
                assert!(roots
                    .iter()
                    .zip(loan.roots())
                    .all(|(a, b)| Rc::ptr_eq(a, b)));
                for change in 0..5 {
                    let mut changed = loan.clone();
                    let mut roots = changed.roots.to_vec();
                    match change {
                        0 => {
                            roots.pop();
                        }
                        1 => roots.reverse(),
                        2 => roots[1] = Rc::clone(&roots[0]),
                        3 => {
                            changed.result_class = OrdinaryNewResultClassV1::Object("Other".into())
                        }
                        _ => {
                            changed.result_class =
                                OrdinaryNewResultClassV1::Object(loan.class().into())
                        }
                    }
                    changed.roots = roots.into_boxed_slice();
                    assert!(classes
                        .recheck_object_call_source_v1(batch, selected, source, &changed)
                        .is_err());
                }
                assert!(OrdinaryNewResultClassClaimsV1::new()
                    .object_call_source_v1(batch, selected, source)
                    .is_err());
            },
        );
    }
}
