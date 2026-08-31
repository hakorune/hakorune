//! Resolver-owned semantic effects for the declared-instance call relation.
//!
//! This module consumes two already-issued source products: the exact
//! call-site/declaration relation and the final parser syntax rows.  It does
//! not inspect an AST, infer from MIR/ABI, or issue a target.  Absence of a
//! callable contract is the explicit `OpaqueObservable` default; a carried
//! Query rune has precedence over that default.

use crate::parser::{
    CallableContractSourceDispositionV1, CallableContractSyntaxV1,
    FinalCallableSemanticSyntaxRowRefV1,
};

use super::{
    DeclaredInstanceCallSourceDispositionV1, DeclaredInstanceMethodIdentityV1, FunctionOwnerIdV1,
    SourceExprSiteV1,
};
use crate::parser::CallableDeclarationIdentityV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeclaredInstanceCallSemanticEffectV1 {
    OpaqueObservable,
    DeclaredQuery { rune_ordinal: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DeclaredInstanceCallEffectIssueV1 {
    RelationSyntaxCoverage {
        relation_rows: usize,
        syntax_rows: usize,
    },
    TargetSyntaxMissing,
    TargetSyntaxDuplicate,
    TargetSyntaxOutsideDeclaredInstance,
}

#[derive(Debug)]
pub(crate) struct VerifiedDeclaredInstanceCallEffectRowV1 {
    caller_identity: CallableDeclarationIdentityV1,
    caller_owner: FunctionOwnerIdV1,
    call_site: SourceExprSiteV1,
    target_identity: CallableDeclarationIdentityV1,
    target_method_identity: DeclaredInstanceMethodIdentityV1,
    effect: DeclaredInstanceCallSemanticEffectV1,
}

impl VerifiedDeclaredInstanceCallEffectRowV1 {
    pub(crate) fn caller_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.caller_identity
    }

    pub(crate) const fn caller_owner(&self) -> FunctionOwnerIdV1 {
        self.caller_owner
    }

    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(crate) fn target_identity(&self) -> &CallableDeclarationIdentityV1 {
        &self.target_identity
    }

    pub(crate) const fn target_method_identity(&self) -> DeclaredInstanceMethodIdentityV1 {
        self.target_method_identity
    }

    pub(crate) const fn effect(&self) -> DeclaredInstanceCallSemanticEffectV1 {
        self.effect
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedDeclaredInstanceCallEffectCatalogV1 {
    rows: Box<[VerifiedDeclaredInstanceCallEffectRowV1]>,
}

impl VerifiedDeclaredInstanceCallEffectCatalogV1 {
    pub(crate) fn rows(&self) -> &[VerifiedDeclaredInstanceCallEffectRowV1] {
        &self.rows
    }

    pub(crate) const fn len(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Debug)]
pub(crate) enum DeclaredInstanceCallEffectSourceDispositionV1 {
    NoRootDeclaredInstanceCall,
    Published(VerifiedDeclaredInstanceCallEffectCatalogV1),
}

pub(crate) struct DeclaredInstanceCallEffectIssuerV1;

impl DeclaredInstanceCallEffectIssuerV1 {
    pub(crate) fn issue(
        relation: &DeclaredInstanceCallSourceDispositionV1,
        syntax_rows: &[FinalCallableSemanticSyntaxRowRefV1<'_>],
    ) -> Result<DeclaredInstanceCallEffectSourceDispositionV1, DeclaredInstanceCallEffectIssueV1>
    {
        let DeclaredInstanceCallSourceDispositionV1::Published(relation) = relation else {
            return Ok(DeclaredInstanceCallEffectSourceDispositionV1::NoRootDeclaredInstanceCall);
        };
        if relation.len() > syntax_rows.len() {
            return Err(DeclaredInstanceCallEffectIssueV1::RelationSyntaxCoverage {
                relation_rows: relation.len(),
                syntax_rows: syntax_rows.len(),
            });
        }

        let mut rows = Vec::with_capacity(relation.len());
        for relation_row in relation.rows() {
            let mut matches = syntax_rows
                .iter()
                .filter(|syntax| syntax.identity().same_as(relation_row.target_identity()));
            let Some(target_syntax) = matches.next() else {
                return Err(DeclaredInstanceCallEffectIssueV1::TargetSyntaxMissing);
            };
            if matches.next().is_some() {
                return Err(DeclaredInstanceCallEffectIssueV1::TargetSyntaxDuplicate);
            }
            if !matches!(
                target_syntax.callable_contract_source(),
                CallableContractSourceDispositionV1::DirectDeclaredInstanceMethod { .. }
            ) {
                return Err(DeclaredInstanceCallEffectIssueV1::TargetSyntaxOutsideDeclaredInstance);
            }
            let effect = match target_syntax.callable_contract_source() {
                CallableContractSourceDispositionV1::DirectDeclaredInstanceMethod { syntax } => {
                    match syntax {
                        None => DeclaredInstanceCallSemanticEffectV1::OpaqueObservable,
                        Some(CallableContractSyntaxV1::Query { source_site }) => {
                            DeclaredInstanceCallSemanticEffectV1::DeclaredQuery {
                                rune_ordinal: source_site.rune_ordinal(),
                            }
                        }
                    }
                }
                CallableContractSourceDispositionV1::OutsideDirectDeclaredInstanceMethod => {
                    return Err(
                        DeclaredInstanceCallEffectIssueV1::TargetSyntaxOutsideDeclaredInstance,
                    )
                }
            };
            rows.push(VerifiedDeclaredInstanceCallEffectRowV1 {
                caller_identity: relation_row.caller_identity().clone(),
                caller_owner: relation_row.caller_owner(),
                call_site: relation_row.call_site().clone(),
                target_identity: relation_row.target_identity().clone(),
                target_method_identity: relation_row.target_method_identity(),
                effect,
            });
        }
        Ok(DeclaredInstanceCallEffectSourceDispositionV1::Published(
            VerifiedDeclaredInstanceCallEffectCatalogV1 {
                rows: rows.into_boxed_slice(),
            },
        ))
    }
}
