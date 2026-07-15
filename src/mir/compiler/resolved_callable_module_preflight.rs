//! MP0-P0 whole-module capability/profile preflight.
//!
//! Every function plan is sealed before this product is returned. This module
//! owns no Builder, MIR draft, publication, runtime, or backend authority.

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::CanonicalCallableKeyV1;

use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::lowering_input::CanonicalLoweringErrorV1;
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;

#[derive(Debug)]
pub(crate) enum CallableModulePreflightErrorV1 {
    Function {
        key: CanonicalCallableKeyV1,
        source: CanonicalLoweringErrorV1,
    },
    DuplicateCanonicalKey(CanonicalCallableKeyV1),
    CardinalityMismatch {
        functions: usize,
        plans: usize,
    },
}

/// One resolved module paired with a complete canonical-keyed plan set.
#[derive(Debug)]
pub(crate) struct VerifiedCallableModulePreflightV1<'a> {
    module: &'a VerifiedResolvedCallableModuleV1,
    plans_by_key: BTreeMap<CanonicalCallableKeyV1, CanonicalFirstFamilyPlanV1<'a>>,
}

impl<'a> VerifiedCallableModulePreflightV1<'a> {
    pub(crate) fn verify(
        module: &'a VerifiedResolvedCallableModuleV1,
    ) -> Result<Self, CallableModulePreflightErrorV1> {
        let mut plans_by_key = BTreeMap::new();
        for key in module.functions_by_key().keys() {
            let input = module.function_input(key).map_err(|source| {
                CallableModulePreflightErrorV1::Function {
                    key: key.clone(),
                    source,
                }
            })?;
            let plan = CanonicalLoweringPreflightV1::verify_function(input).map_err(|source| {
                CallableModulePreflightErrorV1::Function {
                    key: key.clone(),
                    source,
                }
            })?;
            if plans_by_key.insert(key.clone(), plan).is_some() {
                return Err(CallableModulePreflightErrorV1::DuplicateCanonicalKey(
                    key.clone(),
                ));
            }
        }
        if plans_by_key.len() != module.functions_by_key().len() {
            return Err(CallableModulePreflightErrorV1::CardinalityMismatch {
                functions: module.functions_by_key().len(),
                plans: plans_by_key.len(),
            });
        }
        Ok(Self {
            module,
            plans_by_key,
        })
    }

    pub(crate) const fn module(&self) -> &'a VerifiedResolvedCallableModuleV1 {
        self.module
    }

    pub(crate) fn plans_by_key(
        &self,
    ) -> &BTreeMap<CanonicalCallableKeyV1, CanonicalFirstFamilyPlanV1<'a>> {
        &self.plans_by_key
    }
}
