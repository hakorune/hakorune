//! Linear consumption of selected-normal constructor demand tickets.

use super::normal_instance_constructor_admission::{
    InstanceConstructorDemandTicketV1, VerifiedInstanceConstructorPhysicalDemandManifestV1,
};

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
    ) -> Result<(), String> {
        let Some(manifest) = self.manifest.as_ref() else {
            return Err(
                "[freeze:contract][mir/instance-constructor-demand/manifest-missing]".to_owned(),
            );
        };
        let mut found = None;
        for (index, expected) in manifest.expectations().iter().enumerate() {
            if expected.source_id().same_as(ticket.source_id()) && expected.role() == ticket.role()
            {
                if found.replace(index).is_some() {
                    return Err(
                        "[freeze:contract][mir/instance-constructor-demand/duplicate-expected]"
                            .to_owned(),
                    );
                }
            }
        }
        let Some(index) = found else {
            return Err(
                "[freeze:contract][mir/instance-constructor-demand/foreign-ticket]".to_owned(),
            );
        };
        if self.consumed[index] {
            return Err(
                "[freeze:contract][mir/instance-constructor-demand/ticket-reuse]".to_owned(),
            );
        }
        self.consumed[index] = true;
        Ok(())
    }

    pub(super) fn complete(self) -> Result<(), String> {
        let Some(manifest) = self.manifest else {
            return Ok(());
        };
        if self.consumed.iter().all(|consumed| *consumed)
            && self.consumed.len() == manifest.expectations().len()
        {
            Ok(())
        } else {
            Err("[freeze:contract][mir/instance-constructor-demand/incomplete]".to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::normal_instance_constructor_admission::{
        InstanceConstructorDemandManifestBuilderV1, InstanceConstructorDemandRoleV1,
        NormalInstanceConstructorSourceBatchV1,
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
        assert!(ledger
            .consume(InstanceConstructorDemandTicketV1::issue(
                &source_id,
                InstanceConstructorDemandRoleV1::ImmediateDeclaration,
            ))
            .is_err());
        assert!(ledger
            .consume(InstanceConstructorDemandTicketV1::issue(
                &source_id,
                InstanceConstructorDemandRoleV1::ScriptRuntimePrefix,
            ))
            .is_err());
        ledger.complete().expect("complete after one ticket");
    }

    #[test]
    fn empty_manifest_completes_without_a_constructor_demand() {
        let manifest = InstanceConstructorDemandManifestBuilderV1::default().finish();
        InstanceConstructorDemandConsumptionV1::new(Some(manifest))
            .complete()
            .expect("empty manifest");
    }
}
