#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalSourcePlanErrorV1 {
    RootExecutionRelationMismatch,
    MainMustBeStatic,
}
