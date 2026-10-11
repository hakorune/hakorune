//! One read-only selected Static two-exit validation view.
//! Every pending row is checked together; none publishes before collection.
use super::super::PreparedLoopStaticBodyDetachedPacketV1;
use super::*;

pub(crate) struct PendingSelectedStaticLoopViewV1<'a, 'ledger> {
    entry: &'a PreparedRootLexicalCallBindingGroupV1<'ledger>,
    body: &'a PreparedLoopStaticBodyDetachedPacketV1,
    roots: [&'a PendingRootHomeExitV1<'ledger>; 2],
    excluded_routes: &'a [OwnedExprSiteV1],
}

impl<'a, 'ledger> PendingSelectedStaticLoopViewV1<'a, 'ledger> {
    pub(crate) fn new(
        entry: &'a PreparedRootLexicalCallBindingGroupV1<'ledger>,
        body: &'a PreparedLoopStaticBodyDetachedPacketV1,
        roots: [&'a PendingRootHomeExitV1<'ledger>; 2],
        excluded_routes: &'a [OwnedExprSiteV1],
    ) -> Self {
        Self {
            entry,
            body,
            roots,
            excluded_routes,
        }
    }

    pub(crate) fn entry(&self) -> &'a PreparedRootLexicalCallBindingGroupV1<'ledger> {
        self.entry
    }

    pub(crate) fn body(&self) -> &'a PreparedLoopStaticBodyDetachedPacketV1 {
        self.body
    }

    pub(crate) fn roots(&self) -> [&'a PendingRootHomeExitV1<'ledger>; 2] {
        self.roots
    }

    pub(crate) fn root(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Option<&'a PendingRootHomeExitV1<'ledger>> {
        self.roots()
            .into_iter()
            .find(|row| row.owner() == owner && row.site() == site)
    }

    pub(crate) fn excluded_routes(&self) -> &'a [OwnedExprSiteV1] {
        self.excluded_routes
    }
}
