//! Physical carrier tags shared by current callable and GenericG0 lowering.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum PhysicalCallableLaneCarrierV1 {
    ExistingCallableI64,
    U64BitsOnI64,
    /// A borrowed checked-map storage pointer (`ptr` wire); the callee
    /// never owns or disposes the pointed storage.
    CheckedMapStorage,
    /// Original Ordinary source-selected opaque formal. One MIR value;
    /// kind/payload expansion requires the final owner's full-use proof.
    BorrowedTaggedValue,
}

impl PhysicalCallableLaneCarrierV1 {
    /// The physical `MirType` the carrier occupies in the skeleton
    /// signature. `CheckedMapStorage` keeps `MapBox` metadata so emission
    /// can spell the lane `ptr` instead of the unconditional `i64` used by
    /// the scalar carriers.
    pub(in crate::mir) fn mir_type(self) -> crate::mir::MirType {
        match self {
            Self::CheckedMapStorage => crate::mir::MirType::Box("MapBox".to_owned()),
            Self::ExistingCallableI64 | Self::U64BitsOnI64 | Self::BorrowedTaggedValue => {
                crate::mir::MirType::Integer
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PhysicalCallableLaneCarrierV1;
    #[test]
    fn carrier_tags_keep_wire_meaning_out_of_mir_type() {
        assert_ne!(
            PhysicalCallableLaneCarrierV1::ExistingCallableI64,
            PhysicalCallableLaneCarrierV1::U64BitsOnI64
        );
    }
}
