use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::{
    observe_method_calls_shadow_view_v0, FunctionSyntaxViewV1, ReceiverPolicyV1,
    ShadowMethodCallReceiverV0, SourceExprSiteV1,
};

use super::{
    QualifiedCallRouteFactsErrorV1, VerifiedQualifiedCallRouteFactsV1,
    VerifiedQualifiedReceiverLexicalDispositionsV1, VerifiedSourceMethodCallSiteV1,
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedSourceStaticCallTargetV1,
    VerifiedStaticImportAliasViewV1, WholeSourceStaticCallTargetInventoryErrorV1,
};

/// Complete source MethodCall inventory plus its exact static-target subset.
///
/// Every row borrows one declaration-catalog allocation. Missing target rows
/// mean a completely observed non-static call, never a missing traversal.
#[derive(Debug)]
pub(crate) struct VerifiedWholeSourceMethodCallSiteV1<'catalog> {
    call: VerifiedSourceMethodCallSiteV1<'catalog>,
    receiver: ShadowMethodCallReceiverV0,
}

impl<'catalog> VerifiedWholeSourceMethodCallSiteV1<'catalog> {
    pub(crate) const fn call(&self) -> &VerifiedSourceMethodCallSiteV1<'catalog> {
        &self.call
    }

    pub(crate) const fn receiver(&self) -> ShadowMethodCallReceiverV0 {
        self.receiver
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedWholeSourceStaticCallTargetInventoryV1<'catalog> {
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    observed_callers: Box<[CanonicalSameModuleCallableKeyV1]>,
    calls: BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        VerifiedWholeSourceMethodCallSiteV1<'catalog>,
    >,
    targets: VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
}

impl<'catalog> VerifiedWholeSourceStaticCallTargetInventoryV1<'catalog> {
    pub(crate) fn verify(
        declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
        imports: &VerifiedStaticImportAliasViewV1<'catalog>,
    ) -> Result<Self, WholeSourceStaticCallTargetInventoryErrorV1> {
        if !imports.is_branded_by(declarations) {
            return Err(WholeSourceStaticCallTargetInventoryErrorV1::ImportCatalogMismatch);
        }

        let (observed_callers, calls) = observe_all_calls(declarations)?;
        let targets = seal_static_targets(declarations, imports, &calls)?;
        Ok(Self {
            declarations,
            observed_callers,
            calls,
            targets,
        })
    }

    pub(crate) fn is_branded_by(
        &self,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> bool {
        std::ptr::eq(self.declarations, declarations) && self.targets.is_branded_by(declarations)
    }

    pub(crate) fn call(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedSourceMethodCallSiteV1<'catalog>> {
        self.calls
            .get(&(caller.clone(), site.clone()))
            .map(VerifiedWholeSourceMethodCallSiteV1::call)
    }

    pub(crate) fn calls(
        &self,
    ) -> impl Iterator<Item = &VerifiedWholeSourceMethodCallSiteV1<'catalog>> {
        self.calls.values()
    }

    pub(crate) const fn declarations(
        &self,
    ) -> &'catalog VerifiedSameModuleCallableDeclarationCatalogV1 {
        self.declarations
    }

    pub(crate) fn observed_callers(
        &self,
    ) -> impl Iterator<Item = &CanonicalSameModuleCallableKeyV1> {
        self.observed_callers.iter()
    }

    pub(crate) const fn targets(&self) -> &VerifiedSourceStaticCallTargetCatalogV1<'catalog> {
        &self.targets
    }

    pub(crate) fn target(
        &self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
    ) -> Option<&VerifiedSourceStaticCallTargetV1> {
        self.targets.target(caller, site)
    }

    pub(crate) fn len(&self) -> usize {
        self.calls.len()
    }

    pub(crate) const fn observed_declaration_count(&self) -> usize {
        self.observed_callers.len()
    }

    pub(crate) fn target_len(&self) -> usize {
        self.targets.len()
    }

    pub(crate) fn noncandidate_len(&self) -> usize {
        self.len().saturating_sub(self.target_len())
    }
}

fn observe_all_calls<'catalog>(
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
) -> Result<
    (
        Box<[CanonicalSameModuleCallableKeyV1]>,
        BTreeMap<
            (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
            VerifiedWholeSourceMethodCallSiteV1<'catalog>,
        >,
    ),
    WholeSourceStaticCallTargetInventoryErrorV1,
> {
    let mut observed_callers = Vec::with_capacity(declarations.len());
    let mut calls = BTreeMap::new();
    for (caller, declaration) in declarations.declarations() {
        observed_callers.push(caller.clone());
        let receiver_policy = match caller.namespace() {
            SameModuleCallableNamespaceV1::StaticBoxMethod => ReceiverPolicyV1::Absent,
            SameModuleCallableNamespaceV1::InstanceBoxMethod => ReceiverPolicyV1::DeclaredInstance,
        };
        let view = FunctionSyntaxViewV1::from_borrowed_function_parts(
            declaration.params(),
            declaration.body(),
            receiver_policy,
        );
        let observed = observe_method_calls_shadow_view_v0(view).map_err(|cause| {
            WholeSourceStaticCallTargetInventoryErrorV1::MethodCallObservation {
                caller: caller.clone(),
                cause,
            }
        })?;
        for (site, observation) in observed {
            let call = VerifiedSourceMethodCallSiteV1::verify(declarations, caller, site.clone())
                .map_err(WholeSourceStaticCallTargetInventoryErrorV1::MethodCallSite)?;
            if observation.receiver_site() != call.receiver_site() {
                return Err(
                    WholeSourceStaticCallTargetInventoryErrorV1::ObservationReceiverSiteMismatch {
                        caller: caller.clone(),
                        site,
                    },
                );
            }
            let key = (caller.clone(), site);
            let row = VerifiedWholeSourceMethodCallSiteV1 {
                call,
                receiver: observation.receiver(),
            };
            if calls.insert(key.clone(), row).is_some() {
                return Err(
                    WholeSourceStaticCallTargetInventoryErrorV1::DuplicateObservedCall {
                        caller: key.0,
                        site: key.1,
                    },
                );
            }
        }
    }
    Ok((observed_callers.into_boxed_slice(), calls))
}

fn seal_static_targets<'catalog>(
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    imports: &VerifiedStaticImportAliasViewV1<'catalog>,
    calls: &BTreeMap<
        (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1),
        VerifiedWholeSourceMethodCallSiteV1<'catalog>,
    >,
) -> Result<
    VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    WholeSourceStaticCallTargetInventoryErrorV1,
> {
    let qualified_callers = calls
        .values()
        .filter(|row| matches!(row.receiver(), ShadowMethodCallReceiverV0::Qualified(_)))
        .map(|row| row.call().caller().clone())
        .collect::<BTreeSet<_>>();
    let mut lexical = Vec::with_capacity(qualified_callers.len());
    for caller in qualified_callers {
        let Some(declaration) = calls
            .values()
            .find(|row| row.call().caller() == &caller)
            .map(|row| row.call().declaration())
        else {
            continue;
        };
        let canonical_caller = declaration.key();
        let caller_rows = calls
            .values()
            .filter(|row| {
                row.call().caller() == canonical_caller
                    && matches!(row.receiver(), ShadowMethodCallReceiverV0::Qualified(_))
            })
            .map(|row| (row.call(), row.receiver()))
            .collect::<Vec<_>>();
        lexical.push(
            VerifiedQualifiedReceiverLexicalDispositionsV1::seal_from_complete_inventory(
                canonical_caller,
                declaration,
                caller_rows,
            )
            .map_err(WholeSourceStaticCallTargetInventoryErrorV1::QualifiedLexical)?,
        );
    }

    let mut facts = Vec::new();
    for row in calls
        .values()
        .filter(|row| matches!(row.receiver(), ShadowMethodCallReceiverV0::Qualified(_)))
    {
        let call = row.call();
        let lexical = lexical
            .iter()
            .find(|rows| rows.caller() == call.caller())
            .expect("one lexical product per inventoried variable-call caller");
        let ASTNode::Variable { name, .. } = call.receiver() else {
            unreachable!("variable-call filter");
        };
        let canonical_owner = imports.canonical_owner(name).unwrap_or(name);
        if declarations
            .declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                canonical_owner,
                call.method(),
                call.arity() as usize,
            )
            .is_none()
        {
            continue;
        }
        match VerifiedQualifiedCallRouteFactsV1::verify(call, lexical, imports) {
            Ok(row) => facts.push(row),
            Err(
                QualifiedCallRouteFactsErrorV1::ReservedRouteSelected { .. }
                | QualifiedCallRouteFactsErrorV1::ReservedRouteRejected { .. }
                | QualifiedCallRouteFactsErrorV1::DirectReceiverLexicallyBound { .. },
            ) => {}
            Err(error) => {
                return Err(WholeSourceStaticCallTargetInventoryErrorV1::QualifiedRoute(
                    error,
                ))
            }
        }
    }

    let mut targets = VerifiedSourceStaticCallTargetCatalogV1::seal_qualified(imports, facts)
        .map_err(WholeSourceStaticCallTargetInventoryErrorV1::QualifiedTarget)?;
    let current_owner_calls = calls.values().filter_map(|row| {
        let call = row.call();
        if row.receiver() != ShadowMethodCallReceiverV0::CurrentOwner
            || call.caller().namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod
        {
            return None;
        }
        declarations
            .declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                call.caller().owner(),
                call.method(),
                call.arity() as usize,
            )
            .map(|_| call)
    });
    targets = targets
        .extend_current_owner(current_owner_calls)
        .map_err(WholeSourceStaticCallTargetInventoryErrorV1::CurrentOwnerTarget)?;
    Ok(targets)
}
