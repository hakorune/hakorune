//! Exact lexical dispositions for pre-verified qualified MethodCall receivers.

use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationV1,
};
use crate::mir::resolved_semantics::{
    observe_qualified_receiver_shadow_view_v0, FunctionSyntaxViewV1, ReceiverPolicyV1,
    ShadowMethodCallReceiverV0, ShadowQualifiedReceiverDispositionV0, SourceExprSiteV1,
};

use super::{QualifiedReceiverLexicalDispositionErrorV1, VerifiedSourceMethodCallSiteV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QualifiedReceiverLexicalDispositionV1 {
    Bound,
    ProvenUnbound,
}

/// One caller-local, exact-coverage lexical observation product.
///
/// The product retains the catalog declaration identity by reference so rows
/// cannot be paired with an equal key/site from a different catalog. Shadow
/// binding ordinals never escape the traversal.
#[derive(Debug)]
pub(crate) struct VerifiedQualifiedReceiverLexicalDispositionsV1<'catalog> {
    caller: &'catalog CanonicalSameModuleCallableKeyV1,
    declaration: &'catalog VerifiedSameModuleCallableDeclarationV1,
    rows: BTreeMap<SourceExprSiteV1, QualifiedReceiverLexicalDispositionV1>,
}

impl<'catalog> VerifiedQualifiedReceiverLexicalDispositionsV1<'catalog> {
    /// Seals the existing lexical vocabulary from one complete MethodCall
    /// observation. This adapter performs no second source traversal.
    pub(crate) fn seal_from_complete_inventory<'rows>(
        caller: &'catalog CanonicalSameModuleCallableKeyV1,
        declaration: &'catalog VerifiedSameModuleCallableDeclarationV1,
        rows: impl IntoIterator<
            Item = (
                &'rows VerifiedSourceMethodCallSiteV1<'catalog>,
                ShadowMethodCallReceiverV0,
            ),
        >,
    ) -> Result<Self, QualifiedReceiverLexicalDispositionErrorV1>
    where
        'catalog: 'rows,
    {
        let mut dispositions = BTreeMap::new();
        let mut observed = 0usize;
        for (call, receiver) in rows {
            observed += 1;
            if call.caller() != caller {
                return Err(QualifiedReceiverLexicalDispositionErrorV1::MixedCaller {
                    expected: caller.clone(),
                    actual: call.caller().clone(),
                });
            }
            if !std::ptr::eq(call.declaration(), declaration) {
                return Err(
                    QualifiedReceiverLexicalDispositionErrorV1::MixedCallerDeclaration {
                        caller: caller.clone(),
                        site: call.site().clone(),
                    },
                );
            }
            let disposition = match receiver {
                ShadowMethodCallReceiverV0::Qualified(
                    ShadowQualifiedReceiverDispositionV0::Bound,
                ) => QualifiedReceiverLexicalDispositionV1::Bound,
                ShadowMethodCallReceiverV0::Qualified(
                    ShadowQualifiedReceiverDispositionV0::ProvenUnbound,
                ) => QualifiedReceiverLexicalDispositionV1::ProvenUnbound,
                ShadowMethodCallReceiverV0::CurrentOwner
                | ShadowMethodCallReceiverV0::Dynamic => {
                    return Err(
                        QualifiedReceiverLexicalDispositionErrorV1::QualifiedReceiverVariableRequired {
                            caller: caller.clone(),
                            site: call.receiver_site().clone(),
                        },
                    )
                }
            };
            if dispositions
                .insert(call.receiver_site().clone(), disposition)
                .is_some()
            {
                return Err(
                    QualifiedReceiverLexicalDispositionErrorV1::DuplicateReceiverSite {
                        caller: caller.clone(),
                        site: call.receiver_site().clone(),
                    },
                );
            }
        }
        if observed == 0 {
            return Err(QualifiedReceiverLexicalDispositionErrorV1::EmptyRequestSet);
        }
        Ok(Self {
            caller,
            declaration,
            rows: dispositions,
        })
    }

    pub(crate) fn verify(
        call_sites: &[&VerifiedSourceMethodCallSiteV1<'catalog>],
    ) -> Result<Self, QualifiedReceiverLexicalDispositionErrorV1> {
        let Some(first) = call_sites.first() else {
            return Err(QualifiedReceiverLexicalDispositionErrorV1::EmptyRequestSet);
        };
        let caller = first.caller();
        let declaration = first.declaration();
        let mut requested = BTreeSet::new();

        for call in call_sites {
            if call.caller() != caller {
                return Err(QualifiedReceiverLexicalDispositionErrorV1::MixedCaller {
                    expected: caller.clone(),
                    actual: call.caller().clone(),
                });
            }
            if !std::ptr::eq(call.declaration(), declaration) {
                return Err(
                    QualifiedReceiverLexicalDispositionErrorV1::MixedCallerDeclaration {
                        caller: caller.clone(),
                        site: call.site().clone(),
                    },
                );
            }
            if !matches!(call.receiver(), ASTNode::Variable { .. }) {
                return Err(
                    QualifiedReceiverLexicalDispositionErrorV1::QualifiedReceiverVariableRequired {
                        caller: caller.clone(),
                        site: call.receiver_site().clone(),
                    },
                );
            }
            if !requested.insert(call.receiver_site().clone()) {
                return Err(
                    QualifiedReceiverLexicalDispositionErrorV1::DuplicateReceiverSite {
                        caller: caller.clone(),
                        site: call.receiver_site().clone(),
                    },
                );
            }
        }

        let receiver_policy = match caller.namespace() {
            SameModuleCallableNamespaceV1::StaticBoxMethod => ReceiverPolicyV1::Absent,
            SameModuleCallableNamespaceV1::InstanceBoxMethod => ReceiverPolicyV1::DeclaredInstance,
        };
        let view = FunctionSyntaxViewV1::from_borrowed_function_parts(
            declaration.params(),
            declaration.body(),
            receiver_policy,
        );
        let observed = observe_qualified_receiver_shadow_view_v0(view, requested)
            .map_err(QualifiedReceiverLexicalDispositionErrorV1::ShadowTraversal)?;
        let rows = observed
            .into_iter()
            .map(|(site, disposition)| {
                let disposition = match disposition {
                    ShadowQualifiedReceiverDispositionV0::Bound => {
                        QualifiedReceiverLexicalDispositionV1::Bound
                    }
                    ShadowQualifiedReceiverDispositionV0::ProvenUnbound => {
                        QualifiedReceiverLexicalDispositionV1::ProvenUnbound
                    }
                };
                (site, disposition)
            })
            .collect();

        Ok(Self {
            caller,
            declaration,
            rows,
        })
    }

    pub(crate) const fn caller(&self) -> &'catalog CanonicalSameModuleCallableKeyV1 {
        self.caller
    }

    pub(crate) fn disposition_for(
        &self,
        call: &VerifiedSourceMethodCallSiteV1<'catalog>,
    ) -> Option<QualifiedReceiverLexicalDispositionV1> {
        if call.caller() != self.caller || !std::ptr::eq(call.declaration(), self.declaration) {
            return None;
        }
        self.rows.get(call.receiver_site()).copied()
    }

    pub(crate) fn rows(
        &self,
    ) -> impl Iterator<Item = (&SourceExprSiteV1, QualifiedReceiverLexicalDispositionV1)> {
        self.rows
            .iter()
            .map(|(site, disposition)| (site, *disposition))
    }
}
