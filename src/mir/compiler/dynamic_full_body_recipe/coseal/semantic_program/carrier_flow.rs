//! Semantic iteration-flow projection for opaque Dynamic carriers.
//!
//! The rebind transaction remains the sole source of the exact I13/I15/I16
//! relation.  This child only records the recurrence/disposition law around
//! that relation and borrows the already-issued invocation/operator rows.  It
//! never creates a second lifecycle catalog or executes an end operation.

use crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1;
use crate::mir::loop_recipe_contract::{LoopBindingKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use crate::mir::resolved_semantics::SourceStmtSiteV1;

use super::super::{
    DynamicCarrierCurrentDispositionV1, DynamicInvocationCarrierDestinationRefV1,
    DynamicInvocationCarrierPublicationV1, DynamicOperatorCarrierDestinationRefV1,
    DynamicOperatorCarrierPublicationV1, VerifiedDynamicCarrierRebindTransactionProgramV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicCarrierFlowProgramRejectV1 {
    InvocationCoverage,
    InvocationDestination,
    OperatorCoverage,
    OperatorDestination,
    RebindRelation,
    DuplicatePublication,
}

/// Logical disposition only. `EndAuthorized` authorizes a later physical
/// projection; it is not a physical End instruction or cleanup receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicCarrierFlowStateV1 {
    Absent,
    Live,
    EndAuthorized,
    Forwarded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicCarrierFlowBoundaryV1 {
    AfterInvocationOutcome {
        invocation: LoopItemKeyV1,
    },
    FullExpressionBoundary {
        item: LoopItemKeyV1,
    },
    LoopBodyExit {
        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
    },
    ForwardAtRebind {
        write: LoopItemKeyV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DynamicCarrierFlowCurrentInputV1 {
    InitialBorrowedIngress,
    PriorIterationForwarded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DynamicCarrierFlowPublicationRuleV1 {
    producer: LoopItemKeyV1,
    result: LoopValueKeyV1,
    publication: DynamicCarrierFlowStateV1,
    boundary: DynamicCarrierFlowBoundaryV1,
    terminal: DynamicCarrierFlowStateV1,
}

impl DynamicCarrierFlowPublicationRuleV1 {
    pub(super) const fn producer(&self) -> LoopItemKeyV1 {
        self.producer
    }

    pub(super) const fn result(&self) -> LoopValueKeyV1 {
        self.result
    }

    pub(super) const fn boundary(&self) -> DynamicCarrierFlowBoundaryV1 {
        self.boundary
    }

    pub(super) const fn terminal(&self) -> DynamicCarrierFlowStateV1 {
        self.terminal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DynamicCarrierFlowNormalStepV1 {
    current: DynamicCarrierFlowCurrentInputV1,
    replacement: LoopValueKeyV1,
    write: LoopItemKeyV1,
    binding: LoopBindingKeyV1,
    displaced: DynamicCarrierFlowCurrentInputV1,
    new_current: DynamicCarrierFlowCurrentOutputV1,
    backedge: DynamicCarrierFlowBackedgeAuthorizationV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DynamicCarrierFlowCurrentOutputV1 {
    PriorIterationForwarded {
        producer: LoopItemKeyV1,
        result: LoopValueKeyV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DynamicCarrierFlowBackedgeAuthorizationV1 {
    AfterDisplacedAndBodyLocalDischarge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DynamicCarrierFlowFaultStepV1 {
    PreserveCurrentNoReplacementNoBackedge,
}

#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicCarrierFlowProgramV1 {
    rebind: VerifiedDynamicCarrierRebindTransactionProgramV1,
    publications: [DynamicCarrierFlowPublicationRuleV1; 4],
    normal: DynamicCarrierFlowNormalStepV1,
    fault: DynamicCarrierFlowFaultStepV1,
}

impl VerifiedDynamicCarrierFlowProgramV1 {
    #[cfg(test)]
    pub(in crate::mir) fn current(&self) -> DynamicCarrierCurrentDispositionV1 {
        self.rebind.current()
    }

    pub(super) fn publications(&self) -> &[DynamicCarrierFlowPublicationRuleV1; 4] {
        &self.publications
    }

    pub(super) fn backedge(
        &self,
    ) -> (
        crate::mir::loop_recipe_contract::LoopNodeKeyV1,
        LoopItemKeyV1,
    ) {
        (self.rebind.commit.backedge_loop, self.normal.write)
    }

    pub(super) fn return_partition(&self) -> Option<[SourceStmtSiteV1; 2]> {
        use crate::mir::compiler::dynamic_full_body_source::{
            DynamicFullBodySourceRoleV1, DynamicFullBodySourceSiteV1,
        };
        let semantic = &self.rebind.ingress.program().invocation_program.program;
        let source = &semantic.envelope.source;
        let statement = |role| {
            source.rows.iter().find_map(|row| {
                (row.role() == role).then(|| match row.site() {
                    DynamicFullBodySourceSiteV1::Statement(site) => Some(site.clone()),
                    DynamicFullBodySourceSiteV1::Expression(_) => None,
                })?
            })
        };
        let inner = statement(DynamicFullBodySourceRoleV1::InnerReturn)?;
        let outer = statement(DynamicFullBodySourceRoleV1::OuterReturn)?;
        let completion = source
            .completion
            .explicit_sites()
            .iter()
            .collect::<std::collections::BTreeSet<_>>();
        (completion == std::collections::BTreeSet::from([&inner, &outer])).then_some([inner, outer])
    }

    pub(super) fn completion_summary(
        &self,
    ) -> Option<(
        crate::mir::resolved_semantics::FunctionOwnerIdV1,
        crate::mir::resolved_semantics::RegionId,
        bool,
    )> {
        let semantic = &self.rebind.ingress.program().invocation_program.program;
        let completion = &semantic.envelope.source.completion;
        self.return_partition().map(|_| {
            (
                completion.owner(),
                completion.target_function(),
                completion.returns_value(),
            )
        })
    }

    pub(super) fn fault_cut_points(
        &self,
    ) -> crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program::DynamicFullLoopFaultCutPointCatalogRefV2<'_>
    {
        self.rebind.ingress.program().fault_cut_points()
    }

    #[cfg(test)]
    pub(in crate::mir) fn normal(&self) -> DynamicCarrierFlowNormalStepV1 {
        self.normal
    }

    #[cfg(test)]
    pub(in crate::mir) fn fault(&self) -> DynamicCarrierFlowFaultStepV1 {
        self.fault
    }
}

pub(in crate::mir) fn issue_dynamic_carrier_flow_program_v1(
    rebind: VerifiedDynamicCarrierRebindTransactionProgramV1,
) -> Result<VerifiedDynamicCarrierFlowProgramV1, DynamicCarrierFlowProgramRejectV1> {
    let current = match rebind.current() {
        DynamicCarrierCurrentDispositionV1::BorrowedIngressNoEnd => {
            DynamicCarrierFlowCurrentInputV1::InitialBorrowedIngress
        }
        DynamicCarrierCurrentDispositionV1::OwnedCarrierEndExactlyOnceUnlessForwarded => {
            DynamicCarrierFlowCurrentInputV1::PriorIterationForwarded
        }
    };

    let operator = rebind.ingress.program();
    let invocation_rows = operator.invocation_lifecycle().rows().collect::<Vec<_>>();
    if invocation_rows.len() != 2 {
        return Err(DynamicCarrierFlowProgramRejectV1::InvocationCoverage);
    }
    let mut local = None;
    let mut temporary = None;
    for row in invocation_rows {
        if row.publication() != DynamicInvocationCarrierPublicationV1::OnNormalResultPublication
            || row.lifecycle() != DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded
        {
            return Err(DynamicCarrierFlowProgramRejectV1::InvocationDestination);
        }
        match row.destination() {
            DynamicInvocationCarrierDestinationRefV1::LoopBodyLocal { borrowed_by, .. } => {
                if local
                    .replace((row.producer(), row.result(), borrowed_by))
                    .is_some()
                {
                    return Err(DynamicCarrierFlowProgramRejectV1::DuplicatePublication);
                }
            }
            DynamicInvocationCarrierDestinationRefV1::FullExpressionTemporary {
                boundary_item,
                ..
            } => {
                if temporary
                    .replace((row.producer(), row.result(), boundary_item))
                    .is_some()
                {
                    return Err(DynamicCarrierFlowProgramRejectV1::DuplicatePublication);
                }
            }
        }
    }
    let Some((local_producer, local_result, borrowed_by)) = local else {
        return Err(DynamicCarrierFlowProgramRejectV1::InvocationCoverage);
    };
    let Some((temporary_producer, temporary_result, temporary_boundary)) = temporary else {
        return Err(DynamicCarrierFlowProgramRejectV1::InvocationCoverage);
    };

    let operator_rows = operator.operator_lifecycle().rows().collect::<Vec<_>>();
    if operator_rows.len() != 2 {
        return Err(DynamicCarrierFlowProgramRejectV1::OperatorCoverage);
    }
    let mut end_after_invocation = None;
    let mut forward_at_rebind = None;
    for row in operator_rows {
        if row.publication() != DynamicOperatorCarrierPublicationV1::OnNormalResultPublication
            || row.contract().lifecycle()
                != Some(DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded)
        {
            return Err(DynamicCarrierFlowProgramRejectV1::OperatorDestination);
        }
        match row.destination() {
            DynamicOperatorCarrierDestinationRefV1::EndAfterInvocationNormalOrFaultOutcome {
                invocation,
                ..
            } => {
                if end_after_invocation
                    .replace((row.producer(), row.result(), invocation))
                    .is_some()
                {
                    return Err(DynamicCarrierFlowProgramRejectV1::DuplicatePublication);
                }
            }
            DynamicOperatorCarrierDestinationRefV1::ForwardToBindingAtRebindCommit {
                write,
                binding,
                backedge_loop,
                ..
            } => {
                if forward_at_rebind
                    .replace((row.producer(), row.result(), write, binding, backedge_loop))
                    .is_some()
                {
                    return Err(DynamicCarrierFlowProgramRejectV1::DuplicatePublication);
                }
            }
        }
    }
    let Some((end_producer, end_result, invocation)) = end_after_invocation else {
        return Err(DynamicCarrierFlowProgramRejectV1::OperatorCoverage);
    };
    let Some((forward_producer, forward_result, write, binding, backedge_loop)) = forward_at_rebind
    else {
        return Err(DynamicCarrierFlowProgramRejectV1::OperatorCoverage);
    };
    if temporary_producer != borrowed_by
        || invocation != local_producer
        || forward_result != rebind.commit.result
        || write != rebind.commit.write
        || binding != rebind.commit.binding
        || backedge_loop != rebind.commit.backedge_loop
    {
        return Err(DynamicCarrierFlowProgramRejectV1::RebindRelation);
    }

    let publications = [
        DynamicCarrierFlowPublicationRuleV1 {
            producer: end_producer,
            result: end_result,
            publication: DynamicCarrierFlowStateV1::Live,
            boundary: DynamicCarrierFlowBoundaryV1::AfterInvocationOutcome { invocation },
            terminal: DynamicCarrierFlowStateV1::EndAuthorized,
        },
        DynamicCarrierFlowPublicationRuleV1 {
            producer: local_producer,
            result: local_result,
            publication: DynamicCarrierFlowStateV1::Live,
            boundary: DynamicCarrierFlowBoundaryV1::LoopBodyExit {
                loop_key: backedge_loop,
            },
            terminal: DynamicCarrierFlowStateV1::EndAuthorized,
        },
        DynamicCarrierFlowPublicationRuleV1 {
            producer: temporary_producer,
            result: temporary_result,
            publication: DynamicCarrierFlowStateV1::Live,
            boundary: DynamicCarrierFlowBoundaryV1::FullExpressionBoundary {
                item: temporary_boundary,
            },
            terminal: DynamicCarrierFlowStateV1::EndAuthorized,
        },
        DynamicCarrierFlowPublicationRuleV1 {
            producer: forward_producer,
            result: forward_result,
            publication: DynamicCarrierFlowStateV1::Live,
            boundary: DynamicCarrierFlowBoundaryV1::ForwardAtRebind { write },
            terminal: DynamicCarrierFlowStateV1::Forwarded,
        },
    ];
    let mut seen = std::collections::BTreeSet::new();
    for row in publications {
        if !seen.insert((row.producer, row.result)) {
            return Err(DynamicCarrierFlowProgramRejectV1::DuplicatePublication);
        }
    }

    Ok(VerifiedDynamicCarrierFlowProgramV1 {
        rebind,
        publications,
        normal: DynamicCarrierFlowNormalStepV1 {
            current,
            replacement: forward_result,
            write,
            binding,
            displaced: current,
            new_current: DynamicCarrierFlowCurrentOutputV1::PriorIterationForwarded {
                producer: forward_producer,
                result: forward_result,
            },
            backedge:
                DynamicCarrierFlowBackedgeAuthorizationV1::AfterDisplacedAndBodyLocalDischarge,
        },
        fault: DynamicCarrierFlowFaultStepV1::PreserveCurrentNoReplacementNoBackedge,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::dynamic_full_body_recipe::coseal::tests::fixture;
    use crate::mir::compiler::dynamic_full_body_recipe::coseal::{
        issue_dynamic_carrier_ingress_lifecycle_program_v1,
        issue_dynamic_carrier_rebind_transaction_program_v1,
        issue_dynamic_full_loop_semantic_program_v2,
        issue_dynamic_full_loop_source_recipe_envelope_v2,
        issue_dynamic_invocation_carrier_lifecycle_program_v1,
        issue_dynamic_operator_carrier_lifecycle_program_v1,
    };
    use crate::mir::compiler::dynamic_full_body_source::DynamicFullBodyBindingRoleV1;
    use crate::mir::resolved_semantics::HomeDemandV1;

    fn exact_flow() -> VerifiedDynamicCarrierFlowProgramV1 {
        let fixture = fixture(true);
        let parameter_binding = fixture
            .candidate
            .source
            .bindings
            .iter()
            .find(|row| row.role() == DynamicFullBodyBindingRoleV1::Pos)
            .expect("pos binding")
            .binding();
        let envelope =
            issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, &fixture.calls)
                .expect("source/Recipe envelope");
        let semantic =
            issue_dynamic_full_loop_semantic_program_v2(envelope).expect("semantic program");
        let invocation = issue_dynamic_invocation_carrier_lifecycle_program_v1(semantic)
            .expect("invocation lifecycle");
        let operator = issue_dynamic_operator_carrier_lifecycle_program_v1(invocation)
            .expect("operator lifecycle");
        let ingress = issue_dynamic_carrier_ingress_lifecycle_program_v1(
            operator,
            1,
            parameter_binding,
            HomeDemandV1::Handle,
        )
        .expect("ingress lifecycle");
        let rebind =
            issue_dynamic_carrier_rebind_transaction_program_v1(ingress).expect("rebind relation");
        issue_dynamic_carrier_flow_program_v1(rebind).expect("carrier flow")
    }

    #[test]
    fn exact_iteration_flow_seals_publications_and_rebind_transition() {
        let flow = exact_flow();
        assert_eq!(
            flow.current(),
            DynamicCarrierCurrentDispositionV1::BorrowedIngressNoEnd
        );
        let rows = flow
            .publications
            .iter()
            .map(|row| (row.producer, row.result, row.boundary, row.terminal))
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 4);
        assert!(rows.iter().any(|row| {
            row.0 == LoopItemKeyV1::new(5)
                && row.1 == LoopValueKeyV1::new(9)
                && row.2
                    == DynamicCarrierFlowBoundaryV1::AfterInvocationOutcome {
                        invocation: LoopItemKeyV1::new(6),
                    }
                && row.3 == DynamicCarrierFlowStateV1::EndAuthorized
        }));
        assert!(rows.iter().any(|row| {
            row.0 == LoopItemKeyV1::new(6)
                && row.1 == LoopValueKeyV1::new(10)
                && row.2
                    == DynamicCarrierFlowBoundaryV1::LoopBodyExit {
                        loop_key: crate::mir::loop_recipe_contract::LoopNodeKeyV1::new(0),
                    }
                && row.3 == DynamicCarrierFlowStateV1::EndAuthorized
        }));
        assert!(rows.iter().any(|row| {
            row.0 == LoopItemKeyV1::new(7)
                && row.1 == LoopValueKeyV1::new(11)
                && row.2
                    == DynamicCarrierFlowBoundaryV1::FullExpressionBoundary {
                        item: LoopItemKeyV1::new(9),
                    }
                && row.3 == DynamicCarrierFlowStateV1::EndAuthorized
        }));
        assert!(rows.iter().any(|row| {
            row.0 == LoopItemKeyV1::new(15)
                && row.1 == LoopValueKeyV1::new(17)
                && row.2
                    == DynamicCarrierFlowBoundaryV1::ForwardAtRebind {
                        write: LoopItemKeyV1::new(16),
                    }
                && row.3 == DynamicCarrierFlowStateV1::Forwarded
        }));
        assert_eq!(
            flow.normal.current,
            DynamicCarrierFlowCurrentInputV1::InitialBorrowedIngress
        );
        assert_eq!(flow.normal.replacement, LoopValueKeyV1::new(17));
        assert_eq!(flow.normal.write, LoopItemKeyV1::new(16));
        assert_eq!(
            flow.normal.backedge,
            DynamicCarrierFlowBackedgeAuthorizationV1::AfterDisplacedAndBodyLocalDischarge
        );
        assert_eq!(
            flow.fault,
            DynamicCarrierFlowFaultStepV1::PreserveCurrentNoReplacementNoBackedge
        );
    }

    #[test]
    fn carrier_flow_has_no_physical_or_fallback_authority() {
        let source = include_str!("carrier_flow.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("carrier flow production source");
        for forbidden in [
            "ValueId",
            "MirBuilder",
            "ReleaseStrong",
            "Completion",
            "fallback",
            "retry",
            "install_new",
            "take_old",
        ] {
            assert!(
                !source.contains(forbidden),
                "forbidden term in carrier flow: {forbidden}"
            );
        }
    }
}
