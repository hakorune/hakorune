//! Resolver-owned source relation for direct `me.method(...)` calls.
//!
//! This joins facts from one final callable batch. It issues no target,
//! ValueId, effect, ABI, Recipe, MIR, or physical product.

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet};

use crate::parser::{
    CallableDeclarationIdentityV1, ResolverBoxMethodSourceSiteV1,
    ResolverSourceInvocationProvenanceV1,
};

use super::{
    BindingRefV1, DeclaredInstanceMethodIdentityV1, FunctionOwnerIdV1, OwnedExprSiteV1,
    ResolverNominalBoxDeclarationInputV1, ResolverNominalTypeEnvironmentIssueV1,
    ResolverNominalTypeEnvironmentV1, SourceExprSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeclaredInstanceMethodModeV1 {
    Static,
    Instance,
}

/// One parser-issued direct Box method declaration; names/arity validate only
/// after its opaque source identity has entered the relation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DeclaredInstanceMethodSourceRefV1<'source> {
    declaration_identity: &'source CallableDeclarationIdentityV1,
    parser_provenance: &'source ResolverSourceInvocationProvenanceV1,
    source_site: ResolverBoxMethodSourceSiteV1,
    box_name: &'source str,
    method_name: &'source str,
    mode: DeclaredInstanceMethodModeV1,
    parameter_count: u32,
    owner: FunctionOwnerIdV1,
}

impl<'source> DeclaredInstanceMethodSourceRefV1<'source> {
    pub(crate) fn new(
        declaration_identity: &'source CallableDeclarationIdentityV1,
        parser_provenance: &'source ResolverSourceInvocationProvenanceV1,
        source_site: ResolverBoxMethodSourceSiteV1,
        box_name: &'source str,
        method_name: &'source str,
        mode: DeclaredInstanceMethodModeV1,
        parameter_count: u32,
        owner: FunctionOwnerIdV1,
    ) -> Self {
        Self {
            declaration_identity,
            parser_provenance,
            source_site,
            box_name,
            method_name,
            mode,
            parameter_count,
            owner,
        }
    }
}

/// One resolver-issued lexical method call. Its receiver binding is compared
/// to the caller's exact `SourceBindingSiteV1::Receiver` binding.
#[derive(Debug)]
pub(crate) struct DeclaredInstanceCallSourceRefV1<'source> {
    caller_identity: &'source CallableDeclarationIdentityV1,
    caller_provenance: &'source ResolverSourceInvocationProvenanceV1,
    caller_source_site: ResolverBoxMethodSourceSiteV1,
    caller_owner: FunctionOwnerIdV1,
    call_site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver_binding: BindingRefV1,
    root_receiver_binding: Option<BindingRefV1>,
    root_receiver_declaration_count: usize,
    selector: &'source str,
    arity: u32,
}

impl<'source> DeclaredInstanceCallSourceRefV1<'source> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        caller_identity: &'source CallableDeclarationIdentityV1,
        caller_provenance: &'source ResolverSourceInvocationProvenanceV1,
        caller_source_site: ResolverBoxMethodSourceSiteV1,
        caller_owner: FunctionOwnerIdV1,
        call_site: SourceExprSiteV1,
        receiver_site: SourceExprSiteV1,
        receiver_binding: BindingRefV1,
        root_receiver_binding: Option<BindingRefV1>,
        root_receiver_declaration_count: usize,
        selector: &'source str,
        arity: u32,
    ) -> Self {
        Self {
            caller_identity,
            caller_provenance,
            caller_source_site,
            caller_owner,
            call_site,
            receiver_site,
            receiver_binding,
            root_receiver_binding,
            root_receiver_declaration_count,
            selector,
            arity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DeclaredInstanceCallRelationIssueV1 {
    SourceCoverage,
    ForeignMethodSourceIdentity,
    ParserProvenanceMismatch,
    NominalEnvironment(ResolverNominalTypeEnvironmentIssueV1),
    NominalSourceMismatch {
        statement_ordinal: u32,
        expected: Box<str>,
        actual: Box<str>,
    },
    DuplicateVisibleMethodName {
        box_statement_ordinal: u32,
        name: Box<str>,
    },
    RootProfileMismatch,
    RootReceiverCardinality {
        actual: usize,
    },
    RootReceiverBindingMissing,
    RootReceiverBindingMismatch,
    DuplicateCallSite(SourceExprSiteV1),
    TargetMissing {
        box_statement_ordinal: u32,
        name: Box<str>,
    },
    TargetReceiverPolicyMismatch {
        box_statement_ordinal: u32,
        name: Box<str>,
    },
    TargetArityMismatch {
        name: Box<str>,
        expected: u32,
        actual: u32,
    },
}

/// AST-free source relation row. A later package row may turn it into a target.
#[derive(Debug)]
pub(crate) struct VerifiedDeclaredInstanceCallRelationV1 {
    caller_identity: CallableDeclarationIdentityV1,
    caller_owner: FunctionOwnerIdV1,
    call_site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver_binding: BindingRefV1,
    target_identity: CallableDeclarationIdentityV1,
    target_method_identity: DeclaredInstanceMethodIdentityV1,
    target_owner: FunctionOwnerIdV1,
    source_arity: u32,
}

impl VerifiedDeclaredInstanceCallRelationV1 {
    pub(crate) fn caller_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.caller_identity
    }
    pub(crate) const fn caller_owner(&self) -> FunctionOwnerIdV1 {
        self.caller_owner
    }
    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }
    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }
    pub(crate) const fn receiver_binding(&self) -> BindingRefV1 {
        self.receiver_binding
    }
    pub(crate) fn target_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.target_identity
    }
    pub(crate) const fn target_method_identity(&self) -> DeclaredInstanceMethodIdentityV1 {
        self.target_method_identity
    }
    pub(crate) const fn target_owner(&self) -> FunctionOwnerIdV1 {
        self.target_owner
    }
    pub(crate) const fn source_arity(&self) -> u32 {
        self.source_arity
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedDeclaredInstanceCallRelationCatalogV1 {
    parser_provenance: ResolverSourceInvocationProvenanceV1,
    resolver_brand: super::ResolverCatalogBrandV1,
    rows: Box<[VerifiedDeclaredInstanceCallRelationV1]>,
}

impl VerifiedDeclaredInstanceCallRelationCatalogV1 {
    pub(crate) fn rows(&self) -> &[VerifiedDeclaredInstanceCallRelationV1] {
        &self.rows
    }
    pub(crate) const fn len(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Debug)]
pub(crate) enum DeclaredInstanceCallSourceDispositionV1 {
    NoRootDeclaredInstanceCall,
    Published(VerifiedDeclaredInstanceCallRelationCatalogV1),
}

pub(crate) struct DeclaredInstanceCallRelationIssuerV1;

impl DeclaredInstanceCallRelationIssuerV1 {
    pub(crate) fn issue(
        methods: &[DeclaredInstanceMethodSourceRefV1<'_>],
        calls: &[DeclaredInstanceCallSourceRefV1<'_>],
    ) -> Result<DeclaredInstanceCallSourceDispositionV1, DeclaredInstanceCallRelationIssueV1> {
        if calls.is_empty() {
            return Ok(DeclaredInstanceCallSourceDispositionV1::NoRootDeclaredInstanceCall);
        }
        let Some(first) = methods.first() else {
            return Err(DeclaredInstanceCallRelationIssueV1::SourceCoverage);
        };
        if methods
            .iter()
            .skip(1)
            .any(|method| !first.parser_provenance.same_as(method.parser_provenance))
            || calls
                .iter()
                .any(|call| !first.parser_provenance.same_as(call.caller_provenance))
        {
            return Err(DeclaredInstanceCallRelationIssueV1::ParserProvenanceMismatch);
        }

        let boxes = collect_method_inventory(methods)?;
        let environment =
            ResolverNominalTypeEnvironmentV1::issue(boxes.iter().map(|(&statement, name)| {
                ResolverNominalBoxDeclarationInputV1::new(statement, name.clone())
            }))
            .map_err(DeclaredInstanceCallRelationIssueV1::NominalEnvironment)?;
        // Expression paths are relative to an owner.  Brand the duplicate
        // key with that owner so identical `Body(0)/Value` paths in sibling
        // functions (or boxes) remain distinct source sites.
        let mut seen_calls = BTreeSet::<OwnedExprSiteV1>::new();
        let mut relations = Vec::new();
        for call in calls {
            let owned_call_site = OwnedExprSiteV1::new(call.caller_owner, call.call_site.clone());
            if !seen_calls.insert(owned_call_site) {
                return Err(DeclaredInstanceCallRelationIssueV1::DuplicateCallSite(
                    call.call_site.clone(),
                ));
            }
            let Some(caller) = methods.iter().find(|method| {
                method.declaration_identity.same_as(call.caller_identity)
                    && method.parser_provenance.same_as(call.caller_provenance)
                    && method.source_site == call.caller_source_site
            }) else {
                return Err(DeclaredInstanceCallRelationIssueV1::ForeignMethodSourceIdentity);
            };
            if caller.mode != DeclaredInstanceMethodModeV1::Instance {
                return Err(DeclaredInstanceCallRelationIssueV1::RootProfileMismatch);
            }
            let Some(root_binding) = call.root_receiver_binding else {
                return Err(DeclaredInstanceCallRelationIssueV1::RootReceiverBindingMissing);
            };
            if call.root_receiver_declaration_count != 1 {
                return Err(
                    DeclaredInstanceCallRelationIssueV1::RootReceiverCardinality {
                        actual: call.root_receiver_declaration_count,
                    },
                );
            }
            if root_binding.owner() != call.caller_owner
                || call.receiver_binding.owner() != call.caller_owner
            {
                return Err(DeclaredInstanceCallRelationIssueV1::RootReceiverBindingMismatch);
            }
            if call.receiver_binding != root_binding {
                continue;
            }
            let statement = call.caller_source_site.box_statement_ordinal();
            let mut targets = methods.iter().filter(|method| {
                method.parser_provenance.same_as(caller.parser_provenance)
                    && method.source_site.box_statement_ordinal() == statement
                    && method.method_name == call.selector
            });
            let Some(target) = targets.next() else {
                return Err(DeclaredInstanceCallRelationIssueV1::TargetMissing {
                    box_statement_ordinal: statement,
                    name: call.selector.into(),
                });
            };
            if targets.next().is_some() {
                return Err(
                    DeclaredInstanceCallRelationIssueV1::DuplicateVisibleMethodName {
                        box_statement_ordinal: statement,
                        name: call.selector.into(),
                    },
                );
            }
            if target.mode != DeclaredInstanceMethodModeV1::Instance {
                return Err(
                    DeclaredInstanceCallRelationIssueV1::TargetReceiverPolicyMismatch {
                        box_statement_ordinal: statement,
                        name: call.selector.into(),
                    },
                );
            }
            if target.parameter_count != call.arity {
                return Err(DeclaredInstanceCallRelationIssueV1::TargetArityMismatch {
                    name: call.selector.into(),
                    expected: target.parameter_count,
                    actual: call.arity,
                });
            }
            let Some((box_name, nominal_box_type)) = environment.source_declaration(statement)
            else {
                return Err(DeclaredInstanceCallRelationIssueV1::SourceCoverage);
            };
            if box_name != target.box_name {
                return Err(DeclaredInstanceCallRelationIssueV1::NominalSourceMismatch {
                    statement_ordinal: statement,
                    expected: box_name.into(),
                    actual: target.box_name.into(),
                });
            }
            relations.push(VerifiedDeclaredInstanceCallRelationV1 {
                caller_identity: call.caller_identity.clone(),
                caller_owner: call.caller_owner,
                call_site: call.call_site.clone(),
                receiver_site: call.receiver_site.clone(),
                receiver_binding: call.receiver_binding,
                target_identity: target.declaration_identity.clone(),
                target_method_identity: DeclaredInstanceMethodIdentityV1::from_resolver_source(
                    nominal_box_type.brand(),
                    nominal_box_type,
                    target.source_site,
                ),
                target_owner: target.owner,
                source_arity: call.arity,
            });
        }
        if relations.is_empty() {
            return Ok(DeclaredInstanceCallSourceDispositionV1::NoRootDeclaredInstanceCall);
        }
        let (_, first_type) = environment
            .source_declaration(first.source_site.box_statement_ordinal())
            .ok_or(DeclaredInstanceCallRelationIssueV1::SourceCoverage)?;
        Ok(DeclaredInstanceCallSourceDispositionV1::Published(
            VerifiedDeclaredInstanceCallRelationCatalogV1 {
                parser_provenance: first.parser_provenance.clone(),
                resolver_brand: first_type.brand(),
                rows: relations.into_boxed_slice(),
            },
        ))
    }
}

fn collect_method_inventory<'source>(
    methods: &[DeclaredInstanceMethodSourceRefV1<'source>],
) -> Result<BTreeMap<u32, Box<str>>, DeclaredInstanceCallRelationIssueV1> {
    let mut boxes = BTreeMap::<u32, Box<str>>::new();
    let mut visible_names = BTreeSet::<(u32, Box<str>)>::new();
    for method in methods {
        let statement = method.source_site.box_statement_ordinal();
        match boxes.entry(statement) {
            Entry::Vacant(entry) => {
                entry.insert(method.box_name.into());
            }
            Entry::Occupied(entry) if entry.get().as_ref() != method.box_name => {
                return Err(DeclaredInstanceCallRelationIssueV1::NominalSourceMismatch {
                    statement_ordinal: statement,
                    expected: entry.get().clone(),
                    actual: method.box_name.into(),
                });
            }
            Entry::Occupied(_) => {}
        }
        if !visible_names.insert((statement, method.method_name.into())) {
            return Err(
                DeclaredInstanceCallRelationIssueV1::DuplicateVisibleMethodName {
                    box_statement_ordinal: statement,
                    name: method.method_name.into(),
                },
            );
        }
    }
    Ok(boxes)
}
