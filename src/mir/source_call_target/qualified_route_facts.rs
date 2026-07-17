//! Exact qualified-call route facts over S0, L0, and a catalog-branded alias view.

use crate::ast::ASTNode;
use crate::mir::policies::source_method_reserved_route::{
    classify_source_method_reserved_route_v1, SourceMethodReservedRouteContextV1,
    SourceMethodReservedRouteDecisionV1,
};
use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::{
    QualifiedCallRouteFactsErrorV1, QualifiedReceiverLexicalDispositionV1,
    VerifiedQualifiedReceiverLexicalDispositionsV1, VerifiedSourceMethodCallSiteV1,
    VerifiedStaticImportAliasViewV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QualifiedReceiverAdmissionV1 {
    ImportedAlias,
    DirectCanonicalOwner,
}

/// One lifetime-bound route co-seal. Target identity remains outside.
#[derive(Debug)]
pub(crate) struct VerifiedQualifiedCallRouteFactsV1<'facts, 'catalog> {
    call: &'facts VerifiedSourceMethodCallSiteV1<'catalog>,
    lexical: &'facts VerifiedQualifiedReceiverLexicalDispositionsV1<'catalog>,
    imports: &'facts VerifiedStaticImportAliasViewV1<'catalog>,
    lexical_disposition: QualifiedReceiverLexicalDispositionV1,
    admission: QualifiedReceiverAdmissionV1,
}

impl<'facts, 'catalog> VerifiedQualifiedCallRouteFactsV1<'facts, 'catalog> {
    pub(crate) fn verify(
        call: &'facts VerifiedSourceMethodCallSiteV1<'catalog>,
        lexical: &'facts VerifiedQualifiedReceiverLexicalDispositionsV1<'catalog>,
        imports: &'facts VerifiedStaticImportAliasViewV1<'catalog>,
    ) -> Result<Self, QualifiedCallRouteFactsErrorV1> {
        let lexical_disposition = lexical.disposition_for(call).ok_or_else(|| {
            QualifiedCallRouteFactsErrorV1::LexicalDispositionUnavailable {
                caller: call.caller().clone(),
                receiver_site: call.receiver_site().clone(),
            }
        })?;
        if !imports.matches_declaration(call.declaration()) {
            return Err(QualifiedCallRouteFactsErrorV1::ImportCatalogMismatch {
                caller: call.caller().clone(),
            });
        }

        let context = if call.site().node().segments().iter().any(|segment| {
            matches!(
                segment,
                SourcePathSegmentV1::FastMemBodyRoot | SourcePathSegmentV1::FastMemBody(_)
            )
        }) {
            SourceMethodReservedRouteContextV1::FastMemBody
        } else {
            SourceMethodReservedRouteContextV1::Ordinary
        };
        let decision = classify_source_method_reserved_route_v1(
            context,
            call.receiver(),
            call.method(),
            call.arguments(),
        );
        match decision {
            SourceMethodReservedRouteDecisionV1::Ordinary => {}
            SourceMethodReservedRouteDecisionV1::ReservedFail(reason) => {
                return Err(QualifiedCallRouteFactsErrorV1::ReservedRouteRejected {
                    caller: call.caller().clone(),
                    site: call.site().clone(),
                    reason,
                })
            }
            selected => {
                return Err(QualifiedCallRouteFactsErrorV1::ReservedRouteSelected {
                    caller: call.caller().clone(),
                    site: call.site().clone(),
                    disposition: selected.disposition(),
                })
            }
        }

        let receiver = variable_receiver(call);
        let admission = if imports.canonical_owner(receiver).is_some() {
            QualifiedReceiverAdmissionV1::ImportedAlias
        } else {
            if lexical_disposition == QualifiedReceiverLexicalDispositionV1::Bound {
                return Err(
                    QualifiedCallRouteFactsErrorV1::DirectReceiverLexicallyBound {
                        caller: call.caller().clone(),
                        site: call.site().clone(),
                        receiver: receiver.into(),
                    },
                );
            }
            QualifiedReceiverAdmissionV1::DirectCanonicalOwner
        };

        Ok(Self {
            call,
            lexical,
            imports,
            lexical_disposition,
            admission,
        })
    }

    pub(crate) const fn call(&self) -> &VerifiedSourceMethodCallSiteV1<'catalog> {
        self.call
    }

    pub(crate) const fn lexical_disposition(&self) -> QualifiedReceiverLexicalDispositionV1 {
        self.lexical_disposition
    }

    pub(crate) const fn admission(&self) -> QualifiedReceiverAdmissionV1 {
        self.admission
    }

    pub(crate) fn canonical_owner(&self) -> &str {
        let receiver = variable_receiver(self.call);
        match self.admission {
            QualifiedReceiverAdmissionV1::ImportedAlias => self
                .imports
                .canonical_owner(receiver)
                .expect("verified imported alias must remain in immutable view"),
            QualifiedReceiverAdmissionV1::DirectCanonicalOwner => receiver,
        }
    }

    pub(crate) const fn lexical_owner(
        &self,
    ) -> &VerifiedQualifiedReceiverLexicalDispositionsV1<'catalog> {
        self.lexical
    }
}

fn variable_receiver<'catalog>(call: &VerifiedSourceMethodCallSiteV1<'catalog>) -> &'catalog str {
    let ASTNode::Variable { name, .. } = call.receiver() else {
        unreachable!("L0 exact qualified receiver invariant")
    };
    name
}
