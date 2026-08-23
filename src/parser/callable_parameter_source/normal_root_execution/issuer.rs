use super::super::normal_source_plan_surface::{
    ParserNormalSourcePlanSurfaceDispositionV1, ParserNormalSourcePlanSurfaceV1,
    ParserNormalSourcePlanTopLevelRowV1,
};
use super::model::{
    ParserNormalAppExecutionRelationV1, ParserNormalRootExecutionIncompleteV1,
    ParserNormalRootExecutionIntegrityIssueV1, ParserNormalRootExecutionSourceDispositionV1,
    ParserNormalRootExecutionSourceV1,
};

pub(in crate::parser) struct ParserNormalRootExecutionIssuerV1;

impl ParserNormalRootExecutionIssuerV1 {
    pub(in crate::parser) fn issue_once(
        surface: ParserNormalSourcePlanSurfaceDispositionV1,
    ) -> ParserNormalRootExecutionSourceDispositionV1 {
        let bound = match surface {
            ParserNormalSourcePlanSurfaceDispositionV1::Ready(bound) => bound,
            ParserNormalSourcePlanSurfaceDispositionV1::SourceAuthorityUnavailable(error) => {
                return ParserNormalRootExecutionSourceDispositionV1::SourceAuthorityUnavailable(
                    error,
                )
            }
            ParserNormalSourcePlanSurfaceDispositionV1::Incomplete(error) => {
                return ParserNormalRootExecutionSourceDispositionV1::Incomplete(
                    ParserNormalRootExecutionIncompleteV1::Surface(error),
                )
            }
            ParserNormalSourcePlanSurfaceDispositionV1::IntegrityInvalid(error) => {
                return ParserNormalRootExecutionSourceDispositionV1::IntegrityInvalid(
                    ParserNormalRootExecutionIntegrityIssueV1::Surface(error),
                )
            }
        };

        let rows = match bound.surface() {
            ParserNormalSourcePlanSurfaceV1::CompleteEmpty => &[][..],
            ParserNormalSourcePlanSurfaceV1::CompleteRows(rows) => rows.rows(),
        };
        let mut main = None;
        for row in rows {
            let candidate = match row {
                ParserNormalSourcePlanTopLevelRowV1::StaticBox { slot, source }
                    if source.declaration_syntax().name() == "Main" =>
                {
                    if usize::try_from(source.member_count()).ok()
                        != Some(source.member_kinds().len())
                    {
                        return ParserNormalRootExecutionSourceDispositionV1::Incomplete(
                            ParserNormalRootExecutionIncompleteV1::MainMemberCoverage,
                        );
                    }
                    Some((
                        slot.final_statement_slot(),
                        true,
                        source
                            .direct_method_relations()
                            .map(|(_, identity)| identity.clone())
                            .collect::<Vec<_>>(),
                    ))
                }
                ParserNormalSourcePlanTopLevelRowV1::OrdinaryBox { slot, source }
                    if source.diagnostic_name() == "Main" =>
                {
                    Some((
                        slot.final_statement_slot(),
                        false,
                        source
                            .direct_method_relations()
                            .iter()
                            .map(|relation| relation.callable_identity().clone())
                            .collect::<Vec<_>>(),
                    ))
                }
                _ => None,
            };
            let Some(candidate) = candidate else {
                continue;
            };
            if main.is_some() {
                return ParserNormalRootExecutionSourceDispositionV1::IntegrityInvalid(
                    ParserNormalRootExecutionIntegrityIssueV1::DuplicateMain,
                );
            }
            main = Some(candidate);
        }

        let Some((main_statement, main_box_is_static, direct)) = main else {
            return ParserNormalRootExecutionSourceDispositionV1::Ready(
                ParserNormalRootExecutionSourceV1::program_runtime(bound),
            );
        };

        let mut main_callable = None;
        let mut static_children = Vec::new();
        for identity in direct {
            let Some(syntax) = bound.callable_syntax(&identity) else {
                return ParserNormalRootExecutionSourceDispositionV1::Incomplete(
                    ParserNormalRootExecutionIncompleteV1::MainMemberCoverage,
                );
            };
            if syntax.diagnostic_name() == "main" {
                if main_callable.replace(identity).is_some() {
                    return ParserNormalRootExecutionSourceDispositionV1::IntegrityInvalid(
                        ParserNormalRootExecutionIntegrityIssueV1::DuplicateMainMethod,
                    );
                }
            } else {
                static_children.push(identity);
            }
        }
        let Some(main_callable) = main_callable else {
            return ParserNormalRootExecutionSourceDispositionV1::Incomplete(
                ParserNormalRootExecutionIncompleteV1::MainMethodMissing,
            );
        };
        let relation = ParserNormalAppExecutionRelationV1::issue(
            main_statement,
            main_box_is_static,
            main_callable,
            static_children.into_boxed_slice(),
        );
        ParserNormalRootExecutionSourceDispositionV1::Ready(ParserNormalRootExecutionSourceV1::app(
            bound, relation,
        ))
    }
}
