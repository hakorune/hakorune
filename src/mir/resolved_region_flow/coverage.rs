//! Exact assignment-site ownership recorded before flow publication.

use crate::mir::resolved_semantics::SourceExprSiteV1;

#[derive(Debug, Default)]
pub(crate) struct IfFlowCoverageDraftV1 {
    condition_direct: Vec<SourceExprSiteV1>,
    then_direct: Vec<SourceExprSiteV1>,
    else_direct: Vec<SourceExprSiteV1>,
}

impl IfFlowCoverageDraftV1 {
    pub(super) fn record_condition(&mut self, site: SourceExprSiteV1) {
        self.condition_direct.push(site);
    }

    pub(super) fn record_then(&mut self, site: SourceExprSiteV1) {
        self.then_direct.push(site);
    }

    pub(super) fn record_else(&mut self, site: SourceExprSiteV1) {
        self.else_direct.push(site);
    }

    pub(super) fn into_verified(self) -> VerifiedIfFlowCoverageV1 {
        VerifiedIfFlowCoverageV1 {
            condition_direct: self.condition_direct.into_boxed_slice(),
            then_direct: self.then_direct.into_boxed_slice(),
            else_direct: self.else_direct.into_boxed_slice(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedIfFlowCoverageV1 {
    condition_direct: Box<[SourceExprSiteV1]>,
    then_direct: Box<[SourceExprSiteV1]>,
    else_direct: Box<[SourceExprSiteV1]>,
}

impl VerifiedIfFlowCoverageV1 {
    pub(crate) fn condition_direct(&self) -> &[SourceExprSiteV1] {
        &self.condition_direct
    }

    pub(crate) fn then_direct(&self) -> &[SourceExprSiteV1] {
        &self.then_direct
    }

    pub(crate) fn else_direct(&self) -> &[SourceExprSiteV1] {
        &self.else_direct
    }

    pub(super) fn direct_sites(&self) -> impl Iterator<Item = &SourceExprSiteV1> {
        self.condition_direct
            .iter()
            .chain(self.then_direct.iter())
            .chain(self.else_direct.iter())
    }
}

#[derive(Debug, Default)]
pub(crate) struct FunctionFlowCoverageDraftV1 {
    function_direct: Vec<SourceExprSiteV1>,
}

impl FunctionFlowCoverageDraftV1 {
    pub(super) fn record_direct(&mut self, site: SourceExprSiteV1) {
        self.function_direct.push(site);
    }

    pub(super) fn into_verified(self) -> VerifiedFunctionFlowCoverageV1 {
        VerifiedFunctionFlowCoverageV1 {
            function_direct: self.function_direct.into_boxed_slice(),
        }
    }

    pub(super) fn direct_sites(&self) -> impl Iterator<Item = &SourceExprSiteV1> {
        self.function_direct.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedFunctionFlowCoverageV1 {
    function_direct: Box<[SourceExprSiteV1]>,
}

impl VerifiedFunctionFlowCoverageV1 {
    pub(crate) fn function_direct(&self) -> &[SourceExprSiteV1] {
        &self.function_direct
    }
}
