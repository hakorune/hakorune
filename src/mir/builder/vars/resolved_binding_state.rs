//! Builder-side veto gate for one installed canonical BindingId authority.
//!
//! Exact declaration/use/assignment ownership lives in
//! `builder::resolved_lowering`. This small state exists only so every legacy
//! BindingId allocator has one structural fail-fast boundary while a resolved
//! owner is installed.

use crate::mir::resolved_semantics::{FunctionOwnerIdV1, VerifiedResolvedFunctionV1};

#[derive(Debug, Default)]
pub(in crate::mir) struct ResolvedBindingLoweringStateV1 {
    installed_owner: Option<FunctionOwnerIdV1>,
    completion_verified: bool,
}

impl ResolvedBindingLoweringStateV1 {
    pub(in crate::mir::builder) fn install(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
    ) -> Result<(), String> {
        if self.installed_owner.is_some() || self.completion_verified {
            return Err(
                "[freeze:contract][resolved_binding_authority/install_nonempty]".to_string(),
            );
        }
        self.installed_owner = Some(product.owner());
        Ok(())
    }

    pub(in crate::mir::builder) fn finish(
        &mut self,
        owner: FunctionOwnerIdV1,
    ) -> Result<(), String> {
        if self.installed_owner != Some(owner) {
            return Err(format!(
                "[freeze:contract][resolved_binding_authority/finish_owner_mismatch] expected={:?} actual={owner:?}",
                self.installed_owner
            ));
        }
        if self.completion_verified {
            return Err(
                "[freeze:contract][resolved_binding_authority/finish_duplicate]".to_string(),
            );
        }
        self.completion_verified = true;
        Ok(())
    }

    pub(in crate::mir::builder) fn veto_legacy_allocation(&self) -> Result<(), String> {
        if let Some(owner) = self.installed_owner {
            return Err(format!(
                "[freeze:contract][resolved_binding_authority/legacy_allocation_forbidden] owner={owner:?}"
            ));
        }
        Ok(())
    }

    pub(in crate::mir::builder) const fn session_success_is_closed(
        &self,
        requires_resolved_authority: bool,
    ) -> bool {
        if requires_resolved_authority {
            self.installed_owner.is_some() && self.completion_verified
        } else {
            self.installed_owner.is_none() && !self.completion_verified
        }
    }

    pub(in crate::mir::builder) fn is_installed_for(&self, owner: FunctionOwnerIdV1) -> bool {
        self.installed_owner == Some(owner) && !self.completion_verified
    }

    #[cfg(test)]
    pub(in crate::mir::builder) const fn is_installed(&self) -> bool {
        self.installed_owner.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, DeclarationAttrs, Span};
    use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1};

    use super::*;

    fn product() -> std::sync::Arc<VerifiedResolvedFunctionV1> {
        let function = ASTNode::FunctionDeclaration {
            name: "gate_fixture".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        };
        let view = FunctionSyntaxViewV1::from_ast(&function).unwrap();
        FunctionSemanticResolverSessionV1::new(0)
            .unwrap()
            .resolve(view)
            .unwrap()
    }

    #[test]
    fn installed_authority_vetoes_legacy_allocation_until_cleanup() {
        let mut state = ResolvedBindingLoweringStateV1::default();
        let product = product();
        let owner = product.owner();
        state.install(&product).unwrap();
        assert!(state.veto_legacy_allocation().is_err());
        assert!(!state.session_success_is_closed(true));
        state.finish(owner).unwrap();
        assert!(state.veto_legacy_allocation().is_err());
        assert!(state.session_success_is_closed(true));
        assert!(!state.session_success_is_closed(false));
    }
}
