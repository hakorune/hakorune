use crate::ast::BinaryOperator;
use crate::mir::policies::CondProfile;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConditionCanon {
    pub loop_var_candidates: Vec<String>,
    pub cond_profile: CondProfile,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct UpdateCanon {
    pub op: BinaryOperator,
    pub step: i64,
    pub commuted: bool,
}
