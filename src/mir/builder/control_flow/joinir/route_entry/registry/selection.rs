//! Pure ordered recipe-first route selection.
//!
//! This module owns predicate evaluation, suppression, and the resulting
//! registry order. It does not receive a Builder or execute a route.

use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;

use super::{predicates::*, route_id::LoopRouteId, ENTRIES};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecipeFirstRouteSelectionV1 {
    facts_present: bool,
    matched: Box<[LoopRouteId]>,
    raw_execution: Box<[LoopRouteId]>,
    diagnostic_effective: Box<[LoopRouteId]>,
}

impl RecipeFirstRouteSelectionV1 {
    pub(crate) fn facts_present(&self) -> bool {
        self.facts_present
    }

    pub(crate) fn matched_routes(&self) -> &[LoopRouteId] {
        &self.matched
    }

    pub(crate) fn raw_execution_routes(&self) -> &[LoopRouteId] {
        &self.raw_execution
    }

    pub(crate) fn diagnostic_effective_routes(&self) -> &[LoopRouteId] {
        &self.diagnostic_effective
    }

    pub(crate) fn diagnostic_effective_names(&self) -> Vec<&'static str> {
        self.diagnostic_effective
            .iter()
            .map(|route| route.as_str())
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLocatedGenericLoopV1SelectionV1 {
    _seal: LocatedGenericLoopV1SelectionSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct LocatedGenericLoopV1SelectionSealV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LocatedGenericLoopV1SelectionErrorV1 {
    GenericLoopV1NotSelected,
    NonGenericOrOverlappingSelection {
        raw_execution_routes: Box<[LoopRouteId]>,
    },
}

impl RecipeFirstRouteSelectionV1 {
    /// Select the first located profile only when the shared registry resolves
    /// to exactly GenericLoopV1. Later located lowering consumes this token;
    /// this selection performs no route execution or fallback.
    pub(crate) fn verify_located_generic_loop_v1(
        &self,
    ) -> Result<VerifiedLocatedGenericLoopV1SelectionV1, LocatedGenericLoopV1SelectionErrorV1> {
        match self.raw_execution_routes() {
            [LoopRouteId::GenericLoopV1] => Ok(VerifiedLocatedGenericLoopV1SelectionV1 {
                _seal: LocatedGenericLoopV1SelectionSealV1,
            }),
            routes if !routes.contains(&LoopRouteId::GenericLoopV1) => {
                Err(LocatedGenericLoopV1SelectionErrorV1::GenericLoopV1NotSelected)
            }
            routes => Err(
                LocatedGenericLoopV1SelectionErrorV1::NonGenericOrOverlappingSelection {
                    raw_execution_routes: routes.into(),
                },
            ),
        }
    }
}

struct CandidateSuppression {
    if_phi_join_candidate: bool,
    loop_continue_only_candidate: bool,
    loop_cond_continue_only_candidate: bool,
    loop_true_early_exit_candidate: bool,
    loop_true_break_continue_candidate: bool,
    array_join_candidate: bool,
}

impl CandidateSuppression {
    fn from_facts(facts: &CanonicalLoopFacts) -> Self {
        Self {
            if_phi_join_candidate: pred_if_phi_join(facts),
            loop_continue_only_candidate: pred_loop_continue_only(facts),
            loop_cond_continue_only_candidate: pred_loop_cond_continue_only(facts),
            loop_true_early_exit_candidate: pred_loop_true_early_exit(facts),
            loop_true_break_continue_candidate: pred_loop_true_break_continue(facts),
            array_join_candidate: pred_loop_array_join(facts),
        }
    }
}

fn should_skip_candidate(route: LoopRouteId, suppression: &CandidateSuppression) -> bool {
    match route {
        LoopRouteId::LoopCondBreakContinue => {
            suppression.if_phi_join_candidate
                || suppression.loop_continue_only_candidate
                || suppression.loop_cond_continue_only_candidate
                || suppression.array_join_candidate
        }
        LoopRouteId::LoopCondContinueOnly => suppression.loop_continue_only_candidate,
        LoopRouteId::LoopTrueBreakContinue => suppression.loop_true_early_exit_candidate,
        LoopRouteId::GenericLoopV1 => suppression.loop_true_break_continue_candidate,
        _ => false,
    }
}

pub(crate) fn select_recipe_first_routes(
    facts: Option<&CanonicalLoopFacts>,
) -> RecipeFirstRouteSelectionV1 {
    let Some(facts) = facts else {
        return RecipeFirstRouteSelectionV1 {
            facts_present: false,
            matched: Box::default(),
            raw_execution: Box::default(),
            diagnostic_effective: Box::default(),
        };
    };

    let suppression = CandidateSuppression::from_facts(facts);
    let matched = ENTRIES
        .iter()
        .filter(|entry| (entry.predicate)(facts))
        .map(|entry| entry.id)
        .collect::<Box<[_]>>();
    let char_map_candidate = pred_loop_char_map(facts);
    let block_generic_loop_v1 =
        char_map_candidate || pred_loop_simple_while(facts) || pred_nested_loop_minimal(facts);
    let raw_execution = ENTRIES
        .iter()
        .filter(|entry| !should_skip_candidate(entry.id, &suppression))
        .filter(|entry| (entry.predicate)(facts))
        .map(|entry| entry.id)
        .collect::<Box<[_]>>();
    let diagnostic_effective = raw_execution
        .iter()
        .copied()
        .filter(|route| !block_generic_loop_v1 || *route != LoopRouteId::GenericLoopV1)
        .collect::<Box<[_]>>();

    RecipeFirstRouteSelectionV1 {
        facts_present: true,
        matched,
        raw_execution,
        diagnostic_effective,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        select_recipe_first_routes, LocatedGenericLoopV1SelectionErrorV1,
        RecipeFirstRouteSelectionV1, VerifiedLocatedGenericLoopV1SelectionV1,
    };
    use crate::mir::builder::control_flow::joinir::route_entry::registry::route_id::LoopRouteId;
    use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
    use crate::mir::builder::control_flow::plan::LoopPlanExpressionPortV1;
    use crate::mir::callable_result_representation::{
        actual_parser_add_fixture, VerifiedCallableResultLegacySourceViewV1,
    };

    fn selection(
        raw_execution: &[LoopRouteId],
        diagnostic_effective: &[LoopRouteId],
    ) -> RecipeFirstRouteSelectionV1 {
        RecipeFirstRouteSelectionV1 {
            facts_present: true,
            matched: raw_execution.into(),
            raw_execution: raw_execution.into(),
            diagnostic_effective: diagnostic_effective.into(),
        }
    }

    #[test]
    fn located_generic_selection_accepts_exactly_one_generic_route() {
        let selected = selection(&[LoopRouteId::GenericLoopV1], &[LoopRouteId::GenericLoopV1]);

        assert!(matches!(
            selected.verify_located_generic_loop_v1(),
            Ok(VerifiedLocatedGenericLoopV1SelectionV1 { .. })
        ));
    }

    #[test]
    fn located_generic_selection_rejects_missing_generic_route() {
        let selected = selection(
            &[LoopRouteId::LoopSimpleWhile],
            &[LoopRouteId::LoopSimpleWhile],
        );

        assert_eq!(
            selected.verify_located_generic_loop_v1(),
            Err(LocatedGenericLoopV1SelectionErrorV1::GenericLoopV1NotSelected)
        );
    }

    #[test]
    fn located_generic_selection_rejects_overlapping_effective_routes() {
        let selected = selection(
            &[LoopRouteId::LoopSimpleWhile, LoopRouteId::GenericLoopV1],
            &[LoopRouteId::LoopSimpleWhile, LoopRouteId::GenericLoopV1],
        );

        assert!(matches!(
            selected.verify_located_generic_loop_v1(),
            Err(LocatedGenericLoopV1SelectionErrorV1::NonGenericOrOverlappingSelection {
                raw_execution_routes
            }) if raw_execution_routes.as_ref() == [LoopRouteId::LoopSimpleWhile, LoopRouteId::GenericLoopV1]
        ));
    }

    #[test]
    fn located_generic_selection_uses_raw_execution_not_diagnostic_projection() {
        let selected = selection(
            &[LoopRouteId::LoopSimpleWhile, LoopRouteId::GenericLoopV1],
            &[LoopRouteId::LoopSimpleWhile],
        );

        assert!(matches!(
            selected.verify_located_generic_loop_v1(),
            Err(LocatedGenericLoopV1SelectionErrorV1::NonGenericOrOverlappingSelection { .. })
        ));
    }

    #[test]
    fn actual_located_loop_selects_generic_loop_v1_without_a_builder() {
        let activation = actual_parser_add_fixture::plan();
        let caller = actual_parser_add_fixture::caller(&activation);
        let view = VerifiedCallableResultLegacySourceViewV1::verify(&activation, &caller)
            .expect("actual source view");
        let root = view.root_body();
        let loop_root = view.body_stmt(&root, 4).expect("actual Loop Body(4)");
        let port =
            crate::mir::builder::control_flow::plan::LocatedLoopPlanExpressionPortV1::new(view);
        let input = port.borrowed_stmt(&loop_root);
        let crate::ast::ASTNode::Loop {
            condition, body, ..
        } = port.stmt_syntax(&input)
        else {
            panic!("actual fixture root must be a Loop");
        };
        let context = LoopRouteContext::new(&condition, &body, "i0a-selection/0", false, false);
        let outcome =
            crate::mir::builder::control_flow::plan::single_planner::try_build_outcome(&context)
                .expect("actual GenericLoop extraction");
        let selection = select_recipe_first_routes(outcome.facts.as_ref());

        assert_eq!(
            selection.raw_execution_routes(),
            [LoopRouteId::GenericLoopV1]
        );
        assert_eq!(
            selection.diagnostic_effective_routes(),
            [LoopRouteId::GenericLoopV1]
        );
        assert!(selection.verify_located_generic_loop_v1().is_ok());
    }
}
