//! Function-role and direct-call admission policy vocabulary.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DirectCallAdmissionV1 {
    Forbidden,
    FiniteOneOrMore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CanonicalFunctionRolePolicyV1 {
    OrdinaryFirstFamily,
    NormalMain0,
    NormalMainDirectCall0,
}

impl CanonicalFunctionRolePolicyV1 {
    pub(super) const fn rejection_reason(self) -> &'static str {
        match self {
            Self::OrdinaryFirstFamily => "owner_kind_not_first_family",
            Self::NormalMain0 => "owner_kind_not_normal_main0",
            Self::NormalMainDirectCall0 => "owner_kind_not_normal_main_direct_call0",
        }
    }

    pub(super) const fn is_normal_main(self) -> bool {
        matches!(self, Self::NormalMain0 | Self::NormalMainDirectCall0)
    }

    pub(super) const fn allows_zero_parameter_direct_call(self) -> bool {
        matches!(self, Self::NormalMainDirectCall0)
    }
}
