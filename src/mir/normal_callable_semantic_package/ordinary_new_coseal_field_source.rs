//! Borrow original field sources through the one Home consult.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call;
use crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1;
use std::collections::{BTreeMap, BTreeSet};

/// Select the Home walk from original non-Birth writer syntax. Selection is
/// not scalar permission: the walk still owes receiver, RHS and field proofs.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn requires_field_scalar_walk_v1(
    writes: &super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let mut required = false;
    for write in writes
        .writes()
        .iter()
        .filter(|write| write.owner() == owner && !write.in_birth())
    {
        let source = write
            .source()
            .map_err(|_| OrdinaryNewCoSealIssueV1::SourceNavigation {
                site: OwnedExprSiteV1::new(owner, write.assignment().target_site().clone()),
            })?;
        required |= matches!(source.root(), crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment { assignment }
            if assignment.form() == crate::mir::resolved_semantics::ResolvedAssignmentFormV1::Plain);
    }
    Ok(required)
}
/// Same original writer inventory as the borrowed receiver/value source uses.
/// This consult lends syntax; Home qualification still proves each use.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct FieldSourcesWithOriginalWritesV1<'a> {
    caller: Option<&'a CanonicalSameModuleCallableKeyV1>,
    parameter_contracts: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &'a crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    entry: Option<&'a crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    results: &'a BorrowedResultCohortV1,
    ingress: &'a Result<lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    candidates: &'a [OrdinaryNewCandidate],
    result_constructions: &'a [OrdinaryNewSiteResolutionV1],
    writes: Option<&'a super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1>,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
    read_context: field_reads::ReceivedFieldContextV1<'a>,
    core_calls: Option<&'a crate::mir::normal_callable_semantic_package::core_method_source::SourceCoreMethodCallRowsV1>,
    array_census: &'a BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    staged_arguments: Option<&'a mut BTreeMap<OwnedExprSiteV1, field_reads::ArgumentFieldRead>>,
    staged_locals: Option<&'a mut BTreeMap<OwnedExprSiteV1, field_reads::LocalFieldRead>>,
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn field_sources_with_original_writes<
    'a,
>(
    caller: Option<&'a CanonicalSameModuleCallableKeyV1>,
    parameter_contracts: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(
        BindingRefV1,
        &'a crate::parser::ParserOrdinaryBoxSourceRowV1,
    )>,
    entry: Option<&'a crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    results: &'a BorrowedResultCohortV1,
    ingress: &'a Result<lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    candidates: &'a [OrdinaryNewCandidate],
    result_constructions: &'a [OrdinaryNewSiteResolutionV1],
    writes: Option<&'a super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1>,
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'a>,
    read_context: field_reads::ReceivedFieldContextV1<'a>,
    core_calls: Option<&'a crate::mir::normal_callable_semantic_package::core_method_source::SourceCoreMethodCallRowsV1>,
    array_census: &'a BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    staged_arguments: Option<&'a mut BTreeMap<OwnedExprSiteV1, field_reads::ArgumentFieldRead>>,
    staged_locals: Option<&'a mut BTreeMap<OwnedExprSiteV1, field_reads::LocalFieldRead>>,
) -> FieldSourcesWithOriginalWritesV1<'a> {
    FieldSourcesWithOriginalWritesV1 {
        caller,
        parameter_contracts,
        constructors,
        receiver,
        entry,
        results,
        ingress,
        candidates,
        result_constructions,
        writes,
        input,
        read_context,
        core_calls,
        array_census,
        staged_arguments,
        staged_locals,
    }
}

impl
    crate::mir::resolved_semantics::home_new_prefix::HomeFieldSourceConsultV1<
        OrdinaryNewCoSealIssueV1,
    > for FieldSourcesWithOriginalWritesV1<'_>
{
    fn array_text_append_source(
        &mut self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceExprSiteV1,
    ) -> Result<
        Option<Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>>,
        OrdinaryNewCoSealIssueV1,
    > {
        let Some(writes) = self.writes else {
            return Ok(None);
        };
        let (Some(caller), Some(receiver)) = (self.caller, self.receiver) else {
            return entry_write_declined(input, site, 100);
        };
        let Some(call) = input.function().method_call(site) else {
            return entry_write_declined(input, site, 101);
        };
        let Some(argument) = call.arguments().first() else {
            return entry_write_declined(input, site, 102);
        };
        let text = matches!(
            input
                .function()
                .expression_source()
                .literal(argument.site()),
            Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::String(_))
        ) || matches!(input.function().variable_ref(argument.site()), Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding))
                if self.ingress.as_ref().is_ok_and(|ingress| ingress.source_value_sum_v1(binding).is_some_and(|sum| sum.text_only())
                    || ingress.lend_read_sum_v1(argument.site(), binding).is_some_and(|sum| sum.text_only())));
        if !text {
            return entry_write_declined(input, site, 103);
        }
        crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_array_read_source::capture_text_append(input, caller, receiver.1, site, self.constructors, writes)
            .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site: OwnedExprSiteV1::new(input.owner(),site.clone()), issue })
    }
    fn construction_source(
        &mut self,
        site: &OwnedExprSiteV1,
        destination: BindingRefV1,
    ) -> Result<
        Option<Rc<crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1>>,
        OrdinaryNewCoSealIssueV1,
    > {
        if self.writes.is_none() {
            return Ok(None);
        }
        let mut rows = self
            .candidates
            .iter()
            .filter(|row| &row.site == site && row.destination == destination);
        let Some(row) = rows.next() else {
            return Ok(None);
        };
        if rows.next().is_some() {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() });
        }
        Ok(row.construction_source.as_ref().ok().map(Rc::clone))
    }
    fn shared_payload_construction_source(
        &mut self,
        site: &OwnedExprSiteV1,
    ) -> Result<
        Option<Rc<crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1>>,
        OrdinaryNewCoSealIssueV1,
    > {
        if self.writes.is_none() {
            return Ok(None);
        }
        let mut rows = self
            .result_constructions
            .iter()
            .filter(|row| &row.site == site);
        let Some(row) = rows.next() else {
            return Ok(None);
        };
        if rows.next().is_some() {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() });
        }
        Ok(row.construction_source.as_ref().ok().map(Rc::clone))
    }
    fn argument_i64_field(
        &mut self,
        site: &OwnedExprSiteV1,
        receiver_site: &SourceExprSiteV1,
        receiver: BindingRefV1,
        home: BindingRefV1,
        field: &str,
    ) -> Result<bool, OrdinaryNewCoSealIssueV1> {
        if self
            .staged_arguments
            .as_ref()
            .is_some_and(|rows| rows.contains_key(site))
        {
            return Ok(true);
        }
        let Some(field) = terminal_home::argument_integer_field(
            self.constructors,
            self.candidates,
            site,
            self.receiver,
            home,
            field,
        )?
        else {
            return Ok(false);
        };
        if let Some(rows) = self.staged_arguments.as_mut() {
            rows.insert(site.clone(), field_reads::ArgumentFieldRead {
                receiver_site: receiver_site.clone(), object: receiver,
                source: LocalFieldReadSourceV1::from_declaration(field,
                    crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar,
                    Some(crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1::I64.source_type_name().into())),
                progress: field_reads::Progress::Pending,
            });
        }
        Ok(true)
    }
    fn container_field(
        &mut self,
        site: &OwnedExprSiteV1,
        _: &SourceExprSiteV1,
        _: BindingRefV1,
        home: BindingRefV1,
        field: &str,
    ) -> Result<bool, OrdinaryNewCoSealIssueV1> {
        terminal_home::receiver_container_field(self.constructors, self.receiver, site, home, field)
            .map(|field| field.is_some())
    }
    fn array_i64_field(
        &mut self,
        site: &OwnedExprSiteV1,
        _: &SourceExprSiteV1,
        _: BindingRefV1,
        home: BindingRefV1,
        field: &str,
    ) -> Result<bool, OrdinaryNewCoSealIssueV1> {
        terminal_home::receiver_array_i64_field_from_census_v1(
            self.constructors,
            self.receiver,
            self.array_census,
            site,
            home,
            field,
        )
    }
    fn formal_i64_field(
        &mut self,
        site: &OwnedExprSiteV1,
        _: &SourceExprSiteV1,
        _: BindingRefV1,
        home: BindingRefV1,
        field: &str,
    ) -> Result<bool, OrdinaryNewCoSealIssueV1> {
        let Ok(ledger) = self.input.forest().callable_source_ledger(site.owner()) else {
            return Ok(false);
        };
        Ok(array_i64_fields::formal_i64_index_field(
            &ledger,
            self.constructors,
            self.read_context.0.ordinary_box_coverage(),
            home,
            field,
        ))
    }
    fn local_field_read(
        &mut self,
        requests: &[crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1],
        demand: crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1,
    ) -> Result<
        Option<Vec<crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1>>,
        OrdinaryNewCoSealIssueV1,
    > {
        use crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadDemandV1;
        let probe = matches!(
            &demand,
            LocalFieldReadDemandV1::SourceExpression { probe: true, .. }
        );
        let (batch, selected, classes, _, _) = self.read_context;
        let Some(rows) = prove_local_field_read_batch(
            self.constructors,
            self.candidates,
            self.result_constructions,
            batch.ordinary_box_coverage(),
            self.receiver,
            self.entry,
            self.core_calls,
            requests,
            demand,
            self.results,
            self.ingress,
            &mut |home| {
                super::super::lexical::nullable_received_result_class(
                    selected,
                    batch,
                    classes,
                    self.candidates,
                    self.input,
                    self.receiver,
                    home,
                )
            },
            &mut |home| source_formal_class_v1(self.ingress, home),
            Some(self.read_context),
        )?
        else {
            return Ok(None);
        };
        if !probe {
            if let Some(staged) = self.staged_locals.as_mut() {
                return stage_local_field_read_batch(staged, rows).map(Some);
            }
        }
        Ok(Some(
            rows.into_iter()
                .map(|(_, row)| row.source.result().clone())
                .collect(),
        ))
    }
    fn scalar_field(
        &mut self,
        site: &OwnedExprSiteV1,
        _receiver_site: &SourceExprSiteV1,
        _receiver: BindingRefV1,
        home: BindingRefV1,
        field: &str,
    ) -> Result<bool, OrdinaryNewCoSealIssueV1> {
        terminal_home::receiver_scalar_field(self.constructors, self.receiver, site, home, field)
            .map(|field| field.is_some())
    }
    fn owned_local_write(
        &mut self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        acquisition: &OwnedExprSiteV1,
        root: BindingRefV1,
        construction: &Rc<
            crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1,
        >,
        target: &SourceExprSiteV1,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OwnedLocalFieldWriteSourceV1>,
        OrdinaryNewCoSealIssueV1,
    > {
        if self.writes.is_none() {
            return Ok(None);
        }
        let mut candidates = self
            .candidates
            .iter()
            .filter(|row| &row.site == acquisition && row.destination == root);
        let Some(candidate) = candidates.next() else {
            return Ok(None);
        };
        if candidates.next().is_some() {
            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite {
                site: acquisition.clone(),
            });
        }
        if !candidate
            .construction_source
            .as_ref()
            .is_ok_and(|source| Rc::ptr_eq(source, construction))
            || candidate.object != construction.object()
        {
            return Ok(None);
        }
        let Some(original) = self.original_writer(input.owner(), target)? else {
            return Ok(None);
        };
        Ok(original.owned_local_v1(input, acquisition, root, Rc::clone(construction)))
    }
    fn entry_write(
        &mut self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        target: &SourceExprSiteV1,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::EntryFieldWriteSourceV1>,
        OrdinaryNewCoSealIssueV1,
    > {
        let Some(writes) = self.writes else {
            return Ok(None);
        };
        let (Some(receiver), Some(entry)) = (self.receiver, self.entry) else {
            return entry_write_declined(input, target, 0);
        };
        let Some(original) = self.original_writer(input.owner(), target)? else {
            return entry_write_declined(input, target, 1);
        };
        if original.write().in_birth() {
            return entry_write_declined(input, target, 2);
        }
        let site = OwnedExprSiteV1::new(input.owner(), target.clone());
        let Some((field, declared)) = terminal_home::source_declared_field(
            self.constructors,
            receiver.1,
            &site,
            original.write().field(),
        )?
        else {
            return entry_write_declined(input, target, 3);
        };
        if !matches!(declared.as_deref(), None | Some("i64")) {
            return entry_write_declined(input, target, 4);
        }
        let Ok(ingress) = self.ingress else {
            return entry_write_declined(input, target, 5);
        };
        // A neutral source-only write is not an executable entry frame. Require
        // the complete value basis even for a declared slot before selecting it.
        use super::super::super::field_write_claim::{
            EntryFieldWriteValueSourceV1 as Value, TextFieldValueSourceV1,
        };
        // Select Text from the SAME original all-writer SUM, before the numeric
        // demand. A numeric refusal is never reinterpreted as a Text fallback.
        let text = original
            .write()
            .owner_box()
            .and_then(|class| writes.value_source(class, original.write().field()))
            .and_then(|source| source.as_ref().ok())
            .and_then(|source| TextFieldValueSourceV1::lend(ingress, writes, source))
            .filter(|source| declared.is_none() && source.field() == field);
        let value = if let Some(text) = text {
            Value::Text(text)
        } else {
            match lexical_instance_call::ClosedNumericFieldSourceLoanV1::from_declaration(
                self.results,
                ingress,
                field,
                declared.as_deref(),
                original.write().field(),
            ) {
                Ok(value) => Value::Numeric(value),
                Err(issue) => {
                    if crate::config::env::builder_debug_enabled() {
                        let retained = original
                            .write()
                            .owner_box()
                            .and_then(|class| writes.value_source(class, original.write().field()))
                            .map(|source| source.as_ref().err());
                        eprintln!(
                            "[field-source] site={site:?} field={} issue={issue} retained={retained:?}",
                            original.write().field()
                        );
                    }
                    return entry_write_declined(input, target, 6);
                }
            }
        };
        let root = terminal_home::ExpressionFieldRootSourceV1::issue_entry_v1(
            self.constructors,
            receiver,
            entry,
            input.owner(),
            receiver.0,
            field,
        )
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site, issue })?;
        Ok(
            crate::mir::normal_callable_semantic_package::EntryFieldWriteSourceV1::issue(
                input, original, root, field, declared, value,
            ),
        )
    }
    fn requires_initializer_use_frame(
        &mut self,
        site: &OwnedExprSiteV1,
    ) -> Result<bool, OrdinaryNewCoSealIssueV1> {
        if self.writes.is_none() {
            return Ok(false);
        }
        let source = self.ingress.as_ref().map_err(|issue| {
            OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
                site: site.clone(),
                issue: issue.clone(),
            }
        })?;
        OrdinaryNewClaimLedgerV1::original_finite_input_use_request_v1(
            source,
            self.parameter_contracts,
            site.owner(),
        )
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress {
            site: site.clone(),
            issue,
        })
    }
    fn scalar_read_sum(
        &mut self,
        read: &crate::mir::resolved_semantics::home_new_prefix::CurrentScalarReadV1,
    ) -> Option<Rc<crate::mir::normal_callable_semantic_package::SourceValueSumV1>> {
        if self.writes.is_none() {
            return None;
        }
        self.ingress.as_ref().ok()?.lend_current_scalar_sum_v1(read)
    }
    fn finite_entry_read(
        &mut self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        plan: &Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>,
        read: &SourceExprSiteV1,
        binding: BindingRefV1,
        root: BindingRefV1,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::FiniteEntryReadV1>,
        OrdinaryNewCoSealIssueV1,
    > {
        if self.writes.is_none() {
            return Ok(None);
        }
        let Ok(ingress) = self.ingress else {
            return Ok(None);
        };
        let site = OwnedExprSiteV1::new(input.owner(), plan.nodes()[0].site().clone());
        lexical_instance_call::lend_finite_entry_read_v1(
            input,
            ingress,
            self.parameter_contracts,
            &site,
            plan,
            read,
            binding,
            root,
        )
        .map_err(|issue| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site, issue })
    }
    fn original_writer(
        &mut self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        target: &SourceExprSiteV1,
    ) -> Result<
        Option<crate::mir::normal_callable_semantic_package::OriginalFieldWriteSourceLoanV1>,
        OrdinaryNewCoSealIssueV1,
    > {
        let Some(writes) = self.writes else {
            return Ok(None);
        };
        let issue = || OrdinaryNewCoSealIssueV1::SourceNavigation {
            site: OwnedExprSiteV1::new(owner, target.clone()),
        };
        let loan = writes
            .original_write_source_v1(owner, target)
            .map_err(|_| issue())?;
        if let Some(loan) = &loan {
            writes
                .recheck_original_write_source_v1(loan)
                .map_err(|_| issue())?;
        }
        Ok(loan)
    }
    fn assignment_source(
        &mut self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        target: &SourceExprSiteV1,
    ) -> Result<
        Option<Rc<crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1>>,
        OrdinaryNewCoSealIssueV1,
    > {
        if self.writes.is_none() {
            return Ok(None);
        }
        let issue = || OrdinaryNewCoSealIssueV1::SourceNavigation {
            site: OwnedExprSiteV1::new(owner, target.clone()),
        };
        let Some(loan) = self.original_writer(owner, target)? else {
            return Ok(None);
        };
        loan.write()
            .source()
            .map(Rc::clone)
            .map(Some)
            .map_err(|_| issue())
    }
}

#[path = "field_source_children.rs"]
mod children;
pub(in crate::mir::normal_callable_semantic_package) use children::declared_stored_child_field_v1;
pub(super) use children::dynamic_stored_child_field_v1;
pub(crate) use children::StoredChildFieldSourceV1;

/// Default-off (builder debug flag): which numbered check declined an entry field write.
#[inline(never)]
fn entry_write_declined<T>(
    input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    target: &SourceExprSiteV1,
    check: u32,
) -> Result<Option<T>, OrdinaryNewCoSealIssueV1> {
    if crate::config::env::builder_debug_enabled() {
        eprintln!(
            "[entry-write] declined owner={:?} target={target:?} check={check}",
            input.owner()
        );
    }
    Ok(None)
}
