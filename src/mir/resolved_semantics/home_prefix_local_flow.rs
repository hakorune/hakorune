//! Local state for the existing selected-New source prefix, not general Home Flow.
//!
//! Ordinary observations cannot carry an owned Home. The source scanner alone
//! records selected Normal installations; alias initialization stores a Handle.

use super::SelectedNewArgumentKindV1;
use super::{BindingRefV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1, SourceExprSiteV1};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use std::collections::BTreeMap;

/// Source scalar class retained from an exact literal or declaration contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SourceScalarKind {
    Integer,
    Bool,
}

enum StoredLocal {
    Home { acquisition: super::OwnedExprSiteV1 },
    Map,
    Consumed,
    Handle(BindingRefV1),
    Trivial(Option<SourceScalarKind>),
    Uninitialized,
}

pub(super) enum OrdinaryObservation {
    Integer(i64),
    Bool(bool),
    TrivialLocal(BindingRefV1, Option<SourceScalarKind>),
    Handle(BindingRefV1),
}

impl OrdinaryObservation {
    pub(super) fn is_trivial(&self) -> bool {
        match self {
            Self::Integer(_) | Self::Bool(_) | Self::TrivialLocal(..) => true,
            Self::Handle(_) => false,
        }
    }

    pub(super) fn into_selected_argument(self) -> Option<SelectedNewArgumentKindV1> {
        match self {
            Self::Integer(value) => Some(SelectedNewArgumentKindV1::Integer(value)),
            Self::Bool(value) => Some(SelectedNewArgumentKindV1::Bool(value)),
            Self::TrivialLocal(binding, _) => Some(SelectedNewArgumentKindV1::Local { binding }),
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

    // Declaration contracts are borrowed from the sole package issuer. No
    // physical signature or default capability participates in entry admission.
    pub(super) fn install_parameters(
        &mut self,
        parameters: impl IntoIterator<
            Item = (
                u32,
                BindingRefV1,
                crate::mir::callable_parameter_contract::CallableParameterContractKindV1,
            ),
        >,
    ) -> bool {
        let mut count = 0;
        for (ordinal, binding, kind) in parameters {
            if binding.owner() != self.input.owner()
                || self
                    .input
                    .function()
                    .declaration_binding(&super::SourceBindingSiteV1::Parameter { index: ordinal })
                    != Some(binding)
                || self.locals.contains_key(&binding)
            {
                return false;
            }
            use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
            use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
            let value = match kind {
                CallableParameterContractKindV1::ExactTrivial(abi)
                    if abi == ExactTrivialParameterAbiV1::I64 =>
                {
                    StoredLocal::Trivial(Some(SourceScalarKind::Integer))
                }
                CallableParameterContractKindV1::ExactTrivial(_) => return false,
                CallableParameterContractKindV1::OpaqueHandle
                | CallableParameterContractKindV1::ExactText(_) => StoredLocal::Handle(binding),
            };
            self.locals.insert(binding, value);
            count += 1;
        }
        count
            == self
                .input
                .function()
                .declaration_sites()
                .filter(|site| matches!(site, super::SourceBindingSiteV1::Parameter { .. }))
                .count()
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
            StoredLocal::Home { .. } | StoredLocal::Map => {
                Some(OrdinaryObservation::Handle(binding))
            }
            StoredLocal::Handle(root)
                if !matches!(self.locals.get(root), Some(StoredLocal::Consumed)) =>
            {
                Some(OrdinaryObservation::Handle(*root))
            }
            StoredLocal::Handle(_) | StoredLocal::Consumed => None,
            StoredLocal::Trivial(kind) => Some(OrdinaryObservation::TrivialLocal(binding, *kind)),
            StoredLocal::Uninitialized => None,
        }
    }

    pub(super) fn direct_available_home(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<(BindingRefV1, &super::OwnedExprSiteV1)> {
        let ResolvedLexicalRefV1::Local(binding) = self.input.function().variable_ref(site)? else {
            return None;
        };
        match self.locals.get(&binding)? {
            StoredLocal::Home { acquisition } => Some((binding, acquisition)),
            _ => None,
        }
    }
    pub(super) fn consume_home(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::Consumed);
    }
    pub(super) fn install_map(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::Map);
    }

    pub(super) fn install_i64_call_result(&mut self, binding: BindingRefV1) {
        self.locals.insert(
            binding,
            StoredLocal::Trivial(Some(SourceScalarKind::Integer)),
        );
    }

    pub(super) fn install_uninitialized(&mut self, binding: BindingRefV1) {
        self.locals.insert(binding, StoredLocal::Uninitialized);
    }

    pub(super) fn install_selected_normal_home(
        &mut self,
        binding: BindingRefV1,
        acquisition: super::OwnedExprSiteV1,
    ) {
        self.locals
            .insert(binding, StoredLocal::Home { acquisition });
    }

    pub(super) fn install_observed(&mut self, binding: BindingRefV1, value: OrdinaryObservation) {
        let stored = match value {
            OrdinaryObservation::Handle(root) => StoredLocal::Handle(root),
            OrdinaryObservation::Integer(_) => {
                StoredLocal::Trivial(Some(SourceScalarKind::Integer))
            }
            OrdinaryObservation::Bool(_) => StoredLocal::Trivial(Some(SourceScalarKind::Bool)),
            OrdinaryObservation::TrivialLocal(_, kind) => StoredLocal::Trivial(kind),
        };
        self.locals.insert(binding, stored);
    }
}
