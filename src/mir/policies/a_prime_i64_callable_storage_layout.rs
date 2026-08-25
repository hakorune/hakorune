//! Callable storage policy for the selected A-prime exact-i64 cohort.
//!
//! This plain policy row is issued only by the selected physical emitter
//! close.  It is deliberately not constructible from MIR, JSON, or a lane
//! spelling, and it does not describe addressable, spill, or aggregate layout.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum APrimeI64CallableStorageLayoutV1 {
    NonAddressableSsaI64,
}
