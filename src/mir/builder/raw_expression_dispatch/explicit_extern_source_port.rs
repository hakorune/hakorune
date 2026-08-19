//! Exact resolver-issued symbol loan for canonical explicit extern calls.

use super::super::raw_invocation_source_transport::RawSourceTransportPortV1;
use super::super::raw_structured_child_scope::RawStructuredChildScopePortV1;
use super::super::recursive_child_lowering::RawInvocationChildPortV1;
use super::super::recursive_child_lowering::RawLegacyChildLoweringPortV1;

pub(super) trait ExplicitExternSourcePortV1 {
    fn resolved_explicit_extern_symbol_v1(&self) -> Result<Option<Box<str>>, String> {
        Ok(None)
    }
}

impl ExplicitExternSourcePortV1 for RawLegacyChildLoweringPortV1 {}

impl<Port: ExplicitExternSourcePortV1> ExplicitExternSourcePortV1
    for RawStructuredChildScopePortV1<'_, Port>
{
    fn resolved_explicit_extern_symbol_v1(&self) -> Result<Option<Box<str>>, String> {
        self.child().resolved_explicit_extern_symbol_v1()
    }
}

impl ExplicitExternSourcePortV1 for RawInvocationChildPortV1<'_, '_> {
    fn resolved_explicit_extern_symbol_v1(&self) -> Result<Option<Box<str>>, String> {
        let site = self
            .current_source_context_v1()
            .and_then(|context| context.site().cloned())
            .ok_or_else(|| "[freeze:contract][explicit-extern/missing-source-site]".to_owned())?;
        if let Some(ledger) = &self.semantic_ledger {
            return Ok(ledger
                .borrow()
                .explicit_extern_symbol(&site)
                .map(Into::into));
        }
        if let Some(ledger) = &self.callable_ledger {
            return Ok(ledger
                .borrow()
                .explicit_extern_symbol(&site)
                .map(Into::into));
        }
        Ok(None)
    }
}
