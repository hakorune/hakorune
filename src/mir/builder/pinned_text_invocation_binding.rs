//! Session-owned physical identity for the pinned-Text ingress.
//!
//! The module invocation brand and the compile-target capability have
//! different issuers and meanings.  This module relates them only through a
//! live `ModuleBuilderInvocationSessionV1`; it never compares their numeric
//! ordinals or exposes a detached target/brand pair.

use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::compiler::target_capability::PinnedTextCompileTargetCapabilityV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PinnedTextInvocationBindingIssueV1 {
    AlreadyInstalled,
    MissingTarget,
}

/// A non-copyable, callback-scoped relation between one module session and
/// its explicitly issued pinned-Text compile target.
#[must_use = "a pinned-Text invocation binding must stay within its callback"]
#[derive(Debug)]
pub(in crate::mir) struct PinnedTextCompileInvocationBindingRefV1<'session> {
    pub(in crate::mir) brand: ModuleInvocationBrandV1,
    pub(in crate::mir) target: &'session PinnedTextCompileTargetCapabilityV1,
}

impl<'session> PinnedTextCompileInvocationBindingRefV1<'session> {
    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir::builder) const fn target_capability(
        &self,
    ) -> &'session PinnedTextCompileTargetCapabilityV1 {
        self.target
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_invocation_session::ModuleBuilderInvocationSessionV1;
    use crate::mir::builder::{BuilderInvocationConfigV1, MirBuilder};
    use crate::mir::compiler::target_capability::{
        PinnedTextCompileTargetCapabilityIssuerV1, PinnedTextCompileTargetProfileV1,
    };

    #[test]
    fn targetless_session_rejects_selected_binding_and_duplicate_install() {
        let live = MirBuilder::new();
        let config = BuilderInvocationConfigV1::snapshot_for_canonical(&live, None);
        let mut session = ModuleBuilderInvocationSessionV1::open(&live, config);
        assert_eq!(
            session
                .with_pinned_text_invocation_binding(|_| ())
                .unwrap_err(),
            PinnedTextInvocationBindingIssueV1::MissingTarget
        );

        let profile = PinnedTextCompileTargetProfileV1::NyRtTextResidencePtr64As0V1;
        let target = PinnedTextCompileTargetCapabilityIssuerV1::issue(profile).unwrap();
        session
            .install_pinned_text_target_capability(Some(target))
            .expect("first target install");
        let second = PinnedTextCompileTargetCapabilityIssuerV1::issue(profile).unwrap();
        assert_eq!(
            session.install_pinned_text_target_capability(Some(second)),
            Err(PinnedTextInvocationBindingIssueV1::AlreadyInstalled)
        );
        session.with_builder_and_pinned_text_invocation_binding(|_, binding| {
            let binding = binding.expect("installed target binding");
            assert_eq!(binding.brand(), ModuleInvocationBrandV1::legacy_test());
            assert_eq!(binding.target_capability().profile(), profile);
        });
    }
}
