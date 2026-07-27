use crate::ast::ASTNode;
use crate::mir::builder::{SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationV1};
use crate::mir::source_call_target::VerifiedSourceMethodCallSiteV1;

use super::CurrentOwnerInstanceResultTargetErrorV1;

#[derive(Debug)]
pub(crate) struct VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog> {
    call: &'site VerifiedSourceMethodCallSiteV1<'catalog>,
    target: &'catalog VerifiedSameModuleCallableDeclarationV1,
}

impl<'site, 'catalog> VerifiedCurrentOwnerInstanceResultTargetV1<'site, 'catalog> {
    pub(crate) fn seal(
        call: &'site VerifiedSourceMethodCallSiteV1<'catalog>,
    ) -> Result<Self, CurrentOwnerInstanceResultTargetErrorV1> {
        if !matches!(call.receiver(), ASTNode::Me { .. }) {
            return Err(
                CurrentOwnerInstanceResultTargetErrorV1::CanonicalMeReceiverRequired {
                    caller: call.caller().clone(),
                    site: call.site().clone(),
                },
            );
        }
        let caller = call.declaration();
        if caller.key().namespace() != SameModuleCallableNamespaceV1::InstanceBoxMethod {
            return Err(
                CurrentOwnerInstanceResultTargetErrorV1::CallerNotInstanceBoxMethod {
                    caller: call.caller().clone(),
                },
            );
        }

        let owner = caller.key().owner();
        let Some(target) = call.catalog().declaration_for(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
            owner,
            call.method(),
            call.arity() as usize,
        ) else {
            return Err(
                CurrentOwnerInstanceResultTargetErrorV1::TargetOutsideCatalog {
                    owner: owner.into(),
                    method: call.method().into(),
                    arity: call.arity(),
                },
            );
        };
        Ok(Self { call, target })
    }

    pub(crate) const fn call(&self) -> &'site VerifiedSourceMethodCallSiteV1<'catalog> {
        self.call
    }

    pub(crate) const fn target(&self) -> &'catalog VerifiedSameModuleCallableDeclarationV1 {
        self.target
    }
}
