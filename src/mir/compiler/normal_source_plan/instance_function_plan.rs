//! Cumulative disconnected function-plan owner for plain instance methods.
//!
//! This module owns the module source, all-method iteration, exact key
//! coverage, and the cumulative plan vocabulary. Variant-specific source
//! grammar remains in sibling modules.

use std::collections::BTreeMap;

use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};
use crate::mir::compiler::lowering_input::CanonicalLoweringErrorV1;
use crate::mir::compiler::source_projection::{
    SourceNavigationErrorV1, VerifiedSourceProjectionV1,
};
use crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1,
    ResolveOwnerForestErrorV1, VerifiedSemanticOwnerForestV1,
};

use super::instance_integer_return_plan::{
    seal_integer_literal_return_one, VerifiedNormalInstanceIntegerReturnPlanV1,
};
use super::main_source::{NormalMainFunctionSourceErrorV1, NormalMainFunctionSourceViewV1};
use super::module_source::{NormalInstanceMethodSourceLoanErrorV1, VerifiedNormalModuleSourceV1};

#[derive(Debug)]
pub(crate) struct VerifiedNormalInstanceFunctionFactsV1 {
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    receiver: BindingRefV1,
}

impl VerifiedNormalInstanceFunctionFactsV1 {
    pub(super) const fn new(
        forest: VerifiedSemanticOwnerForestV1,
        projection: VerifiedSourceProjectionV1,
        receiver: BindingRefV1,
    ) -> Self {
        Self {
            forest,
            projection,
            receiver,
        }
    }

    pub(crate) const fn receiver(&self) -> BindingRefV1 {
        self.receiver
    }

    pub(crate) fn owner_count(&self) -> usize {
        self.forest.owner_count()
    }
}

#[derive(Debug)]
pub(crate) enum VerifiedNormalInstanceFunctionPlanV1 {
    IntegerLiteralReturn(VerifiedNormalInstanceIntegerReturnPlanV1),
}

impl VerifiedNormalInstanceFunctionPlanV1 {
    pub(crate) const fn as_integer_literal_return(
        &self,
    ) -> Option<&VerifiedNormalInstanceIntegerReturnPlanV1> {
        match self {
            Self::IntegerLiteralReturn(plan) => Some(plan),
        }
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalInstanceFunctionPlanSetV1 {
    source: VerifiedNormalModuleSourceV1,
    plans: BTreeMap<CanonicalSameModuleCallableKeyV1, VerifiedNormalInstanceFunctionPlanV1>,
}

impl VerifiedNormalInstanceFunctionPlanSetV1 {
    pub(crate) fn source_identity(&self) -> &str {
        self.source.source_identity()
    }

    pub(crate) fn plans(
        &self,
    ) -> impl Iterator<
        Item = (
            &CanonicalSameModuleCallableKeyV1,
            &VerifiedNormalInstanceFunctionPlanV1,
        ),
    > {
        self.plans.iter()
    }

    pub(crate) fn len(&self) -> usize {
        self.plans.len()
    }

    pub(super) fn borrow_exact_main_function(
        &self,
    ) -> Result<NormalMainFunctionSourceViewV1<'_>, NormalMainFunctionSourceErrorV1> {
        self.source.borrow_exact_main_function()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneralFunctionPlanStageV1 {
    Inventory,
    Source,
    Resolve,
    Recipe,
    Completion,
    Pairing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneralFunctionSignatureStopV1 {
    Parameters,
    ParameterDeclarations,
    ReturnAnnotation,
    Uses,
    Attributes,
}

#[derive(Debug)]
pub(crate) enum GeneralFunctionPlanErrorV1 {
    NoInstanceMethod,
    Source {
        key: CanonicalSameModuleCallableKeyV1,
        cause: NormalInstanceMethodSourceLoanErrorV1,
    },
    UnsupportedSignature {
        key: CanonicalSameModuleCallableKeyV1,
        reason: GeneralFunctionSignatureStopV1,
    },
    ResolverSession(ResolveFunctionErrorV1),
    Resolver {
        key: CanonicalSameModuleCallableKeyV1,
        cause: ResolveOwnerForestErrorV1,
    },
    Projection {
        key: CanonicalSameModuleCallableKeyV1,
        cause: SourceNavigationErrorV1,
    },
    Input {
        key: CanonicalSameModuleCallableKeyV1,
        cause: CanonicalLoweringErrorV1,
    },
    FactCoverage {
        key: CanonicalSameModuleCallableKeyV1,
        reason: &'static str,
    },
    UnsupportedBody {
        key: CanonicalSameModuleCallableKeyV1,
        reason: &'static str,
    },
    Completion {
        key: CanonicalSameModuleCallableKeyV1,
        cause: FunctionCompletionVerificationErrorV1,
    },
    Pairing {
        key: CanonicalSameModuleCallableKeyV1,
        reason: &'static str,
    },
    PlanKeyCoverageMismatch {
        expected: usize,
        actual: usize,
    },
}

impl GeneralFunctionPlanErrorV1 {
    pub(crate) const fn stage(&self) -> GeneralFunctionPlanStageV1 {
        match self {
            Self::NoInstanceMethod => GeneralFunctionPlanStageV1::Inventory,
            Self::Source { .. } | Self::UnsupportedSignature { .. } => {
                GeneralFunctionPlanStageV1::Source
            }
            Self::ResolverSession(_)
            | Self::Resolver { .. }
            | Self::Projection { .. }
            | Self::Input { .. }
            | Self::FactCoverage { .. } => GeneralFunctionPlanStageV1::Resolve,
            Self::UnsupportedBody { .. } => GeneralFunctionPlanStageV1::Recipe,
            Self::Completion { .. } => GeneralFunctionPlanStageV1::Completion,
            Self::Pairing { .. } | Self::PlanKeyCoverageMismatch { .. } => {
                GeneralFunctionPlanStageV1::Pairing
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct RejectedGeneralFunctionPlanSetV1 {
    owner: VerifiedNormalModuleSourceV1,
    error: GeneralFunctionPlanErrorV1,
}

impl RejectedGeneralFunctionPlanSetV1 {
    pub(crate) fn source_identity(&self) -> &str {
        self.owner.source_identity()
    }

    pub(crate) const fn stage(&self) -> GeneralFunctionPlanStageV1 {
        self.error.stage()
    }

    pub(crate) const fn error(&self) -> &GeneralFunctionPlanErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

impl VerifiedNormalModuleSourceV1 {
    pub(crate) fn seal_instance_function_plans(
        self,
    ) -> Result<VerifiedNormalInstanceFunctionPlanSetV1, RejectedGeneralFunctionPlanSetV1> {
        let keys = self
            .callable_catalog()
            .keys()
            .filter(|key| key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod)
            .cloned()
            .collect::<Vec<_>>();
        if keys.is_empty() {
            return Err(reject(self, GeneralFunctionPlanErrorV1::NoInstanceMethod));
        }

        let mut resolver = match FunctionSemanticResolverSessionV1::new(0) {
            Ok(resolver) => resolver,
            Err(error) => {
                return Err(reject(
                    self,
                    GeneralFunctionPlanErrorV1::ResolverSession(error),
                ))
            }
        };
        let mut plans = BTreeMap::new();
        for key in &keys {
            let view = match self.borrow_instance_method_source(key) {
                Ok(view) => view,
                Err(cause) => {
                    return Err(reject(
                        self,
                        GeneralFunctionPlanErrorV1::Source {
                            key: key.clone(),
                            cause,
                        },
                    ))
                }
            };
            let plan = match seal_integer_literal_return_one(&mut resolver, view) {
                Ok(plan) => VerifiedNormalInstanceFunctionPlanV1::IntegerLiteralReturn(plan),
                Err(error) => return Err(reject(self, error)),
            };
            if plans.insert(key.clone(), plan).is_some() {
                return Err(reject(
                    self,
                    GeneralFunctionPlanErrorV1::Pairing {
                        key: key.clone(),
                        reason: "duplicate_plan_key",
                    },
                ));
            }
        }

        if !plans.keys().eq(keys.iter()) {
            let actual = plans.len();
            return Err(reject(
                self,
                GeneralFunctionPlanErrorV1::PlanKeyCoverageMismatch {
                    expected: keys.len(),
                    actual,
                },
            ));
        }

        Ok(VerifiedNormalInstanceFunctionPlanSetV1 {
            source: self,
            plans,
        })
    }
}

fn reject(
    owner: VerifiedNormalModuleSourceV1,
    error: GeneralFunctionPlanErrorV1,
) -> RejectedGeneralFunctionPlanSetV1 {
    RejectedGeneralFunctionPlanSetV1 { owner, error }
}
