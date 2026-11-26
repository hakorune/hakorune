//! Pre-link provider admission projections.
//!
//! The mutable `BoxCallableRegistry` remains a compatibility/draft surface.
//! This child owns only the consuming TextScan admission used by the selected
//! AOT activation checkpoint.  It has no runtime lookup or executable address.

#[cfg(test)]
mod admitted_registry;
mod aot_admission;
mod call_metadata;
#[cfg(test)]
mod seal;

pub(crate) use call_metadata::DynamicV2AotCallMetadataProjectionV1;
