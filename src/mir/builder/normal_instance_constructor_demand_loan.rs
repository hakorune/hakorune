//! Linear consumption of selected-normal constructor demand tickets.

use super::normal_instance_constructor_admission::{
    InstanceConstructorDemandTicketV1, VerifiedInstanceConstructorPhysicalDemandManifestV1,
};

/// Typed failures owned by the linear constructor-demand loan.
///
/// The installed manifest and the move-only ticket remain the authorities.
/// This enum only preserves their consume/complete failures until the
/// existing package-adapter `String` diagnostic boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum InstanceConstructorDemandLoanIssueV1 {
    ManifestMissing,
    DuplicateExpected,
    ForeignTicket,
    TicketReuse,
    Incomplete,
}

impl std::fmt::Display for InstanceConstructorDemandLoanIssueV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let diagnostic = match self {
            Self::ManifestMissing => {
                "[freeze:contract][mir/instance-constructor-demand/manifest-missing]"
            }
            Self::DuplicateExpected => {
                "[freeze:contract][mir/instance-constructor-demand/duplicate-expected]"
            }
            Self::ForeignTicket => {
                "[freeze:contract][mir/instance-constructor-demand/foreign-ticket]"
            }
            Self::TicketReuse => "[freeze:contract][mir/instance-constructor-demand/ticket-reuse]",
            Self::Incomplete => "[freeze:contract][mir/instance-constructor-demand/incomplete]",
        };
        formatter.write_str(diagnostic)
    }
}

#[derive(Debug)]
pub(super) struct InstanceConstructorDemandConsumptionV1 {
    manifest: Option<VerifiedInstanceConstructorPhysicalDemandManifestV1>,
    consumed: Vec<bool>,
}

impl InstanceConstructorDemandConsumptionV1 {
    pub(super) fn new(
        manifest: Option<VerifiedInstanceConstructorPhysicalDemandManifestV1>,
    ) -> Self {
        let consumed = manifest
            .as_ref()
            .map(|manifest| vec![false; manifest.expectations().len()])
            .unwrap_or_default();
        Self { manifest, consumed }
    }

    pub(super) fn consume(
        &mut self,
        ticket: InstanceConstructorDemandTicketV1,
    ) -> Result<(), InstanceConstructorDemandLoanIssueV1> {
        let Some(manifest) = self.manifest.as_ref() else {
            return Err(InstanceConstructorDemandLoanIssueV1::ManifestMissing);
        };
        let mut found = None;
        for (index, expected) in manifest.expectations().iter().enumerate() {
            if expected.source_id().same_as(ticket.source_id()) && expected.role() == ticket.role()
            {
                if found.replace(index).is_some() {
                    return Err(InstanceConstructorDemandLoanIssueV1::DuplicateExpected);
                }
            }
        }
        let Some(index) = found else {
            return Err(InstanceConstructorDemandLoanIssueV1::ForeignTicket);
        };
        if self.consumed[index] {
            return Err(InstanceConstructorDemandLoanIssueV1::TicketReuse);
        }
        self.consumed[index] = true;
        Ok(())
    }

    pub(super) fn complete(self) -> Result<(), InstanceConstructorDemandLoanIssueV1> {
        let Some(manifest) = self.manifest else {
            return Ok(());
        };
        if self.consumed.iter().all(|consumed| *consumed)
            && self.consumed.len() == manifest.expectations().len()
        {
            Ok(())
        } else {
            Err(InstanceConstructorDemandLoanIssueV1::Incomplete)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::normal_instance_constructor_admission::{
        InstanceConstructorDemandExpectationV1, InstanceConstructorDemandManifestBuilderV1,
        InstanceConstructorDemandRoleV1, NormalInstanceConstructorSourceBatchV1,
        VerifiedInstanceConstructorPhysicalDemandManifestV1,
    };

    #[test]
    fn consumes_each_role_once_and_rejects_reuse_or_swap() {
        let batch = NormalInstanceConstructorSourceBatchV1::for_test(
            4,
            "Page",
            ["birth/0".to_owned()],
            InstanceConstructorDemandRoleV1::ImmediateDeclaration,
        );
        let mut builder = InstanceConstructorDemandManifestBuilderV1::default();
        builder.issue_batch(&batch).expect("manifest issue");
        let manifest = builder.finish();
        let mut ledger = InstanceConstructorDemandConsumptionV1::new(Some(manifest));
        let source_id = batch.sources()[0].source_id().clone();
        ledger
            .consume(InstanceConstructorDemandTicketV1::issue(
                &source_id,
                InstanceConstructorDemandRoleV1::ImmediateDeclaration,
            ))
            .expect("first ticket");
        assert_eq!(
            ledger
                .consume(InstanceConstructorDemandTicketV1::issue(
                    &source_id,
                    InstanceConstructorDemandRoleV1::ImmediateDeclaration,
                ))
                .expect_err("ticket reuse"),
            InstanceConstructorDemandLoanIssueV1::TicketReuse
        );
        assert_eq!(
            ledger
                .consume(InstanceConstructorDemandTicketV1::issue(
                    &source_id,
                    InstanceConstructorDemandRoleV1::ScriptRuntimePrefix,
                ))
                .expect_err("foreign role"),
            InstanceConstructorDemandLoanIssueV1::ForeignTicket
        );
        ledger.complete().expect("complete after one ticket");

        let mut missing = InstanceConstructorDemandConsumptionV1::new(None);
        assert_eq!(
            missing
                .consume(InstanceConstructorDemandTicketV1::issue(
                    &source_id,
                    InstanceConstructorDemandRoleV1::ImmediateDeclaration,
                ))
                .expect_err("missing manifest"),
            InstanceConstructorDemandLoanIssueV1::ManifestMissing
        );

        let duplicate_manifest =
            VerifiedInstanceConstructorPhysicalDemandManifestV1::from_expectations_for_test(vec![
                InstanceConstructorDemandExpectationV1::new(
                    &source_id,
                    InstanceConstructorDemandRoleV1::ImmediateDeclaration,
                ),
                InstanceConstructorDemandExpectationV1::new(
                    &source_id,
                    InstanceConstructorDemandRoleV1::ImmediateDeclaration,
                ),
            ]);
        let mut duplicate = InstanceConstructorDemandConsumptionV1::new(Some(duplicate_manifest));
        assert_eq!(
            duplicate
                .consume(InstanceConstructorDemandTicketV1::issue(
                    &source_id,
                    InstanceConstructorDemandRoleV1::ImmediateDeclaration,
                ))
                .expect_err("duplicate expected rows"),
            InstanceConstructorDemandLoanIssueV1::DuplicateExpected
        );

        let mut incomplete_builder = InstanceConstructorDemandManifestBuilderV1::default();
        incomplete_builder
            .issue_batch(&batch)
            .expect("incomplete manifest issue");
        assert_eq!(
            InstanceConstructorDemandConsumptionV1::new(Some(incomplete_builder.finish()))
                .complete()
                .expect_err("unconsumed ticket"),
            InstanceConstructorDemandLoanIssueV1::Incomplete
        );

        let diagnostics = [
            (
                InstanceConstructorDemandLoanIssueV1::ManifestMissing,
                "[freeze:contract][mir/instance-constructor-demand/manifest-missing]",
            ),
            (
                InstanceConstructorDemandLoanIssueV1::DuplicateExpected,
                "[freeze:contract][mir/instance-constructor-demand/duplicate-expected]",
            ),
            (
                InstanceConstructorDemandLoanIssueV1::ForeignTicket,
                "[freeze:contract][mir/instance-constructor-demand/foreign-ticket]",
            ),
            (
                InstanceConstructorDemandLoanIssueV1::TicketReuse,
                "[freeze:contract][mir/instance-constructor-demand/ticket-reuse]",
            ),
            (
                InstanceConstructorDemandLoanIssueV1::Incomplete,
                "[freeze:contract][mir/instance-constructor-demand/incomplete]",
            ),
        ];
        for (issue, expected) in diagnostics {
            assert_eq!(issue.to_string(), expected);
        }
    }

    #[test]
    fn empty_manifest_completes_without_a_constructor_demand() {
        let manifest = InstanceConstructorDemandManifestBuilderV1::default().finish();
        InstanceConstructorDemandConsumptionV1::new(Some(manifest))
            .complete()
            .expect("empty manifest");
    }
}
