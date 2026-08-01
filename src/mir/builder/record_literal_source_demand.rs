//! Receipt-backed source demand for the fully explicit RecordLiteral route.
//!
//! The raw dispatcher asks this capability before preparing a structured
//! child scope. Legacy and Deferred routes return `None`; a Complete Script
//! route must hold one exact sealed receipt for the current literal site.

use crate::ast::ASTNode;

use super::raw_invocation_source_transport::RawSourceTransportPortV1;
use super::recursive_child_lowering::{RawInvocationChildPortV1, RawLegacyChildLoweringPortV1};

pub(in crate::mir::builder) trait RecordLiteralSourceDemandPortV1 {
    fn record_literal_explicit_field_count_v1(
        &self,
        literal: &ASTNode,
    ) -> Result<Option<u32>, String>;
}

impl RecordLiteralSourceDemandPortV1 for RawLegacyChildLoweringPortV1 {
    fn record_literal_explicit_field_count_v1(
        &self,
        _literal: &ASTNode,
    ) -> Result<Option<u32>, String> {
        Ok(None)
    }
}

impl RecordLiteralSourceDemandPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn record_literal_explicit_field_count_v1(
        &self,
        literal: &ASTNode,
    ) -> Result<Option<u32>, String> {
        let Some(ledger) = &self.semantic_ledger else {
            return Ok(None);
        };
        if !matches!(literal, ASTNode::RecordLiteral { .. }) {
            return Err("[freeze:contract][script-record/non-record-demand]".to_owned());
        }
        let site = self
            .current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| "[freeze:contract][script-record/missing-literal-site]".to_owned())?;
        ledger
            .borrow()
            .record_literal_explicit_field_count(&site)
            .map(Some)
            .ok_or_else(|| "[freeze:contract][script-record/missing-sealed-receipt]".to_owned())
    }
}
