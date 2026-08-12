//! Transport-only schema for the future Dynamic CallSlot ABI.
//!
//! This module deliberately has no runtime dispatch, provider lookup, or
//! compiler semantic authority.  Invocation/Recipe/CallSlot products remain
//! the meaning owners; a later provider capability may borrow this wire.

use std::convert::TryFrom;

/// Revision 2 keeps the fixed layout and adds a lease-free Normal form for
/// `ImmediateI64`.  The `V1` type suffix names the unchanged C layout, not
/// the validity revision.
pub const DYNAMIC_V2_WIRE_REVISION_V2: u32 = 2;
pub const DYNAMIC_V2_FORWARDED_NONE_V1: u32 = u32::MAX;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicV2WireTagV1 {
    Invalid = 0,
    HostHandle = 1,
    ImmediateI64 = 2,
}

impl TryFrom<u32> for DynamicV2WireTagV1 {
    type Error = DynamicV2WireSchemaRejectV1;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        match raw {
            0 => Ok(Self::Invalid),
            1 => Ok(Self::HostHandle),
            2 => Ok(Self::ImmediateI64),
            _ => Err(DynamicV2WireSchemaRejectV1::UnknownTag(raw)),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicV2CallStatusV1 {
    Normal = 0,
    Fault = 1,
    Suspended = 2,
}

impl TryFrom<u32> for DynamicV2CallStatusV1 {
    type Error = DynamicV2WireSchemaRejectV1;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        match raw {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Fault),
            2 => Ok(Self::Suspended),
            _ => Err(DynamicV2WireSchemaRejectV1::UnknownStatus(raw)),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicV2CallDispositionV1 {
    None = 0,
    Forwarded = 1,
    EndAuthorized = 2,
}

impl TryFrom<u32> for DynamicV2CallDispositionV1 {
    type Error = DynamicV2WireSchemaRejectV1;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        match raw {
            0 => Ok(Self::None),
            1 => Ok(Self::Forwarded),
            2 => Ok(Self::EndAuthorized),
            _ => Err(DynamicV2WireSchemaRejectV1::UnknownDisposition(raw)),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicV2CallFaultCodeV1 {
    None = 0,
    InvalidReceiver = 1,
    InvalidHandle = 2,
    Arity = 3,
    UnsupportedSlot = 4,
    TypeMismatch = 5,
    Range = 6,
    Runtime = 7,
    InvalidResult = 8,
}

impl TryFrom<u32> for DynamicV2CallFaultCodeV1 {
    type Error = DynamicV2WireSchemaRejectV1;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        match raw {
            0 => Ok(Self::None),
            1 => Ok(Self::InvalidReceiver),
            2 => Ok(Self::InvalidHandle),
            3 => Ok(Self::Arity),
            4 => Ok(Self::UnsupportedSlot),
            5 => Ok(Self::TypeMismatch),
            6 => Ok(Self::Range),
            7 => Ok(Self::Runtime),
            8 => Ok(Self::InvalidResult),
            _ => Err(DynamicV2WireSchemaRejectV1::UnknownFaultCode(raw)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicV2WireSchemaRejectV1 {
    UnknownTag(u32),
    UnknownStatus(u32),
    UnknownDisposition(u32),
    UnknownFaultCode(u32),
    NonZeroReserved,
    InvalidValueTag,
    InvalidNormalOutcome,
    InvalidFaultOutcome,
    InvalidSuspendedOutcome,
    SuspendedNotSupported,
    ForwardedLaneOutOfRange,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynamicV2WireValueV1 {
    pub tag: u32,
    pub reserved: u32,
    pub payload: u64,
}

impl DynamicV2WireValueV1 {
    pub const fn new(tag: DynamicV2WireTagV1, payload: u64) -> Self {
        Self {
            tag: tag as u32,
            reserved: 0,
            payload,
        }
    }

    pub fn validate(self) -> Result<DynamicV2WireTagV1, DynamicV2WireSchemaRejectV1> {
        if self.reserved != 0 {
            return Err(DynamicV2WireSchemaRejectV1::NonZeroReserved);
        }
        let tag = DynamicV2WireTagV1::try_from(self.tag)?;
        if tag == DynamicV2WireTagV1::Invalid && self.payload != 0 {
            return Err(DynamicV2WireSchemaRejectV1::InvalidValueTag);
        }
        Ok(tag)
    }
}

/// The call result is intentionally not Clone/Copy: a later runtime owner may
/// attach a one-shot lease to `lease_token`.
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct DynamicV2CallOutV1 {
    pub status: u32,
    pub fault_code: u32,
    pub result_tag: u32,
    pub disposition: u32,
    pub forwarded_input: u32,
    pub reserved: u32,
    pub value_payload: u64,
    pub lease_token: u64,
    pub continuation_token: u64,
}

impl DynamicV2CallOutV1 {
    pub fn validate_transport(
        &self,
    ) -> Result<DynamicV2CallStatusV1, DynamicV2WireSchemaRejectV1> {
        if self.reserved != 0 {
            return Err(DynamicV2WireSchemaRejectV1::NonZeroReserved);
        }
        let status = DynamicV2CallStatusV1::try_from(self.status)?;
        let fault = DynamicV2CallFaultCodeV1::try_from(self.fault_code)?;
        let tag = DynamicV2WireTagV1::try_from(self.result_tag)?;
        let disposition = DynamicV2CallDispositionV1::try_from(self.disposition)?;
        match status {
            DynamicV2CallStatusV1::Normal => {
                if fault != DynamicV2CallFaultCodeV1::None
                    || tag == DynamicV2WireTagV1::Invalid
                    || self.continuation_token != 0
                {
                    return Err(DynamicV2WireSchemaRejectV1::InvalidNormalOutcome);
                }
                match disposition {
                    DynamicV2CallDispositionV1::None
                        if tag != DynamicV2WireTagV1::ImmediateI64
                            || self.forwarded_input != DYNAMIC_V2_FORWARDED_NONE_V1
                            || self.lease_token != 0 =>
                    {
                        return Err(DynamicV2WireSchemaRejectV1::InvalidNormalOutcome)
                    }
                    DynamicV2CallDispositionV1::Forwarded
                        if tag != DynamicV2WireTagV1::HostHandle
                            || self.forwarded_input == DYNAMIC_V2_FORWARDED_NONE_V1
                            || self.lease_token != 0 =>
                    {
                        return Err(DynamicV2WireSchemaRejectV1::InvalidNormalOutcome)
                    }
                    DynamicV2CallDispositionV1::EndAuthorized
                        if tag != DynamicV2WireTagV1::HostHandle
                            || self.forwarded_input != DYNAMIC_V2_FORWARDED_NONE_V1
                            || self.lease_token == 0 =>
                    {
                        return Err(DynamicV2WireSchemaRejectV1::InvalidNormalOutcome)
                    }
                    _ => {}
                }
            }
            DynamicV2CallStatusV1::Fault => {
                if fault == DynamicV2CallFaultCodeV1::None
                    || tag != DynamicV2WireTagV1::Invalid
                    || self.value_payload != 0
                    || disposition != DynamicV2CallDispositionV1::None
                    || self.forwarded_input != DYNAMIC_V2_FORWARDED_NONE_V1
                    || self.lease_token != 0
                    || self.continuation_token != 0
                {
                    return Err(DynamicV2WireSchemaRejectV1::InvalidFaultOutcome);
                }
            }
            DynamicV2CallStatusV1::Suspended => {
                if fault != DynamicV2CallFaultCodeV1::None
                    || tag != DynamicV2WireTagV1::Invalid
                    || self.value_payload != 0
                    || disposition != DynamicV2CallDispositionV1::None
                    || self.forwarded_input != DYNAMIC_V2_FORWARDED_NONE_V1
                    || self.lease_token != 0
                    || self.continuation_token == 0
                {
                    return Err(DynamicV2WireSchemaRejectV1::InvalidSuspendedOutcome);
                }
            }
        }
        Ok(status)
    }

    pub fn validate_for_synchronous_emitter(
        &self,
    ) -> Result<(), DynamicV2WireSchemaRejectV1> {
        if self.validate_transport()? == DynamicV2CallStatusV1::Suspended {
            return Err(DynamicV2WireSchemaRejectV1::SuspendedNotSupported);
        }
        Ok(())
    }

    pub fn validate_forwarded_arity(
        &self,
        argc: u32,
    ) -> Result<(), DynamicV2WireSchemaRejectV1> {
        if argc == DYNAMIC_V2_FORWARDED_NONE_V1 {
            return Err(DynamicV2WireSchemaRejectV1::ForwardedLaneOutOfRange);
        }
        self.validate_transport()?;
        if self.disposition == DynamicV2CallDispositionV1::Forwarded as u32
            && self.forwarded_input != DYNAMIC_V2_FORWARDED_NONE_V1
            && self.forwarded_input > argc
        {
            return Err(DynamicV2WireSchemaRejectV1::ForwardedLaneOutOfRange);
        }
        Ok(())
    }
}

const _: [(); 16] = [(); std::mem::size_of::<DynamicV2WireValueV1>()];
const _: [(); 8] = [(); std::mem::align_of::<DynamicV2WireValueV1>()];
const _: [(); 48] = [(); std::mem::size_of::<DynamicV2CallOutV1>()];
const _: [(); 8] = [(); std::mem::align_of::<DynamicV2CallOutV1>()];

#[cfg(test)]
mod tests {
    use super::*;

    fn normal_end_authorized() -> DynamicV2CallOutV1 {
        DynamicV2CallOutV1 {
            status: DynamicV2CallStatusV1::Normal as u32,
            fault_code: DynamicV2CallFaultCodeV1::None as u32,
            result_tag: DynamicV2WireTagV1::HostHandle as u32,
            disposition: DynamicV2CallDispositionV1::EndAuthorized as u32,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: 42,
            lease_token: 7,
            continuation_token: 0,
        }
    }

    fn normal_immediate_i64() -> DynamicV2CallOutV1 {
        DynamicV2CallOutV1 {
            status: DynamicV2CallStatusV1::Normal as u32,
            fault_code: DynamicV2CallFaultCodeV1::None as u32,
            result_tag: DynamicV2WireTagV1::ImmediateI64 as u32,
            disposition: DynamicV2CallDispositionV1::None as u32,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: 0,
            lease_token: 0,
            continuation_token: 0,
        }
    }

    #[test]
    fn c_layout_is_fixed_width() {
        assert_eq!(std::mem::size_of::<DynamicV2WireValueV1>(), 16);
        assert_eq!(std::mem::align_of::<DynamicV2WireValueV1>(), 8);
        assert_eq!(std::mem::size_of::<DynamicV2CallOutV1>(), 48);
        assert_eq!(std::mem::align_of::<DynamicV2CallOutV1>(), 8);
        assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, status), 0);
        assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, fault_code), 4);
        assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, result_tag), 8);
        assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, disposition), 12);
        assert_eq!(
            std::mem::offset_of!(DynamicV2CallOutV1, forwarded_input),
            16
        );
        assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, reserved), 20);
        assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, value_payload), 24);
        assert_eq!(std::mem::offset_of!(DynamicV2CallOutV1, lease_token), 32);
        assert_eq!(
            std::mem::offset_of!(DynamicV2CallOutV1, continuation_token),
            40
        );
    }

    #[test]
    fn normal_zero_is_not_a_failure_sentinel() {
        let mut out = normal_end_authorized();
        out.value_payload = 0;
        assert_eq!(out.validate_transport(), Ok(DynamicV2CallStatusV1::Normal));
    }

    #[test]
    fn immediate_i64_normal_has_no_lifecycle_disposition() {
        let out = normal_immediate_i64();
        assert_eq!(out.validate_transport(), Ok(DynamicV2CallStatusV1::Normal));
        assert!(out.validate_forwarded_arity(0).is_ok());
    }

    #[test]
    fn immediate_i64_cannot_publish_lease_or_forwarded_lane() {
        let mut out = normal_immediate_i64();
        out.lease_token = 1;
        assert_eq!(
            out.validate_transport(),
            Err(DynamicV2WireSchemaRejectV1::InvalidNormalOutcome)
        );
        out.lease_token = 0;
        out.disposition = DynamicV2CallDispositionV1::EndAuthorized as u32;
        out.lease_token = 1;
        assert_eq!(
            out.validate_transport(),
            Err(DynamicV2WireSchemaRejectV1::InvalidNormalOutcome)
        );
        out.lease_token = 0;
        out.disposition = DynamicV2CallDispositionV1::Forwarded as u32;
        out.forwarded_input = 0;
        assert_eq!(
            out.validate_transport(),
            Err(DynamicV2WireSchemaRejectV1::InvalidNormalOutcome)
        );
    }

    #[test]
    fn fault_cannot_publish_value_or_lease() {
        let mut out = normal_end_authorized();
        out.status = DynamicV2CallStatusV1::Fault as u32;
        out.fault_code = DynamicV2CallFaultCodeV1::Runtime as u32;
        out.result_tag = DynamicV2WireTagV1::Invalid as u32;
        out.disposition = DynamicV2CallDispositionV1::None as u32;
        out.value_payload = 0;
        out.lease_token = 0;
        assert_eq!(out.validate_transport(), Ok(DynamicV2CallStatusV1::Fault));
        out.lease_token = 1;
        assert_eq!(
            out.validate_transport(),
            Err(DynamicV2WireSchemaRejectV1::InvalidFaultOutcome)
        );
    }

    #[test]
    fn suspended_is_schema_valid_but_sync_rejects() {
        let out = DynamicV2CallOutV1 {
            status: DynamicV2CallStatusV1::Suspended as u32,
            fault_code: 0,
            result_tag: DynamicV2WireTagV1::Invalid as u32,
            disposition: DynamicV2CallDispositionV1::None as u32,
            forwarded_input: DYNAMIC_V2_FORWARDED_NONE_V1,
            reserved: 0,
            value_payload: 0,
            lease_token: 0,
            continuation_token: 9,
        };
        assert_eq!(
            out.validate_transport(),
            Ok(DynamicV2CallStatusV1::Suspended)
        );
        assert_eq!(
            out.validate_for_synchronous_emitter(),
            Err(DynamicV2WireSchemaRejectV1::SuspendedNotSupported)
        );
    }

    #[test]
    fn forwarded_requires_lane_and_no_lease() {
        let mut out = normal_end_authorized();
        out.disposition = DynamicV2CallDispositionV1::Forwarded as u32;
        out.forwarded_input = 1;
        out.lease_token = 0;
        assert!(out.validate_forwarded_arity(1).is_ok());
        assert_eq!(
            out.validate_forwarded_arity(0),
            Err(DynamicV2WireSchemaRejectV1::ForwardedLaneOutOfRange)
        );
        out.lease_token = 2;
        assert_eq!(
            out.validate_transport(),
            Err(DynamicV2WireSchemaRejectV1::InvalidNormalOutcome)
        );
    }

    #[test]
    fn end_authorized_sentinel_is_not_an_input_lane() {
        assert!(normal_end_authorized().validate_forwarded_arity(0).is_ok());
    }

    #[test]
    fn sentinel_reserved_arity_is_rejected() {
        assert_eq!(
            normal_end_authorized().validate_forwarded_arity(u32::MAX),
            Err(DynamicV2WireSchemaRejectV1::ForwardedLaneOutOfRange)
        );
    }

    #[test]
    fn unknown_and_reserved_values_reject() {
        assert_eq!(
            DynamicV2WireValueV1 {
                tag: 99,
                reserved: 0,
                payload: 0,
            }
            .validate(),
            Err(DynamicV2WireSchemaRejectV1::UnknownTag(99))
        );
        let mut out = normal_end_authorized();
        out.reserved = 1;
        assert_eq!(
            out.validate_transport(),
            Err(DynamicV2WireSchemaRejectV1::NonZeroReserved)
        );
    }
}
