use super::fastpath::{LocalFastPathFact, LocalFastPathFallbackReason};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlanEpoch(pub u32);

impl PlanEpoch {
    pub const INITIAL: Self = Self(0);

    #[inline]
    pub const fn is_initial(self) -> bool {
        self.0 == Self::INITIAL.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FastPathDecision {
    Allow(LocalFastPathFact),
    Deny(LocalFastPathFallbackReason),
}

impl FastPathDecision {
    #[inline]
    pub const fn allow(fact: LocalFastPathFact) -> Self {
        Self::Allow(fact)
    }

    #[inline]
    pub const fn deny(reason: LocalFastPathFallbackReason) -> Self {
        Self::Deny(reason)
    }

    #[inline]
    pub const fn is_allow(&self) -> bool {
        matches!(self, Self::Allow(_))
    }

    #[inline]
    pub const fn is_deny(&self) -> bool {
        matches!(self, Self::Deny(_))
    }

    #[inline]
    pub const fn fact(&self) -> Option<&LocalFastPathFact> {
        match self {
            Self::Allow(fact) => Some(fact),
            Self::Deny(_) => None,
        }
    }

    #[inline]
    pub const fn deny_reason(&self) -> Option<LocalFastPathFallbackReason> {
        match self {
            Self::Allow(_) => None,
            Self::Deny(reason) => Some(*reason),
        }
    }
}
