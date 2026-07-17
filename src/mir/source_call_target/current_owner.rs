use crate::ast::ASTNode;
use crate::mir::builder::SameModuleCallableNamespaceV1;

use super::{
    CurrentOwnerStaticCallTargetErrorV1, CurrentOwnerStaticReceiverV1,
    VerifiedCurrentOwnerStaticCallTargetV1, VerifiedSourceMethodCallSiteV1,
    VerifiedSourceStaticCallTargetCatalogV1, VerifiedSourceStaticCallTargetV1,
};

impl<'catalog> VerifiedSourceStaticCallTargetCatalogV1<'catalog> {
    pub(crate) fn extend_current_owner<'site>(
        mut self,
        calls: impl IntoIterator<Item = &'site VerifiedSourceMethodCallSiteV1<'catalog>>,
    ) -> Result<Self, CurrentOwnerStaticCallTargetErrorV1>
    where
        'catalog: 'site,
    {
        let declarations = self.declarations;
        let mut calls = calls.into_iter().collect::<Vec<_>>();
        calls.sort_by(|left, right| {
            (left.caller(), left.site()).cmp(&(right.caller(), right.site()))
        });

        for call in calls {
            if !std::ptr::eq(call.catalog(), declarations) {
                return Err(CurrentOwnerStaticCallTargetErrorV1::CallCatalogMismatch {
                    caller: call.caller().clone(),
                    site: call.site().clone(),
                });
            }
            if !matches!(call.receiver(), ASTNode::Me { .. }) {
                return Err(
                    CurrentOwnerStaticCallTargetErrorV1::CanonicalMeReceiverRequired {
                        caller: call.caller().clone(),
                        site: call.site().clone(),
                    },
                );
            }
            let caller = call.declaration();
            if caller.key().namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod {
                return Err(
                    CurrentOwnerStaticCallTargetErrorV1::CallerNotStaticBoxMethod {
                        caller: call.caller().clone(),
                    },
                );
            }

            let row_key = (call.caller().clone(), call.site().clone());
            if self.rows.contains_key(&row_key) {
                return Err(CurrentOwnerStaticCallTargetErrorV1::DuplicateCallSite {
                    caller: call.caller().clone(),
                    site: call.site().clone(),
                });
            }

            let owner = caller.key().owner();
            let Some(target) = declarations.declaration_for(
                SameModuleCallableNamespaceV1::StaticBoxMethod,
                owner,
                call.method(),
                call.arity() as usize,
            ) else {
                return Err(CurrentOwnerStaticCallTargetErrorV1::TargetOutsideCatalog {
                    owner: owner.into(),
                    method: call.method().into(),
                    arity: call.arity(),
                });
            };

            self.rows.insert(
                row_key,
                VerifiedSourceStaticCallTargetV1::CurrentOwnerStatic(
                    VerifiedCurrentOwnerStaticCallTargetV1::new(
                        CurrentOwnerStaticReceiverV1::CanonicalMe,
                        target.key().clone(),
                    ),
                ),
            );
        }
        Ok(self)
    }
}
