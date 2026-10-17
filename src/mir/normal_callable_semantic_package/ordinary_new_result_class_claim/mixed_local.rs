//! Mixed local result source, with all original outcomes and borrowed support.
//! This loan issues neither a received Home nor an executable result packet.
use super::call_witness::OriginalLocalCallSourceV1;
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::AppMainSourceRefV1;
use crate::mir::normal_callable_semantic_package::{
    model::OwnedCallableParameterContractDeclarationV1,
    selected_mapping::VerifiedSelectedCallableBatchMapV1,
    VerifiedInstanceConstructorSemanticBatchV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1, SourceBindingSiteV1};
use hakorune_mir_defs::CanonicalObjectIdV1;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct MixedLocalCallSourceLoanV1 {
    source: OriginalLocalCallSourceV1,
    destination: BindingRefV1,
    object: CanonicalObjectIdV1,
    anchor: MixedCallerAnchorV1,
    callee_ordinal: u32,
    roots: Box<[Rc<ResultOriginWitnessV1>]>,
}
/// Caller support is separate from the callee's complete result law. A local
/// acquisition is never substituted into a caller formal or outgoing Return.
#[derive(Debug, Clone)]
enum MixedCallerAnchorV1 {
    Formal {
        binding: BindingRefV1,
        ordinal: u32,
        witnesses: Box<[Rc<ResultOriginWitnessV1>]>,
    },
    Acquired {
        binding: BindingRefV1,
        source: Box<OriginalLocalCallSourceV1>,
        roots: Box<[Rc<ResultOriginWitnessV1>]>,
    },
}
impl MixedCallerAnchorV1 {
    fn same_as(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Formal {
                    binding: a,
                    ordinal: ao,
                    witnesses: aw,
                },
                Self::Formal {
                    binding: b,
                    ordinal: bo,
                    witnesses: bw,
                },
            ) => a == b && ao == bo && same_nodes(aw, bw),
            (
                Self::Acquired {
                    binding: a,
                    source: ac,
                    roots: ar,
                },
                Self::Acquired {
                    binding: b,
                    source: bc,
                    roots: br,
                },
            ) => a == b && ac == bc && same_nodes(ar, br),
            _ => false,
        }
    }
}
impl PartialEq for MixedLocalCallSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
            && self.destination == other.destination
            && self.object == other.object
            && self.anchor.same_as(&other.anchor)
            && self.callee_ordinal == other.callee_ordinal
            && same_nodes(&self.roots, &other.roots)
    }
}
impl Eq for MixedLocalCallSourceLoanV1 {}
fn same_nodes(left: &[Rc<ResultOriginWitnessV1>], right: &[Rc<ResultOriginWitnessV1>]) -> bool {
    left.len() == right.len() && left.iter().zip(right).all(|(a, b)| Rc::ptr_eq(a, b))
}
impl MixedLocalCallSourceLoanV1 {
    pub(crate) fn call(&self) -> &OwnedExprSiteV1 {
        &self.source.call
    }
    pub(crate) fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.source.declaration
    }
    pub(crate) fn destination(&self) -> BindingRefV1 {
        self.destination
    }
    pub(crate) fn receiver(&self) -> &SourceExprSiteV1 {
        self.source.receiver.as_ref().expect("mixed method source")
    }
    pub(crate) fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.source.key
    }
    pub(crate) fn callee_owner(&self) -> FunctionOwnerIdV1 {
        self.source.callee_owner
    }
    pub(crate) fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }
    pub(crate) fn support(&self) -> BindingRefV1 {
        match &self.anchor {
            MixedCallerAnchorV1::Formal { binding, .. }
            | MixedCallerAnchorV1::Acquired { binding, .. } => *binding,
        }
    }
    pub(crate) fn formal_support_v1(&self) -> Option<(BindingRefV1, u32)> {
        match &self.anchor {
            MixedCallerAnchorV1::Formal {
                binding, ordinal, ..
            } => Some((*binding, *ordinal)),
            MixedCallerAnchorV1::Acquired { .. } => None,
        }
    }
    pub(crate) fn acquired_call_v1(&self) -> Option<&OwnedExprSiteV1> {
        match &self.anchor {
            MixedCallerAnchorV1::Acquired { source, .. } => Some(&source.call),
            MixedCallerAnchorV1::Formal { .. } => None,
        }
    }
    pub(crate) fn acquired_declaration_v1(&self) -> Option<&SourceBindingSiteV1> {
        match &self.anchor {
            MixedCallerAnchorV1::Acquired { source, .. } => Some(&source.declaration),
            MixedCallerAnchorV1::Formal { .. } => None,
        }
    }
    /// Source-only correspondence to the retained acquisition, without Home.
    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_acquired_nullable_actual_v1(
        &self, actual: BindingRefV1,
    ) -> bool {
        let MixedCallerAnchorV1::Acquired { binding, source, roots } = &self.anchor else { return false; };
        if *binding != actual || source.call.owner() != actual.owner() { return false; }
        let fresh = roots.iter().any(|root| matches!(root.origin(), ResultValueOriginV1::Fresh(class)
            if class.as_ref() == self.fresh_class()));
        fresh && roots.iter().any(|root| matches!(root.origin(), ResultValueOriginV1::Null))
            && roots.iter().all(|root| matches!(root.origin(), ResultValueOriginV1::Null)
                || matches!(root.origin(), ResultValueOriginV1::Fresh(class) if class.as_ref() == self.fresh_class()))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn corroborates_acquired_mixed_actual_v1(
        &self, actual: BindingRefV1, parent: &Self,
    ) -> bool {
        let MixedCallerAnchorV1::Acquired { binding, source, roots } = &self.anchor else { return false; };
        *binding == actual && parent.destination == actual && source.as_ref() == &parent.source
            && parent.object == self.object && same_nodes(roots, &parent.roots)
    }

    pub(crate) fn callee_ordinal(&self) -> u32 {
        self.callee_ordinal
    }
    pub(in crate::mir::normal_callable_semantic_package) fn roots(
        &self,
    ) -> &[Rc<ResultOriginWitnessV1>] {
        &self.roots
    }
    pub(in crate::mir::normal_callable_semantic_package) fn fresh_class(&self) -> &str {
        self.roots
            .iter()
            .find_map(|root| match root.origin() {
                ResultValueOriginV1::Fresh(class) => Some(class.as_ref()),
                _ => None,
            })
            .expect("mixed source retained fresh class")
    }
    pub(crate) fn formal_witnesses_v1(&self) -> Option<&[Rc<ResultOriginWitnessV1>]> {
        match &self.anchor {
            MixedCallerAnchorV1::Formal { witnesses, .. } => Some(witnesses),
            MixedCallerAnchorV1::Acquired { .. } => None,
        }
    }
}

impl OrdinaryNewResultClassClaimsV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn mixed_local_call_source_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        call: &OwnedExprSiteV1,
        destination: BindingRefV1,
        app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<Option<MixedLocalCallSourceLoanV1>, &'static str> {
        let Some(source) = self.original_local_call_source_v1(
            batch,
            selected,
            fields,
            call,
            destination,
            parameters,
            app_main,
        )?
        else {
            return Ok(None);
        };
        if self.get(&source.key).is_some() {
            return Ok(None);
        }
        let roots = self.checked_original_source_roots_v1(&source.key, source.callee_owner)?;
        let fresh: BTreeSet<_> = roots
            .iter()
            .filter_map(|root| match root.origin() {
                ResultValueOriginV1::Fresh(class) => Some(class.as_ref()),
                _ => None,
            })
            .collect();
        let forwards: BTreeSet<_> = roots
            .iter()
            .filter_map(|root| match root.origin() {
                ResultValueOriginV1::ForwardFormal { ordinal } => Some(*ordinal),
                _ => None,
            })
            .collect();
        if fresh.is_empty() || forwards.is_empty() {
            return Ok(None);
        }
        if fresh.len() != 1 || forwards.len() != 1 || source.receiver.is_none() {
            return Err("mixed-local/source-family");
        }
        let class = *fresh.first().expect("one fresh class");
        let row = batch
            .ordinary_box_coverage()
            .row_for(class)
            .map_err(|_| "mixed-local/class-source")?
            .ok_or("mixed-local/class-source")?;
        let object = constructors
            .object_for(&row)
            .map_err(|_| "mixed-local/class-object")?;
        let callee_ordinal = *forwards.first().expect("one forwarded formal");
        let support = source
            .actuals
            .get(callee_ordinal as usize)
            .and_then(|row| row.binding)
            .ok_or("mixed-local/actual-root")?;
        let kind = batch
            .with_lowering_input(source.caller_slot, |input| {
                input.function().binding(support).map(|row| row.kind())
            })
            .map_err(|_| "mixed-local/caller-input")?
            .ok_or("mixed-local/actual-binding")?;
        let anchor = match kind {
            BindingKindV1::Parameter { .. } => {
                let (binding, ordinal) =
                    source.formal_actual_v1(batch, parameters, callee_ordinal)?;
                let witnesses = source.compose_at_v1(call, self, batch, parameters)?;
                let classes_agree = batch
                    .with_lowering_input(source.caller_slot, |input| {
                        witnesses.iter().all(|witness| {
                            child_relation::class_agrees(
                                witness.origin(),
                                object,
                                input,
                                source.caller_slot,
                                batch,
                                constructors,
                                parameters,
                            )
                        })
                    })
                    .map_err(|_| "mixed-local/caller-input")?;
                if !classes_agree {
                    return Err("mixed-local/class-disagreement");
                }
                MixedCallerAnchorV1::Formal {
                    binding,
                    ordinal,
                    witnesses: witnesses.into_boxed_slice(),
                }
            }
            BindingKindV1::Local { .. } => {
                let acquisition = batch
                    .with_lowering_input(source.caller_slot, |input| {
                        let mut rows = input
                            .function()
                            .expression_source()
                            .initializers()
                            .filter(|row| row.binding() == support);
                        let row = rows.next().ok_or("mixed-local/acquisition-missing")?;
                        if rows.next().is_some() || support == destination {
                            return Err("mixed-local/acquisition-identity");
                        }
                        row.initializer_site()
                            .cloned()
                            .map(|site| OwnedExprSiteV1::new(call.owner(), site))
                            .ok_or("mixed-local/acquisition-initializer")
                    })
                    .map_err(|_| "mixed-local/caller-input")??;
                if acquisition == *call {
                    return Err("mixed-local/acquisition-cycle");
                }
                let original = self
                    .original_local_call_source_v1(
                        batch,
                        selected,
                        fields,
                        &acquisition,
                        support,
                        parameters,
                        app_main,
                    )?
                    .ok_or("mixed-local/acquisition-call")?;
                let acquired_roots = self.checked_acquired_roots_v1(&original, class)?;
                MixedCallerAnchorV1::Acquired {
                    binding: support,
                    source: Box::new(original),
                    roots: acquired_roots.into_iter().cloned().collect(),
                }
            }
            _ => return Err("mixed-local/anchor-kind"),
        };
        let loan = MixedLocalCallSourceLoanV1 {
            source,
            destination,
            object,
            anchor,
            callee_ordinal,
            roots: roots.into_iter().cloned().collect(),
        };
        self.recheck_mixed_local_roots_v1(&loan)?;
        Ok(Some(loan))
    }

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_mixed_local_source_v1(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        constructors: &VerifiedInstanceConstructorSemanticBatchV1,
        fields: &super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        parameters: &[OwnedCallableParameterContractDeclarationV1],
        loan: &MixedLocalCallSourceLoanV1,
        app_main: Option<AppMainSourceRefV1<'_>>,
    ) -> Result<(), &'static str> {
        let original = self
            .original_local_call_source_v1(
                batch,
                selected,
                fields,
                loan.call(),
                loan.destination(),
                parameters,
                app_main,
            )?
            .ok_or("mixed-local/source-missing")?;
        if original != loan.source {
            return Err("mixed-local/source-drift");
        }
        self.recheck_mixed_local_roots_v1(loan)?;
        let MixedCallerAnchorV1::Formal {
            binding,
            ordinal,
            witnesses,
        } = &loan.anchor
        else {
            let MixedCallerAnchorV1::Acquired {
                binding, source, ..
            } = &loan.anchor
            else {
                unreachable!()
            };
            let current = self
                .original_local_call_source_v1(
                    batch,
                    selected,
                    fields,
                    &source.call,
                    *binding,
                    parameters,
                    app_main,
                )?
                .ok_or("mixed-local/acquisition-missing")?;
            if current != **source {
                return Err("mixed-local/acquisition-drift");
            }
            return Ok(());
        };
        if original.formal_actual_v1(batch, parameters, loan.callee_ordinal)?
            != (*binding, *ordinal)
        {
            return Err("mixed-local/formal-drift");
        }
        let current = original.compose_at_v1(loan.call(), self, batch, parameters)?;
        if !batch
            .with_lowering_input(original.caller_slot, |input| {
                current.iter().all(|witness| {
                    child_relation::class_agrees(
                        witness.origin(),
                        loan.object,
                        input,
                        original.caller_slot,
                        batch,
                        constructors,
                        parameters,
                    )
                })
            })
            .map_err(|_| "mixed-local/caller-input")?
        {
            return Err("mixed-local/class-drift");
        }
        if current.len() != witnesses.len()
            || current.iter().zip(witnesses).any(|(a, b)| {
                if a.site() != b.site() || a.origin() != b.origin() {
                    return true;
                }
                match (a.step(), b.step()) {
                    (
                        ResultWitnessStepV1::Call {
                            site: a,
                            key: ak,
                            callee: ac,
                            substitution: asub,
                        },
                        ResultWitnessStepV1::Call {
                            site: b,
                            key: bk,
                            callee: bc,
                            substitution: bsub,
                        },
                    ) => a != b || ak != bk || !Rc::ptr_eq(ac, bc) || asub != bsub,
                    _ => true,
                }
            })
        {
            return Err("mixed-local/composition-drift");
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn recheck_mixed_local_roots_v1(
        &self,
        loan: &MixedLocalCallSourceLoanV1,
    ) -> Result<(), &'static str> {
        if self.get(loan.key()).is_some() {
            return Err("mixed-local/legacy-class-drift");
        }
        let roots = self.checked_original_source_roots_v1(loan.key(), loan.callee_owner())?;
        if roots.len() != loan.roots.len()
            || roots
                .iter()
                .zip(&loan.roots)
                .any(|(a, b)| !Rc::ptr_eq(a, b))
        {
            return Err("mixed-local/original-roots-drift");
        }
        let mut fresh = false;
        let mut forwarded = false;
        for root in &roots {
            match root.origin() {
                ResultValueOriginV1::Null => {}
                ResultValueOriginV1::Fresh(class) if class.as_ref() == loan.fresh_class() => {
                    fresh = true
                }
                ResultValueOriginV1::ForwardFormal { ordinal }
                    if *ordinal == loan.callee_ordinal =>
                {
                    forwarded = true
                }
                _ => return Err("mixed-local/origin-drift"),
            }
        }
        if !fresh || !forwarded {
            return Err("mixed-local/mixed-roots-missing");
        }
        if loan
            .source
            .actuals
            .get(loan.callee_ordinal as usize)
            .and_then(|row| row.binding)
            != Some(loan.support())
        {
            return Err("mixed-local/anchor-actual-drift");
        }
        let MixedCallerAnchorV1::Formal {
            binding,
            ordinal,
            witnesses,
        } = &loan.anchor
        else {
            let MixedCallerAnchorV1::Acquired {
                source,
                roots: retained,
                ..
            } = &loan.anchor
            else {
                unreachable!()
            };
            let current = self.checked_acquired_roots_v1(source, loan.fresh_class())?;
            if current.len() != retained.len()
                || current.iter().zip(retained).any(|(a, b)| !Rc::ptr_eq(a, b))
            {
                return Err("mixed-local/acquisition-roots-drift");
            }
            return Ok(());
        };
        if roots.len() != witnesses.len() {
            return Err("mixed-local/composed-census");
        }
        for (root, witness) in roots.iter().zip(witnesses) {
            let ResultWitnessStepV1::Call {
                site,
                key,
                callee,
                substitution,
            } = witness.step()
            else {
                return Err("mixed-local/witness-role");
            };
            if site != loan.call()
                || witness.site() != loan.call()
                || key != loan.key()
                || !Rc::ptr_eq(callee, root)
            {
                return Err("mixed-local/witness-identity");
            }
            match (root.formal_ordinal(), substitution) {
                (None, None) => {}
                (Some(original), Some(actual))
                    if original == loan.callee_ordinal
                        && actual.callee_ordinal == original
                        && actual.binding == *binding
                        && actual.caller_ordinal == *ordinal
                        && loan
                            .source
                            .actuals
                            .get(original as usize)
                            .is_some_and(|source| {
                                source.site == actual.argument_site
                                    && source.binding == Some(actual.binding)
                            }) => {}
                _ => return Err("mixed-local/formal-substitution"),
            }
            match (root.origin(), witness.origin()) {
                (ResultValueOriginV1::Null, ResultValueOriginV1::Null) => {}
                (ResultValueOriginV1::Fresh(a), ResultValueOriginV1::Fresh(b)) if a == b => {}
                (
                    ResultValueOriginV1::ForwardFormal { ordinal: original },
                    ResultValueOriginV1::ForwardFormal { ordinal: caller },
                ) if *original == loan.callee_ordinal && *caller == *ordinal => {}
                _ => return Err("mixed-local/origin-drift"),
            }
        }
        Ok(())
    }

    fn checked_acquired_roots_v1<'a>(
        &'a self,
        source: &OriginalLocalCallSourceV1,
        class: &str,
    ) -> Result<Vec<&'a Rc<ResultOriginWitnessV1>>, &'static str> {
        let roots = self.checked_original_source_roots_v1(&source.key, source.callee_owner)?;
        let mut fresh = false;
        for root in &roots {
            match root.origin() {
                ResultValueOriginV1::Fresh(original) if original.as_ref() == class => fresh = true,
                ResultValueOriginV1::Null | ResultValueOriginV1::ForwardFormal { .. } => {}
                _ => return Err("mixed-local/acquisition-class"),
            }
        }
        if !fresh {
            return Err("mixed-local/acquisition-class");
        }
        Ok(roots)
    }
}
