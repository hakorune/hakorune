//! Typed source loan for the canonical object-return handoff observer.
//! Carries the same immutable witnesses; no acquired Home or transfer is issued.
use super::*;
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct ObjectReturnCallQualificationV1 {
    value: OwnedExprSiteV1,
    call: OwnedExprSiteV1,
    key: CanonicalSameModuleCallableKeyV1,
    class: OrdinaryNewResultClassV1,
    witnesses: Box<[Rc<ResultOriginWitnessV1>]>,
}
impl ObjectReturnCallQualificationV1 {
    pub(crate) fn value(&self) -> &OwnedExprSiteV1 {
        &self.value
    }
    pub(crate) fn call(&self) -> &OwnedExprSiteV1 {
        &self.call
    }
    pub(crate) fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.key
    }
    pub(crate) fn class(&self) -> &OrdinaryNewResultClassV1 {
        &self.class
    }
    pub(crate) fn witnesses(&self) -> &[Rc<ResultOriginWitnessV1>] {
        &self.witnesses
    }
}
impl PartialEq for ObjectReturnCallQualificationV1 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.call == other.call
            && self.key == other.key
            && self.class == other.class
            && self.witnesses.len() == other.witnesses.len()
            && self
                .witnesses
                .iter()
                .zip(other.witnesses.iter())
                .all(|(a, b)| Rc::ptr_eq(a, b))
    }
}
impl Eq for ObjectReturnCallQualificationV1 {}

impl OrdinaryNewResultClassClaimsV1 {
    /// A retained source obligation selects the existing verified terminal walk.
    pub(in crate::mir::normal_callable_semantic_package) fn has_object_call_return(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> bool {
        self.source_rows()
            .flat_map(|(_, exits)| exits.iter())
            .any(|exit| {
                exit.site().owner() == owner
                    && self.object_return_qualification(exit.site()).is_some()
            })
    }

    /// Retain exact callee call-return dependencies before caller observation.
    /// Primitive leaf returns need no dependency; none issues result execution.
    pub(in crate::mir::normal_callable_semantic_package) fn object_return_dependencies(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Option<Box<[ObjectReturnCallQualificationV1]>> {
        let class = self.get(key)?;
        if !matches!(
            class,
            OrdinaryNewResultClassV1::Object(_) | OrdinaryNewResultClassV1::NullableObject(_)
        ) {
            return None;
        }
        let exits = self.outcomes(key)?;
        if exits.is_empty() {
            return None;
        }
        let mut dependencies = Vec::new();
        for exit in exits {
            if exit.site().owner() != owner || exit.witnesses().is_empty() {
                return None;
            }
            if exit
                .witnesses()
                .iter()
                .any(|witness| matches!(witness.step(), ResultWitnessStepV1::Call { .. }))
            {
                let loan = self.object_return_qualification(exit.site())?;
                if loan.class().class() != class.class() {
                    return None;
                }
                dependencies.push(loan);
            } else if !exit.witnesses().iter().all(|witness| {
                witness.site() == exit.site()
                    && match (witness.step(), witness.origin()) {
                        (ResultWitnessStepV1::NullLiteral, ResultValueOriginV1::Null) => true,
                        (
                            ResultWitnessStepV1::FreshConstruction,
                            ResultValueOriginV1::Fresh(name),
                        ) => Some(name.as_ref()) == class.class(),
                        _ => false,
                    }
            }) {
                return None;
            }
        }
        (!dependencies.is_empty()).then(|| dependencies.into_boxed_slice())
    }

    /// All original return values associated with one acquisition. No single
    /// terminal value stands in for another return of the same received local.
    pub(in crate::mir::normal_callable_semantic_package) fn qualifications_for_call(
        &self,
        call: &OwnedExprSiteV1,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Box<[ObjectReturnCallQualificationV1]> {
        let mut loans: Vec<_> = self
            .source_rows()
            .flat_map(|(_, exits)| exits.iter())
            .filter_map(|exit| self.object_return_qualification(exit.site()))
            .filter(|loan| loan.call() == call && loan.key() == key)
            .collect();
        loans.sort_by(|a, b| a.value().cmp(b.value()));
        loans.into_boxed_slice()
    }

    /// Only the existing solver product can lend this source qualification.
    /// The terminal observer must still seal arguments/live receiver and the
    /// completed-index owner must verify exact callee lifecycle obligations.
    pub(crate) fn object_return_qualification(
        &self,
        value: &OwnedExprSiteV1,
    ) -> Option<ObjectReturnCallQualificationV1> {
        let mut rows = self
            .source_rows()
            .flat_map(|(_, exits)| exits.iter())
            .filter(|row| row.site() == value);
        let row = rows.next()?;
        if rows.next().is_some() {
            return None;
        }
        let first = row.witnesses().first()?;
        let ResultWitnessStepV1::Call {
            site: call, key, ..
        } = first.step()
        else {
            return None;
        };
        if call.owner() != value.owner() {
            return None;
        }
        let class = self.get(key)?;
        if !matches!(
            class,
            OrdinaryNewResultClassV1::Object(_) | OrdinaryNewResultClassV1::NullableObject(_)
        ) {
            return None;
        }
        let callee_exits = self.outcomes(key)?;
        for witness in row.witnesses() {
            let ResultWitnessStepV1::Call {
                site,
                key: target,
                callee,
                substitution,
            } = witness.step()
            else {
                return None;
            };
            if witness.site() != value
                || site != call
                || target != key
                || substitution.is_some()
                || !callee_exits
                    .iter()
                    .flat_map(|exit| exit.witnesses())
                    .any(|candidate| Rc::ptr_eq(candidate, callee))
            {
                return None;
            }
            match witness.origin() {
                ResultValueOriginV1::Null => {}
                ResultValueOriginV1::Fresh(name) if Some(name.as_ref()) == class.class() => {}
                _ => return None,
            }
        }
        Some(ObjectReturnCallQualificationV1 {
            value: value.clone(),
            call: call.clone(),
            key: key.clone(),
            class: class.clone(),
            witnesses: row.witnesses().iter().map(Rc::clone).collect(),
        })
    }
}

#[cfg(test)]
#[path = "object_return_loan_tests.rs"]
mod tests;
