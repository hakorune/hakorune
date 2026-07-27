//! One-driver prefix/selected/suffix schedule for the bounded Stage-B row.

use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_statement_v1, RawInvocationChildPortV1,
};
use crate::mir::builder::stmts::block_driver::{drive_legacy_block_v1, LegacyBlockDescentPortV1};
use crate::mir::preloop_stageb_carrier::{
    PreparedPreloopStageBFunctionBodyRecipeV1, PreparedPreloopStageBFunctionIngressV1,
};
use crate::mir::source_instance_result_contract::PreparedPreloopLocatedArgumentV1;
use crate::mir::{MirBuilder, ValueId};

use super::super::preloop_located_outer_completion::complete_preloop_located_outer_request_v1;
use super::super::preloop_outer_carrier_assignment::complete_preloop_carrier_assignment_v1;
use super::super::preloop_outer_carrier_transaction::complete_preloop_outer_carrier_call_v1;
use super::super::preloop_outer_carrier_type::{
    publish_preloop_outer_carrier_integer_v1, CompletedPreloopStageBCarrierV1,
};
use super::rejection::{
    OwnedPreloopStageBSelectedTransactionRejectionV1, PreloopStageBBodyScheduleCauseV1,
    PreloopStageBBodyScheduleStageV1, RejectedPreloopStageBBodyScheduleV1,
};

#[derive(Debug)]
enum PreloopStageBBodyScheduleStateV1<'site, 'view, 'catalog> {
    Pending {
        source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
    },
    Transitioning,
    Published(CompletedPreloopStageBCarrierV1),
    Rejected(RejectedPreloopStageBBodyScheduleV1),
}

pub(super) struct PreloopStageBBodySchedulePortV1<'site, 'view, 'catalog, 'port, 'collector> {
    body: &'catalog [ASTNode],
    ordinary: RawInvocationChildPortV1<'port, 'collector>,
    selected_index: usize,
    suffix_start: usize,
    state: PreloopStageBBodyScheduleStateV1<'site, 'view, 'catalog>,
}

#[derive(Debug)]
pub(super) struct CompletedPreloopStageBBodyScheduleV1 {
    body_value: ValueId,
    carrier: CompletedPreloopStageBCarrierV1,
    _seal: CompletedPreloopStageBBodyScheduleSealV1,
}

#[derive(Debug)]
struct CompletedPreloopStageBBodyScheduleSealV1;

impl CompletedPreloopStageBBodyScheduleV1 {
    pub(super) const fn body_value(&self) -> ValueId {
        self.body_value
    }

    pub(super) const fn carrier(&self) -> &CompletedPreloopStageBCarrierV1 {
        &self.carrier
    }

    pub(super) fn discard(self) {
        self.carrier.discard();
        let _ = self.body_value;
    }
}

impl<'site, 'view, 'catalog, 'port, 'collector>
    PreloopStageBBodySchedulePortV1<'site, 'view, 'catalog, 'port, 'collector>
{
    pub(super) fn prepare(
        ordinary: RawInvocationChildPortV1<'port, 'collector>,
        source: PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog>,
        recipe: PreparedPreloopStageBFunctionBodyRecipeV1,
    ) -> Result<Self, RejectedPreloopStageBBodyScheduleV1> {
        let body = source.selected().parent().view().declaration().body();
        let expected = recipe.body_handoff().body_statement_count() as usize;
        if body.len() != expected {
            let actual = body.len();
            return Err(RejectedPreloopStageBBodyScheduleV1::pending(
                source,
                recipe,
                PreloopStageBBodyScheduleStageV1::Preflight,
                PreloopStageBBodyScheduleCauseV1::BodyCardinalityMismatch { expected, actual },
            ));
        }
        let selected_index = recipe.body_handoff().prefix_statement_count() as usize;
        if selected_index >= body.len() {
            let len = body.len();
            return Err(RejectedPreloopStageBBodyScheduleV1::pending(
                source,
                recipe,
                PreloopStageBBodyScheduleStageV1::Preflight,
                PreloopStageBBodyScheduleCauseV1::SelectedIndexUnavailable {
                    selected: selected_index,
                    len,
                },
            ));
        }
        let suffix_start = recipe.body_handoff().suffix_statement_start() as usize;
        let Some(expected_suffix_start) = selected_index.checked_add(1) else {
            return Err(RejectedPreloopStageBBodyScheduleV1::pending(
                source,
                recipe,
                PreloopStageBBodyScheduleStageV1::Preflight,
                PreloopStageBBodyScheduleCauseV1::SelectedIndexUnavailable {
                    selected: selected_index,
                    len: body.len(),
                },
            ));
        };
        if suffix_start != expected_suffix_start || suffix_start > body.len() {
            return Err(RejectedPreloopStageBBodyScheduleV1::pending(
                source,
                recipe,
                PreloopStageBBodyScheduleStageV1::Preflight,
                PreloopStageBBodyScheduleCauseV1::SuffixStartMismatch {
                    expected: expected_suffix_start,
                    actual: suffix_start,
                },
            ));
        }

        Ok(Self {
            body,
            ordinary,
            selected_index,
            suffix_start,
            state: PreloopStageBBodyScheduleStateV1::Pending { source, recipe },
        })
    }

    fn lower_ordinary(
        &mut self,
        builder: &mut MirBuilder,
        index: usize,
        stage: PreloopStageBBodyScheduleStageV1,
    ) -> Result<ValueId, String> {
        match drive_legacy_statement_v1(builder, &mut self.ordinary, self.body[index].clone()) {
            Ok(value) => Ok(value),
            Err(detail) => {
                let state = std::mem::replace(
                    &mut self.state,
                    PreloopStageBBodyScheduleStateV1::Transitioning,
                );
                let cause = PreloopStageBBodyScheduleCauseV1::OrdinaryDescent {
                    index,
                    detail: detail.clone().into_boxed_str(),
                };
                let rejected = match state {
                    PreloopStageBBodyScheduleStateV1::Pending { source, recipe } => {
                        RejectedPreloopStageBBodyScheduleV1::pending(source, recipe, stage, cause)
                    }
                    PreloopStageBBodyScheduleStateV1::Published(carrier) => {
                        RejectedPreloopStageBBodyScheduleV1::published(carrier, stage, cause)
                    }
                    PreloopStageBBodyScheduleStateV1::Rejected(rejected) => rejected,
                    PreloopStageBBodyScheduleStateV1::Transitioning => {
                        unreachable!("private synchronous body-schedule transition escaped")
                    }
                };
                self.state = PreloopStageBBodyScheduleStateV1::Rejected(rejected);
                Err(detail)
            }
        }
    }

    fn lower_selected(&mut self, builder: &mut MirBuilder) -> Result<ValueId, String> {
        let state = std::mem::replace(
            &mut self.state,
            PreloopStageBBodyScheduleStateV1::Transitioning,
        );
        let PreloopStageBBodyScheduleStateV1::Pending { source, recipe } = state else {
            unreachable!("selected body row is reached exactly once by the existing driver")
        };

        let physical = match complete_preloop_located_outer_request_v1(
            builder,
            self.ordinary.reborrow(),
            source,
        ) {
            Ok(physical) => physical,
            Err(rejected) => {
                let detail = rejected.bounded_report();
                let rejected = RejectedPreloopStageBBodyScheduleV1::selected(
                    OwnedPreloopStageBSelectedTransactionRejectionV1::Outer(
                        rejected.into_owned_rejection_v1(),
                    ),
                    detail.into_boxed_str(),
                );
                self.state = PreloopStageBBodyScheduleStateV1::Rejected(rejected);
                return Err("[mir/preloop-stageb/body-schedule/selected-outer]".to_owned());
            }
        };
        let carrier = match complete_preloop_outer_carrier_call_v1(physical, recipe) {
            Ok(carrier) => carrier,
            Err(rejected) => {
                let detail = rejected.bounded_report();
                let rejected = RejectedPreloopStageBBodyScheduleV1::selected(
                    OwnedPreloopStageBSelectedTransactionRejectionV1::Carrier(
                        rejected.into_owned_rejection_v1(),
                    ),
                    detail.into_boxed_str(),
                );
                self.state = PreloopStageBBodyScheduleStateV1::Rejected(rejected);
                return Err("[mir/preloop-stageb/body-schedule/selected-carrier]".to_owned());
            }
        };
        let assignment = match complete_preloop_carrier_assignment_v1(builder, carrier) {
            Ok(assignment) => assignment,
            Err(rejected) => {
                let detail = rejected.bounded_report();
                let rejected = RejectedPreloopStageBBodyScheduleV1::selected(
                    OwnedPreloopStageBSelectedTransactionRejectionV1::Assignment(
                        rejected.into_owned_rejection_v1(),
                    ),
                    detail.into_boxed_str(),
                );
                self.state = PreloopStageBBodyScheduleStateV1::Rejected(rejected);
                return Err("[mir/preloop-stageb/body-schedule/selected-assignment]".to_owned());
            }
        };
        let publication = match publish_preloop_outer_carrier_integer_v1(
            assignment,
            &mut builder.function_state.type_ctx,
        ) {
            Ok(publication) => publication,
            Err(rejected) => {
                let detail = rejected.bounded_report();
                let rejected = RejectedPreloopStageBBodyScheduleV1::selected(
                    OwnedPreloopStageBSelectedTransactionRejectionV1::Publication(
                        rejected.into_owned_rejection_v1(),
                    ),
                    detail,
                );
                self.state = PreloopStageBBodyScheduleStateV1::Rejected(rejected);
                return Err("[mir/preloop-stageb/body-schedule/selected-publication]".to_owned());
            }
        };
        let destination = publication.destination();
        self.state =
            PreloopStageBBodyScheduleStateV1::Published(publication.into_stageb_carrier_v1());
        Ok(destination)
    }

    fn reject_driver(&mut self, detail: String) {
        if matches!(self.state, PreloopStageBBodyScheduleStateV1::Rejected(_)) {
            return;
        }
        let state = std::mem::replace(
            &mut self.state,
            PreloopStageBBodyScheduleStateV1::Transitioning,
        );
        let cause = PreloopStageBBodyScheduleCauseV1::Driver {
            detail: detail.into_boxed_str(),
        };
        let rejected = match state {
            PreloopStageBBodyScheduleStateV1::Pending { source, recipe } => {
                RejectedPreloopStageBBodyScheduleV1::pending(
                    source,
                    recipe,
                    PreloopStageBBodyScheduleStageV1::Prefix,
                    cause,
                )
            }
            PreloopStageBBodyScheduleStateV1::Published(carrier) => {
                RejectedPreloopStageBBodyScheduleV1::published(
                    carrier,
                    PreloopStageBBodyScheduleStageV1::Suffix,
                    cause,
                )
            }
            PreloopStageBBodyScheduleStateV1::Rejected(rejected) => rejected,
            PreloopStageBBodyScheduleStateV1::Transitioning => {
                unreachable!("private synchronous body-schedule transition escaped")
            }
        };
        self.state = PreloopStageBBodyScheduleStateV1::Rejected(rejected);
    }

    fn into_rejection(self) -> RejectedPreloopStageBBodyScheduleV1 {
        match self.state {
            PreloopStageBBodyScheduleStateV1::Rejected(rejected) => rejected,
            PreloopStageBBodyScheduleStateV1::Pending { .. }
            | PreloopStageBBodyScheduleStateV1::Published(_)
            | PreloopStageBBodyScheduleStateV1::Transitioning => {
                unreachable!("driver rejection must retain one typed schedule rejection")
            }
        }
    }

    pub(super) fn finish(
        self,
        body_value: ValueId,
    ) -> Result<CompletedPreloopStageBBodyScheduleV1, RejectedPreloopStageBBodyScheduleV1> {
        match self.state {
            PreloopStageBBodyScheduleStateV1::Published(carrier) => {
                Ok(CompletedPreloopStageBBodyScheduleV1 {
                    body_value,
                    carrier,
                    _seal: CompletedPreloopStageBBodyScheduleSealV1,
                })
            }
            PreloopStageBBodyScheduleStateV1::Pending { source, recipe } => {
                Err(RejectedPreloopStageBBodyScheduleV1::pending(
                    source,
                    recipe,
                    PreloopStageBBodyScheduleStageV1::Completion,
                    PreloopStageBBodyScheduleCauseV1::SelectedNotReached,
                ))
            }
            PreloopStageBBodyScheduleStateV1::Rejected(rejected) => Err(rejected),
            PreloopStageBBodyScheduleStateV1::Transitioning => {
                unreachable!("private synchronous body-schedule transition escaped")
            }
        }
    }
}

impl LegacyBlockDescentPortV1 for PreloopStageBBodySchedulePortV1<'_, '_, '_, '_, '_> {
    type SuffixInput<'a>
        = &'a [ASTNode]
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.body.len()
    }

    fn suffix_route_input(&self, index: usize) -> Result<Option<Self::SuffixInput<'_>>, String> {
        if index < self.selected_index {
            return Ok(Some(&self.body[index..self.selected_index]));
        }
        if index == self.selected_index {
            return Ok(None);
        }
        Ok(
            matches!(self.state, PreloopStageBBodyScheduleStateV1::Published(_))
                .then_some(&self.body[index..]),
        )
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        index: usize,
    ) -> Result<ValueId, String> {
        if index < self.selected_index {
            return self.lower_ordinary(builder, index, PreloopStageBBodyScheduleStageV1::Prefix);
        }
        if index == self.selected_index {
            return self.lower_selected(builder);
        }
        debug_assert!(index >= self.suffix_start);
        if !matches!(self.state, PreloopStageBBodyScheduleStateV1::Published(_)) {
            return Err("[mir/preloop-stageb/body-schedule/suffix-before-publication]".to_owned());
        }
        self.lower_ordinary(builder, index, PreloopStageBBodyScheduleStageV1::Suffix)
    }
}

pub(super) fn drive_preloop_stageb_body_schedule_v1(
    builder: &mut MirBuilder,
    ordinary: RawInvocationChildPortV1<'_, '_>,
    ingress: PreparedPreloopStageBFunctionIngressV1,
) -> Result<CompletedPreloopStageBBodyScheduleV1, RejectedPreloopStageBBodyScheduleV1> {
    match ingress.with_prepared_located_argument(|source, recipe| {
        let mut port = PreloopStageBBodySchedulePortV1::prepare(ordinary, source, recipe)?;
        let body_value = match drive_legacy_block_v1(builder, &mut port) {
            Ok(value) => value,
            Err(detail) => {
                port.reject_driver(detail);
                return Err(port.into_rejection());
            }
        };
        port.finish(body_value)
    }) {
        Ok(result) => result,
        Err(rejected) => Err(RejectedPreloopStageBBodyScheduleV1::ingress(rejected)),
    }
}
