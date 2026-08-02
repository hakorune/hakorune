//! Execution-scoped observation for one already-selected registry schedule.
//!
//! The router issues this immediately before registry execution.  It captures
//! observations only: selection, facts, environment, and recipe-contract
//! presence.  It does not decide route policy, terminality, or lowering.

use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;

use super::{route_id::LoopRouteId, types::RouterEnv};

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
    facts: Option<&'execution CanonicalLoopFacts>,
    env: &'execution RouterEnv,
    recipe_contract: RecipeContractDispositionV1,
}

impl<'execution> RouteExecutionWitnessV1<'execution> {
    /// Issues one witness from observations already computed by the router.
    pub(crate) fn issue(
        raw_schedule: &'execution [LoopRouteId],
        facts: Option<&'execution CanonicalLoopFacts>,
        env: &'execution RouterEnv,
        recipe_contract_present: bool,
    ) -> Self {
        Self {
            raw_schedule,
            facts,
            env,
            recipe_contract: RecipeContractDispositionV1::from_present(recipe_contract_present),
        }
    }

    pub(crate) fn raw_schedule(&self) -> &[LoopRouteId] {
        self.raw_schedule
    }

    pub(crate) fn facts(&self) -> Option<&CanonicalLoopFacts> {
        self.facts
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
        mut execute: impl FnMut(&Self, LoopRouteId) -> Result<Option<T>, E>,
    ) -> Result<RouteExecutionResultV1<'execution, T>, E> {
        for route in self.raw_schedule {
            if let Some(value) = execute(&self, *route)? {
                return Ok(RouteExecutionResultV1::Succeeded {
                    route: *route,
                    value,
                });
            }
        }
        Ok(RouteExecutionResultV1::Exhausted(self))
    }
}

/// The sole two outcomes of a fully scoped registry schedule execution.
pub(crate) enum RouteExecutionResultV1<'execution, T> {
    Succeeded { route: LoopRouteId, value: T },
    Exhausted(RouteExecutionWitnessV1<'execution>),
}

#[cfg(test)]
mod tests {
    use super::{RouteExecutionResultV1, RouteExecutionWitnessV1};
    use crate::mir::builder::control_flow::joinir::route_entry::registry::{
        route_id::LoopRouteId, RouterEnv,
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
        let witness = RouteExecutionWitnessV1::issue(&schedule, None, &env, false);

        assert_eq!(witness.raw_schedule(), schedule);
        assert!(witness.facts().is_none());
        assert!(!witness.strict_or_dev());
        assert!(!witness.planner_required());
        assert!(!witness.has_body_local());
        assert!(!witness.recipe_contract_present());
    }

    #[test]
    fn exhausted_schedule_returns_the_same_scoped_witness_by_value() {
        let env = env();
        let schedule = [LoopRouteId::LoopTrueEarlyExit, LoopRouteId::SplitScan];
        let witness = RouteExecutionWitnessV1::issue(&schedule, None, &env, true);
        let mut attempted = Vec::new();

        let result = witness.execute_selected_in_order(|_, route| {
            attempted.push(route);
            Ok::<_, ()>(None::<u8>)
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
        let witness = RouteExecutionWitnessV1::issue(&schedule, None, &env, false);
        let mut attempted = Vec::new();

        let result = witness.execute_selected_in_order(|_, route| {
            attempted.push(route);
            Ok::<_, ()>((route == LoopRouteId::LoopTrueEarlyExit).then_some(7_u8))
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
    fn error_stops_before_later_schedule_entries() {
        let env = env();
        let schedule = [LoopRouteId::LoopTrueEarlyExit, LoopRouteId::SplitScan];
        let witness = RouteExecutionWitnessV1::issue(&schedule, None, &env, false);
        let mut attempted = Vec::new();

        let result = witness.execute_selected_in_order(|_, route| {
            attempted.push(route);
            Err::<Option<u8>, _>("route failed")
        });

        assert!(matches!(result, Err("route failed")));
        assert_eq!(attempted, [LoopRouteId::LoopTrueEarlyExit]);
    }
}
