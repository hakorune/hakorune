//! One unpublished selected Stage-B instance-function session.

use std::convert::Infallible;

use crate::mir::builder::calls::function_session::{
    LegacyFunctionPayloadPendingSessionV1, LegacyFunctionPayloadSessionErrorV1,
};
use crate::mir::builder::calls::instance_method_draft_preparation::{
    prepare_instance_method_draft_body_v1, run_function_body_step_tree_guard_v1,
    InstanceMethodDraftPreparationRequestV1,
};
use crate::mir::builder::port_aware_function_draft_impl::{
    prepare_port_aware_draft_body_completion_v1, PortAwarePreparedDraftBodyV1,
};
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use crate::mir::preloop_stageb_carrier::{
    PreparedPreloopStageBFunctionIngressV1, PreparedPreloopStageBInstanceDraftSourceV1,
};
use crate::mir::{MirBuilder, MirFunction};

use super::body_schedule::{
    drive_preloop_stageb_body_schedule_v1, CompletedPreloopStageBBodyScheduleV1,
};
use super::session_rejection::{
    PreloopStageBInstanceFunctionPrimaryRejectionV1, PreloopStageBInstanceFunctionStageV1,
    RejectedPreloopStageBInstanceFunctionSessionV1,
};

#[derive(Debug)]
pub(super) struct PreparedPreloopStageBInstanceFunctionV1 {
    source: PreparedPreloopStageBInstanceDraftSourceV1,
    ingress: PreparedPreloopStageBFunctionIngressV1,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct CompletedPreloopStageBInstanceFunctionPayloadV1 {
    schedule: CompletedPreloopStageBBodyScheduleV1,
    _seal: CompletedPreloopStageBInstanceFunctionPayloadSealV1,
}

#[derive(Debug)]
struct CompletedPreloopStageBInstanceFunctionPayloadSealV1;

pub(in crate::mir::builder) struct PendingPreloopStageBInstanceFunctionSessionV1<'builder> {
    pending: LegacyFunctionPayloadPendingSessionV1<
        'builder,
        CompletedPreloopStageBInstanceFunctionPayloadV1,
    >,
    _seal: PendingPreloopStageBInstanceFunctionSessionSealV1,
}

struct PendingPreloopStageBInstanceFunctionSessionSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct CompletedPreloopStageBInstanceFunctionV1 {
    draft: MirFunction,
    payload: CompletedPreloopStageBInstanceFunctionPayloadV1,
    _seal: CompletedPreloopStageBInstanceFunctionSealV1,
}

#[derive(Debug)]
struct CompletedPreloopStageBInstanceFunctionSealV1;

impl PreparedPreloopStageBInstanceFunctionV1 {
    pub(super) fn prepare(
        ingress: PreparedPreloopStageBFunctionIngressV1,
    ) -> Result<Self, PreloopStageBInstanceFunctionPrimaryRejectionV1> {
        let source = match ingress.instance_draft_source() {
            Ok(source) => source,
            Err(cause) => {
                return Err(PreloopStageBInstanceFunctionPrimaryRejectionV1::source(
                    ingress, cause,
                ))
            }
        };
        Ok(Self { source, ingress })
    }

    pub(super) fn discard(self) {
        let _ = self;
    }
}

impl CompletedPreloopStageBInstanceFunctionPayloadV1 {
    pub(super) const fn schedule(&self) -> &CompletedPreloopStageBBodyScheduleV1 {
        &self.schedule
    }

    pub(in crate::mir::builder) fn discard(self) {
        self.schedule.discard();
    }
}

impl PendingPreloopStageBInstanceFunctionSessionV1<'_> {
    #[cfg(test)]
    pub(super) fn parent_is_captured_for_test(&self) -> bool {
        self.pending.parent_is_captured_for_test()
    }

    pub(in crate::mir::builder) fn complete(self) -> CompletedPreloopStageBInstanceFunctionV1 {
        self.pending
            .complete_before_restore(|draft, payload| {
                Ok::<_, Infallible>(CompletedPreloopStageBInstanceFunctionV1 {
                    draft,
                    payload,
                    _seal: CompletedPreloopStageBInstanceFunctionSealV1,
                })
            })
            .expect("infallible Stage-B pending-session completion")
    }
}

impl CompletedPreloopStageBInstanceFunctionV1 {
    pub(in crate::mir::builder) const fn draft(&self) -> &MirFunction {
        &self.draft
    }

    pub(super) const fn payload(&self) -> &CompletedPreloopStageBInstanceFunctionPayloadV1 {
        &self.payload
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (MirFunction, CompletedPreloopStageBInstanceFunctionPayloadV1) {
        (self.draft, self.payload)
    }
}

pub(super) fn capture_preloop_stageb_instance_function_v1<'builder>(
    builder: &'builder mut MirBuilder,
    mut child_port: RawInvocationChildPortV1<'_, '_>,
    prepared: PreparedPreloopStageBInstanceFunctionV1,
) -> Result<
    PendingPreloopStageBInstanceFunctionSessionV1<'builder>,
    RejectedPreloopStageBInstanceFunctionSessionV1,
> {
    let session_name = prepared.source.function_name.clone();
    let body_snapshot = prepared.source.body.clone();
    let pending = builder
        .capture_legacy_function_payload_pending_session_v1(
            &session_name,
            body_snapshot,
            move |builder| {
                lower_preloop_stageb_instance_function_v1(builder, &mut child_port, prepared)
            },
        )
        .map_err(RejectedPreloopStageBInstanceFunctionSessionV1::from_session)?;
    Ok(PendingPreloopStageBInstanceFunctionSessionV1 {
        pending,
        _seal: PendingPreloopStageBInstanceFunctionSessionSealV1,
    })
}

pub(in crate::mir::builder) fn capture_preloop_stageb_instance_function_from_ingress_v1<
    'builder,
>(
    builder: &'builder mut MirBuilder,
    child_port: RawInvocationChildPortV1<'_, '_>,
    ingress: PreparedPreloopStageBFunctionIngressV1,
) -> Result<
    PendingPreloopStageBInstanceFunctionSessionV1<'builder>,
    RejectedPreloopStageBInstanceFunctionSessionV1,
> {
    let prepared = PreparedPreloopStageBInstanceFunctionV1::prepare(ingress)
        .map_err(RejectedPreloopStageBInstanceFunctionSessionV1::from_primary)?;
    capture_preloop_stageb_instance_function_v1(builder, child_port, prepared)
}

fn lower_preloop_stageb_instance_function_v1(
    builder: &mut MirBuilder,
    child_port: &mut RawInvocationChildPortV1<'_, '_>,
    prepared: PreparedPreloopStageBInstanceFunctionV1,
) -> Result<
    (MirFunction, CompletedPreloopStageBInstanceFunctionPayloadV1),
    PreloopStageBInstanceFunctionPrimaryRejectionV1,
> {
    let request = InstanceMethodDraftPreparationRequestV1::new(
        prepared.source.function_name.clone(),
        prepared.source.box_name.clone(),
        prepared.source.params.clone(),
        prepared.source.param_decls.clone(),
        prepared.source.return_type_name.clone(),
        prepared.source.body.clone(),
        prepared.source.uses.clone(),
        prepared.source.attrs.clone(),
    );
    let body = match prepare_instance_method_draft_body_v1(builder, request) {
        Ok(body) => body,
        Err(detail) => {
            return Err(PreloopStageBInstanceFunctionPrimaryRejectionV1::prepared(
                prepared,
                PreloopStageBInstanceFunctionStageV1::Preparation,
                detail,
            ))
        }
    };
    let function_name = prepared.source.function_name.clone();
    if let Err(detail) = run_function_body_step_tree_guard_v1(builder, body.body(), &function_name)
    {
        return Err(PreloopStageBInstanceFunctionPrimaryRejectionV1::prepared(
            prepared,
            PreloopStageBInstanceFunctionStageV1::StepTree,
            detail,
        ));
    }
    let PreparedPreloopStageBInstanceFunctionV1 { source: _, ingress } = prepared;
    let schedule = drive_preloop_stageb_body_schedule_v1(builder, child_port.reborrow(), ingress)
        .map_err(PreloopStageBInstanceFunctionPrimaryRejectionV1::body)?;
    let finalizer = match prepare_port_aware_draft_body_completion_v1(builder) {
        Ok(finalizer) => finalizer,
        Err(detail) => {
            return Err(PreloopStageBInstanceFunctionPrimaryRejectionV1::finalizer(
                schedule, detail,
            ))
        }
    };
    let draft = match child_port
        .with_headers(|headers| builder.finalize_function_draft_with_headers(finalizer, headers))
    {
        Ok(draft) => draft,
        Err(detail) => {
            return Err(PreloopStageBInstanceFunctionPrimaryRejectionV1::finalizer(
                schedule, detail,
            ))
        }
    };
    Ok((
        draft,
        CompletedPreloopStageBInstanceFunctionPayloadV1 {
            schedule,
            _seal: CompletedPreloopStageBInstanceFunctionPayloadSealV1,
        },
    ))
}
