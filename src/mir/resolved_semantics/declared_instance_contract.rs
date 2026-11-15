//! Stable resolver declaration identity retained by current instance call relation/effect.

use super::ResolverCatalogBrandV1;

/// Stable identity used for exact selected-declaration row matching.
/// Catalog provenance and source-site brands remain aggregate-level checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct DeclaredInstanceMethodIdentityV1 {
    resolver_brand: ResolverCatalogBrandV1,
    nominal_box_type: super::ResolverNominalBoxTypeIdV1,
    box_statement_ordinal: u32,
    method_member_ordinal: u32,
}

impl DeclaredInstanceMethodIdentityV1 {
    /// Build the stable declaration identity from the already co-sealed
    /// resolver source coordinates. This is a projection only; it does not
    /// look up a name, arity, target, or physical symbol.
    pub(crate) fn from_resolver_source(
        resolver_brand: ResolverCatalogBrandV1,
        nominal_box_type: super::ResolverNominalBoxTypeIdV1,
        source_site: crate::parser::ResolverBoxMethodSourceSiteV1,
    ) -> Self {
        Self {
            resolver_brand,
            nominal_box_type,
            box_statement_ordinal: source_site.box_statement_ordinal(),
            method_member_ordinal: source_site.member_ordinal(),
        }
    }
}
