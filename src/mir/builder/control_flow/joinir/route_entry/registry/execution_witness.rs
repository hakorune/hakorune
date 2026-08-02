//! Execution-scoped observation for one already-selected registry schedule.
//!
//! The router issues this immediately before registry execution.  It captures
//! observations only: selection, facts, environment, and recipe-contract
//! presence.  It does not decide route policy, terminality, or lowering.

use super::{
    route_id::LoopRouteId,
    types::{RouterEnv, SharedAbsentContractDeclineRouteV1},
};

/// The recipe-contract observation captured at the registry boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RecipeContractDispositionV1 {
    Present,
    Absent,
}

impl RecipeContractDispositionV1 {
    pub(crate) fn from_present(recipe_contract_present: bool) -> Self {
        if recipe_contract_present {
            Self::Present
        } else {
            Self::Absent
        }
    }
}

/// One non-Clone execution observation for the selected raw registry schedule.
///
/// The witness is intentionally consumed by `execute_selected_in_order`.  A
/// caller receives it back only when every captured route declined execution.
pub(crate) struct RouteExecutionWitnessV1<'execution> {
    raw_schedule: &'execution [LoopRouteId],
    env: &'execution RouterEnv,
    recipe_contract: RecipeContractDispositionV1,
}

impl<'execution> RouteExecutionWitnessV1<'execution> {
    /// Issues one witness from observations already computed by the router.
    pub(crate) fn issue(
        raw_schedule: &'execution [LoopRouteId],
        env: &'execution RouterEnv,
        recipe_contract_present: bool,
    ) -> Self {
        Self {
            raw_schedule,
            env,
            recipe_contract: RecipeContractDispositionV1::from_present(recipe_contract_present),
        }
    }

    pub(crate) fn raw_schedule(&self) -> &[LoopRouteId] {
        self.raw_schedule
    }

    pub(crate) fn strict_or_dev(&self) -> bool {
        self.env.strict_or_dev
    }

    pub(crate) fn planner_required(&self) -> bool {
        self.env.planner_required
    }

    pub(crate) fn has_body_local(&self) -> bool {
        self.env.has_body_local
    }

    pub(crate) fn recipe_contract_present(&self) -> bool {
        matches!(self.recipe_contract, RecipeContractDispositionV1::Present)
    }

    /// Runs the exact captured schedule without selection or policy recompute.
    ///
    /// A successful route or an error consumes this witness.  Only the exact
    /// exhausted schedule returns it, preserving the single execution scope.
    pub(crate) fn execute_selected_in_order<T, E>(
        self,
        mut execute: impl FnMut(
            &Self,
            &RouteExecutionAttemptV1<'_, 'execution>,
        ) -> Result<RouteAttemptOutcomeV1<T>, E>,
    ) -> Result<RouteExecutionResultV1<'execution, T>, E> {
        for cursor in 0..self.raw_schedule.len() {
            let attempt = RouteExecutionAttemptV1 {
                witness: &self,
                cursor,
            };
            match execute(&self, &attempt)? {
                RouteAttemptOutcomeV1::Succeeded(value) => {
                    return Ok(RouteExecutionResultV1::Succeeded {
                        route: attempt.current_route(),
                        value,
                    });
                }
                RouteAttemptOutcomeV1::PreEffectDeclined(_)
                | RouteAttemptOutcomeV1::PreEffectBlocked(_)
                | RouteAttemptOutcomeV1::PostEffectRetryDebt(_) => {}
                RouteAttemptOutcomeV1::SharedAbsentContractDeclined(decline) => {
                    decline.consume_at(&attempt);
                }
            }
        }
        Ok(RouteExecutionResultV1::Exhausted(self))
    }
}

/// A stack-local observation of one exact route attempt within a witness.
///
/// It cannot outlive the witness execution and exposes only the current route,
/// its cursor, and the exact suffix after that route. It does not grant any
/// scheduling, builder-effect, or retry authority.
pub(crate) struct RouteExecutionAttemptV1<'attempt, 'execution> {
    witness: &'attempt RouteExecutionWitnessV1<'execution>,
    cursor: usize,
}

impl<'attempt, 'execution> RouteExecutionAttemptV1<'attempt, 'execution> {
    pub(crate) fn current_route(&self) -> LoopRouteId {
        self.witness.raw_schedule[self.cursor]
    }

    pub(crate) fn cursor(&self) -> usize {
        self.cursor
    }

    pub(crate) fn exact_after_current_suffix(&self) -> &[LoopRouteId] {
        &self.witness.raw_schedule[self.cursor + 1..]
    }

    pub(crate) fn strict_or_dev(&self) -> bool {
        self.witness.strict_or_dev()
    }

    pub(crate) fn planner_required(&self) -> bool {
        self.witness.planner_required()
    }

    pub(crate) fn has_body_local(&self) -> bool {
        self.witness.has_body_local()
    }

    pub(crate) fn recipe_contract_present(&self) -> bool {
        self.witness.recipe_contract_present()
    }

    /// Issues the exact shared decline only before compose/lower can run.
    pub(crate) fn issue_shared_absent_contract_decline(
        &self,
        policy: SharedAbsentContractDeclineRouteV1,
    ) -> Result<SharedAbsentContractDeclineV1, String> {
        if policy.route_id() != self.current_route() {
            return Err(format!(
                "route_standard absent-contract policy mismatch: expected {}, got {}",
                policy.route_id(),
                self.current_route()
            ));
        }
        if !policy.declines(self.planner_required(), self.recipe_contract_present()) {
            return Err(
                "route_standard issued shared absent-contract decline outside its captured branch"
                    .to_string(),
            );
        }
        Ok(SharedAbsentContractDeclineV1 {
            route: self.current_route(),
        })
    }
}

/// Private outcome for one captured route attempt.
///
/// The legacy scheduler still advances through typed non-success outcomes
/// while M3-E proves the pure policy.  The typed categories prevent policy
/// code from confusing pre-effect decline with post-effect Generic debt.
pub(crate) enum RouteAttemptOutcomeV1<T> {
    Succeeded(T),
    /// A source/policy gate declined before compose or lower touched Builder.
    PreEffectDeclined(PreEffectDeclineReasonV1),
    /// A selected route was blocked by a release/policy gate before effects.
    PreEffectBlocked(PreEffectBlockedReasonV1),
    /// Legacy scheduler debt after a route reached compose/lower or Generic.
    PostEffectRetryDebt(PostEffectRetryDebtV1),
    SharedAbsentContractDeclined(SharedAbsentContractDeclineV1),
}

impl<T> RouteAttemptOutcomeV1<T> {
    /// Seals a selected Loop route at its physical boundary.
    ///
    /// Every non-Generic selected Loop composer emits a Loop-root plan whose
    /// lowering must produce a completion value. `None` therefore terminates
    /// the selected attempt instead of advancing to a later route.
    pub(crate) fn from_selected_loop_option(result: Option<T>) -> Result<Self, String> {
        match result {
            Some(value) => Ok(Self::Succeeded(value)),
            None => Err(crate::mir::builder::control_flow::lower::Freeze::contract(
                "selected Loop route produced no completion after physical lowering",
            )
            .to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreEffectDeclineReasonV1 {
    PlannerRequiredOnly,
    NestedLoopShapeUnavailable,
    NestedLoopFactsUnavailable,
    NestedComposerUnavailable,
    GenericFactsUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreEffectBlockedReasonV1 {
    ReleaseNestedLoopGate,
    ReleaseLoopCondGate,
    SelectedFactsUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PostEffectRetryDebtV1 {
    GenericLegacy,
}

/// Non-Clone proof that this exact cursor took the shared pre-effect decline.
pub(crate) struct SharedAbsentContractDeclineV1 {
    route: LoopRouteId,
}

impl SharedAbsentContractDeclineV1 {
    fn consume_at(&self, attempt: &RouteExecutionAttemptV1<'_, '_>) {
        debug_assert_eq!(self.route, attempt.current_route());
    }
}

/// The sole two outcomes of a fully scoped registry schedule execution.
pub(crate) enum RouteExecutionResultV1<'execution, T> {
    Succeeded { route: LoopRouteId, value: T },
    Exhausted(RouteExecutionWitnessV1<'execution>),
}

#[cfg(test)]
mod tests {
    use super::{
        PreEffectBlockedReasonV1, PreEffectDeclineReasonV1, RouteAttemptOutcomeV1,
        RouteExecutionResultV1, RouteExecutionWitnessV1,
    };
    use crate::mir::builder::control_flow::joinir::route_entry::registry::{
        route_id::LoopRouteId, RouterEnv, SharedAbsentContractDeclineRouteV1,
    };

    fn env() -> RouterEnv {
        RouterEnv {
            strict_or_dev: false,
            planner_required: false,
            has_body_local: false,
        }
    }

    #[test]
    fn witness_captures_router_observations_without_recomputing_them() {
        let env = env();
        let schedule = [LoopRouteId::LoopArrayJoin];
        let witness = RouteExecutionWitnessV1::issue(&schedule, &env, false);

        assert_eq!(witness.raw_schedule(), schedule);
        assert!(!witness.strict_or_dev());
        assert!(!witness.planner_required());
        assert!(!witness.has_body_local());
        assert!(!witness.recipe_contract_present());
    }

    #[test]
    fn exhausted_schedule_returns_the_same_scoped_witness_by_value() {
        let env = env();
        let schedule = [LoopRouteId::LoopTrueEarlyExit, LoopRouteId::SplitScan];
        let witness = RouteExecutionWitnessV1::issue(&schedule, &env, true);
        let mut attempted = Vec::new();

        let result = witness.execute_selected_in_order(|_, attempt| {
            attempted.push(attempt.current_route());
            Ok::<_, ()>(RouteAttemptOutcomeV1::<u8>::PreEffectDeclined(
                PreEffectDeclineReasonV1::NestedLoopShapeUnavailable,
            ))
        });

        let RouteExecutionResultV1::Exhausted(witness) = result.expect("no route errors") else {
            panic!("all declined routes must exhaust the captured schedule");
        };
        assert_eq!(attempted, schedule);
        assert_eq!(witness.raw_schedule(), schedule);
        assert!(witness.recipe_contract_present());
    }

    #[test]
    fn success_stops_at_the_first_route_without_visiting_later_schedule_entries() {
        let env = env();
        let schedule = [LoopRouteId::LoopTrueEarlyExit, LoopRouteId::SplitScan];
        let witness = RouteExecutionWitnessV1::issue(&schedule, &env, false);
        let mut attempted = Vec::new();

        let result = witness.execute_selected_in_order(|_, attempt| {
            let route = attempt.current_route();
            attempted.push(route);
            Ok::<_, ()>(
                RouteAttemptOutcomeV1::from_selected_loop_option(
                    (route == LoopRouteId::LoopTrueEarlyExit).then_some(7_u8),
                )
                .expect("selected loop test route completes"),
            )
        });

        assert!(matches!(
            result,
            Ok(RouteExecutionResultV1::Succeeded {
                route: LoopRouteId::LoopTrueEarlyExit,
                value: 7,
            })
        ));
        assert_eq!(attempted, [LoopRouteId::LoopTrueEarlyExit]);
    }

    #[test]
    fn selected_loop_none_is_a_terminal_error_not_suffix_retry() {
        let result = RouteAttemptOutcomeV1::<u8>::from_selected_loop_option(None);

        assert!(matches!(result, Err(message) if message.contains("no completion")));
    }

    #[test]
    fn blocked_outcome_keeps_legacy_suffix_advance_until_pure_policy_cutover() {
        let env = env();
        let schedule = [LoopRouteId::LoopTrueEarlyExit, LoopRouteId::SplitScan];
        let witness = RouteExecutionWitnessV1::issue(&schedule, &env, false);
        let mut attempted = Vec::new();

        let result = witness.execute_selected_in_order(|_, attempt| {
            attempted.push(attempt.current_route());
            if attempt.current_route() == LoopRouteId::LoopTrueEarlyExit {
                return Ok::<_, ()>(RouteAttemptOutcomeV1::PreEffectBlocked(
                    PreEffectBlockedReasonV1::ReleaseNestedLoopGate,
                ));
            }
            Ok(RouteAttemptOutcomeV1::Succeeded(11_u8))
        });

        assert!(matches!(
            result,
            Ok(RouteExecutionResultV1::Succeeded {
                route: LoopRouteId::SplitScan,
                value: 11,
            })
        ));
        assert_eq!(attempted, schedule);
    }

    #[test]
    fn error_stops_before_later_schedule_entries() {
        let env = env();
        let schedule = [LoopRouteId::LoopTrueEarlyExit, LoopRouteId::SplitScan];
        let witness = RouteExecutionWitnessV1::issue(&schedule, &env, false);
        let mut attempted = Vec::new();

        let result = witness.execute_selected_in_order(|_, attempt| {
            attempted.push(attempt.current_route());
            Err::<RouteAttemptOutcomeV1<u8>, _>("route failed")
        });

        assert!(matches!(result, Err("route failed")));
        assert_eq!(attempted, [LoopRouteId::LoopTrueEarlyExit]);
    }

    #[test]
    fn attempt_observes_its_cursor_and_exact_suffix_without_owning_schedule() {
        let env = env();
        let schedule = [
            LoopRouteId::LoopTrueEarlyExit,
            LoopRouteId::LoopArrayJoin,
            LoopRouteId::SplitScan,
        ];
        let witness = RouteExecutionWitnessV1::issue(&schedule, &env, false);
        let mut observed = Vec::new();

        let result = witness.execute_selected_in_order(|_, attempt| {
            observed.push((
                attempt.current_route(),
                attempt.cursor(),
                attempt.exact_after_current_suffix().to_vec(),
            ));
            Ok::<_, ()>(RouteAttemptOutcomeV1::<u8>::PreEffectDeclined(
                PreEffectDeclineReasonV1::NestedLoopShapeUnavailable,
            ))
        });

        assert!(matches!(result, Ok(RouteExecutionResultV1::Exhausted(_))));
        assert_eq!(
            observed,
            vec![
                (
                    LoopRouteId::LoopTrueEarlyExit,
                    0,
                    vec![LoopRouteId::LoopArrayJoin, LoopRouteId::SplitScan],
                ),
                (LoopRouteId::LoopArrayJoin, 1, vec![LoopRouteId::SplitScan]),
                (LoopRouteId::SplitScan, 2, vec![]),
            ]
        );
    }

    #[test]
    fn shared_decline_advances_only_the_captured_suffix() {
        let env = env();
        let schedule = [LoopRouteId::LoopTrueEarlyExit, LoopRouteId::SplitScan];
        let witness = RouteExecutionWitnessV1::issue(&schedule, &env, false);
        let mut attempted = Vec::new();

        let result = witness.execute_selected_in_order(|_, attempt| {
            attempted.push(attempt.current_route());
            if attempt.current_route() == LoopRouteId::LoopTrueEarlyExit {
                let decline = attempt.issue_shared_absent_contract_decline(
                    SharedAbsentContractDeclineRouteV1::LoopTrueEarlyExit,
                )?;
                return Ok::<_, String>(RouteAttemptOutcomeV1::SharedAbsentContractDeclined(
                    decline,
                ));
            }
            Ok::<_, String>(RouteAttemptOutcomeV1::Succeeded(9_u8))
        });

        assert!(matches!(
            result,
            Ok(RouteExecutionResultV1::Succeeded {
                route: LoopRouteId::SplitScan,
                value: 9,
            })
        ));
        assert_eq!(attempted, schedule);
    }

    #[test]
    fn shared_decline_issuer_rejects_any_nonshared_captured_branch() {
        let schedule = [LoopRouteId::LoopTrueEarlyExit];
        let planner_env = RouterEnv {
            planner_required: true,
            ..env()
        };
        let witness = RouteExecutionWitnessV1::issue(&schedule, &planner_env, false);

        let result = witness.execute_selected_in_order(|_, attempt| {
            attempt
                .issue_shared_absent_contract_decline(
                    SharedAbsentContractDeclineRouteV1::LoopTrueEarlyExit,
                )
                .map(RouteAttemptOutcomeV1::<u8>::SharedAbsentContractDeclined)
        });

        assert!(matches!(result, Err(message) if message.contains("outside its captured branch")));
    }
}
