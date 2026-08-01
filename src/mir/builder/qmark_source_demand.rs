//! Receipt-backed source demand for the root-QMark propagation route.

use crate::ast::ASTNode;

use super::raw_invocation_source_transport::RawSourceTransportPortV1;
use super::recursive_child_lowering::{RawInvocationChildPortV1, RawLegacyChildLoweringPortV1};

pub(in crate::mir::builder) trait QMarkPropagationSourceDemandPortV1 {
    fn has_qmark_propagation_receipt_v1(&self, qmark: &ASTNode) -> Result<bool, String>;
}

impl QMarkPropagationSourceDemandPortV1 for RawLegacyChildLoweringPortV1 {
    fn has_qmark_propagation_receipt_v1(&self, _qmark: &ASTNode) -> Result<bool, String> {
        Ok(false)
    }
}

impl QMarkPropagationSourceDemandPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn has_qmark_propagation_receipt_v1(&self, qmark: &ASTNode) -> Result<bool, String> {
        let Some(ledger) = &self.semantic_ledger else {
            return Ok(false);
        };
        if !matches!(qmark, ASTNode::QMarkPropagate { .. }) {
            return Err("[freeze:contract][script-qmark/non-qmark-demand]".to_owned());
        }
        let site = self
            .current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| "[freeze:contract][script-qmark/missing-site]".to_owned())?;
        ledger
            .borrow()
            .has_qmark_propagation_receipt(&site)
            .then_some(true)
            .ok_or_else(|| "[freeze:contract][script-qmark/missing-sealed-receipt]".to_owned())
    }
}
