//! Passive Home relation vocabulary for the resolver semantic boundary.
//!
//! This module brands source-root and destination references and defines the
//! capability/result enums used by the later Home ABI issuer. It does not
//! classify types, inspect bodies, or connect Home relations to MIR.

use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_HOME_RELATION_BRAND: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct HomeRelationBrandV1(u64);

impl HomeRelationBrandV1 {
    fn issue() -> Result<Self, HomeRelationRejectV1> {
        let value = NEXT_HOME_RELATION_BRAND
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .map_err(|_| HomeRelationRejectV1::BrandExhausted)?;
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct HomeRootRefV1 {
    brand: HomeRelationBrandV1,
    source_ordinal: u32,
}

impl HomeRootRefV1 {
    pub(crate) const fn brand(self) -> HomeRelationBrandV1 {
        self.brand
    }

    pub(crate) const fn source_ordinal(self) -> u32 {
        self.source_ordinal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct HomeDestinationV1 {
    brand: HomeRelationBrandV1,
    source_ordinal: u32,
}

impl HomeDestinationV1 {
    pub(crate) const fn brand(self) -> HomeRelationBrandV1 {
        self.brand
    }

    pub(crate) const fn source_ordinal(self) -> u32 {
        self.source_ordinal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HomeDemandV1 {
    Handle,
    Home,
    SharedHome,
    Trivial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HomeResultRelationV1 {
    Unit,
    Trivial,
    HomeToCaller,
    FromReceiver,
    FromParameter(u16),
    SharedHomeToCaller,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum HomeRelationRejectV1 {
    BrandExhausted,
    DuplicateRootSource { source_ordinal: u32 },
    DuplicateDestinationSource { source_ordinal: u32 },
    ForeignBrand,
}

#[derive(Debug)]
pub(crate) struct HomeRelationBrandIssuerV1 {
    brand: HomeRelationBrandV1,
    roots: BTreeSet<u32>,
    destinations: BTreeSet<u32>,
}

impl HomeRelationBrandIssuerV1 {
    pub(crate) fn issue() -> Result<Self, HomeRelationRejectV1> {
        Ok(Self {
            brand: HomeRelationBrandV1::issue()?,
            roots: BTreeSet::new(),
            destinations: BTreeSet::new(),
        })
    }

    pub(crate) const fn brand(&self) -> HomeRelationBrandV1 {
        self.brand
    }

    pub(crate) fn root(
        &mut self,
        source_ordinal: u32,
    ) -> Result<HomeRootRefV1, HomeRelationRejectV1> {
        if !self.roots.insert(source_ordinal) {
            return Err(HomeRelationRejectV1::DuplicateRootSource { source_ordinal });
        }
        Ok(HomeRootRefV1 {
            brand: self.brand,
            source_ordinal,
        })
    }

    pub(crate) fn destination(
        &mut self,
        source_ordinal: u32,
    ) -> Result<HomeDestinationV1, HomeRelationRejectV1> {
        if !self.destinations.insert(source_ordinal) {
            return Err(HomeRelationRejectV1::DuplicateDestinationSource { source_ordinal });
        }
        Ok(HomeDestinationV1 {
            brand: self.brand,
            source_ordinal,
        })
    }

    pub(crate) fn require_same_brand(
        &self,
        root: HomeRootRefV1,
        destination: HomeDestinationV1,
    ) -> Result<(), HomeRelationRejectV1> {
        if root.brand != self.brand || destination.brand != self.brand {
            return Err(HomeRelationRejectV1::ForeignBrand);
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "home_relation_tests.rs"]
mod tests;
