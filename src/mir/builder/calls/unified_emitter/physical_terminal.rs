//! Source-neutral terminal for the finalized generic unified Call.
//!
//! This module owns the sole typed value receipt for the generic physical
//! `MirInstruction::Call` branch. Compatibility routes, rewrites, BoxCall,
//! and calls without a destination never construct that receipt.

use crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1;
use crate::mir::builder::{MirBuilder, MirInstruction, ValueId};
use crate::mir::definitions::call_unified::MirCall;

use super::post_success::PreparedUnifiedCallPostSuccessV1;

/// One successful generic physical Call with an exact final destination.
///
/// The private seal deliberately keeps this product non-forgeable outside the
/// sole physical terminal. Source policy and result typing do not belong here.
#[derive(Debug)]
pub(in crate::mir::builder) struct CompletedUnifiedValueCallEmissionV1 {
    final_destination: ValueId,
    _seal: CompletedUnifiedValueCallEmissionSealV1,
}

#[derive(Debug)]
struct CompletedUnifiedValueCallEmissionSealV1;

impl CompletedUnifiedValueCallEmissionV1 {
    pub(in crate::mir::builder) const fn final_destination(&self) -> ValueId {
        self.final_destination
    }
}

/// Result of the generic physical terminal before a value receipt is required.
#[derive(Debug)]
pub(super) enum CompletedUnifiedCallEmissionV1 {
    NoDestination,
    Value(CompletedUnifiedValueCallEmissionV1),
}

/// A successful unified-emitter route which is not the generic Call terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UnifiedCallAlternateRouteV1 {
    EarlyStringLikeRewrite,
    SpecialEqualsRewrite,
    KnownOrUniqueRewrite,
    AdditionalGlobalResolver,
    KnownArrayWrite,
    BoxCall,
}

/// Internal outcome used by compatibility facades and the later receipt API.
#[derive(Debug)]
pub(super) enum UnifiedCallEmissionOutcomeV1 {
    Alternate(UnifiedCallAlternateRouteV1),
    Generic(CompletedUnifiedCallEmissionV1),
}

/// Emit the already-finalized generic Call and commit its existing
/// post-success facts before constructing a typed value receipt.
pub(super) fn emit_finalized_generic_call_v1(
    builder: &mut MirBuilder,
    call: MirCall,
    map_write_replay: Option<
        crate::mir::builder::types::map_value::post_success::PreparedMapWriteReplayV1,
    >,
    lookup: Option<&dyn FunctionSignatureLookupV1>,
) -> Result<CompletedUnifiedCallEmissionV1, String> {
    let final_destination = call.dst;
    let prepared_post_success = PreparedUnifiedCallPostSuccessV1::prepare(
        call.dst,
        &call.callee,
        &call.args,
        map_write_replay,
        lookup,
    );

    let call_inst = MirInstruction::Call {
        dst: call.dst,
        func: ValueId::INVALID, // Compatibility field; Callee is the call target SSOT.
        callee: Some(call.callee),
        args: call.args,
        effects: call.effects,
    };

    builder.emit_instruction(call_inst)?;
    prepared_post_success.commit_after_success(builder);

    Ok(match final_destination {
        Some(final_destination) => {
            CompletedUnifiedCallEmissionV1::Value(CompletedUnifiedValueCallEmissionV1 {
                final_destination,
                _seal: CompletedUnifiedValueCallEmissionSealV1,
            })
        }
        None => CompletedUnifiedCallEmissionV1::NoDestination,
    })
}
