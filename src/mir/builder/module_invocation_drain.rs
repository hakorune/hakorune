//! Condition-function policy shared by root admission.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum ConditionFnPolicyV1 {
    Required,
    Optional,
    Forbidden,
}
