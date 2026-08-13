//! Canonical close-time issuer for the selected A-prime physical receipt.
//!
//! This is a transport projection only.  It borrows the already-emitted
//! CallOut corridor, formal header, and DraftSeal Completion witnesses; it
//! never scans MIR or reselects a role from a string.

use super::callout_corridor::DynamicV2CallOutCorridorV1;
use super::formal_header::DynamicV2OpenedFormalHeaderV1;
use super::{DynamicV2I8EmitterRejectV1, DynamicV2PhysicalSessionBrandV1};
use crate::mir::a_prime_i64_physical_receipt::{
    APrimeI64BackendFamilyV1, APrimeI64CallArgumentReceiptV1, APrimeI64CallEdgeReceiptV1,
    APrimeI64LaneV1, APrimeI64ParameterReceiptV1, APrimeI64PhysicalReceiptV1,
    APrimeI64ReturnReceiptV1, A_PRIME_I64_FORMAL_PARAMETER_COUNT,
};
use crate::mir::builder::resolved_lowering::ReadyFunctionDraftSealV1;
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::loop_recipe_contract::LoopValueKeyV1;

const V0: LoopValueKeyV1 = LoopValueKeyV1::new(0);
const V1: LoopValueKeyV1 = LoopValueKeyV1::new(1);
const V2: LoopValueKeyV1 = LoopValueKeyV1::new(2);

/// Issue exactly once, after the canonical session has closed its physical
/// cursor and before DraftSeal snapshots function metadata.
pub(super) fn issue(
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
    formals: &DynamicV2OpenedFormalHeaderV1,
    corridor: &DynamicV2CallOutCorridorV1,
    ready: &ReadyFunctionDraftSealV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<APrimeI64PhysicalReceiptV1, DynamicV2I8EmitterRejectV1> {
    if !corridor.matches(brand) {
        return Err(reject(
            "A-prime receipt corridor has a foreign session brand",
        ));
    }
    let pos = formals
        .value_for_recipe(V1)
        .ok_or_else(|| reject("A-prime receipt missing pos formal"))?;
    let end = formals
        .value_for_recipe(V2)
        .ok_or_else(|| reject("A-prime receipt missing end formal"))?;
    let parameters = vec![
        APrimeI64ParameterReceiptV1 {
            role: "pos".into(),
            formal_parameter_index: 1,
            value_id: pos,
            lane: APrimeI64LaneV1::ImmediateI64,
        },
        APrimeI64ParameterReceiptV1 {
            role: "end".into(),
            formal_parameter_index: 2,
            value_id: end,
            lane: APrimeI64LaneV1::ImmediateI64,
        },
    ];
    let calls = vec![
        APrimeI64CallEdgeReceiptV1 {
            site_id: corridor.i6_site(),
            role: "substring".into(),
            target_fingerprint: "substring/2".into(),
            receiver_role: "src".into(),
            receiver_value_id: corridor.i6_receiver(),
            receiver_lane: APrimeI64LaneV1::OpaqueHandle,
            arguments: ["start", "end"]
                .into_iter()
                .zip(corridor.i6_arguments())
                .enumerate()
                .map(
                    |(ordinal, (role, value_id))| APrimeI64CallArgumentReceiptV1 {
                        ordinal,
                        role: role.into(),
                        value_id,
                        lane: APrimeI64LaneV1::ImmediateI64,
                    },
                )
                .collect(),
            result_value_id: corridor.i6_result(),
            result_lane: APrimeI64LaneV1::OpaqueHandle,
        },
        APrimeI64CallEdgeReceiptV1 {
            site_id: corridor.i7_site(),
            role: "index_of".into(),
            target_fingerprint: "indexOf/1".into(),
            receiver_role: "pred_chars".into(),
            receiver_value_id: corridor.i7_receiver(),
            receiver_lane: APrimeI64LaneV1::OpaqueHandle,
            arguments: vec![APrimeI64CallArgumentReceiptV1 {
                ordinal: 0,
                role: "ch".into(),
                value_id: corridor.i7_argument(),
                lane: APrimeI64LaneV1::OpaqueHandle,
            }],
            result_value_id: corridor.i7_result(),
            result_lane: APrimeI64LaneV1::ImmediateI64,
        },
    ];
    let completion_sites = demand.source_relation().completion_sites();
    let observations = ready.return_observations().map_err(|error| reject(error))?;
    let mut returns = [None, None];
    for observation in observations {
        let slot = if *observation.site() == *completion_sites[0] {
            &mut returns[0]
        } else if *observation.site() == *completion_sites[1] {
            &mut returns[1]
        } else {
            return Err(reject(
                "Completion return site is foreign to A-prime relation",
            ));
        };
        if slot.is_some() {
            return Err(reject(
                "duplicate Completion return site in A-prime receipt",
            ));
        }
        *slot = Some(APrimeI64ReturnReceiptV1 {
            site: if *observation.site() == *completion_sites[0] {
                "inner"
            } else {
                "outer"
            }
            .into(),
            block: observation.block(),
            value_id: observation.value(),
            lane: APrimeI64LaneV1::ImmediateI64,
        });
    }
    let returns = returns
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| reject("A-prime receipt is missing an exact Completion return"))?;
    APrimeI64PhysicalReceiptV1::seal(
        APrimeI64BackendFamilyV1::Llvm,
        A_PRIME_I64_FORMAL_PARAMETER_COUNT,
        parameters,
        calls,
        returns,
    )
    .map_err(|error| reject(format!("A-prime receipt validation: {error:?}")))
}

fn reject(message: impl Into<String>) -> DynamicV2I8EmitterRejectV1 {
    DynamicV2I8EmitterRejectV1::DraftSeal(message.into())
}
