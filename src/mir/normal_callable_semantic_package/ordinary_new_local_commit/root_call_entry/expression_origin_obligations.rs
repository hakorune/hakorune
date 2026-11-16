//! Borrow existing read obligations; source/operand/effect checks stay with
//! their original preflight, and no second inventory is stored here.
use super::*;
impl ExpressionPhysicalProjectionRefV1<'_> {
    pub(in crate::mir) fn visit_i64_origins_v1(
        &self,
        visit: &mut dyn FnMut(&ExpressionReferenceReadV1) -> Result<(), String>,
    ) -> Result<(), String> {
        if !self.frame.reference_coverage_is_exact()
            || self.values.len() != self.frame.source().nodes().len()
        {
            return Err(freeze("print-expression/source-coverage"));
        }
        self.validate_reference_census_v1()?;
        for (&node, read) in self.reference_reads {
            let source = self
                .frame
                .references()
                .get(&node)
                .ok_or_else(|| freeze("expression/reference-read-census"))?;
            read.visit_i64_origin_v1(self.frame.source(), node, source, visit)?;
        }
        for packet in self.calls {
            packet.visit_i64_origins_v1(visit)?;
        }
        for packet in self.dynamic_calls {
            packet.visit_i64_origins_v1(visit)?;
        }
        Ok(())
    }
}
