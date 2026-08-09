//! Source-backed dynamic formal/local/Loop carrier authority.
//!
//! This module runs only at the existing normal-callable semantic source
//! seal, while canonical syntax and the matching resolver forest are both
//! available. It emits no MIR and never treats raw `MirType::Unknown` as
//! semantic evidence.

use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_semantics::{
    project_source_node_v1, BindingKindV1, BindingOriginV1, BindingRefV1,
    CallableFunctionSyntaxViewV1, CallableSemanticSourceLedgerView, FunctionOwnerIdV1,
    ProjectedSourceNodeV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1, SourceBindingSiteV1,
    SourceExprSiteV1, SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1,
    VerifiedCallableLoopMembershipV1, VerifiedSemanticOwnerForestV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum SourceBackedDynamicCallableIssueV1 {
    SourceProjection(String),
    MissingFunctionSyntax,
    ParameterDeclarationCardinality { names: usize, declarations: usize },
    ParameterNameMismatch { index: u32 },
    MissingParameterBinding { index: u32 },
    ParameterBindingMismatch { index: u32 },
    InvalidLocalDeclaration(SourceBindingSiteV1),
    MissingInitializerLexicalRef(SourceExprSiteV1),
    ForeignInitializerBinding(SourceExprSiteV1),
    DuplicateDynamicFormal(BindingRefV1),
    DuplicateDynamicLocal(BindingRefV1),
    LoopMembership(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct VerifiedDynamicFormalSourceV1 {
    parameter_ordinal: u32,
    binding: BindingRefV1,
}

impl VerifiedDynamicFormalSourceV1 {
    pub(super) const fn parameter_ordinal(self) -> u32 {
        self.parameter_ordinal
    }

    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VerifiedDynamicLocalInitializationSourceV1 {
    formal: BindingRefV1,
    local: BindingRefV1,
    declaration: SourceBindingSiteV1,
    initializer: SourceExprSiteV1,
}

impl VerifiedDynamicLocalInitializationSourceV1 {
    pub(super) const fn formal(&self) -> BindingRefV1 {
        self.formal
    }

    pub(super) const fn local(&self) -> BindingRefV1 {
        self.local
    }

    pub(super) const fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }

    pub(super) const fn initializer(&self) -> &SourceExprSiteV1 {
        &self.initializer
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VerifiedDynamicLoopCarrierSourceV1 {
    local: BindingRefV1,
    condition_reads: Box<[SourceExprSiteV1]>,
    body_rebinds: Box<[SourceExprSiteV1]>,
}

impl VerifiedDynamicLoopCarrierSourceV1 {
    pub(super) const fn local(&self) -> BindingRefV1 {
        self.local
    }

    pub(super) fn condition_reads(&self) -> &[SourceExprSiteV1] {
        &self.condition_reads
    }

    pub(super) fn body_rebinds(&self) -> &[SourceExprSiteV1] {
        &self.body_rebinds
    }
}

#[derive(Debug)]
pub(super) struct VerifiedDynamicLoopSourceV1 {
    membership: VerifiedCallableLoopMembershipV1,
    carriers: Box<[VerifiedDynamicLoopCarrierSourceV1]>,
}

impl VerifiedDynamicLoopSourceV1 {
    pub(super) const fn membership(&self) -> &VerifiedCallableLoopMembershipV1 {
        &self.membership
    }

    pub(super) fn carriers(&self) -> &[VerifiedDynamicLoopCarrierSourceV1] {
        &self.carriers
    }
}

/// Complete source-backed dynamic inventory for one resolved callable.
///
/// The product is deliberately non-`Clone`. Partial row types have no public
/// constructors, so later stages cannot combine a formal from one callable
/// with a local or Loop from another.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedSourceBackedDynamicCallableV1 {
    owner: FunctionOwnerIdV1,
    formals: Box<[VerifiedDynamicFormalSourceV1]>,
    local_initializations: Box<[VerifiedDynamicLocalInitializationSourceV1]>,
    loops: Box<[VerifiedDynamicLoopSourceV1]>,
}

impl VerifiedSourceBackedDynamicCallableV1 {
    pub(in crate::mir) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    /// Returns the exact source formal from which one Dynamic binding derives.
    ///
    /// A formal is its own origin. A supported local copy retains the formal
    /// sealed by the source product. No type, MIR value, or method name is
    /// consulted here.
    pub(in crate::mir) fn origin_for_binding(&self, binding: BindingRefV1) -> Option<BindingRefV1> {
        if self.formals.iter().any(|row| row.binding() == binding) {
            return Some(binding);
        }
        self.local_initializations
            .iter()
            .find(|row| row.local() == binding)
            .map(VerifiedDynamicLocalInitializationSourceV1::formal)
    }

    pub(super) fn formals(&self) -> &[VerifiedDynamicFormalSourceV1] {
        &self.formals
    }

    pub(super) fn local_initializations(&self) -> &[VerifiedDynamicLocalInitializationSourceV1] {
        &self.local_initializations
    }

    pub(super) fn loops(&self) -> &[VerifiedDynamicLoopSourceV1] {
        &self.loops
    }
}

pub(super) struct SourceBackedDynamicCallableIssuerV1;

pub(in crate::mir) fn issue_source_backed_dynamic_callable_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
) -> Result<VerifiedSourceBackedDynamicCallableV1, String> {
    SourceBackedDynamicCallableIssuerV1::issue_from_resolved_input(input)
        .map_err(|error| format!("{error:?}"))
}

impl SourceBackedDynamicCallableIssuerV1 {
    pub(super) fn issue(
        function: &ASTNode,
        forest: &VerifiedSemanticOwnerForestV1,
        projection: &VerifiedSourceProjectionV1,
    ) -> Result<VerifiedSourceBackedDynamicCallableV1, SourceBackedDynamicCallableIssueV1> {
        let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            function, forest, projection,
        )
        .map_err(|error| {
            SourceBackedDynamicCallableIssueV1::SourceProjection(format!("{error:?}"))
        })?;
        Self::issue_from_resolved_input(input)
    }

    pub(super) fn issue_from_resolved_input(
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<VerifiedSourceBackedDynamicCallableV1, SourceBackedDynamicCallableIssueV1> {
        let function = input.source().root();
        let syntax = CallableFunctionSyntaxViewV1::from_function_ast(function)
            .ok_or(SourceBackedDynamicCallableIssueV1::MissingFunctionSyntax)?;
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .map_err(|error| {
                SourceBackedDynamicCallableIssueV1::SourceProjection(format!("{error:?}"))
            })?;
        let formals = issue_formals(syntax, &ledger)?;
        let dynamic_bindings = formals
            .iter()
            .map(|row| row.binding())
            .collect::<BTreeSet<_>>();
        let local_initializations =
            issue_local_initializations(function, &ledger, &dynamic_bindings)?;
        let loops = issue_loop_carriers(&ledger, &local_initializations)?;
        Ok(VerifiedSourceBackedDynamicCallableV1 {
            owner: input.owner(),
            formals: formals.into_boxed_slice(),
            local_initializations: local_initializations.into_boxed_slice(),
            loops: loops.into_boxed_slice(),
        })
    }
}

fn issue_formals(
    syntax: CallableFunctionSyntaxViewV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
) -> Result<Vec<VerifiedDynamicFormalSourceV1>, SourceBackedDynamicCallableIssueV1> {
    let header = syntax.header();
    if header.params().len() != header.param_decls().len() {
        return Err(
            SourceBackedDynamicCallableIssueV1::ParameterDeclarationCardinality {
                names: header.params().len(),
                declarations: header.param_decls().len(),
            },
        );
    }
    let records = ledger.bindings().collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut rows = Vec::new();
    for (index, (name, declaration)) in header.params().iter().zip(header.param_decls()).enumerate()
    {
        let index = u32::try_from(index).map_err(|_| {
            SourceBackedDynamicCallableIssueV1::ParameterDeclarationCardinality {
                names: header.params().len(),
                declarations: header.param_decls().len(),
            }
        })?;
        if declaration.name != *name {
            return Err(SourceBackedDynamicCallableIssueV1::ParameterNameMismatch { index });
        }
        if declaration.declared_type_name.is_some() {
            continue;
        }
        let site = SourceBindingSiteV1::Parameter { index };
        let binding = ledger
            .declaration_binding(&site)
            .ok_or(SourceBackedDynamicCallableIssueV1::MissingParameterBinding { index })?;
        let Some(record) = records.get(&binding) else {
            return Err(SourceBackedDynamicCallableIssueV1::ParameterBindingMismatch { index });
        };
        if record.kind() != (BindingKindV1::Parameter { index })
            || record.origin() != &BindingOriginV1::Source(site.clone())
        {
            return Err(SourceBackedDynamicCallableIssueV1::ParameterBindingMismatch { index });
        }
        if !seen.insert(binding) {
            return Err(SourceBackedDynamicCallableIssueV1::DuplicateDynamicFormal(
                binding,
            ));
        }
        rows.push(VerifiedDynamicFormalSourceV1 {
            parameter_ordinal: index,
            binding,
        });
    }
    Ok(rows)
}

fn issue_local_initializations(
    function: &ASTNode,
    ledger: &CallableSemanticSourceLedgerView<'_>,
    dynamic_formals: &BTreeSet<BindingRefV1>,
) -> Result<Vec<VerifiedDynamicLocalInitializationSourceV1>, SourceBackedDynamicCallableIssueV1> {
    let lexical_refs = ledger.variable_refs().collect::<BTreeMap<_, _>>();
    let mut rows = Vec::new();
    let mut seen_locals = BTreeSet::new();
    for declaration in ledger.declaration_sites() {
        let SourceBindingSiteV1::Local { statement, ordinal } = declaration else {
            continue;
        };
        let initializer = SourcePathV1::from_node(statement.node())
            .child(SourcePathSegmentV1::Initializer(*ordinal))
            .expr();
        let Some(ResolvedLexicalRefV1::Local(formal)) =
            lexical_refs.get(&initializer).map(|resolved| **resolved)
        else {
            continue;
        };
        if !dynamic_formals.contains(&formal) {
            continue;
        }
        let Some(ProjectedSourceNodeV1::Node(ASTNode::Local {
            variables,
            initial_values,
            declared_type_names,
            ..
        })) = project_source_node_v1(function, statement.node())
        else {
            return Err(SourceBackedDynamicCallableIssueV1::InvalidLocalDeclaration(
                declaration.clone(),
            ));
        };
        let ordinal_usize = *ordinal as usize;
        if variables.get(ordinal_usize).is_none()
            || initial_values
                .get(ordinal_usize)
                .and_then(Option::as_deref)
                .is_none()
            || declared_type_names.get(ordinal_usize) != Some(&None)
        {
            continue;
        }
        if !ledger
            .source_site_inventory()
            .contains_expression(&initializer)
        {
            return Err(
                SourceBackedDynamicCallableIssueV1::MissingInitializerLexicalRef(initializer),
            );
        }
        let local = ledger.declaration_binding(declaration).ok_or_else(|| {
            SourceBackedDynamicCallableIssueV1::InvalidLocalDeclaration(declaration.clone())
        })?;
        if formal.owner() != ledger.owner() || local.owner() != ledger.owner() {
            return Err(SourceBackedDynamicCallableIssueV1::ForeignInitializerBinding(initializer));
        }
        if !seen_locals.insert(local) {
            return Err(SourceBackedDynamicCallableIssueV1::DuplicateDynamicLocal(
                local,
            ));
        }
        rows.push(VerifiedDynamicLocalInitializationSourceV1 {
            formal,
            local,
            declaration: declaration.clone(),
            initializer,
        });
    }
    Ok(rows)
}

fn issue_loop_carriers(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    local_initializations: &[VerifiedDynamicLocalInitializationSourceV1],
) -> Result<Vec<VerifiedDynamicLoopSourceV1>, SourceBackedDynamicCallableIssueV1> {
    let loop_sites = ledger.loop_sites().cloned().collect::<Vec<_>>();
    let mut rows = Vec::new();
    for loop_site in &loop_sites {
        let membership = ledger.resolved_loop_source(loop_site).map_err(|error| {
            SourceBackedDynamicCallableIssueV1::LoopMembership(format!("{error:?}"))
        })?;
        let mut carriers = Vec::new();
        for initialization in local_initializations {
            let condition_reads = ledger
                .variable_refs()
                .filter_map(|(site, resolved)| {
                    (resolved == &ResolvedLexicalRefV1::Local(initialization.local())
                        && nearest_loop_site(site, &loop_sites) == Some(loop_site)
                        && is_loop_condition_site(site, loop_site))
                    .then(|| site.clone())
                })
                .collect::<Vec<_>>();
            let body_rebinds = ledger
                .assignment_targets()
                .filter_map(|(site, target)| {
                    (target == &ResolvedAssignmentTargetV1::BindingRebind(initialization.local())
                        && nearest_loop_site(site, &loop_sites) == Some(loop_site)
                        && is_loop_body_site(site, loop_site))
                    .then(|| site.clone())
                })
                .collect::<Vec<_>>();
            if !condition_reads.is_empty() && !body_rebinds.is_empty() {
                carriers.push(VerifiedDynamicLoopCarrierSourceV1 {
                    local: initialization.local(),
                    condition_reads: condition_reads.into_boxed_slice(),
                    body_rebinds: body_rebinds.into_boxed_slice(),
                });
            }
        }
        rows.push(VerifiedDynamicLoopSourceV1 {
            membership,
            carriers: carriers.into_boxed_slice(),
        });
    }
    Ok(rows)
}

fn nearest_loop_site<'a>(
    site: &SourceExprSiteV1,
    loops: &'a [SourceStmtSiteV1],
) -> Option<&'a SourceStmtSiteV1> {
    loops
        .iter()
        .filter(|loop_site| {
            site.node()
                .segments()
                .starts_with(loop_site.node().segments())
        })
        .max_by_key(|loop_site| loop_site.node().segments().len())
}

fn is_loop_condition_site(site: &SourceExprSiteV1, loop_site: &SourceStmtSiteV1) -> bool {
    let prefix = loop_site.node().segments();
    site.node().segments().get(prefix.len()) == Some(&SourcePathSegmentV1::LoopCondition)
}

fn is_loop_body_site(site: &SourceExprSiteV1, loop_site: &SourceStmtSiteV1) -> bool {
    let prefix = loop_site.node().segments();
    matches!(
        site.node().segments().get(prefix.len()),
        Some(SourcePathSegmentV1::LoopBody(_))
    )
}

#[cfg(test)]
#[path = "normal_callable_dynamic_source_tests.rs"]
mod tests;
