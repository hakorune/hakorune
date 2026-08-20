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

fn classify_script_direct_static_claim_ingress_v1(
    ledger_installed: bool,
    context: Option<&RawInvocationSourceContextV1>,
) -> Result<ScriptDirectStaticClaimIngressV1, String> {
    if !ledger_installed {
        return Ok(ScriptDirectStaticClaimIngressV1::Unavailable);
    }
    let Some(context) = context else {
        return Err(
            "[freeze:contract][script-direct-static/claim-ingress-source-context]".to_owned(),
        );
    };
    match context {
        RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::ScriptRoot,
            ..
        } => Ok(ScriptDirectStaticClaimIngressV1::Available),
        RawInvocationSourceContextV1::Located { .. } => {
            Err("[freeze:contract][script-direct-static/claim-ingress-foreign-lineage]".to_owned())
        }
        RawInvocationSourceContextV1::UnlocatedCompatibility { .. } => Err(
            "[freeze:contract][script-direct-static/claim-ingress-source-location-lost]".to_owned(),
        ),
    }
}

impl RawInvocationChildPortV1<'_, '_> {
    pub(in crate::mir::builder) fn script_direct_static_claim_ingress_inner_v1(
        &mut self,
        _box_name: &str,
        _method: &str,
        _argument_count: usize,
    ) -> Result<ScriptDirectStaticClaimIngressV1, String> {
        let context = self.current_source_context_v1();
        classify_script_direct_static_claim_ingress_v1(
            self.semantic_ledger.is_some(),
            context.as_ref(),
        )
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
        let Some(context) = self.current_source_context_v1() else {
            return Err("[freeze:contract][script-direct-static/claim-source-context]".to_owned());
        };
        let RawInvocationSourceContextV1::Located {
            root: RawInvocationRootLineageV1::ScriptRoot,
            site,
            ..
        } = context
        else {
            return Err(
                "[freeze:contract][script-direct-static/claim-source-location-lost]".to_owned(),
            );
        };

        let call_site = SourceExprSiteV1::from_node(site.clone());
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
        let pending = ledger
            .borrow()
            .validate_direct_static_claim(&call_site, |row| {
                let target = row.target();
                if target.namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod
                    || target.owner() != box_name
                    || target.name() != method
                    || target.arity() != expected_arity
                    || row.receiver_site() != &expected_receiver
                    || row.argument_sites() != expected_arguments.as_slice()
                    || !matches!(
                        row.representation(),
                        VerifiedCallableResultRepresentationV1::ExactI64
                    )
                {
                    return Err(
                        "[freeze:contract][script-direct-static/claim-source-drift]".to_owned()
                    );
                }
                Ok(())
            })?;
        if !pending {
            return Ok(ScriptDirectStaticClaimTakeV1::Absent);
        }
        let take = ledger.borrow_mut().take_direct_static_claim(&call_site)?;
        match take {
            ScriptDirectStaticClaimTakeV1::Claimed(claimed) => {
                Ok(ScriptDirectStaticClaimTakeV1::Claimed(claimed))
            }
            ScriptDirectStaticClaimTakeV1::Absent => {
                Err("[freeze:contract][script-direct-static/claim-state-drift]".to_owned())
            }
            ScriptDirectStaticClaimTakeV1::Unavailable => {
                Err("[freeze:contract][script-direct-static/claim-consumer-unavailable]".to_owned())
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::raw_invocation_source_transport::RawUnlocatedPortalV1;
    use crate::mir::builder::RawSourceLocatorV1;

    fn located(root: RawInvocationRootLineageV1) -> RawInvocationSourceContextV1 {
        RawInvocationSourceContextV1::Located {
            root,
            site: SourcePathV1::program_body().node(),
            body_kind: None,
        }
    }

    #[test]
    fn no_ledger_is_unavailable_without_source_context() {
        assert_eq!(
            classify_script_direct_static_claim_ingress_v1(false, None),
            Ok(ScriptDirectStaticClaimIngressV1::Unavailable)
        );
    }

    #[test]
    fn ledger_accepts_only_script_root_context() {
        assert_eq!(
            classify_script_direct_static_claim_ingress_v1(
                true,
                Some(&located(RawInvocationRootLineageV1::ScriptRoot)),
            ),
            Ok(ScriptDirectStaticClaimIngressV1::Available)
        );
    }

    #[test]
    fn ledger_rejects_unlocated_context_before_descent() {
        let context = RawInvocationSourceContextV1::UnlocatedCompatibility {
            reason: RawUnlocatedPortalV1::CallObject,
            expected_lineage: None,
        };
        assert_eq!(
            classify_script_direct_static_claim_ingress_v1(true, Some(&context)),
            Err(
                "[freeze:contract][script-direct-static/claim-ingress-source-location-lost]"
                    .to_owned()
            )
        );
    }

    #[test]
    fn ledger_rejects_foreign_lineage_before_descent() {
        let context = located(RawInvocationRootLineageV1::Main(
            RawSourceLocatorV1::for_test(0, "Main", "main", "Main.main/0", 0),
        ));
        assert_eq!(
            classify_script_direct_static_claim_ingress_v1(true, Some(&context)),
            Err("[freeze:contract][script-direct-static/claim-ingress-foreign-lineage]".to_owned())
        );
    }
}
