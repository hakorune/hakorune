use super::FunctionMetadata;

impl FunctionMetadata {
    pub(crate) fn admit_checked_callout_plan(
        &mut self,
        plan: crate::mir::checked_callout::CheckedCallOutSitePlanV1,
    ) -> Result<(), crate::mir::checked_callout::CheckedCallOutPlanRejectV1> {
        self.checked_callout_site_plans.admit(plan)
    }

    pub(crate) fn checked_callout_plan(
        &self,
        site: crate::mir::checked_callout::CheckedCallOutSiteIdV1,
    ) -> Option<&crate::mir::checked_callout::CheckedCallOutSitePlanV1> {
        self.checked_callout_site_plans.get(site)
    }


    pub(crate) fn verify_checked_callout_function(
        &self,
        function: &crate::mir::MirFunction,
    ) -> Result<
        crate::mir::checked_callout::VerifiedCheckedCallOutFunctionV1,
        crate::mir::checked_callout::CheckedCallOutFunctionRejectV1,
    > {
        self.checked_callout_site_plans.verify_function(function)
    }
}
