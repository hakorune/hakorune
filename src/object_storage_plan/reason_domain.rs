use super::{
    DynamicReason, EscapeReason, GenericBoxReason, LocalFastPathFallbackReason,
    ObjectPublicationReason,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReasonDomain {
    StorageRepresentation,
    PublicationBoundary,
    FastPathEligibility,
}

impl ReasonDomain {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StorageRepresentation => "storage_representation",
            Self::PublicationBoundary => "publication_boundary",
            Self::FastPathEligibility => "fastpath_eligibility",
        }
    }
}

pub trait ReasonDomainClass {
    fn reason_domain(self) -> ReasonDomain;
}

impl ReasonDomainClass for GenericBoxReason {
    #[inline]
    fn reason_domain(self) -> ReasonDomain {
        ReasonDomain::StorageRepresentation
    }
}

impl ReasonDomainClass for EscapeReason {
    #[inline]
    fn reason_domain(self) -> ReasonDomain {
        ReasonDomain::StorageRepresentation
    }
}

impl ReasonDomainClass for DynamicReason {
    #[inline]
    fn reason_domain(self) -> ReasonDomain {
        ReasonDomain::StorageRepresentation
    }
}

impl ReasonDomainClass for ObjectPublicationReason {
    #[inline]
    fn reason_domain(self) -> ReasonDomain {
        ReasonDomain::PublicationBoundary
    }
}

impl ReasonDomainClass for LocalFastPathFallbackReason {
    #[inline]
    fn reason_domain(self) -> ReasonDomain {
        ReasonDomain::FastPathEligibility
    }
}
