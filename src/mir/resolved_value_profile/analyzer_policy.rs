//! Local policy vocabulary for the exact representation analyzer.

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ReturnPolicyV1 {
    RootFinalOnly,
    Forbidden,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum DirectCallPolicyV1 {
    Forbidden,
    FiniteOneOrMore,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum RootProfilePolicyV1 {
    OrdinaryFirstFamily,
    NormalMain0,
}
