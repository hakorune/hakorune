use crate::ast::ASTNode;
use crate::parser::NormalParserSourceLineageV1;
use crate::r#macro::NormalCallableTransformCompatibilityV1;

/// The parser/macro origin carried by the existing compatibility lane.
///
/// This is transport evidence only.  It is deliberately not a resolver
/// product and cannot issue a callable package, Recipe, Join, or physical
/// target.  The materializer is the only current co-seal issuer.
#[derive(Debug)]
pub(crate) struct NormalCallableCompatibilityOriginV1 {
    ast: ASTNode,
    reason: NormalCallableTransformCompatibilityV1,
    lineage: NormalParserSourceLineageV1,
    _seal: NormalCallableCompatibilityOriginSealV1,
}

#[derive(Debug)]
pub(crate) struct NormalCallableCompatibilityOriginSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalCallableCompatibilityOriginErrorV1 {
    ExpectedProgramRoot,
}

impl std::fmt::Display for NormalCallableCompatibilityOriginErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedProgramRoot => formatter.write_str(
                "[normal-callable/compatibility-origin] transformed source must remain a Program",
            ),
        }
    }
}

impl std::error::Error for NormalCallableCompatibilityOriginErrorV1 {}

impl NormalCallableCompatibilityOriginV1 {
    /// Co-seal the already-issued transformed AST, macro reason, and parser
    /// lineage.  Callers must not reconstruct any of these facts from names,
    /// paths, or the AST after this point.
    pub(crate) fn issue(
        ast: ASTNode,
        reason: NormalCallableTransformCompatibilityV1,
        lineage: NormalParserSourceLineageV1,
    ) -> Result<Self, NormalCallableCompatibilityOriginErrorV1> {
        if !matches!(ast, ASTNode::Program { .. }) {
            return Err(NormalCallableCompatibilityOriginErrorV1::ExpectedProgramRoot);
        }
        Ok(Self {
            ast,
            reason,
            lineage,
            _seal: NormalCallableCompatibilityOriginSealV1,
        })
    }

    pub(crate) fn ast(&self) -> &ASTNode {
        &self.ast
    }

    pub(crate) fn reason(&self) -> &NormalCallableTransformCompatibilityV1 {
        &self.reason
    }

    pub(crate) fn lineage(&self) -> &NormalParserSourceLineageV1 {
        &self.lineage
    }
}

#[cfg(test)]
mod tests {
    use super::NormalCallableCompatibilityOriginV1;
    use crate::ast::ASTNode;
    use crate::parser::{NormalCallableParserCompatibilityV1, NormalParserSourceLineageV1};
    use crate::r#macro::NormalCallableTransformCompatibilityV1;

    #[test]
    fn carrier_keeps_ast_reason_and_lineage_together() {
        let lineage = NormalParserSourceLineageV1::issue(
            "compat.hako",
            crate::mir::CanonicalSourceBytesDigestV1::from_utf8_bytes(b"Program"),
            hakorune_frontend_parser::parser::GrammarProfile::Canonical,
            7,
            1,
            1,
        )
        .expect("lineage");
        let carrier = NormalCallableCompatibilityOriginV1::issue(
            ASTNode::Program {
                statements: vec![],
                span: crate::ast::Span::unknown(),
            },
            NormalCallableTransformCompatibilityV1::Parser(
                NormalCallableParserCompatibilityV1::MixedProgram,
            ),
            lineage,
        )
        .expect("carrier");

        assert!(matches!(carrier.ast(), ASTNode::Program { .. }));
        assert!(matches!(
            carrier.reason(),
            NormalCallableTransformCompatibilityV1::Parser(
                NormalCallableParserCompatibilityV1::MixedProgram
            )
        ));
        assert_eq!(carrier.lineage().source_identity(), "compat.hako");
        assert_eq!(carrier.lineage().receipt_counts(), (1, 1));
    }

    #[test]
    fn carrier_rejects_non_program_before_transport() {
        let lineage = NormalParserSourceLineageV1::issue(
            "compat.hako",
            crate::mir::CanonicalSourceBytesDigestV1::from_utf8_bytes(b"Literal"),
            hakorune_frontend_parser::parser::GrammarProfile::Canonical,
            7,
            1,
            1,
        )
        .expect("lineage");
        let rejected = NormalCallableCompatibilityOriginV1::issue(
            ASTNode::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                span: crate::ast::Span::unknown(),
            },
            NormalCallableTransformCompatibilityV1::Parser(
                NormalCallableParserCompatibilityV1::MixedProgram,
            ),
            lineage,
        )
        .expect_err("non-program must not enter the carrier");
        assert_eq!(
            rejected,
            super::NormalCallableCompatibilityOriginErrorV1::ExpectedProgramRoot
        );
    }
}
