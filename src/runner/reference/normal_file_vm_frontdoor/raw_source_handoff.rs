//! Atomic Raw-route closure for one parser product and its Script-A sibling.
//!
//! The parser product decides whether source-backed or explicit compatibility
//! extraction is available.  This boundary keeps the unselected Script-A
//! input inside the same affine operation, so callers cannot extract syntax
//! while silently dropping its sibling.

use super::NormalFileSourceReceiptV1;
use crate::ast::ASTNode;
use crate::mir::{RawVmReferenceInvocationV1, RawVmReferenceSupportProfileV1};
use crate::parser::{
    callable_parameter_source::{
        CanonicalScriptSourceRowsDispositionV1, ParsedProgramWithCallableParameterSourceV1,
    },
    ParserNormalRawVmSourceExtractionErrorV1, ParserNormalRawVmSourceKindV1,
    PreparedParserNormalCompatibilityRawVmV1, PreparedParserNormalRawVmSourceRouteV1,
    PreparedParserNormalSourceBackedRawVmV1, RejectedParserNormalRawVmSourceExtractionV1,
};

#[derive(Debug)]
pub(super) struct PreparedCanonicalParserRawSourceV1 {
    source: ParsedProgramWithCallableParameterSourceV1,
    script_rows: CanonicalScriptSourceRowsDispositionV1,
    downstream: RawVmReferenceSupportProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: PreparedCanonicalParserRawSourceSealV1,
}

#[derive(Debug)]
struct PreparedCanonicalParserRawSourceSealV1;

#[derive(Debug)]
pub(super) struct PreparedRawVmSourceExtractionV1 {
    ast: ASTNode,
    kind: ParserNormalRawVmSourceKindV1,
    downstream: RawVmReferenceSupportProfileV1,
    receipt: NormalFileSourceReceiptV1,
    _seal: PreparedRawVmSourceExtractionSealV1,
}

#[derive(Debug)]
struct PreparedRawVmSourceExtractionSealV1;

struct ParserNormalRootExecutionRawVmDiscardIssuerV1;

struct RawCompatibilitySourceExtractionIssuerV1;

struct RawVmSourceExtractionTailV1 {
    downstream: RawVmReferenceSupportProfileV1,
    receipt: NormalFileSourceReceiptV1,
}

#[derive(Debug)]
pub(super) struct RejectedRawVmSourceExtractionV1 {
    rejected: RejectedParserNormalRawVmSourceExtractionV1,
    script_rows: CanonicalScriptSourceRowsDispositionV1,
    receipt: NormalFileSourceReceiptV1,
}

impl PreparedCanonicalParserRawSourceV1 {
    pub(super) fn new(
        source: ParsedProgramWithCallableParameterSourceV1,
        script_rows: CanonicalScriptSourceRowsDispositionV1,
        downstream: RawVmReferenceSupportProfileV1,
        receipt: NormalFileSourceReceiptV1,
    ) -> Self {
        Self {
            source,
            script_rows,
            downstream,
            receipt,
            _seal: PreparedCanonicalParserRawSourceSealV1,
        }
    }

    pub(super) fn extract_once(
        self,
    ) -> Result<PreparedRawVmSourceExtractionV1, RejectedRawVmSourceExtractionV1> {
        let Self {
            source,
            script_rows,
            downstream,
            receipt,
            _seal,
        } = self;
        drop(_seal);
        let route = match source.prepare_raw_vm_source_route() {
            Ok(route) => route,
            Err(rejected) => {
                return Err(RejectedRawVmSourceExtractionV1 {
                    rejected,
                    script_rows,
                    receipt,
                });
            }
        };
        let tail = RawVmSourceExtractionTailV1 {
            downstream,
            receipt,
        };
        Ok(match route {
            PreparedParserNormalRawVmSourceRouteV1::SourceBacked(source) => {
                ParserNormalRootExecutionRawVmDiscardIssuerV1::issue_once(source, script_rows, tail)
            }
            PreparedParserNormalRawVmSourceRouteV1::Compatibility(source) => {
                RawCompatibilitySourceExtractionIssuerV1::issue_once(source, script_rows, tail)
            }
        })
    }

    pub(super) fn discard_at_wrong_route_terminal(self) {
        let Self {
            source,
            script_rows,
            downstream,
            receipt,
            _seal,
        } = self;
        source.discard_after_source_plan_rejection();
        script_rows.discard_at_named_terminal();
        drop((downstream, receipt, _seal));
    }
}

impl ParserNormalRootExecutionRawVmDiscardIssuerV1 {
    fn issue_once(
        source: PreparedParserNormalSourceBackedRawVmV1,
        script_rows: CanonicalScriptSourceRowsDispositionV1,
        tail: RawVmSourceExtractionTailV1,
    ) -> PreparedRawVmSourceExtractionV1 {
        script_rows.discard_at_named_terminal();
        PreparedRawVmSourceExtractionV1 {
            ast: source.into_ast_after_named_raw_discard(),
            kind: ParserNormalRawVmSourceKindV1::SourceBacked,
            downstream: tail.downstream,
            receipt: tail.receipt,
            _seal: PreparedRawVmSourceExtractionSealV1,
        }
    }
}

impl RawCompatibilitySourceExtractionIssuerV1 {
    fn issue_once(
        source: PreparedParserNormalCompatibilityRawVmV1,
        script_rows: CanonicalScriptSourceRowsDispositionV1,
        tail: RawVmSourceExtractionTailV1,
    ) -> PreparedRawVmSourceExtractionV1 {
        script_rows.discard_at_named_terminal();
        PreparedRawVmSourceExtractionV1 {
            ast: source.into_ast_after_named_compatibility_extraction(),
            kind: ParserNormalRawVmSourceKindV1::Compatibility,
            downstream: tail.downstream,
            receipt: tail.receipt,
            _seal: PreparedRawVmSourceExtractionSealV1,
        }
    }
}

impl PreparedRawVmSourceExtractionV1 {
    pub(super) fn ast(&self) -> &ASTNode {
        &self.ast
    }

    pub(super) fn into_invocation(
        self,
        source_identity: Box<str>,
    ) -> (RawVmReferenceInvocationV1, NormalFileSourceReceiptV1) {
        let Self {
            ast,
            kind,
            downstream,
            receipt,
            _seal,
        } = self;
        match kind {
            ParserNormalRawVmSourceKindV1::SourceBacked
            | ParserNormalRawVmSourceKindV1::Compatibility => {}
        }
        drop(_seal);
        (
            downstream.into_invocation(ast, Some(source_identity)),
            receipt,
        )
    }

    pub(super) fn discard(self) {
        let Self {
            ast,
            kind,
            downstream,
            receipt,
            _seal,
        } = self;
        match kind {
            ParserNormalRawVmSourceKindV1::SourceBacked
            | ParserNormalRawVmSourceKindV1::Compatibility => {}
        }
        drop((ast, downstream, receipt, _seal));
    }

    #[cfg(test)]
    pub(super) const fn kind(&self) -> ParserNormalRawVmSourceKindV1 {
        self.kind
    }
}

impl RejectedRawVmSourceExtractionV1 {
    pub(super) const fn error(&self) -> ParserNormalRawVmSourceExtractionErrorV1 {
        self.rejected.error()
    }

    pub(super) fn discard(self) {
        self.script_rows.discard_at_named_terminal();
        self.rejected.discard();
        drop(self.receipt);
    }
}

impl PreparedCanonicalParserRawSourceV1 {
    #[cfg(test)]
    pub(super) fn receipt(&self) -> &NormalFileSourceReceiptV1 {
        &self.receipt
    }
}
