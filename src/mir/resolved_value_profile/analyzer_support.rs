use crate::mir::compiler::located::LocatedStmtV1;

use super::super::error::{AnalysisFailureV1, AnalysisResultV1, TrivialProfileContractErrorV1};
use super::AnalyzerV1;

impl<'a> AnalyzerV1<'a> {
    pub(super) fn source_navigation(&self, error: impl ToString) -> AnalysisFailureV1 {
        TrivialProfileContractErrorV1::SourceNavigation {
            detail: error.to_string(),
        }
        .into()
    }

    pub(super) fn claim_if_control(
        &mut self,
        statement: &LocatedStmtV1<'a>,
    ) -> AnalysisResultV1<()> {
        let site = statement.site().clone();
        if !self.expected_if_sites.contains(&site) || !self.visited_if_sites.insert(site.clone()) {
            return Err(TrivialProfileContractErrorV1::IfControlCoverageMismatch {
                missing: Box::new([]),
                extra: vec![site].into_boxed_slice(),
            }
            .into());
        }
        Ok(())
    }

    pub(super) fn verify_if_control_coverage(&self) -> AnalysisResultV1<()> {
        if self.expected_if_sites == self.visited_if_sites {
            return Ok(());
        }
        Err(TrivialProfileContractErrorV1::IfControlCoverageMismatch {
            missing: self
                .expected_if_sites
                .difference(&self.visited_if_sites)
                .cloned()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            extra: self
                .visited_if_sites
                .difference(&self.expected_if_sites)
                .cloned()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
        .into())
    }
}
