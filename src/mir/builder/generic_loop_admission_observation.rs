//! Diagnostic-only first-admission observation for the bounded callable Loop
//! handoff.  This module owns no GenericLoop semantics, route selection, or
//! ValueId/type repair; it only retains the already-issued source relations
//! long enough for one opt-in audit row.

#![allow(dead_code)]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::parser::CallableMethodSourceObservationV1;

use super::raw_invocation_source_transport::RawInvocationSourceContextV1;
use super::stmts::LocalInitializerObservationSinkV1;
use super::stmts::LocalInitializerObservationV1;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct GenericLoopAdmissionObservationV1 {
    method_source: CallableMethodSourceObservationV1,
    loop_source: RawInvocationSourceContextV1,
    initializers: Box<[LocalInitializerObservationV1]>,
    admission_index: u32,
}

impl GenericLoopAdmissionObservationV1 {
    pub(in crate::mir::builder) fn issue(
        method_source: Option<CallableMethodSourceObservationV1>,
        loop_source: &RawInvocationSourceContextV1,
        initializers: Vec<LocalInitializerObservationV1>,
        admission_index: u32,
    ) -> Option<Self> {
        if !crate::config::env::joinir_dev::debug_enabled() {
            return None;
        }
        method_source.map(|method_source| Self {
            method_source,
            loop_source: loop_source.clone(),
            initializers: initializers.into_boxed_slice(),
            admission_index,
        })
    }

    pub(in crate::mir::builder) fn method_source(&self) -> &CallableMethodSourceObservationV1 {
        &self.method_source
    }

    pub(in crate::mir::builder) fn loop_source(&self) -> &RawInvocationSourceContextV1 {
        &self.loop_source
    }

    pub(in crate::mir::builder) fn initializers(&self) -> &[LocalInitializerObservationV1] {
        &self.initializers
    }

    pub(in crate::mir::builder) const fn admission_index(&self) -> u32 {
        self.admission_index
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct GenericLoopAdmissionDiagnosticStateV1 {
    method_source: Option<CallableMethodSourceObservationV1>,
    local_initializers: LocalInitializerObservationSinkV1,
    next_index: Rc<Cell<u32>>,
    observations: Rc<RefCell<Vec<GenericLoopAdmissionObservationV1>>>,
}

impl GenericLoopAdmissionDiagnosticStateV1 {
    pub(in crate::mir::builder) fn new() -> Self {
        Self {
            method_source: None,
            local_initializers: Rc::new(RefCell::new(Vec::new())),
            next_index: Rc::new(Cell::new(0)),
            observations: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub(in crate::mir::builder) fn reborrow(&self) -> Self {
        Self {
            method_source: self.method_source.clone(),
            local_initializers: self.local_initializers.clone(),
            next_index: self.next_index.clone(),
            observations: self.observations.clone(),
        }
    }

    pub(in crate::mir::builder) fn replace_method_source(
        &mut self,
        source: Option<CallableMethodSourceObservationV1>,
    ) -> Option<CallableMethodSourceObservationV1> {
        std::mem::replace(&mut self.method_source, source)
    }

    pub(in crate::mir::builder) fn method_source(
        &self,
    ) -> Option<&CallableMethodSourceObservationV1> {
        self.method_source.as_ref()
    }

    pub(in crate::mir::builder) fn local_initializer_sink(
        &self,
    ) -> LocalInitializerObservationSinkV1 {
        self.local_initializers.clone()
    }

    pub(in crate::mir::builder) fn issue_for_loop(
        &self,
        loop_source: &RawInvocationSourceContextV1,
    ) -> Option<GenericLoopAdmissionObservationV1> {
        if !crate::config::env::joinir_dev::debug_enabled() {
            return None;
        }
        let Some(method_source) = self.method_source.clone() else {
            return None;
        };
        let observation = GenericLoopAdmissionObservationV1::issue(
            Some(method_source),
            loop_source,
            std::mem::take(&mut *self.local_initializers.borrow_mut()),
            self.next_admission_index(),
        );
        if let Some(observation) = observation.as_ref() {
            self.observations.borrow_mut().push(observation.clone());
        }
        observation
    }

    fn next_admission_index(&self) -> u32 {
        let index = self.next_index.get();
        self.next_index.set(index.saturating_add(1));
        index
    }

    pub(in crate::mir::builder) fn observations(&self) -> Vec<GenericLoopAdmissionObservationV1> {
        self.observations.borrow().clone()
    }
}
