//! A lexical analysis/proof scope for one immutable MIR and projection.
//! Source ownership is still independently checked by each consumer.
use super::physical_boundary::FinishedBindings;
use super::root_home::BranchScalarPhiObservationV1 as Branch;
use super::*;
use crate::mir::verification::geometry::{FunctionVerificationFacts, FunctionVerificationGeometry};
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug)]
pub(in crate::mir) struct PhysicalValidationState {
    function: *const MirFunction,
    projection: Option<*const FinishedBindings>,
    finalized: bool,
    facts: Rc<FunctionVerificationFacts>,
    completed: RefCell<BTreeMap<*const Branch, Rc<Branch>>>,
}
pub(in crate::mir) struct PhysicalValidationGuard<'a> {
    ledger: &'a OrdinaryNewClaimLedgerV1,
    // These borrows prevent mutation until the guard restores the old scope.
    _function: &'a MirFunction,
    _projection: Option<&'a FinishedBindings>,
    previous: Option<Option<Rc<PhysicalValidationState>>>,
}
impl Drop for PhysicalValidationGuard<'_> {
    fn drop(&mut self) {
        if let Some(previous) = self.previous.take() {
            self.ledger.physical_validation_scope.replace(previous);
        }
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir) fn physical_validation_scope_v1<'a>(
        &'a self,
        function: &'a MirFunction,
        projection: Option<&'a FinishedBindings>,
        finalized: bool,
    ) -> PhysicalValidationGuard<'a> {
        let same = self
            .physical_validation_scope
            .borrow()
            .as_ref()
            .is_some_and(|scope| {
                scope.finalized == finalized
                    && std::ptr::eq(scope.function, function)
                    && scope.projection == projection.map(|p| p as *const _)
            });
        let previous = if same {
            None
        } else {
            Some(
                self.physical_validation_scope
                    .replace(Some(Rc::new(PhysicalValidationState {
                        function,
                        projection: projection.map(|p| p as *const _),
                        finalized,
                        facts: Rc::default(),
                        completed: RefCell::new(BTreeMap::new()),
                    }))),
            )
        };
        PhysicalValidationGuard {
            ledger: self,
            _function: function,
            _projection: projection,
            previous,
        }
    }
    pub(in crate::mir) fn with_function_geometry_v1<R>(
        &self,
        function: &MirFunction,
        check: impl FnOnce(&FunctionVerificationGeometry<'_>) -> R,
    ) -> R {
        let state = self
            .physical_validation_scope
            .borrow()
            .as_ref()
            .filter(|scope| std::ptr::eq(scope.function, function))
            .cloned();
        match state {
            Some(scope) => check(&FunctionVerificationGeometry::borrow(
                function,
                &scope.facts,
            )),
            None => check(&FunctionVerificationGeometry::new(function)),
        }
    }
    pub(super) fn with_physical_geometry_v1<R>(
        &self,
        function: &MirFunction,
        projection: Option<&FinishedBindings>,
        finalized: bool,
        check: impl FnOnce(
            &FunctionVerificationGeometry<'_>,
            &RefCell<BTreeMap<*const Branch, Rc<Branch>>>,
        ) -> Result<R, String>,
    ) -> Result<R, String> {
        let _scope = self.physical_validation_scope_v1(function, projection, finalized);
        let scope = Rc::clone(self.physical_validation_scope.borrow().as_ref().unwrap());
        check(
            &FunctionVerificationGeometry::borrow(function, &scope.facts),
            &scope.completed,
        )
    }
}

#[derive(Debug)]
pub(in super::super) enum RootNewValidation {
    Unregistered,
    Pending(FunctionOwnerIdV1),
    Checked(FunctionOwnerIdV1, physical_boundary::PhysicalBoundary),
    FinishingChecked {
        owner: FunctionOwnerIdV1,
        symbol: String,
        projection: Rc<physical_boundary::FinishedBindings>,
    },
    ArtifactFinalized {
        owner: FunctionOwnerIdV1,
        symbol: String,
        projection: Rc<physical_boundary::FinishedBindings>,
    },
}

/// Physical validation retained for one selected ordinary child.  This is
/// request-local finishing state, not a source receipt or a second owner.
#[derive(Debug)]
pub(in super::super) enum ChildPhysicalValidation {
    Checked {
        symbol: String,
        boundary: physical_boundary::PhysicalBoundary,
    },
    FinishingChecked {
        symbol: String,
        projection: physical_boundary::FinishedBindings,
    },
}
