//! Local state for the existing selected-New source prefix, not general Home Flow.
//!
//! Ordinary observations cannot carry an owned Home. The source scanner alone
//! records selected Normal installations; alias initialization stores a Handle.

use super::SelectedNewArgumentKindV1;
use super::{BindingRefV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1, SourceExprSiteV1};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use std::collections::BTreeMap;

enum StoredLocal {
    Home(BindingRefV1),
    Handle(BindingRefV1),
    Trivial,
    Uninitialized,
}

pub(super) enum OrdinaryObservation {
    Integer(i64),
    Bool(bool),
    TrivialLocal(BindingRefV1),
    Handle(BindingRefV1),
}

impl OrdinaryObservation {
    pub(super) fn is_trivial(&self) -> bool {
        match self {
            Self::Integer(_) | Self::Bool(_) | Self::TrivialLocal(_) => true,
            Self::Handle(_) => false,
        }
    }

    pub(super) fn into_selected_argument(self) -> Option<SelectedNewArgumentKindV1> {
        match self {
            Self::Integer(value) => Some(SelectedNewArgumentKindV1::Integer(value)),
            Self::Bool(value) => Some(SelectedNewArgumentKindV1::Bool(value)),
            Self::TrivialLocal(binding) => Some(SelectedNewArgumentKindV1::Local { binding }),
            Self::Handle(_) => None,
        }
    }
}

pub(super) struct PrefixLocalFlow<'source> {
    input: ResolvedFunctionLoweringInputV1<'source>,
    locals: BTreeMap<BindingRefV1, StoredLocal>,
}

impl<'source> PrefixLocalFlow<'source> {
    pub(super) fn new(input: ResolvedFunctionLoweringInputV1<'source>) -> Self {
        Self {
            input,
            locals: BTreeMap::new(),
        }
    }

    pub(super) fn observe(&self, site: &SourceExprSiteV1) -> Option<OrdinaryObservation> {
        match self.input.function().expression_source().literal(site) {
            Some(ResolvedLiteralSourceV1::Integer(value)) => {
                return Some(OrdinaryObservation::Integer(*value));
            }
            Some(ResolvedLiteralSourceV1::Bool(value)) => {
                return Some(OrdinaryObservation::Bool(*value));
            }
            _ => {}
        }
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        match self.locals.get(&binding)? {
            StoredLocal::Home(root) | StoredLocal::Handle(root) => {
                Some(OrdinaryObservation::Handle(*root))
            }
            StoredLocal::Trivial => Some(OrdinaryObservation::TrivialLocal(binding)),
            StoredLocal::Uninitialized => None,
        }
    }

    pub(super) fn install_uninitialized(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::Uninitialized);
    }

    pub(super) fn install_selected_normal_home(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::Home(binding));
    }

    pub(super) fn install_observed(&mut self, binding: BindingRefV1, value: OrdinaryObservation) {
        let stored = match value {
            OrdinaryObservation::Handle(root) => StoredLocal::Handle(root),
            OrdinaryObservation::Integer(_)
            | OrdinaryObservation::Bool(_)
            | OrdinaryObservation::TrivialLocal(_) => StoredLocal::Trivial,
        };
        self.locals.insert(binding, stored);
    }
}
