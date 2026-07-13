//! Typed module ingress before any MIR Builder effects.
//!
//! B0-L2a installs disconnected transport vocabulary only. The resolved
//! source-unit seal has no production constructor until exact source
//! projection is available in B0-L2b.

use std::fmt;

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::VerifiedSemanticOwnerForestV1;

#[derive(Debug)]
struct CanonicalSyntaxOwnerV1 {
    root: ASTNode,
}

#[derive(Debug)]
struct ResolvedSourceUnitSealV1;

/// Immutable canonical syntax bundled with the forest sealed for that syntax.
///
/// There is deliberately no production constructor in B0-L2a. B0-L2b must
/// provide the exact source-projection proof before this value becomes
/// constructible outside focused tests.
#[derive(Debug)]
pub struct VerifiedResolvedSourceUnitV1 {
    syntax: CanonicalSyntaxOwnerV1,
    forest: VerifiedSemanticOwnerForestV1,
    _seal: ResolvedSourceUnitSealV1,
}

impl VerifiedResolvedSourceUnitV1 {
    pub(crate) fn syntax_root(&self) -> &ASTNode {
        &self.syntax.root
    }

    pub(crate) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub fn lowering_input(&self) -> ResolvedModuleLoweringInputV1<'_> {
        ResolvedModuleLoweringInputV1 { source_unit: self }
    }
}

/// Canonical module input. It cannot be assembled from a bare AST.
#[derive(Debug, Clone, Copy)]
pub struct ResolvedModuleLoweringInputV1<'a> {
    source_unit: &'a VerifiedResolvedSourceUnitV1,
}

impl<'a> ResolvedModuleLoweringInputV1<'a> {
    pub fn source_unit(self) -> &'a VerifiedResolvedSourceUnitV1 {
        self.source_unit
    }
}

/// Explicit provenance for inputs that do not carry canonical source identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LegacyModuleOriginV1 {
    BareAst,
    ProgramV0Compatibility,
    ReplCompatibility,
}

/// Legacy input owns syntax only and structurally cannot carry a sealed forest.
#[derive(Debug)]
pub struct LegacyModuleLoweringInputV1 {
    ast: ASTNode,
    origin: LegacyModuleOriginV1,
}

impl LegacyModuleLoweringInputV1 {
    pub fn bare_ast(ast: ASTNode) -> Self {
        Self {
            ast,
            origin: LegacyModuleOriginV1::BareAst,
        }
    }

    pub fn program_v0_compatibility(ast: ASTNode) -> Self {
        Self {
            ast,
            origin: LegacyModuleOriginV1::ProgramV0Compatibility,
        }
    }

    pub fn repl_compatibility(ast: ASTNode) -> Self {
        Self {
            ast,
            origin: LegacyModuleOriginV1::ReplCompatibility,
        }
    }

    pub(super) fn into_parts(self) -> (ASTNode, LegacyModuleOriginV1) {
        (self.ast, self.origin)
    }
}

/// Typed canonical preflight failures. No variant authorizes legacy retry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalLoweringErrorV1 {
    CapabilityNotActivated {
        boundary: &'static str,
    },
    RequestRouteInvariant {
        expected: &'static str,
        actual: &'static str,
    },
    UnsupportedCanonicalOwnerKind,
    UnsupportedCanonicalSyntaxKind,
    UnsupportedCanonicalControlRoute,
    MissingCorePlanSiteCarrier,
    MissingLambdaOwnerTransport,
}

impl fmt::Display for CanonicalLoweringErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapabilityNotActivated { boundary } => write!(
                formatter,
                "[freeze:contract][canonical_lowering/capability_not_activated] boundary={boundary}"
            ),
            Self::RequestRouteInvariant { expected, actual } => write!(
                formatter,
                "[freeze:contract][canonical_lowering/request_route_invariant] expected={expected} actual={actual}"
            ),
            Self::UnsupportedCanonicalOwnerKind => formatter.write_str(
                "[freeze:contract][canonical_lowering/unsupported_owner_kind]",
            ),
            Self::UnsupportedCanonicalSyntaxKind => formatter.write_str(
                "[freeze:contract][canonical_lowering/unsupported_syntax_kind]",
            ),
            Self::UnsupportedCanonicalControlRoute => formatter.write_str(
                "[freeze:contract][canonical_lowering/unsupported_control_route]",
            ),
            Self::MissingCorePlanSiteCarrier => formatter.write_str(
                "[freeze:contract][canonical_lowering/missing_coreplan_site_carrier]",
            ),
            Self::MissingLambdaOwnerTransport => formatter.write_str(
                "[freeze:contract][canonical_lowering/missing_lambda_owner_transport]",
            ),
        }
    }
}

impl std::error::Error for CanonicalLoweringErrorV1 {}

pub(super) enum MirLoweringRequestV1<'a> {
    Resolved(ResolvedModuleLoweringInputV1<'a>),
    Legacy(LegacyModuleLoweringInputV1),
}

pub(super) enum MirLoweringRequestErrorV1 {
    Canonical(CanonicalLoweringErrorV1),
    Legacy(String),
}

impl MirLoweringRequestErrorV1 {
    pub(super) fn into_canonical(self) -> CanonicalLoweringErrorV1 {
        match self {
            Self::Canonical(error) => error,
            Self::Legacy(_) => CanonicalLoweringErrorV1::RequestRouteInvariant {
                expected: "canonical_error",
                actual: "legacy_error",
            },
        }
    }

    pub(super) fn into_legacy(self) -> String {
        match self {
            Self::Legacy(error) => error,
            Self::Canonical(error) => format!(
                "[freeze:contract][canonical_lowering/request_route_invariant] expected=legacy_error actual=canonical_error detail={error}"
            ),
        }
    }
}

#[cfg(test)]
fn verified_source_unit_for_test(
    root: ASTNode,
    forest: VerifiedSemanticOwnerForestV1,
) -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1 {
        syntax: CanonicalSyntaxOwnerV1 { root },
        forest,
        _seal: ResolvedSourceUnitSealV1,
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1};

    use super::*;

    fn function() -> ASTNode {
        ASTNode::FunctionDeclaration {
            name: "fixture".into(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: vec![ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }],
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        }
    }

    #[test]
    fn explicit_legacy_origins_do_not_carry_a_forest() {
        for (input, expected) in [
            (
                LegacyModuleLoweringInputV1::bare_ast(function()),
                LegacyModuleOriginV1::BareAst,
            ),
            (
                LegacyModuleLoweringInputV1::program_v0_compatibility(function()),
                LegacyModuleOriginV1::ProgramV0Compatibility,
            ),
            (
                LegacyModuleLoweringInputV1::repl_compatibility(function()),
                LegacyModuleOriginV1::ReplCompatibility,
            ),
        ] {
            let (ast, origin) = input.into_parts();
            assert_eq!(origin, expected);
            assert!(matches!(ast, ASTNode::FunctionDeclaration { .. }));
        }
    }

    #[test]
    fn verified_unit_is_the_only_resolved_input_factory() {
        let root = function();
        let forest = FunctionSemanticResolverSessionV1::new(0)
            .unwrap()
            .resolve_forest(FunctionSyntaxViewV1::from_ast(&root).unwrap())
            .unwrap();
        let unit = verified_source_unit_for_test(root, forest);
        let input = unit.lowering_input();

        assert!(matches!(
            input.source_unit().syntax_root(),
            ASTNode::FunctionDeclaration { .. }
        ));
        assert_eq!(input.source_unit().forest().owner_count(), 1);
    }

    #[test]
    fn resolved_entry_stops_before_builder_effects() {
        let root = function();
        let forest = FunctionSemanticResolverSessionV1::new(0)
            .unwrap()
            .resolve_forest(FunctionSyntaxViewV1::from_ast(&root).unwrap())
            .unwrap();
        let unit = verified_source_unit_for_test(root, forest);
        let mut compiler = crate::mir::MirCompiler::with_options(false);

        let error = compiler
            .compile_resolved(unit.lowering_input(), Some("fixture.hako"))
            .unwrap_err();

        assert_eq!(
            error,
            CanonicalLoweringErrorV1::CapabilityNotActivated { boundary: "B0-L2a" }
        );
        assert!(compiler.builder.current_module.is_none());
    }
}
