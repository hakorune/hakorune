//! Source identity lent to the checked loop call port.

use crate::mir::resolved_semantics::SourceExprSiteV1;


#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CoreCallSourceV1 {
    Unlocated,
    LocatedMethodCall(SourceExprSiteV1),
}
