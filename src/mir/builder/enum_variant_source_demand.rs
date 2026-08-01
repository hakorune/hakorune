//! Receipt-backed source demand for direct enum-variant construction.
//!
//! Only a Complete Script semantic source can select this route. Every other
//! ingress deliberately returns `None` and retains raw FromCall preflight.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::EnumVariantAdmissionV1;

use super::raw_invocation_source_transport::RawSourceTransportPortV1;
use super::recursive_child_lowering::{RawInvocationChildPortV1, RawLegacyChildLoweringPortV1};

pub(in crate::mir::builder) trait EnumVariantSourceDemandPortV1 {
    fn enum_variant_admission_v1(
        &self,
        expression: &ASTNode,
    ) -> Result<Option<EnumVariantAdmissionV1>, String>;
}

impl EnumVariantSourceDemandPortV1 for RawLegacyChildLoweringPortV1 {
    fn enum_variant_admission_v1(
        &self,
        _expression: &ASTNode,
    ) -> Result<Option<EnumVariantAdmissionV1>, String> {
        Ok(None)
    }
}

impl EnumVariantSourceDemandPortV1 for RawInvocationChildPortV1<'_, '_> {
    fn enum_variant_admission_v1(
        &self,
        expression: &ASTNode,
    ) -> Result<Option<EnumVariantAdmissionV1>, String> {
        let Some(ledger) = &self.semantic_ledger else {
            return Ok(None);
        };
        if !matches!(expression, ASTNode::FromCall { .. }) {
            return Err("[freeze:contract][script-enum/non-from-call-demand]".to_owned());
        }
        let site = self
            .current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| "[freeze:contract][script-enum/missing-from-call-site]".to_owned())?;
        ledger
            .borrow()
            .enum_variant_demand(&site)
            .cloned()
            .map(Some)
            .ok_or_else(|| "[freeze:contract][script-enum/missing-sealed-receipt]".to_owned())
    }
}
