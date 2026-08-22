use std::collections::BTreeMap;

use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::core_method_result_kind::{
    lookup_core_method_result_row_v2, CoreMethodResultKindV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::{
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedSourceStaticCallTargetV1,
};
use crate::mir::source_core_receiver::SourceCoreReceiverFactV1;

use super::call_row::VerifiedCallableResultCallSiteV1;
use super::call_substitution::{exact_requirements, substitute_required_arguments};
use super::expression_proof::I64ExpressionFactV1;
use super::{
    CallableResultCatalogErrorV1, CallableResultUnavailableReasonV1,
    VerifiedCallableResultDispositionV1, VerifiedCallableResultRepresentationV1,
};

/// Read-only composition context for one caller's exact source call sites.
pub(super) struct CallProofContextV1<'target, 'catalog, 'rows> {
    caller: &'catalog CanonicalSameModuleCallableKeyV1,
    targets: &'target VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    result_rows:
        &'rows BTreeMap<CanonicalSameModuleCallableKeyV1, VerifiedCallableResultDispositionV1>,
}

impl<'target, 'catalog, 'rows> CallProofContextV1<'target, 'catalog, 'rows> {
    pub(super) const fn new(
        caller: &'catalog CanonicalSameModuleCallableKeyV1,
        targets: &'target VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
        result_rows: &'rows BTreeMap<
            CanonicalSameModuleCallableKeyV1,
            VerifiedCallableResultDispositionV1,
        >,
    ) -> Self {
        Self {
            caller,
            targets,
            result_rows,
        }
    }

    pub(super) fn prove_method_call(
        &self,
        site: SourceExprSiteV1,
        method: &str,
        arguments: &[I64ExpressionFactV1],
        receiver_fact: Option<SourceCoreReceiverFactV1>,
    ) -> Result<CallProofOutcomeV1<'target>, CallableResultCatalogErrorV1> {
        if let Some(source_target) = self.source_target(&site) {
            let target = source_target.target();
            let Some(disposition) = self.result_rows.get(target) else {
                return Ok(CallProofOutcomeV1 {
                    fact: I64ExpressionFactV1::PendingDependency,
                    row: None,
                });
            };
            return Ok(match disposition {
                VerifiedCallableResultDispositionV1::ExactI64 {
                    required_i64_arguments,
                } => {
                    let fact = substitute_required_arguments(required_i64_arguments, arguments);
                    let row = exact_requirements(&fact).map(|requirements| {
                        VerifiedCallableResultCallSiteV1::same_module_static(
                            source_target,
                            VerifiedCallableResultRepresentationV1::ExactI64,
                            required_i64_arguments.clone(),
                            requirements
                                .iter()
                                .copied()
                                .collect::<Vec<_>>()
                                .into_boxed_slice(),
                        )
                    });
                    CallProofOutcomeV1 { fact, row }
                }
                VerifiedCallableResultDispositionV1::ExactNominalBox { box_name } => {
                    CallProofOutcomeV1 {
                        fact: I64ExpressionFactV1::ExactNominalBox(box_name.clone()),
                        row: Some(VerifiedCallableResultCallSiteV1::same_module_static(
                            source_target,
                            VerifiedCallableResultRepresentationV1::ExactNominalBox {
                                box_name: box_name.clone(),
                            },
                            Box::new([]),
                            Box::new([]),
                        )),
                    }
                }
                VerifiedCallableResultDispositionV1::Unavailable(reason) => {
                    let fact = if *reason == CallableResultUnavailableReasonV1::RecursiveDependency
                    {
                        I64ExpressionFactV1::Unknown(
                            CallableResultUnavailableReasonV1::RecursiveDependency,
                        )
                    } else {
                        I64ExpressionFactV1::Unknown(
                            CallableResultUnavailableReasonV1::StaticCallResultUnavailable,
                        )
                    };
                    CallProofOutcomeV1 { fact, row: None }
                }
            });
        }

        self.prove_core_string_method(receiver_fact, method, arguments)
    }

    fn source_target(
        &self,
        site: &SourceExprSiteV1,
    ) -> Option<&'target VerifiedSourceStaticCallTargetV1> {
        self.targets.target(self.caller, site)
    }

    fn prove_core_string_method(
        &self,
        receiver_fact: Option<SourceCoreReceiverFactV1>,
        method: &str,
        arguments: &[I64ExpressionFactV1],
    ) -> Result<CallProofOutcomeV1<'target>, CallableResultCatalogErrorV1> {
        let Some(receiver_fact) = receiver_fact else {
            return Ok(CallProofOutcomeV1::unavailable_target());
        };
        let arity = u32::try_from(arguments.len()).map_err(|_| {
            CallableResultCatalogErrorV1::CallArityOverflow {
                caller: self.caller.clone(),
                arity: arguments.len(),
            }
        })?;
        let Some(contract) = lookup_core_method_result_row_v2("StringBox", method, arity) else {
            return Ok(CallProofOutcomeV1::unavailable_target());
        };
        Ok(match contract.result_kind {
            CoreMethodResultKindV1::I64Value => CallProofOutcomeV1 {
                fact: I64ExpressionFactV1::exact_empty(),
                row: Some(VerifiedCallableResultCallSiteV1::core_string_method(
                    receiver_fact,
                    contract,
                )),
            },
            CoreMethodResultKindV1::BoolValue
            | CoreMethodResultKindV1::StringValue
            | CoreMethodResultKindV1::NoValue => CallProofOutcomeV1 {
                fact: I64ExpressionFactV1::KnownNonI64,
                row: None,
            },
            CoreMethodResultKindV1::Dynamic => CallProofOutcomeV1 {
                fact: I64ExpressionFactV1::Unknown(
                    CallableResultUnavailableReasonV1::CoreMethodResultUnavailable,
                ),
                row: None,
            },
        })
    }
}

pub(super) struct CallProofOutcomeV1<'target> {
    pub(super) fact: I64ExpressionFactV1,
    pub(super) row: Option<VerifiedCallableResultCallSiteV1<'target>>,
}

impl CallProofOutcomeV1<'_> {
    fn unavailable_target() -> Self {
        Self {
            fact: I64ExpressionFactV1::Unknown(
                CallableResultUnavailableReasonV1::StaticCallTargetAuthorityUnavailable,
            ),
            row: None,
        }
    }
}
