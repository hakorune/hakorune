use super::super::source_coverage::CoveredSourceSiteV1;
use super::product::{ResolvedIfControlMaterializationV1, VerifiedLocatedIfControlV1};
use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceStmtSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FunctionIfControlUseErrorV1 {
    ForeignOwner,
    Duplicate,
    WrongOrder,
    Unexpected,
    MissingMaterializationRow,
    Missing,
}

pub(crate) struct FunctionIfControlUseLedgerV1 {
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) expected_sites: Box<[SourceStmtSiteV1]>,
    pub(super) rows: Vec<Option<VerifiedLocatedIfControlV1>>,
    pub(super) next: usize,
}

impl FunctionIfControlUseLedgerV1 {
    pub(crate) fn claim(
        &mut self,
        statement: &LocatedStmtV1<'_>,
    ) -> Result<ResolvedIfControlMaterializationV1, FunctionIfControlUseErrorV1> {
        if statement.owner() != self.owner {
            return Err(FunctionIfControlUseErrorV1::ForeignOwner);
        }
        let site = statement.site();
        if self.expected_sites.get(self.next) == Some(site) {
            let row = self
                .rows
                .get_mut(self.next)
                .and_then(Option::take)
                .ok_or(FunctionIfControlUseErrorV1::MissingMaterializationRow)?;
            self.next += 1;
            return Ok(row.into_materialization());
        }
        if self.expected_sites[..self.next].contains(site) {
            return Err(FunctionIfControlUseErrorV1::Duplicate);
        }
        if self.expected_sites[self.next..].contains(site) {
            return Err(FunctionIfControlUseErrorV1::WrongOrder);
        }
        Err(FunctionIfControlUseErrorV1::Unexpected)
    }

    pub(crate) fn finish(self) -> Result<(), FunctionIfControlUseErrorV1> {
        if self.next == self.expected_sites.len() {
            Ok(())
        } else {
            Err(FunctionIfControlUseErrorV1::Missing)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IfControlCoverageUseErrorV1 {
    ForeignOwner,
    Duplicate,
    WrongOrder,
    Unexpected,
    Missing,
}

#[derive(Debug)]
pub(crate) struct IfControlCoverageUseV1 {
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) expected: Box<[CoveredSourceSiteV1]>,
    pub(super) next: usize,
}

impl IfControlCoverageUseV1 {
    pub(in crate::mir::resolved_control_flow) fn claim(
        &mut self,
        actual: &CoveredSourceSiteV1,
    ) -> Result<(), IfControlCoverageUseErrorV1> {
        if actual.owner() != self.owner {
            return Err(IfControlCoverageUseErrorV1::ForeignOwner);
        }
        if self.expected.get(self.next) == Some(actual) {
            self.next += 1;
            return Ok(());
        }
        if self.expected[..self.next].contains(actual) {
            return Err(IfControlCoverageUseErrorV1::Duplicate);
        }
        if self.expected[self.next..].contains(actual) {
            return Err(IfControlCoverageUseErrorV1::WrongOrder);
        }
        Err(IfControlCoverageUseErrorV1::Unexpected)
    }

    pub(crate) fn finish(self) -> Result<(), IfControlCoverageUseErrorV1> {
        if self.next == self.expected.len() {
            Ok(())
        } else {
            Err(IfControlCoverageUseErrorV1::Missing)
        }
    }
}
