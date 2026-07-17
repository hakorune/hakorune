use crate::ast::ASTNode;
use crate::mir::builder::{
    SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;

use super::{
    CurrentOwnerStaticCallCandidateV1, CurrentOwnerStaticCallTargetErrorV1,
    CurrentOwnerStaticReceiverV1, VerifiedCurrentOwnerStaticCallTargetV1,
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedSourceStaticCallTargetV1,
};

impl CurrentOwnerStaticCallCandidateV1 {
    pub(crate) fn from_method_call(
        caller: crate::mir::builder::CanonicalSameModuleCallableKeyV1,
        site: SourceExprSiteV1,
        expression: &ASTNode,
    ) -> Result<Self, CurrentOwnerStaticCallTargetErrorV1> {
        let ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } = expression
        else {
            return Err(CurrentOwnerStaticCallTargetErrorV1::SourceMethodCallRequired);
        };
        if !matches!(object.as_ref(), ASTNode::Me { .. }) {
            return Err(CurrentOwnerStaticCallTargetErrorV1::CanonicalMeReceiverRequired);
        }
        if method.is_empty() {
            return Err(CurrentOwnerStaticCallTargetErrorV1::EmptyMethod);
        }
        let arity = checked_explicit_arity(method, arguments.len())?;
        Ok(Self {
            caller,
            site,
            receiver: CurrentOwnerStaticReceiverV1::CanonicalMe,
            method: method.clone().into_boxed_str(),
            arity,
        })
    }
}

pub(super) fn checked_explicit_arity(
    method: &str,
    arity: usize,
) -> Result<u32, CurrentOwnerStaticCallTargetErrorV1> {
    u32::try_from(arity).map_err(|_| CurrentOwnerStaticCallTargetErrorV1::ArityOverflow {
        method: method.into(),
    })
}

impl VerifiedSourceStaticCallTargetCatalogV1 {
    pub(crate) fn extend_current_owner(
        mut self,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
        candidates: impl IntoIterator<Item = CurrentOwnerStaticCallCandidateV1>,
    ) -> Result<Self, CurrentOwnerStaticCallTargetErrorV1> {
        let mut candidates = candidates.into_iter().collect::<Vec<_>>();
        candidates.sort_by(|left, right| {
            (left.caller(), left.site()).cmp(&(right.caller(), right.site()))
        });

        for candidate in candidates {
            let Some(caller) = declarations.declaration(candidate.caller()) else {
                return Err(CurrentOwnerStaticCallTargetErrorV1::CallerOutsideCatalog {
                    caller: candidate.caller().clone(),
                });
            };
            if caller.key().namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod {
                return Err(
                    CurrentOwnerStaticCallTargetErrorV1::CallerNotStaticBoxMethod {
                        caller: candidate.caller().clone(),
                    },
                );
            }

            let row_key = (candidate.caller().clone(), candidate.site().clone());
            if self.rows.contains_key(&row_key) {
                return Err(CurrentOwnerStaticCallTargetErrorV1::DuplicateCallSite {
                    caller: candidate.caller().clone(),
                    site: candidate.site().clone(),
                });
            }

            let owner = caller.key().owner();
            let Some(target) = declarations.declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                owner,
                candidate.method(),
                candidate.arity() as usize,
            ) else {
                return Err(CurrentOwnerStaticCallTargetErrorV1::TargetOutsideCatalog {
                    owner: owner.into(),
                    method: candidate.method().into(),
                    arity: candidate.arity(),
                });
            };

            self.rows.insert(
                row_key,
                VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(
                    VerifiedCurrentOwnerStaticCallTargetV1::new(
                        candidate.receiver(),
                        target.key().clone(),
                    ),
                ),
            );
        }
        Ok(self)
    }
}
