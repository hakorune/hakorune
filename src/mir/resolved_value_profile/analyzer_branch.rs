use std::collections::BTreeSet;

use crate::mir::compiler::located::LocatedBodyV1;
use crate::mir::resolved_semantics::BindingRefV1;

use super::super::analyzer_policy::ReturnPolicyV1;
use super::super::error::AnalysisResultV1;
use super::{AnalyzerV1, ValueEnvironmentV1};

impl<'a> AnalyzerV1<'a> {
    pub(super) fn analyze_branch(
        &mut self,
        body: &LocatedBodyV1<'a>,
        baseline: &ValueEnvironmentV1,
    ) -> AnalysisResultV1<(ValueEnvironmentV1, BTreeSet<BindingRefV1>)> {
        let mut environment = baseline.clone();
        let mut writes = BTreeSet::new();
        self.analyze_body(
            body,
            &mut environment,
            &mut writes,
            ReturnPolicyV1::Forbidden,
        )?;
        environment.retain(|binding, _| baseline.contains_key(binding));
        writes.retain(|binding| baseline.contains_key(binding));
        Ok((environment, writes))
    }
}
