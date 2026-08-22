//! Typed module ingress before any MIR Builder effects.
//!
//! B0-L2b seals canonical syntax, the semantic owner forest, and immutable
//! exact-source projection into one disconnected transport bundle. The bundle
//! is constructed atomically from one owned syntax root at SA3-B.

use std::collections::BTreeMap;
use std::fmt;

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, VerifiedResolvedBodyShapeInventoryV1, VerifiedSemanticOwnerForestV1,
};

use super::source_projection::VerifiedSourceProjectionV1;

#[derive(Debug)]
struct CanonicalSyntaxOwnerV1 {
    root: ASTNode,
}

#[derive(Debug)]
struct ResolvedSourceUnitSealV1;

/// Immutable canonical syntax bundled with the forest sealed for that syntax.
///
/// Syntax, forest, and projection cannot be supplied independently. The sole
/// production constructor resolves and seals all three from the same owned AST.
#[derive(Debug)]
pub struct VerifiedResolvedSourceUnitV1 {
    syntax: CanonicalSyntaxOwnerV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    body_shapes: BTreeMap<FunctionOwnerIdV1, VerifiedResolvedBodyShapeInventoryV1>,
    _seal: ResolvedSourceUnitSealV1,
}

impl VerifiedResolvedSourceUnitV1 {
    /// Resolve one canonical function source unit without exposing a seam for
    /// pairing foreign syntax and semantic products.
    pub fn resolve_function(root: ASTNode) -> Result<Self, CanonicalLoweringErrorV1> {
        use crate::mir::resolved_semantics::{
            FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1,
        };

        let view = FunctionSyntaxViewV1::from_ast(&root).ok_or_else(|| {
            CanonicalLoweringErrorV1::SourceUnitResolution {
                detail: "root_is_not_function_declaration".to_string(),
            }
        })?;
        let mut session = FunctionSemanticResolverSessionV1::new(0).map_err(|error| {
            CanonicalLoweringErrorV1::SourceUnitResolution {
                detail: format!("{error:?}"),
            }
        })?;
        let (forest, body_shapes) = session.resolve_forest_with_body_shapes(view).map_err(|error| {
            CanonicalLoweringErrorV1::SourceUnitResolution {
                detail: format!("{error:?}"),
            }
        })?;
        let projection = VerifiedSourceProjectionV1::seal(&root, &forest).map_err(|error| {
            CanonicalLoweringErrorV1::SourceNavigation {
                detail: error.to_string(),
            }
        })?;
        Ok(Self {
            syntax: CanonicalSyntaxOwnerV1 { root },
            forest,
            projection,
            body_shapes,
            _seal: ResolvedSourceUnitSealV1,
        })
    }

    pub(crate) fn syntax_root(&self) -> &ASTNode {
        &self.syntax.root
    }

    pub(crate) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub(crate) fn projection(&self) -> &VerifiedSourceProjectionV1 {
        &self.projection
    }

    pub(crate) fn body_shape(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Option<&VerifiedResolvedBodyShapeInventoryV1> {
        self.body_shapes.get(&owner)
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
}

/// Legacy input owns syntax only and structurally cannot carry a sealed forest.
#[derive(Debug)]
pub(in crate::mir) struct LegacyModuleLoweringInputV1 {
    ast: ASTNode,
    origin: LegacyModuleOriginV1,
}

impl LegacyModuleLoweringInputV1 {
    pub(in crate::mir) fn bare_ast(ast: ASTNode) -> Self {
        Self {
            ast,
            origin: LegacyModuleOriginV1::BareAst,
        }
    }

    pub(super) const fn ast(&self) -> &ASTNode {
        &self.ast
    }

    pub(super) const fn origin(&self) -> LegacyModuleOriginV1 {
        self.origin
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
    SourceUnitResolution {
        detail: String,
    },
    SourceNavigation {
        detail: String,
    },
    UnsupportedFirstFamilyShape {
        site: String,
        actual: &'static str,
        reason: &'static str,
    },
    ResolvedRegionFlow {
        detail: String,
    },
    ResolvedFunctionCompletion {
        detail: String,
    },
    MirVerificationFailed {
        errors: Box<[String]>,
    },
    DuplicateFunctionPublication {
        function_name: String,
    },
    BuilderContract {
        detail: String,
    },
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
            Self::SourceUnitResolution { detail } => write!(
                formatter,
                "[freeze:contract][canonical_lowering/source_unit_resolution] detail={detail}"
            ),
            Self::SourceNavigation { detail } => write!(
                formatter,
                "[freeze:contract][canonical_lowering/source_navigation] detail={detail}"
            ),
            Self::UnsupportedFirstFamilyShape {
                site,
                actual,
                reason,
            } => write!(
                formatter,
                "[freeze:contract][canonical_lowering/unsupported_first_family_shape] site={site} actual={actual} reason={reason}"
            ),
            Self::ResolvedRegionFlow { detail } => write!(
                formatter,
                "[freeze:contract][canonical_lowering/resolved_region_flow] detail={detail}"
            ),
            Self::ResolvedFunctionCompletion { detail } => write!(
                formatter,
                "[freeze:contract][canonical_lowering/resolved_function_completion] detail={detail}"
            ),
            Self::MirVerificationFailed { errors } => write!(
                formatter,
                "[freeze:contract][canonical_lowering/mir_verification_failed] count={} errors={}",
                errors.len(),
                errors.join(" | ")
            ),
            Self::DuplicateFunctionPublication { function_name } => write!(
                formatter,
                "[freeze:contract][canonical_lowering/duplicate_function_publication] function={function_name}"
            ),
            Self::BuilderContract { detail } => write!(
                formatter,
                "[freeze:contract][canonical_lowering/builder_contract] detail={detail}"
            ),
        }
    }
}

impl std::error::Error for CanonicalLoweringErrorV1 {}

#[cfg(test)]
pub(super) fn verified_source_unit_for_test(root: ASTNode) -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

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
        let (ast, origin) = LegacyModuleLoweringInputV1::bare_ast(function()).into_parts();
        assert_eq!(origin, LegacyModuleOriginV1::BareAst);
        assert!(matches!(ast, ASTNode::FunctionDeclaration { .. }));
    }

    #[test]
    fn verified_unit_is_the_only_resolved_input_factory() {
        let root = function();
        let unit = verified_source_unit_for_test(root);
        let input = unit.lowering_input();

        assert!(matches!(
            input.source_unit().syntax_root(),
            ASTNode::FunctionDeclaration { .. }
        ));
        assert_eq!(input.source_unit().forest().owner_count(), 1);
        let root_owner = input.source_unit().forest().roots()[0];
        let body_shape = input
            .source_unit()
            .body_shape(root_owner)
            .expect("resolver body-shape product");
        assert_eq!(body_shape.owner(), root_owner);
    }

    #[test]
    fn bare_constructor_does_not_fabricate_a_body_shape() {
        let root = function();
        let unit = verified_source_unit_for_test(root);
        let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            unit.syntax_root(),
            unit.forest(),
            unit.projection(),
        )
        .expect("bare mechanical input");
        assert!(input.body_shape().is_none());
    }

    #[test]
    fn resolved_entry_activates_only_the_closed_first_family() {
        let root = function();
        let unit = verified_source_unit_for_test(root);
        let mut compiler = crate::mir::MirCompiler::with_options(false);

        let result = compiler
            .compile_resolved(unit.lowering_input(), Some("fixture.hako"))
            .unwrap();

        assert!(result.module.functions.contains_key("fixture/0"));
        assert!(compiler.builder.current_module.is_none());
    }

    #[test]
    fn unsupported_shape_fails_before_builder_effects_without_legacy_retry() {
        let mut root = function();
        let ASTNode::FunctionDeclaration { body, .. } = &mut root else {
            unreachable!()
        };
        body.push(ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            body: Vec::new(),
            span: Span::unknown(),
        });
        let unit = verified_source_unit_for_test(root);
        let mut compiler = crate::mir::MirCompiler::with_options(false);

        let error = compiler
            .compile_resolved(unit.lowering_input(), Some("unsupported.hako"))
            .unwrap_err();

        assert!(matches!(
            error,
            CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape { .. }
        ));
        assert!(compiler.builder.current_module.is_none());
        assert_eq!(compiler.builder.core_ctx.next_binding_id, 0);
    }
}
