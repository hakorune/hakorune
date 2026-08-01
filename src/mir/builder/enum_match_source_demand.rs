//! Receipt-backed source demand for direct EnumMatch scrutinee descent.

use crate::ast::ASTNode;

use super::raw_invocation_source_transport::RawSourceTransportPortV1;
use super::recursive_child_lowering::{RawInvocationChildPortV1, RawLegacyChildLoweringPortV1};

pub(in crate::mir::builder) trait EnumMatchSourceDemandPortV1 {
    fn has_enum_match_scrutinee_receipt_v1(&self, expression: &ASTNode) -> Result<bool, String>;
}

impl EnumMatchSourceDemandPortV1 for RawLegacyChildLoweringPortV1 {
    fn has_enum_match_scrutinee_receipt_v1(&self, _expression: &ASTNode) -> Result<bool, String> {
        Ok(false)
    }
}

impl EnumMatchSourceDemandPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn has_enum_match_scrutinee_receipt_v1(&self, expression: &ASTNode) -> Result<bool, String> {
        let Some(ledger) = &self.semantic_ledger else {
            return Ok(false);
        };
        if !matches!(expression, ASTNode::EnumMatchExpr { .. }) {
            return Err("[freeze:contract][script-enum-match/non-enum-match-demand]".to_owned());
        }
        let site = self
            .current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| "[freeze:contract][script-enum-match/missing-site]".to_owned())?;
        ledger
            .borrow()
            .has_enum_match_scrutinee_receipt(&site)
            .then_some(true)
            .ok_or_else(|| "[freeze:contract][script-enum-match/missing-sealed-receipt]".to_owned())
    }
}
