//! P0c-B1 exact two-function, one-sibling-edge activation witness.
//!
//! The complete catalog/module products remain the semantic authorities. This
//! box only proves the narrow production cardinality before Builder effects.

use crate::mir::resolved_semantics::{CanonicalCallableKeyV1, FunctionOwnerIdV1};

use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use super::resolved_callable_module_preflight::{
    CallableModulePreflightErrorV1, VerifiedCallableModulePreflightV1,
};

#[derive(Debug)]
pub(crate) enum SiblingCallActivationErrorV1 {
    FunctionCardinality { actual: usize },
    RootCardinality(CanonicalCallableKeyV1),
    MissingRootFunction(CanonicalCallableKeyV1),
    DirectCallCardinality { actual: usize },
    SelfCallOnly { owner: FunctionOwnerIdV1 },
    Preflight(CallableModulePreflightErrorV1),
}

pub(crate) struct VerifiedSiblingCallModulePlanV1<'a> {
    preflight: VerifiedCallableModulePreflightV1<'a>,
}

impl<'a> VerifiedSiblingCallModulePlanV1<'a> {
    pub(crate) fn verify(
        module: &'a VerifiedResolvedCallableModuleV1,
    ) -> Result<Self, SiblingCallActivationErrorV1> {
        if module.functions_by_key().len() != 2 {
            return Err(SiblingCallActivationErrorV1::FunctionCardinality {
                actual: module.functions_by_key().len(),
            });
        }

        let mut edge = None;
        let mut edge_count = 0usize;
        for (key, unit) in module.functions_by_key() {
            let [caller] = unit.forest().roots() else {
                return Err(SiblingCallActivationErrorV1::RootCardinality(key.clone()));
            };
            let function = unit
                .forest()
                .owner(*caller)
                .ok_or_else(|| SiblingCallActivationErrorV1::MissingRootFunction(key.clone()))?;
            for (_, target) in function.direct_call_targets() {
                edge_count += 1;
                edge = Some((*caller, target.callable().owner()));
            }
        }
        if edge_count != 1 {
            return Err(SiblingCallActivationErrorV1::DirectCallCardinality { actual: edge_count });
        }
        let Some((caller, target)) = edge else {
            return Err(SiblingCallActivationErrorV1::DirectCallCardinality { actual: edge_count });
        };
        if caller == target {
            return Err(SiblingCallActivationErrorV1::SelfCallOnly { owner: caller });
        }

        let preflight = VerifiedCallableModulePreflightV1::verify(module)
            .map_err(SiblingCallActivationErrorV1::Preflight)?;
        Ok(Self { preflight })
    }

    pub(crate) fn into_preflight(self) -> VerifiedCallableModulePreflightV1<'a> {
        self.preflight
    }
}
