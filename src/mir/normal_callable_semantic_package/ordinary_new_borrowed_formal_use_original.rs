//! Canonical original read/use obligations, independent of transport classifiers.
//! Source identity only: no current read, execution, object or ABI authority.
use super::*;
use crate::mir::resolved_semantics::source_value_flow::VerifiedSourceValueFlowV1;
use crate::mir::resolved_semantics::ResolvedInitializerRelationV1;
use std::sync::Arc;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct OriginalFormalUseSourceV1
{
    owner: FunctionOwnerIdV1,
    batch_slot: u32,
    mode: crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1,
    parameters: Box<[(u32, BindingRefV1, CallableParameterContractKindV1)]>,
    roots: BTreeMap<BindingRefV1, BindingRefV1>,
    reads: Box<[(SourceExprSiteV1, ResolvedLexicalRefV1)]>,
    flow: Option<Arc<VerifiedSourceValueFlowV1>>,
    initializers: Box<[ResolvedInitializerRelationV1]>,
    assignments: Box<[(SourceExprSiteV1, ResolvedAssignmentTargetV1)]>,
    captures: Box<[(OwnedExprSiteV1, BindingRefV1, bool)]>,
}
impl OriginalFormalUseSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn issue(
        input: ResolvedFunctionLoweringInputV1<'_>,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> Result<Rc<Self>, BorrowedFormalUseDraftErrorV1> {
        let function = input.function();
        if input.owner() != contract.owner
            || function.owner() != contract.owner
            || function
                .declaration_sites()
                .filter(|site| matches!(site, SourceBindingSiteV1::Parameter { .. }))
                .count()
                != contract.parameters.len()
        {
            return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
        }
        let mut origins = BTreeMap::new();
        for (index, formal) in contract.parameters.iter().enumerate() {
            if formal.ordinal as usize != index
                || formal.binding.owner() != contract.owner
                || function.declaration_binding(&SourceBindingSiteV1::Parameter {
                    index: formal.ordinal,
                }) != Some(formal.binding)
                || !matches!(function.binding(formal.binding).map(|row| row.kind()),
                Some(BindingKindV1::Parameter { index }) if index == formal.ordinal)
            {
                return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
            }
            if formal.kind.is_ordinary_borrowed_handle() {
                if origins.insert(formal.binding, formal.binding).is_some() {
                    return Err(BorrowedFormalUseDraftErrorV1::SourceIdentity);
                }
            }
        }

        let reads: Box<[_]> = function
            .variable_refs()
            .map(|(site, reference)| (site.clone(), *reference))
            .collect();
        let assignments: Box<[_]> = function
            .assignment_targets()
            .map(|(site, target)| (site.clone(), target.clone()))
            .collect();
        let mut captures = Vec::new();
        for (owner, descendant) in input.forest().semantic_owners() {
            let mut retain_assignment =
                |site: &SourceExprSiteV1, target: &ResolvedAssignmentTargetV1| {
                    if let ResolvedAssignmentTargetV1::UpvarRebind(upvar) = target {
                        if upvar.source().owner() == input.owner() {
                            captures.push((
                                OwnedExprSiteV1::new(owner, site.clone()),
                                upvar.source(),
                                true,
                            ));
                        }
                    }
                };
            if owner == input.owner() {
                for (site, target) in &assignments {
                    retain_assignment(site, target);
                }
            } else {
                for (site, target) in descendant.assignment_targets() {
                    retain_assignment(site, target);
                }
            }
            let mut retain = |site: &SourceExprSiteV1, reference: &ResolvedLexicalRefV1| {
                if let ResolvedLexicalRefV1::Upvar(upvar) = reference {
                    if upvar.source().owner() == input.owner() {
                        captures.push((
                            OwnedExprSiteV1::new(owner, site.clone()),
                            upvar.source(),
                            false,
                        ));
                    }
                }
            };
            if owner == input.owner() {
                for (site, reference) in &reads {
                    retain(site, reference);
                }
            } else {
                for (site, reference) in descendant.variable_refs() {
                    retain(site, reference);
                }
            }
        }
        Ok(Rc::new(Self {
            owner: input.owner(),
            batch_slot: contract.batch_slot,
            mode: contract.mode,
            parameters: contract
                .parameters
                .iter()
                .map(|row| (row.ordinal, row.binding, row.kind.clone()))
                .collect(),
            roots: origins,
            reads,
            flow: function.expression_source().value_flow().map(Arc::clone),
            initializers: function
                .expression_source()
                .initializers()
                .cloned()
                .collect(),
            assignments,
            captures: captures.into_boxed_slice(),
        }))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn matches_contract(
        &self,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> bool {
        self.owner == contract.owner
            && self.batch_slot == contract.batch_slot
            && self.mode == contract.mode
            && self.parameters.len() == contract.parameters.len()
            && self.parameters.iter().zip(&contract.parameters).all(
                |((ordinal, binding, kind), row)| {
                    *ordinal == row.ordinal && *binding == row.binding && kind == &row.kind
                },
            )
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn roots(
        &self,
    ) -> &BTreeMap<BindingRefV1, BindingRefV1> {
        &self.roots
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn reads(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, &ResolvedLexicalRefV1)> {
        self.reads.iter().map(|(site, reference)| (site, reference))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn initializers(
        &self,
    ) -> &[ResolvedInitializerRelationV1] {
        &self.initializers
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn assignments(
        &self,
    ) -> &[(SourceExprSiteV1, ResolvedAssignmentTargetV1)] {
        &self.assignments
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn captures(
        &self,
    ) -> &[(OwnedExprSiteV1, BindingRefV1, bool)] {
        &self.captures
    }
    /// Exact retained initializer and current source flow, not raw read membership.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn initializer_entry_alias_v1(
        &self, initializer: &ResolvedInitializerRelationV1, source_binding: Option<BindingRefV1>,
    ) -> Result<Option<BindingRefV1>, BorrowedFormalUseDraftErrorV1> {
        let reject = || BorrowedFormalUseDraftErrorV1::SourceIdentity;
        if initializer.binding().owner() != self.owner
            || self.initializers.iter().filter(|row| *row == initializer).count() != 1 { return Err(reject()); }
        let Some(source_binding) = source_binding else { return Ok(None) };
        let SourceBindingSiteV1::Local { statement, ordinal } = initializer.declaration_site() else { return Err(reject()) };
        let site = crate::mir::resolved_semantics::SourcePathV1::from_node(statement.node())
            .child(crate::mir::resolved_semantics::SourcePathSegmentV1::Initializer(*ordinal)).expr();
        if initializer.initializer_site() != Some(&site)
            || source_binding.owner() != self.owner
            || self.reads.iter().filter(|(at, reference)| *at == site
                && *reference == ResolvedLexicalRefV1::Local(source_binding)).count() != 1 { return Err(reject()); }
        let Some(flow) = &self.flow else { return Ok(None) };
        let Some(point) = flow.read(&site, source_binding) else { return Ok(None) };
        let Some(formal) = flow.alias_entry_origin(point, |site|
            self.reads.iter().find(|(at, _)| at == site).map(|(_, reference)| *reference)) else { return Ok(None) };
        if self.roots.get(&formal) != Some(&formal) { return Ok(None); }
        if initializer.declared_type_name().is_some() || formal == initializer.binding() { return Err(reject()); }
        Ok(Some(formal))
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn lend_read(
        self: &Rc<Self>,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        formal: BindingRefV1,
    ) -> Option<OriginalFormalReadLoanV1> {
        let index = self.reads.iter().position(|(original, reference)| {
            original == site && *reference == ResolvedLexicalRefV1::Local(binding)
        })?;
        if !self.roots.contains_key(&formal) {
            return None;
        }
        // Raw membership only. A Join/unknown point stays an obligation for the
        // existing source/current/Loop owner; this loan grants no alias view.
        let point = self.flow.as_ref().and_then(|flow| flow.read(site, binding));
        Some(OriginalFormalReadLoanV1 {
            original: Rc::clone(self),
            index,
            formal,
            point,
        })
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call)
struct OriginalFormalReadLoanV1
{
    original: Rc<OriginalFormalUseSourceV1>,
    index: usize,
    formal: BindingRefV1,
    point: Option<crate::mir::resolved_semantics::source_value_flow::SourceValueRefV1>,
}
impl PartialEq for OriginalFormalReadLoanV1 {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.original, &other.original)
            && self.index == other.index
            && self.formal == other.formal
            && self.point == other.point
    }
}
impl Eq for OriginalFormalReadLoanV1 {}
impl OriginalFormalReadLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn matches_contract(
        &self,
        contract: &OwnedCallableParameterContractDeclarationV1,
    ) -> bool {
        self.original.matches_contract(contract)
    }
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn matches(
        &self,
        site: &SourceExprSiteV1,
        binding: BindingRefV1,
        formal: BindingRefV1,
    ) -> bool {
        self.formal == formal
            && self.original.reads.get(self.index)
                == Some(&(site.clone(), ResolvedLexicalRefV1::Local(binding)))
    }
}
