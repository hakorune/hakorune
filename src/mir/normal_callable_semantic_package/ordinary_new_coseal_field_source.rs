//! Borrow original field sources through the one Home consult.
use super::*;
use std::collections::{BTreeMap, BTreeSet};
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call;

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
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) struct ScalarFieldsWithOriginalWritesV1<'a, F> {
    reads: F,
    caller: Option<&'a CanonicalSameModuleCallableKeyV1>,
    parameter_contracts: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    constructors: &'a VerifiedInstanceConstructorSemanticBatchV1,
    receiver: Option<(BindingRefV1, &'a crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    entry: Option<&'a crate::mir::resolved_semantics::VerifiedInstanceEntryHomeLoanV1>,
    results: &'a BorrowedResultCohortV1,
    ingress: &'a Result<lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    candidates: &'a [OrdinaryNewCandidate],
    writes: &'a super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
}

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn scalar_fields_with_original_writes<
    'a,
    F,
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
    writes: &'a super::super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    reads: F,
) -> ScalarFieldsWithOriginalWritesV1<'a, F>
where
    F: FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, OrdinaryNewCoSealIssueV1>,
{
    ScalarFieldsWithOriginalWritesV1 {
        reads,
        caller,
        parameter_contracts,
        constructors,
        receiver,
        entry,
        results,
        ingress,
        candidates,
        writes,
    }
}

impl<F>
    crate::mir::resolved_semantics::home_new_prefix::HomeFieldSourceConsultV1<
        OrdinaryNewCoSealIssueV1,
    > for ScalarFieldsWithOriginalWritesV1<'_, F>
where
    F: FnMut(
        &OwnedExprSiteV1,
        &SourceExprSiteV1,
        BindingRefV1,
        BindingRefV1,
        &str,
    ) -> Result<bool, OrdinaryNewCoSealIssueV1>,
{
    fn array_text_append_source(
        &mut self,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        site: &SourceExprSiteV1,
    ) -> Result<
        Option<Rc<crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1>>,
        OrdinaryNewCoSealIssueV1,
    > {
        let (Some(caller), Some(receiver)) = (self.caller, self.receiver) else {
            return Ok(None);
        };
        let Some(call) = input.function().method_call(site) else {
            return Ok(None);
        };
        let Some(argument) = call.arguments().first() else {
            return Ok(None);
        };
        let text = matches!(
            input
                .function()
                .expression_source()
                .literal(argument.site()),
            Some(crate::mir::resolved_semantics::ResolvedLiteralSourceV1::String(_))
        ) || matches!(input.function().variable_ref(argument.site()), Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding))
                if self.ingress.as_ref().is_ok_and(|ingress| ingress.source_value_sum_v1(binding).is_some_and(|sum| sum.text_only())));
        if !text {
            return Ok(None);
        }
        crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_array_read_source::capture_text_append(input, caller, receiver.1, site, self.constructors, self.writes)
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
    fn scalar_field(
        &mut self,
        site: &OwnedExprSiteV1,
        receiver_site: &SourceExprSiteV1,
        receiver: BindingRefV1,
        home: BindingRefV1,
        field: &str,
    ) -> Result<bool, OrdinaryNewCoSealIssueV1> {
        (self.reads)(site, receiver_site, receiver, home, field)
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
        let (Some(receiver), Some(entry)) = (self.receiver, self.entry) else {
            return Ok(None);
        };
        let Some(original) = self.original_writer(input.owner(), target)? else {
            return Ok(None);
        };
        if original.write().in_birth() {
            return Ok(None);
        }
        let site = OwnedExprSiteV1::new(input.owner(), target.clone());
        let Some((field, declared)) = terminal_home::source_declared_field(
            self.constructors,
            receiver.1,
            &site,
            original.write().field(),
        )?
        else {
            return Ok(None);
        };
        if !matches!(declared.as_deref(), None | Some("i64")) {
            return Ok(None);
        }
        let Ok(ingress) = self.ingress else {
            return Ok(None);
        };
        // A neutral source-only write is not an executable entry frame. Require
        // the complete value basis even for a declared slot before selecting it.
        use super::super::super::field_write_claim::{
            EntryFieldWriteValueSourceV1 as Value, TextFieldValueSourceV1,
        };
        // Select Text from the SAME original all-writer SUM, before the numeric
        // demand. A numeric refusal is never reinterpreted as a Text fallback.
        let text = original.write().owner_box()
            .and_then(|class| self.writes.value_source(class, original.write().field()))
            .and_then(|source| source.as_ref().ok())
            .and_then(|source| TextFieldValueSourceV1::lend(ingress, self.writes, source))
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
                            .and_then(|class| self.writes.value_source(class, original.write().field()))
                            .map(|source| source.as_ref().err());
                        eprintln!(
                            "[field-source] site={site:?} field={} issue={issue} retained={retained:?}",
                            original.write().field()
                        );
                    }
                    return Ok(None);
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
                input,
                original,
                root,
                field,
                declared,
                value,
            ),
        )
    }
    fn requires_initializer_use_frame(
        &mut self,
        site: &OwnedExprSiteV1,
    ) -> Result<bool, OrdinaryNewCoSealIssueV1> {
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
        let issue = || OrdinaryNewCoSealIssueV1::SourceNavigation {
            site: OwnedExprSiteV1::new(owner, target.clone()),
        };
        let loan = self
            .writes
            .original_write_source_v1(owner, target)
            .map_err(|_| issue())?;
        if let Some(loan) = &loan {
            self.writes
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct DynamicStoredChildFieldV1 {
    field: hakorune_mir_defs::CanonicalFieldRefV1,
    child: hakorune_mir_defs::CanonicalObjectIdV1,
    child_class: Box<str>,
    nullable: bool,
    paired_fields: BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>,
    parent_class: Box<str>,
    field_name: Box<str>,
    relations: Vec<std::rc::Rc<super::super::super::result_class_claim::SourceConstructionChildRelationV1>>,
}

/// Project the exact New-site child relations for one untyped field. The
/// relation cohort is already complete at issuance; recheck same-site field
/// pairing and homogeneous Null/Fresh alternatives before using it as a
/// stored-call source.
pub(super) fn dynamic_stored_child_field_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    facts: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    parent_class: &str,
    field_name: &str,
) -> Result<Option<DynamicStoredChildFieldV1>, String> {
    use super::super::super::result_class_claim::ResultValueOriginV1;
    let Some(parent_source) = batch.ordinary_box_coverage().row_for(parent_class)
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))? else {
        return Ok(None);
    };
    let parent_object = constructors.object_for(parent_source)
        .map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?;
    let field = constructors.with_source_object_definition(parent_source, |object, definition| {
        let mut matches = definition.fields().iter().enumerate()
            .filter(|(_, row)| row.name == field_name);
        let (ordinal, declaration) = matches.next()?;
        if matches.next().is_some() || declaration.is_weak || declaration.declared_type_name.is_some() {
            return None;
        }
        hakorune_mir_defs::CanonicalFieldRefV1::from_declaration_ordinal(object, ordinal)
    }).map_err(|error| format!("[freeze:contract][stored-child/definition]{error:?}"))?;
    let Some(field) = field else { return Ok(None); };
    if field.object() != parent_object { return Ok(None); }

    let mut per_outer = BTreeMap::<OwnedExprSiteV1,
        BTreeMap<hakorune_mir_defs::CanonicalFieldRefV1, bool>>::new();
    let mut child = None;
    let mut nullable = false;
    let mut relations = Vec::new();
    for relation in facts.construction_child_relation_loans().filter(|relation|
        relation.is_unqualified_dynamic_v1() && relation.field().object() == parent_object)
    {
        if relation.unqualified_field_source_v1(parent_class).is_err() || relation.witnesses().is_empty() {
            return Ok(None);
        }
        let first = &relation.witnesses()[0];
        let nonnull = match first.origin() {
            ResultValueOriginV1::Null => false,
            ResultValueOriginV1::Fresh(class) => {
                let Some(source) = batch.ordinary_box_coverage().row_for(class).ok().flatten() else {
                    return Ok(None);
                };
                if constructors.object_for(source).ok() != Some(relation.child()) { return Ok(None); }
                true
            }
            _ => return Ok(None),
        };
        if !relation.witnesses().iter().all(|witness| match (nonnull, witness.origin()) {
            (false, ResultValueOriginV1::Null) => true,
            (true, ResultValueOriginV1::Fresh(class)) => batch.ordinary_box_coverage()
                .row_for(class).ok().flatten()
                .and_then(|source| constructors.object_for(&source).ok()) == Some(relation.child()),
            _ => false,
        }) || child.is_some_and(|prior| prior != relation.child()) {
            return Ok(None);
        }
        relations.push(std::rc::Rc::clone(relation));
        child = Some(relation.child());
        nullable |= !nonnull;
        if per_outer.entry(relation.outer().clone()).or_default()
            .insert(relation.field(), nonnull).is_some() {
            return Ok(None);
        }
    }
    let Some(child) = child else { return Ok(None); };
    let Some(target_group) = per_outer.values().find(|fields| fields.contains_key(&field)) else {
        return Ok(None);
    };
    let paired_fields: BTreeSet<_> = target_group.keys().copied().collect();
    if per_outer.is_empty() || paired_fields.is_empty() || per_outer.values().any(|fields| {
        fields.keys().copied().collect::<BTreeSet<_>>() != paired_fields
            || fields.values().copied().collect::<BTreeSet<_>>().len() != 1
    }) {
        return Ok(None);
    }
    let mut child_classes = batch.ordinary_box_coverage().rows().iter().filter(|source|
        constructors.object_for(source).ok() == Some(child));
    let Some(child_source) = child_classes.next() else { return Ok(None); };
    if child_classes.next().is_some() { return Ok(None); }
    Ok(Some(DynamicStoredChildFieldV1 {
        field, child, child_class: child_source.name().into(), nullable, paired_fields,
        parent_class: parent_class.into(), field_name: field_name.into(), relations,
    }))
}

impl DynamicStoredChildFieldV1 {
    pub(in crate::mir) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 { self.field }
    pub(in crate::mir) fn child(&self) -> hakorune_mir_defs::CanonicalObjectIdV1 { self.child }
    pub(in crate::mir) fn child_class(&self) -> &str { &self.child_class }
    pub(super) fn nullable(&self) -> bool { self.nullable }
    pub(super) fn paired_field(&self, field: hakorune_mir_defs::CanonicalFieldRefV1) -> bool {
        self.paired_fields.contains(&field)
    }
    /// The same complete immutable parent cohort, not a new class inference.
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_v1(
        &self, facts: &super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1, field_name: &str,
    ) -> bool {
        let current: Vec<_> = facts.construction_child_relation_loans().filter(|row|
            row.is_unqualified_dynamic_v1() && row.field().object() == self.field.object()).collect();
        if self.field_name.as_ref() != field_name || self.relations.is_empty()
            || current.len() != self.relations.len()
            || !current.iter().zip(&self.relations).all(|(a, b)| std::rc::Rc::ptr_eq(a, b)
                && a.unqualified_field_source_v1(&self.parent_class).is_ok()) {
            return false;
        }
        // Re-derive the summary from the same immutable sources. Matching two
        // mutable coordinates is not a substitute for original field identity.
        use super::super::super::result_class_claim::ResultValueOriginV1;
        let mut per_outer = BTreeMap::<OwnedExprSiteV1, BTreeMap<_, bool>>::new();
        let mut nullable = false;
        let mut requested = false;
        for relation in &self.relations {
            if relation.child() != self.child || relation.witnesses().is_empty() { return false; }
            let nonnull = match relation.witnesses()[0].origin() {
                ResultValueOriginV1::Null => false,
                ResultValueOriginV1::Fresh(class) if class.as_ref() == self.child_class() => true,
                _ => return false,
            };
            if !relation.witnesses().iter().all(|witness| match witness.origin() {
                ResultValueOriginV1::Null => !nonnull,
                ResultValueOriginV1::Fresh(class) => nonnull && class.as_ref() == self.child_class(),
                _ => false,
            }) { return false; }
            if relation.field() == self.field {
                if relation.original_field_name_v1() != Ok(field_name) { return false; }
                requested = true;
            }
            nullable |= !nonnull;
            if per_outer.entry(relation.outer().clone()).or_default()
                .insert(relation.field(), nonnull).is_some() { return false; }
        }
        requested && nullable == self.nullable && per_outer.values().all(|fields|
            fields.keys().copied().collect::<BTreeSet<_>>() == self.paired_fields
                && fields.values().copied().collect::<BTreeSet<_>>().len() == 1)
    }
}
