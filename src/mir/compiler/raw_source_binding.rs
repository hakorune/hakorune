//! RAW-SOURCE0-BIND0: compiler-owned Raw source binding.
//!
//! This module is intentionally disconnected from public compilation ingress.
//! It seals one legacy source projection and Builder configuration before the
//! compiler issuer mints a Raw token.  It does not open a session, collector,
//! ledger, or publication route; RAW-SOURCE0-LOWER0 owns that later transition.

use super::lowering_input::{LegacyModuleLoweringInputV1, LegacyModuleOriginV1};
use super::raw_runtime_inputs::{RawRuntimeInputCaptureErrorV1, RawRuntimeInputSnapshotV1};
use super::source_bound_package::{InvocationIdentityIssuerV1, SourceBindingErrorV1};
use crate::ast::ASTNode;
use crate::mir::builder::{
    BuilderInvocationConfigV1, OwnedRawSourceV1, RawCallableMainCompatibilityDispositionV1,
    RawSourceOriginV1, RawSourceProjectionErrorV1,
};
use crate::mir::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1, ModuleInvocationTokenV1,
};
use crate::mir::module_invocation_policy::ModuleInvocationPolicyV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawCallableMainSelectionV1 {
    Omitted,
    Required,
}

#[derive(Debug)]
pub(in crate::mir) struct RawIngressRequestV1 {
    input: LegacyModuleLoweringInputV1,
    config: BuilderInvocationConfigV1,
    module_name: Box<str>,
    callable_main: RawCallableMainSelectionV1,
}

impl RawIngressRequestV1 {
    pub(in crate::mir) fn new(
        input: LegacyModuleLoweringInputV1,
        config: BuilderInvocationConfigV1,
        module_name: impl Into<Box<str>>,
        callable_main: RawCallableMainSelectionV1,
    ) -> Self {
        Self {
            input,
            config,
            module_name: module_name.into(),
            callable_main,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawSourceBindingErrorV1 {
    ProgramV0OutsideRawSource0,
    Projection(RawSourceProjectionErrorV1),
    CallableMainRequiredForScript,
    Identity(SourceBindingErrorV1),
    RuntimeInputs(RawRuntimeInputCaptureErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawSourceContinuationV1 {
    origin: RawSourceOriginV1,
    callable_main: RawCallableMainCompatibilityDispositionV1,
    policy: ModuleInvocationPolicyV1,
    runtime_inputs: RawRuntimeInputSnapshotV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawPostCallableMainContinuationV1 {
    origin: RawSourceOriginV1,
    policy: ModuleInvocationPolicyV1,
    runtime_inputs: RawRuntimeInputSnapshotV1,
}

impl RawPostCallableMainContinuationV1 {
    pub(in crate::mir) const fn origin(&self) -> RawSourceOriginV1 {
        self.origin
    }

    pub(in crate::mir) const fn policy(&self) -> ModuleInvocationPolicyV1 {
        self.policy
    }

    pub(in crate::mir) const fn runtime_inputs(&self) -> &RawRuntimeInputSnapshotV1 {
        &self.runtime_inputs
    }
}

impl RawSourceContinuationV1 {
    pub(in crate::mir) const fn origin(&self) -> RawSourceOriginV1 {
        self.origin
    }

    pub(in crate::mir) const fn callable_main(&self) -> RawCallableMainCompatibilityDispositionV1 {
        self.callable_main
    }

    pub(in crate::mir) const fn policy(&self) -> ModuleInvocationPolicyV1 {
        self.policy
    }

    pub(in crate::mir) const fn runtime_inputs(&self) -> &RawRuntimeInputSnapshotV1 {
        &self.runtime_inputs
    }

    pub(in crate::mir) fn into_callable_main_decision(
        self,
    ) -> (
        RawPostCallableMainContinuationV1,
        RawCallableMainCompatibilityDispositionV1,
    ) {
        let Self {
            origin,
            callable_main,
            policy,
            runtime_inputs,
        } = self;
        (
            RawPostCallableMainContinuationV1 {
                origin,
                policy,
                runtime_inputs,
            },
            callable_main,
        )
    }
}

#[derive(Debug)]
pub(in crate::mir) struct SourceBoundRawPackageV1 {
    token: ModuleInvocationTokenV1,
    source: OwnedRawSourceV1,
    continuation: RawSourceContinuationV1,
    config: BuilderInvocationConfigV1,
    module_name: Box<str>,
}

impl SourceBoundRawPackageV1 {
    pub(in crate::mir) fn bind(
        issuer: &mut InvocationIdentityIssuerV1,
        request: RawIngressRequestV1,
    ) -> Result<Self, RejectedRawSourceBindingV1> {
        let runtime_inputs = match RawRuntimeInputSnapshotV1::capture() {
            Ok(snapshot) => snapshot,
            Err(error) => {
                return Err(RejectedRawSourceBindingV1::with_request(
                    request,
                    RawSourceBindingErrorV1::RuntimeInputs(error),
                ));
            }
        };
        let RawIngressRequestV1 {
            input,
            config,
            module_name,
            callable_main,
        } = request;
        let (ast, origin) = input.into_parts();
        let source_origin = match origin {
            LegacyModuleOriginV1::BareAst => RawSourceOriginV1::BareAst,
            LegacyModuleOriginV1::ReplCompatibility => RawSourceOriginV1::ReplCompatibility,
            LegacyModuleOriginV1::ProgramV0Compatibility => {
                return Err(RejectedRawSourceBindingV1::without_source(
                    ast,
                    origin,
                    config,
                    module_name,
                    RawSourceBindingErrorV1::ProgramV0OutsideRawSource0,
                ));
            }
        };
        let source = match OwnedRawSourceV1::bind_with_owner(ast, source_origin) {
            Ok(source) => source,
            Err((ast, error)) => {
                return Err(RejectedRawSourceBindingV1::without_source(
                    ast,
                    origin,
                    config,
                    module_name,
                    RawSourceBindingErrorV1::Projection(error),
                ));
            }
        };
        if source.projection().is_script()
            && matches!(callable_main, RawCallableMainSelectionV1::Required)
        {
            return Err(RejectedRawSourceBindingV1::with_source(
                source,
                config,
                module_name,
                RawSourceBindingErrorV1::CallableMainRequiredForScript,
            ));
        }
        let disposition = match callable_main {
            RawCallableMainSelectionV1::Omitted => {
                RawCallableMainCompatibilityDispositionV1::NotSelected
            }
            RawCallableMainSelectionV1::Required => {
                RawCallableMainCompatibilityDispositionV1::Selected
            }
        };
        let continuation = RawSourceContinuationV1 {
            origin: source.origin(),
            callable_main: disposition,
            policy: ModuleInvocationPolicyV1::policy_for_family(ModuleInvocationFamilyV1::Raw),
            runtime_inputs,
        };
        let token = match issuer.issue_raw() {
            Ok(token) => token,
            Err(error) => {
                return Err(RejectedRawSourceBindingV1::with_source(
                    source,
                    config,
                    module_name,
                    RawSourceBindingErrorV1::Identity(error),
                ));
            }
        };
        Ok(Self {
            token,
            source,
            continuation,
            config,
            module_name,
        })
    }

    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.token.brand()
    }

    pub(in crate::mir) const fn family(&self) -> ModuleInvocationFamilyV1 {
        self.token.family()
    }

    pub(in crate::mir) const fn token(&self) -> &ModuleInvocationTokenV1 {
        &self.token
    }

    pub(in crate::mir) const fn source(&self) -> &OwnedRawSourceV1 {
        &self.source
    }

    pub(in crate::mir) const fn continuation(&self) -> &RawSourceContinuationV1 {
        &self.continuation
    }

    pub(in crate::mir) const fn config(&self) -> &BuilderInvocationConfigV1 {
        &self.config
    }

    pub(in crate::mir) fn module_name(&self) -> &str {
        &self.module_name
    }

    /// LOWER0's only source/package handoff.  The owned source retains the
    /// projection; the continuation carries policy only, so locator authority
    /// is not cloned into a second owner.
    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        ModuleInvocationTokenV1,
        OwnedRawSourceV1,
        RawSourceContinuationV1,
        BuilderInvocationConfigV1,
        Box<str>,
    ) {
        (
            self.token,
            self.source,
            self.continuation,
            self.config,
            self.module_name,
        )
    }
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawSourceBindingV1 {
    ast: ASTNodeOwnerV1,
    source: Option<OwnedRawSourceV1>,
    config: BuilderInvocationConfigV1,
    module_name: Box<str>,
    error: RawSourceBindingErrorV1,
}

#[derive(Debug)]
enum ASTNodeOwnerV1 {
    Original(crate::ast::ASTNode, LegacyModuleOriginV1),
    AlreadyProjected,
}

impl RejectedRawSourceBindingV1 {
    fn without_source(
        ast: crate::ast::ASTNode,
        origin: LegacyModuleOriginV1,
        config: BuilderInvocationConfigV1,
        module_name: Box<str>,
        error: RawSourceBindingErrorV1,
    ) -> Self {
        Self {
            ast: ASTNodeOwnerV1::Original(ast, origin),
            source: None,
            config,
            module_name,
            error,
        }
    }

    fn with_source(
        source: OwnedRawSourceV1,
        config: BuilderInvocationConfigV1,
        module_name: Box<str>,
        error: RawSourceBindingErrorV1,
    ) -> Self {
        Self {
            ast: ASTNodeOwnerV1::AlreadyProjected,
            source: Some(source),
            config,
            module_name,
            error,
        }
    }

    fn with_request(request: RawIngressRequestV1, error: RawSourceBindingErrorV1) -> Self {
        let RawIngressRequestV1 {
            input,
            config,
            module_name,
            callable_main: _,
        } = request;
        let (ast, origin) = input.into_parts();
        Self {
            ast: ASTNodeOwnerV1::Original(ast, origin),
            source: None,
            config,
            module_name,
            error,
        }
    }

    pub(in crate::mir) const fn error(&self) -> &RawSourceBindingErrorV1 {
        &self.error
    }

    #[cfg(test)]
    pub(in crate::mir) fn has_unpublished_source_owner(&self) -> bool {
        self.source.is_some() || matches!(self.ast, ASTNodeOwnerV1::Original(..))
    }
}
