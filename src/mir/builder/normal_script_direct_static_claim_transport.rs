//! Selected-normal Script direct-static claim transport.
//!
//! This child owns only the source-ledger accessors for the raw invocation
//! port.  Physical emission remains in the dedicated bridge and the large
//! source transport owner keeps only thin trait forwarding methods.

use crate::ast::ASTNode;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourcePathSegmentV1, SourcePathV1};

use super::super::callable_declaration_catalog::SameModuleCallableNamespaceV1;
use super::super::normal_script_semantic_lowering_state::{
    ScriptDirectStaticClaimTakeV1, ScriptDirectStaticClaimedRowV1,
};
use super::super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawSourceTransportPortV1,
};
use super::super::recursive_child_lowering_port::ScriptDirectStaticClaimIngressV1;
use super::RawInvocationChildPortV1;

impl RawInvocationChildPortV1<'_, '_> {
    pub(in crate::mir::builder) fn script_direct_static_claim_ingress_inner_v1(
        &mut self,
        _box_name: &str,
        _method: &str,
        _argument_count: usize,
    ) -> Result<ScriptDirectStaticClaimIngressV1, String> {
        if self.semantic_ledger.is_none() {
            return Ok(ScriptDirectStaticClaimIngressV1::Unavailable);
        }
        let Some(context) = self.current_source_context_v1() else {
            return Err(
                "[freeze:contract][script-direct-static/claim-ingress-source-context]".to_owned(),
            );
        };
        match context {
            RawInvocationSourceContextV1::Located {
                root: RawInvocationRootLineageV1::ScriptRoot,
                ..
            } => Ok(ScriptDirectStaticClaimIngressV1::Available),
            _ => Ok(ScriptDirectStaticClaimIngressV1::Unavailable),
        }
    }

    pub(in crate::mir::builder) fn take_script_direct_static_claim_inner_v1(
        &mut self,
        box_name: &str,
        method: &str,
        _receiver: &ASTNode,
        arguments: &[ASTNode],
    ) -> Result<ScriptDirectStaticClaimTakeV1, String> {
        let Some(ledger) = self.semantic_ledger.clone() else {
            return Ok(ScriptDirectStaticClaimTakeV1::Unavailable);
        };
        let Some(RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::ScriptRoot,
            site,
            ..
        }) = self.current_source_context_v1()
        else {
            let Some(context) = self.current_source_context_v1() else {
                return Err(
                    "[freeze:contract][script-direct-static/claim-source-context]".to_owned(),
                );
            };
            return match context {
                RawInvocationSourceContextV1::Located { .. }
                | RawInvocationSourceContextV1::UnlocatedCompatibility(_) => {
                    Ok(ScriptDirectStaticClaimTakeV1::Unavailable)
                }
            };
        };

        let call_site = SourceExprSiteV1::from_node(site.clone());
        let take = ledger.borrow_mut().take_direct_static_claim(&call_site)?;
        let ScriptDirectStaticClaimTakeV1::Claimed(claimed) = take else {
            return Ok(take);
        };

        let target = claimed.target();
        let expected_receiver = SourceExprSiteV1::from_node(
            SourcePathV1::from_node(&site)
                .child(SourcePathSegmentV1::Receiver)
                .node(),
        );
        let expected_arguments = arguments
            .iter()
            .enumerate()
            .map(|(index, _)| {
                SourceExprSiteV1::from_node(
                    SourcePathV1::from_node(&site)
                        .child(SourcePathSegmentV1::Argument(index as u32))
                        .node(),
                )
            })
            .collect::<Vec<_>>();
        let expected_arity = u32::try_from(arguments.len()).map_err(|_| {
            "[freeze:contract][script-direct-static/claim-arity-overflow]".to_owned()
        })?;
        if target.namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod
            || target.owner() != box_name
            || target.name() != method
            || target.arity() != expected_arity
            || claimed.row().receiver_site() != &expected_receiver
            || claimed.argument_sites() != expected_arguments.as_slice()
            || !matches!(
                claimed.representation(),
                VerifiedCallableResultRepresentationV1::ExactI64
            )
        {
            return Err("[freeze:contract][script-direct-static/claim-source-drift]".to_owned());
        }
        Ok(ScriptDirectStaticClaimTakeV1::Claimed(claimed))
    }

    pub(in crate::mir::builder) fn complete_script_direct_static_claim_inner_v1(
        &mut self,
        claimed: ScriptDirectStaticClaimedRowV1,
    ) -> Result<(), String> {
        let Some(ledger) = self.semantic_ledger.clone() else {
            return Err(
                "[freeze:contract][script-direct-static/claim-consumer-unavailable]".to_owned(),
            );
        };
        let result = ledger.borrow_mut().complete_direct_static_claim(claimed);
        result
    }
}
