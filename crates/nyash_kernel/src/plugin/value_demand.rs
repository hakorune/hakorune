//! Runtime-private demand vocabulary for phase-289x.
//!
//! This module is intentionally structural first: it names the demand contract
//! that existing helpers already implement, without changing exported ABI or
//! lowering behavior.
#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ValueDemand {
    ReadRef,
    EncodeImmediate,
    EncodeAlias,
    OwnedPayload,
    StableObject,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StorageDemand {
    CellResidence,
    ImmediateResidence,
    GenericResidence,
    DegradeGeneric,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PublishDemand {
    ExternalBoundary,
    GenericFallback,
    ExplicitApi,
    NeedStableObject,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MutationDemand {
    InvalidateAliases,
    DropEpoch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DemandSet {
    pub(crate) value: &'static [ValueDemand],
    pub(crate) storage: &'static [StorageDemand],
    pub(crate) publish: &'static [PublishDemand],
    pub(crate) mutation: &'static [MutationDemand],
}

impl DemandSet {
    pub(crate) const fn new(
        value: &'static [ValueDemand],
        storage: &'static [StorageDemand],
        publish: &'static [PublishDemand],
        mutation: &'static [MutationDemand],
    ) -> Self {
        Self {
            value,
            storage,
            publish,
            mutation,
        }
    }
}

pub(crate) const CODEC_GENERIC: DemandSet = DemandSet::new(
    &[ValueDemand::StableObject],
    &[StorageDemand::GenericResidence],
    &[],
    &[],
);

pub(crate) const CODEC_ARRAY_FAST_BORROW_STRING: DemandSet = DemandSet::new(
    &[ValueDemand::EncodeImmediate, ValueDemand::EncodeAlias],
    &[
        StorageDemand::ImmediateResidence,
        StorageDemand::GenericResidence,
    ],
    &[],
    &[],
);

pub(crate) const CODEC_ARRAY_BORROW_STRING_ONLY: DemandSet = DemandSet::new(
    &[ValueDemand::EncodeAlias],
    &[StorageDemand::GenericResidence],
    &[],
    &[],
);

pub(crate) const CODEC_MAP_KEY_BORROW_STRING: DemandSet = DemandSet::new(
    &[ValueDemand::EncodeImmediate, ValueDemand::EncodeAlias],
    &[StorageDemand::GenericResidence],
    &[],
    &[],
);

pub(crate) const CODEC_MAP_VALUE_BORROW_STRING: DemandSet = DemandSet::new(
    &[ValueDemand::EncodeAlias],
    &[StorageDemand::GenericResidence],
    &[],
    &[],
);

pub(crate) const ARRAY_TEXT_READ_REF: DemandSet =
    DemandSet::new(&[ValueDemand::ReadRef], &[], &[], &[]);

pub(crate) const ARRAY_TEXT_OWNED_CELL: DemandSet = DemandSet::new(
    &[ValueDemand::OwnedPayload],
    &[StorageDemand::CellResidence],
    &[],
    &[MutationDemand::InvalidateAliases],
);

pub(crate) const ARRAY_GENERIC_GET_ENCODED: DemandSet = DemandSet::new(
    &[
        ValueDemand::EncodeImmediate,
        ValueDemand::EncodeAlias,
        ValueDemand::StableObject,
    ],
    &[
        StorageDemand::ImmediateResidence,
        StorageDemand::GenericResidence,
    ],
    &[],
    &[],
);

pub(crate) const BORROWED_ALIAS_ENCODE: DemandSet = DemandSet::new(
    &[ValueDemand::EncodeAlias],
    &[],
    &[],
    &[MutationDemand::DropEpoch],
);

pub(crate) const BORROWED_ALIAS_FALLBACK_PUBLISH: DemandSet = DemandSet::new(
    &[ValueDemand::StableObject],
    &[],
    &[PublishDemand::NeedStableObject],
    &[],
);

pub(crate) const PUBLISH_EXTERNAL_BOUNDARY: DemandSet = DemandSet::new(
    &[ValueDemand::StableObject],
    &[],
    &[PublishDemand::ExternalBoundary],
    &[],
);

pub(crate) const PUBLISH_GENERIC_FALLBACK: DemandSet = DemandSet::new(
    &[ValueDemand::StableObject],
    &[],
    &[PublishDemand::GenericFallback],
    &[],
);

pub(crate) const PUBLISH_EXPLICIT_API: DemandSet = DemandSet::new(
    &[ValueDemand::StableObject],
    &[],
    &[PublishDemand::ExplicitApi],
    &[],
);

pub(crate) const PUBLISH_NEED_STABLE_OBJECT: DemandSet = DemandSet::new(
    &[ValueDemand::StableObject],
    &[],
    &[PublishDemand::NeedStableObject],
    &[],
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codec_array_fast_names_immediate_and_alias_demands() {
        assert_eq!(
            CODEC_ARRAY_FAST_BORROW_STRING.value,
            &[ValueDemand::EncodeImmediate, ValueDemand::EncodeAlias]
        );
        assert_eq!(
            CODEC_ARRAY_FAST_BORROW_STRING.storage,
            &[
                StorageDemand::ImmediateResidence,
                StorageDemand::GenericResidence
            ]
        );
    }

    #[test]
    fn array_text_cell_names_unpublished_storage_demand() {
        assert_eq!(ARRAY_TEXT_READ_REF.value, &[ValueDemand::ReadRef]);
        assert_eq!(ARRAY_TEXT_OWNED_CELL.value, &[ValueDemand::OwnedPayload]);
        assert_eq!(
            ARRAY_TEXT_OWNED_CELL.storage,
            &[StorageDemand::CellResidence]
        );
        assert_eq!(
            ARRAY_TEXT_OWNED_CELL.mutation,
            &[MutationDemand::InvalidateAliases]
        );
    }

    #[test]
    fn array_generic_get_encoded_names_immediate_alias_and_stable_fallback() {
        assert_eq!(
            ARRAY_GENERIC_GET_ENCODED.value,
            &[
                ValueDemand::EncodeImmediate,
                ValueDemand::EncodeAlias,
                ValueDemand::StableObject
            ]
        );
        assert_eq!(
            ARRAY_GENERIC_GET_ENCODED.storage,
            &[
                StorageDemand::ImmediateResidence,
                StorageDemand::GenericResidence
            ]
        );
    }

    #[test]
    fn borrowed_alias_fallback_is_publish_demand() {
        assert_eq!(BORROWED_ALIAS_ENCODE.mutation, &[MutationDemand::DropEpoch]);
        assert_eq!(
            BORROWED_ALIAS_FALLBACK_PUBLISH.publish,
            &[PublishDemand::NeedStableObject]
        );
    }

    #[test]
    fn publish_demands_name_boundary_effects() {
        assert_eq!(
            PUBLISH_EXTERNAL_BOUNDARY.publish,
            &[PublishDemand::ExternalBoundary]
        );
        assert_eq!(
            PUBLISH_GENERIC_FALLBACK.publish,
            &[PublishDemand::GenericFallback]
        );
        assert_eq!(PUBLISH_EXPLICIT_API.publish, &[PublishDemand::ExplicitApi]);
        assert_eq!(
            PUBLISH_NEED_STABLE_OBJECT.publish,
            &[PublishDemand::NeedStableObject]
        );
    }
}
