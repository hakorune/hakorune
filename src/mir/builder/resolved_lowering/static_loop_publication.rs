//! One source-bound result-publication cohort for the selected Static I64 Loop.
//! All four rows are checked before their owner moves any of them.

use crate::mir::builder::module_lowering_invocation::ModuleLoweringPortV1;
use crate::mir::builder::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SameModuleCallableCatalogBrandV1};
use crate::mir::definitions::MirCall;
use crate::mir::function::MirFunction;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::normal_callable_semantic_package::VerifiedStaticLoopPacketSourceV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::{BasicBlockId, MirInstruction};

pub(in crate::mir::builder) struct SelectedStaticLoopPublicationBatchV1 {
    brand: SameModuleCallableCatalogBrandV1,
    caller: CanonicalSameModuleCallableKeyV1,
    sites_and_targets: [(SourceExprSiteV1, CanonicalSameModuleCallableKeyV1); 4],
}

impl SelectedStaticLoopPublicationBatchV1 {
    pub(in crate::mir::builder) fn preflight_source(
        semantic: &VerifiedStaticI64LoopSemanticV2,
        entry_packet: &VerifiedStaticLoopPacketSourceV1,
        module_port: &ModuleLoweringPortV1<'_>,
    ) -> Result<Self, String> {
        let (entry, header, body) = semantic.source_calls();
        let tail = semantic.tail_call();
        let originals = [
            entry.original(),
            header.original(),
            body.original(),
            tail.original(),
        ];
        if originals[1..]
            .iter()
            .any(|source| !source.same_catalog_as(originals[0]))
        {
            return Err("[freeze:contract][callable-loop/publication-source-cohort-drift]".into());
        }
        let (caller, entry_site) = entry_packet.publication_source();
        let sites = [
            entry_site,
            header.call_site().site(),
            body.call_site().site(),
            tail.call_site().site(),
        ];
        let handoffs = sites.map(|site| {
            module_port
                .selected_static_result_handoff_for_source(caller, site)
                .ok_or_else(|| {
                    "[freeze:contract][callable-loop/publication-four-site-missing]".to_owned()
                })
        });
        let [entry_handoff, header_handoff, body_handoff, tail_handoff] = handoffs;
        if !entry_packet.corroborates_publication_handoff(entry_handoff?)
            || !header.corroborates_publication_handoff(header_handoff?)
            || !body.corroborates_publication_handoff(body_handoff?)
            || !tail.corroborates_publication_handoff(tail_handoff?)
        {
            return Err("[freeze:contract][callable-loop/publication-four-site-drift]".into());
        }
        Ok(Self {
            brand: entry.publication_brand(),
            caller: caller.clone(),
            sites_and_targets: [
                (entry_site.clone(), entry.original().target().clone()),
                (
                    header.call_site().site().clone(),
                    header.original().target().clone(),
                ),
                (
                    body.call_site().site().clone(),
                    body.original().target().clone(),
                ),
                (
                    tail.call_site().site().clone(),
                    tail.original().target().clone(),
                ),
            ],
        })
    }

    pub(super) fn corroborate_detached_zeroarg_invokes(
        &self,
        function: &MirFunction,
        header: BasicBlockId,
        header_normal: BasicBlockId,
        tail: BasicBlockId,
        tail_normal: BasicBlockId,
    ) -> Result<(), String> {
        self.check_zeroarg(function, 1, header, header_normal)?;
        self.check_zeroarg(function, 3, tail, tail_normal)
    }

    fn check_zeroarg(
        &self,
        function: &MirFunction,
        index: usize,
        source: BasicBlockId,
        normal: BasicBlockId,
    ) -> Result<(), String> {
        let reject =
            || "[freeze:contract][callable-loop/publication-zeroarg-physical-drift]".to_owned();
        let target = self.sites_and_targets[index]
            .1
            .canonical_global_target_v1()
            .map_err(|_| reject())?;
        let expected = MirCall::global(None, target, vec![]);
        let source_block = function.blocks.get(&source).ok_or_else(reject)?;
        let normal_block = function.blocks.get(&normal).ok_or_else(reject)?;
        if !matches!(source_block.terminator.as_ref(),
            Some(MirInstruction::Invoke {
                operation: InvokeOperation::Call { call, result: InvokeCallResultKind::I64 },
                normal_landing, ..
            }) if call == &expected && *normal_landing == normal)
            || normal_block.instructions.iter().filter(|instruction| matches!(instruction,
                MirInstruction::InvokeNormalResult { invoke_block, .. } if *invoke_block == source
            )).count() != 1
        {
            return Err(reject());
        }
        Ok(())
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        SameModuleCallableCatalogBrandV1,
        CanonicalSameModuleCallableKeyV1,
        [(SourceExprSiteV1, CanonicalSameModuleCallableKeyV1); 4],
    ) {
        (self.brand, self.caller, self.sites_and_targets)
    }
}
