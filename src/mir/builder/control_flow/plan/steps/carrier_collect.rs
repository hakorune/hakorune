//! Step: Carrier init collection (Pass 2 of carrier analysis).
//! (plan::steps SSOT)
//!
//! Input: carrier_vars (from recipe-specific Pass 1)
//! Output: carrier_inits (BTreeMap<String, ValueId>)
//! Fail-Fast: Missing variable → immediate error

