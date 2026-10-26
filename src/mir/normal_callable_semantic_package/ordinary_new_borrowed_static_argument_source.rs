//! Source-purpose connection at the original actual observation. No Home rights.
use super::*;
use super::super::call_result_source::{
    OrdinaryArgumentCallSourceV1, ordinary_argument_call_source_v1, original_argument_call_source_v1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_value_profile::expression_plan::{
    call_argument_expression_source_plan_v1, ExpressionPlanRootV1, ExpressionSourcePlanV1,
};

#[derive(Debug, Clone)]
enum ArgumentMethodSourceV1 {
    Core(Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>),
    OrdinaryCall(OrdinaryArgumentCallSourceV1),
    DynamicArrayCarrierAppend(
        crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1,
    ),
}
impl PartialEq for ArgumentMethodSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Core(a), Self::Core(b)) => Rc::ptr_eq(a, b),
            (Self::OrdinaryCall(a), Self::OrdinaryCall(b)) => a == b,
            (Self::DynamicArrayCarrierAppend(a), Self::DynamicArrayCarrierAppend(b)) => a == b,
            _ => false,
        }
    }
}
impl Eq for ArgumentMethodSourceV1 {}
impl ArgumentMethodSourceV1 {
    fn core(&self) -> Option<&Rc<crate::mir::normal_callable_semantic_package::core_method_source::OrdinaryCoreSourceCallV1>>{
        match self {
            Self::Core(row) => Some(row),
            _ => None,
        }
    }
    fn dynamic_array_carrier_append(
        &self,
    ) -> Option<&crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1> {
        match self {
            Self::DynamicArrayCarrierAppend(source) => Some(source),
            _ => None,
        }
    }
}

/// One original source loan, never an evaluation, entry or ownership grant.
/// Core and ordinary calls retain their SAME original sources; Instance syntax
/// alone grants no Text class. The common evaluator requires the completed result.
#[derive(Debug, Clone)]
pub(crate) struct ArgumentExpressionSourceLoanV1 {
    plan: Rc<ExpressionSourcePlanV1>,
    i64_result: bool,
    calls: BTreeMap<usize, ArgumentMethodSourceV1>,
    dynamic_array_carrier_append:
        Option<crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1>,
}
impl PartialEq for ArgumentExpressionSourceLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.plan, &other.plan)
            && self.i64_result == other.i64_result
            && self.calls == other.calls
            && self.dynamic_array_carrier_append == other.dynamic_array_carrier_append
    }
}
impl Eq for ArgumentExpressionSourceLoanV1 {}
impl ArgumentExpressionSourceLoanV1 {
    pub(crate) fn requires_i64_v1(&self) -> bool {
        self.i64_result
    }
    pub(crate) fn plan(&self) -> &Rc<ExpressionSourcePlanV1> {
        &self.plan
    }
    pub(crate) fn dynamic_array_carrier_append_v1(
        &self,
    ) -> Option<&crate::mir::normal_callable_semantic_package::NamedArrayHolderSourceLoanV1> {
        self.dynamic_array_carrier_append.as_ref()
    }
    pub(crate) fn dynamic_array_carrier_call_is_v1(&self, site: &SourceExprSiteV1) -> bool {
        self.calls.iter().any(|(node, source)| {
            source
                .dynamic_array_carrier_append()
                .is_some_and(|original| {
                    original.owner() == self.plan.owner()
                        && original.call_site() == site
                        && self
                            .plan
                            .nodes()
                            .get(*node)
                            .is_some_and(|node| node.site() == site)
                })
        })
    }
    pub(crate) fn dynamic_array_carrier_append_root_v1(&self) -> bool {
        let Some(holder) = self.dynamic_array_carrier_append.as_ref() else {
            return false;
        };
        let Some(root) = self.plan.nodes().first() else {
            return false;
        };
        let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
            call,
            ordinal,
            ..
        } = self.plan.root()
        else {
            return false;
        };
        *ordinal < call.arity()
            && holder.binding().owner() == self.plan.owner()
            && holder
                .append_candidate(&OwnedExprSiteV1::new(
                    self.plan.owner(),
                    call.site().clone(),
                ))
                .is_some_and(|candidate| candidate.site() == root.site())
    }
    pub(crate) fn ordinary_call_is(&self, site: &SourceExprSiteV1) -> bool {
        self.calls.iter().any(|(node, source)| matches!(source, ArgumentMethodSourceV1::OrdinaryCall(original)
            if original.call_site().owner() == self.plan.owner() && original.call_site().site() == site
                && self.plan.nodes().get(*node).is_some_and(|node| node.site() == site)))
    }
    pub(crate) fn signed_operand_contains(&self, site: &SourceExprSiteV1) -> bool {
        if !self.plan.nodes().iter().any(|node| node.site() == site) {
            return false;
        }
        let site_path = site.node().segments();
        let result_input = self.i64_result
            && self
                .plan
                .nodes_outside_calls()
                .any(|(_, node)| node.site() == site);
        let core_input = self
            .calls
            .values()
            .filter_map(ArgumentMethodSourceV1::core)
            .any(|receipt| {
                let core = receipt.source();
                let call_site = core.source().site();
                core.source()
                    .arguments()
                    .iter()
                    .zip(core.target().parameters())
                    .any(|(argument, parameter)| {
                        if *parameter
                            != crate::mir::resolved_semantics::CoreMethodHomeParameterRelationV1::I64Parameter
                        {
                            return false;
                        }
                        let operand_path = argument.site().node().segments();
                        site_path.starts_with(operand_path)
                            && !self.plan.nodes().iter().any(|node| {
                                matches!(
                                    node.source(),
                                    crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::MethodCall
                                        | crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::DirectCall
                                ) && node.site() != call_site
                                    && node
                                        .site()
                                        .node()
                                        .segments()
                                        .starts_with(operand_path)
                                    && site_path
                                        .starts_with(node.site().node().segments())
                            })
                    })
            });
        result_input || core_input
    }
    pub(crate) fn dynamic_operand_contains(&self, site: &SourceExprSiteV1) -> bool {
        self.calls
            .values()
            .filter_map(ArgumentMethodSourceV1::core)
            .any(|receipt| receipt.operands().dynamic_i64_operand_contains(site))
    }
    pub(crate) fn text_operand_is(&self, site: &SourceExprSiteV1) -> bool {
        self.calls.values().filter_map(ArgumentMethodSourceV1::core).any(|receipt| {
            let core = receipt.source(); site == core.source().receiver_site()
                || core.source().arguments().iter().zip(core.target().parameters()).any(|(arg, kind)|
                    *kind == crate::mir::resolved_semantics::CoreMethodHomeParameterRelationV1::TextParameter && arg.site() == site)
        }) || !self.i64_result && self.resolver_source().is_none() && self.plan.nodes_outside_calls().any(|(_, node)| node.site() == site
            && matches!(node.source(), crate::mir::resolved_value_profile::expression_plan::ExpressionNodeSourceV1::Reference(_)))
    }
    pub(crate) fn resolver_source(
        &self,
    ) -> Option<&crate::mir::resolved_semantics::VerifiedResolverOrdinaryCoreMethodSourceV1> {
        self.calls
            .get(&0)
            .and_then(ArgumentMethodSourceV1::core)
            .map(|core| core.source())
    }
}

/// Invocation-local borrowed view of the SAME original ordinal product. The
/// optional catalog is unnecessary for Static (its source owns all formals),
/// and mandatory for Instance; absence never invents an empty formal contract.
struct ArgumentSourceIdentityRefV1<'a> {
    source: super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1<'a>,
    candidates: &'a [BorrowedCallActualCandidateV1],
    slots: &'a BTreeMap<u32, ArgumentExpressionV1>,
    formals: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractV1],
}
fn argument_source_identity_v1<'a>(
    prepared: &'a PreparedBorrowedFormalIngressV1,
    contracts: Option<&'a [OwnedCallableParameterContractDeclarationV1]>,
    rows: &'a PreparedBorrowedCallActualsV1,
    site: &OwnedExprSiteV1,
) -> Result<Option<ArgumentSourceIdentityRefV1<'a>>, String> {
    use super::super::super::borrowed_formal_uses::BorrowedCallSourceLoanV1 as Loan;
    let rows = rows.source_static_snapshot_v1().unwrap_or(rows);
    match &rows.phase {
        BorrowedCallActualEvidencePhaseV1::SourceStatic(identity) => {
            let identity = validate_static_source_actuals_v1(
                prepared,
                rows,
                site,
                identity.source.required_i64_arguments(),
            )?;
            Ok(Some(ArgumentSourceIdentityRefV1 {
                source: Loan::Static(&identity.source),
                candidates: &identity.candidates,
                slots: &identity.argument_sources,
                formals: identity.source.parameters(),
            }))
        }
        BorrowedCallActualEvidencePhaseV1::SourceInstance(identity)
        | BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(identity) => {
            let reject = || freeze("borrowed-instance/argument-source-identity");
            let contracts = contracts.ok_or_else(reject)?;
            let original = crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1::original_instance_input_call_v1(prepared, contracts, site)?.ok_or_else(reject)?;
            if original.source.require_instance()? != &identity.source
                || original.arguments != identity.incoming_arguments
            {
                return Err(reject());
            }
            let mut matches = contracts
                .iter()
                .filter(|row| row.owner == identity.source.callee_owner());
            let contract = matches.next().ok_or_else(reject)?;
            if matches.next().is_some()
                || contract.batch_slot != identity.source.target_batch_slot()
                || contract.mode != Loan::Instance(&identity.source).declaration_mode()
                || contract.parameters.len() != identity.candidates.len()
                || contract
                    .parameters
                    .iter()
                    .enumerate()
                    .any(|(index, formal)| {
                        formal.ordinal as usize != index
                            || formal.binding.owner() != contract.owner
                            || identity.source.argument_sites().get(index)
                                != identity
                                    .candidates
                                    .get(index)
                                    .map(|candidate| &candidate.site)
                    })
            {
                return Err(reject());
            }
            let mut rebuilt = instance_source::prepare_instance_source_actuals_v1(
                prepared,
                contracts,
                site,
                &identity.candidates,
            )?
            .ok_or_else(reject)?;
            model::retain_instance_argument_sources_v1(&mut rebuilt, identity)?;
            if rebuilt.ordered_arguments != rows.ordered_arguments {
                return Err(reject());
            }
            match &rows.phase {
                BorrowedCallActualEvidencePhaseV1::SourceInstance(_) if &rebuilt != rows => {
                    return Err(reject())
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableSourceInstance(_) => {
                    rows.checked_ordered_arguments_v1(&original)?;
                }
                _ => {}
            }
            Ok(Some(ArgumentSourceIdentityRefV1 {
                source: Loan::Instance(&identity.source),
                candidates: &identity.candidates,
                slots: &identity.argument_sources,
                formals: &contract.parameters,
            }))
        }
        BorrowedCallActualEvidencePhaseV1::SourceObject(identity)
        | BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(identity) => {
            let reject = || freeze("borrowed-object/source-actual-identity");
            let contracts = contracts.ok_or_else(reject)?;
            let mut originals = prepared
                .source_incoming
                .exact_rows()
                .filter(|row| &row.call == site);
            let original = originals.next().ok_or_else(reject)?;
            if originals.next().is_some()
                || original.source.require_instance()? != &identity.source
                || original.arguments != identity.incoming_arguments
            {
                return Err(reject());
            }
            let mut matches = contracts
                .iter()
                .filter(|row| row.owner == identity.source.callee_owner());
            let contract = matches.next().ok_or_else(reject)?;
            if matches.next().is_some()
                || contract.batch_slot != identity.source.target_batch_slot()
                || contract.mode != Loan::Instance(&identity.source).declaration_mode()
                || contract.parameters.len() != identity.candidates.len()
                || contract
                    .parameters
                    .iter()
                    .enumerate()
                    .any(|(index, formal)| {
                        formal.ordinal as usize != index
                            || formal.binding.owner() != contract.owner
                            || identity.source.argument_sites().get(index)
                                != identity
                                    .candidates
                                    .get(index)
                                    .map(|candidate| &candidate.site)
                    })
            {
                return Err(reject());
            }
            let mut rebuilt = object_source::prepare_object_source_actuals_v1(
                prepared,
                contracts,
                site,
                &identity.candidates,
            )?
            .ok_or_else(reject)?;
            model::retain_object_argument_sources_v1(&mut rebuilt, identity)?;
            if rebuilt.ordered_arguments != rows.ordered_arguments {
                return Err(reject());
            }
            match &rows.phase {
                BorrowedCallActualEvidencePhaseV1::SourceObject(_) if &rebuilt != rows => {
                    return Err(reject())
                }
                BorrowedCallActualEvidencePhaseV1::ExecutableSourceObject(_) => {
                    rows.checked_ordered_arguments_v1(original)?;
                }
                _ => {}
            }
            Ok(Some(ArgumentSourceIdentityRefV1 {
                source: Loan::Instance(&identity.source),
                candidates: &identity.candidates,
                slots: &identity.argument_sources,
                formals: &contract.parameters,
            }))
        }
        _ => Ok(None),
    }
}

/// Shared by the passive projection and source lender. In particular this
/// checks the complete original tuple, not just the requested ordinal.
impl OrdinaryNewClaimLedgerV1 {
    /// Retain the canonical source frame for a computed borrowed actual. This
    /// source loan does not alter its passive phase or close the callee cohort.
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_expression_actual_source_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> Result<PreparedBorrowedFormalActualV1, String> {
        let frame = self.qualified_argument_expression_frame_v1(call, ordinal, site)?;
        let rows = self
            .borrowed_formal_actuals
            .get(call)
            .ok_or_else(|| freeze("argument-expression/observation-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let contracts = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("argument-expression/source-contracts"))?;
        let identity = argument_source_identity_v1(
            self.borrowed_formal_source
                .as_ref()
                .ok_or_else(|| freeze("argument-expression/source-missing"))?
                .as_ref()
                .map_err(Clone::clone)?,
            Some(contracts),
            rows,
            call,
        )?
        .ok_or_else(|| freeze("argument-expression/source-phase"))?;
        let formal = identity
            .formals
            .get(ordinal as usize)
            .filter(|formal| {
                formal.ordinal == ordinal
                    && matches!(formal.kind, CallableParameterContractKindV1::OpaqueHandle)
            })
            .ok_or_else(|| freeze("borrowed-expression/source-formal"))?;
        Ok(PreparedBorrowedFormalActualV1 {
            ordinal,
            site: site.clone(),
            formal: formal.binding,
            source: BorrowedFormalActualSourceV1::retain_expression_frame_v1(frame)?,
        })
    }

    pub(in crate::mir) fn argument_expression_frame_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> Result<
        Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
        String,
    > {
        let frame = self.qualified_argument_expression_frame_v1(call, ordinal, site)?;
        if !frame.is_i64() {
            return Err(freeze("argument-expression/home-membership"));
        }
        Ok(frame)
    }

    fn root_flow_argument_expression_frame_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> Result<
        Option<
            Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
        >,
        String,
    > {
        let Some(flow) = self
            .completion_for_owner(call.owner())
            .and_then(|completion| completion.cleanup().root_flow())
        else {
            return Ok(None);
        };
        let mut matches = flow
            .expression_plans()
            .iter()
            .filter_map(|plan| plan.argument_frame())
            .filter(|frame| {
                frame.source().owner() == call.owner()
                    && Rc::ptr_eq(frame.source_loan().plan(), frame.source())
                    && frame.evaluation().reference_coverage_is_exact()
                    && matches!(
                        frame.source().root(),
                        ExpressionPlanRootV1::CallArgument {
                            call: original,
                            ordinal: original_ordinal,
                            ..
                        } if original.owner() == call.owner()
                            && original.site() == call.site()
                            && *original_ordinal == ordinal
                            && original.arguments().get(ordinal as usize).is_some_and(|argument|
                                argument.ordinal() == ordinal && argument.site() == site)
                            && frame.source().nodes().first().is_some_and(|root| root.site() == site)
                    )
            });
        let Some(frame) = matches.next() else {
            return Ok(None);
        };
        if matches.next().is_some() {
            return Err(freeze("argument-expression/root-flow-ambiguous"));
        }
        Ok(Some(Rc::clone(frame)))
    }

    /// Borrow original qualification only. This issues no executable actual,
    /// entry, ABI or publication permission; those keep their original owners.
    pub(in crate::mir) fn qualified_argument_expression_frame_v1(
        &self,
        call: &OwnedExprSiteV1,
        ordinal: u32,
        site: &SourceExprSiteV1,
    ) -> Result<
        Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedArgumentExpressionFrameV1>,
        String,
    > {
        let Some(rows) = self.borrowed_formal_actuals.get(call) else {
            let root = self.root_flow_argument_expression_frame_v1(call, ordinal, site)?;
            let loop_frame = self.source_loop_argument_expression_frame_v2(call, ordinal, site)?;
            let frame = match (root, loop_frame) {
                (Some(root), Some(loop_frame)) if Rc::ptr_eq(&root, &loop_frame) => Some(root),
                (Some(_), Some(_)) => {
                    return Err(freeze("argument-expression/root-flow-ambiguous"))
                }
                (Some(frame), None) | (None, Some(frame)) => Some(frame),
                (None, None) => None,
            };
            return frame.ok_or_else(|| freeze("argument-expression/observation-missing"));
        };
        let prepared = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("argument-expression/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        let rows = rows.as_ref().map_err(Clone::clone)?;
        if rows.dynamic_invocation_arguments.is_some() {
            let original = dynamic_invocation_argument_source_v1(rows, call, ordinal)?
                .ok_or_else(|| freeze("dynamic-argument/source-missing"))?;
            let dynamic = rows
                .dynamic_invocation_arguments
                .as_ref()
                .expect("checked Dynamic argument row");
            let Some(ArgumentExpressionV1::Qualified(frame)) = dynamic.arguments.get(&ordinal)
            else {
                return Err(freeze("dynamic-argument/frame-missing"));
            };
            if frame.source_loan() != &original
                || !Rc::ptr_eq(frame.source(), original.plan())
                || !frame.is_i64()
                || !frame.evaluation().reference_coverage_is_exact()
                || frame
                    .source()
                    .nodes()
                    .first()
                    .is_none_or(|node| node.site() != site)
                || dynamic.source.argument_sites().get(ordinal as usize) != Some(site)
            {
                return Err(freeze("dynamic-argument/frame-identity"));
            }
            return Ok(Rc::clone(frame));
        }
        let contracts = self
            .original_parameter_contracts
            .as_deref()
            .ok_or_else(|| freeze("argument-expression/source-contracts"))?;
        let identity = argument_source_identity_v1(prepared, Some(contracts), rows, call)?
            .ok_or_else(|| freeze("argument-expression/source-phase"))?;
        if let Some(snapshot) = rows.source_static_snapshot_v1() {
            let BorrowedCallActualEvidencePhaseV1::SourceStatic(original) = &snapshot.phase else {
                unreachable!("source snapshot")
            };
            validate_qualified_argument_sources_v1(
                prepared,
                &self.borrowed_formal_actuals,
                call,
                original,
            )?;
        }
        let Some(ArgumentExpressionV1::Qualified(frame)) = identity.slots.get(&ordinal) else {
            return Err(format!(
                "{}: call={call:?}; ordinal={ordinal}; argument={site:?}",
                freeze("argument-expression/frame-missing")
            ));
        };
        let source = lend_argument_expression_source_v1(
            prepared,
            &self.borrowed_formal_actuals,
            contracts,
            call,
            ordinal,
        )?
        .ok_or_else(|| freeze("argument-expression/source-missing"))?;
        if frame.source_loan() != &source
            || !Rc::ptr_eq(frame.source(), source.plan())
            || !frame.evaluation().reference_coverage_is_exact()
            || identity.source.argument_sites().get(ordinal as usize) != Some(site)
            || !self
                .completion_for_owner(call.owner())
                .and_then(|completion| completion.cleanup().root_flow())
                .is_some_and(|flow| {
                    flow.expression_plans()
                        .iter()
                        .filter_map(|plan| plan.argument_frame())
                        .any(|original| Rc::ptr_eq(original, frame))
                })
        {
            return Err(freeze("argument-expression/home-membership"));
        }
        Ok(Rc::clone(frame))
    }
}

#[path = "ordinary_new_argument_expression_lifecycle.rs"]
mod lifecycle;
#[path = "ordinary_new_argument_expression_lending.rs"]
mod lending;
pub(super) use lifecycle::corroborates_argument_source_v1;
pub(in crate::mir::normal_callable_semantic_package) use lifecycle::bind_argument_sources_v1;
pub(in crate::mir::normal_callable_semantic_package) use lifecycle::stage_observed_call_actuals_v1;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use lifecycle::validate_static_source_actuals_v1;
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) use lifecycle::validate_qualified_argument_sources_v1;
pub(in crate::mir::normal_callable_semantic_package) use lifecycle::complete_argument_expression_v1;
pub(in crate::mir::normal_callable_semantic_package) use lending::lend_argument_expression_source_v1;
pub(in crate::mir::normal_callable_semantic_package) use lending::lend_array_carrier_append_argument_source_v1;
pub(in crate::mir::normal_callable_semantic_package) use lending::observed_argument_source_v1;
pub(in crate::mir::normal_callable_semantic_package) use lending::observed_carrier_array_append_argument_source_v1;
pub(in crate::mir::normal_callable_semantic_package) use lending::lend_text_entry_read_v1;
pub(in crate::mir::normal_callable_semantic_package) use lending::lend_field_array_text_entry_read_v1;
pub(in crate::mir::normal_callable_semantic_package) use lending::lend_expression_core_source_v1;
use lending::{lend_argument_source_inner_v1, dynamic_invocation_argument_source_v1};
