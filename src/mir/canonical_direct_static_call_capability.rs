//! Durable backend-capability witness for canonical direct static calls.
//!
//! The row is explicit metadata. Backend admission must not infer it by
//! scanning generic `MirInstruction::Call` instructions, source names, or
//! parameter/return contracts.

pub(crate) const CANONICAL_DIRECT_STATIC_CALL_CAPABILITY_V1: &str =
    "canonical_direct_static_call_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalDirectStaticCallCapabilityV1 {
    schema_version: u8,
}

impl CanonicalDirectStaticCallCapabilityV1 {
    pub(crate) const fn v1() -> Self {
        Self { schema_version: 1 }
    }

    pub(crate) const fn schema_version(self) -> u8 {
        self.schema_version
    }

    /// Install the function-level witness before expression lowering begins.
    /// Existing rows always indicate a producer-order violation.
    pub(crate) fn install_for_function(
        rows: &mut Vec<Self>,
        required: bool,
    ) -> Result<(), &'static str> {
        if !rows.is_empty() {
            return Err("[freeze:contract][canonical_direct_call/capability_preexisting]");
        }
        if required {
            rows.push(Self::v1());
        }
        Ok(())
    }

    /// Verify the already-installed witness without mutating function metadata.
    pub(crate) fn verify_for_emission(rows: &[Self]) -> Result<(), &'static str> {
        match rows {
            [] => Err("[freeze:contract][canonical_direct_call/capability_missing]"),
            [row] if row.schema_version() == 1 => Ok(()),
            [_] => Err("[freeze:contract][canonical_direct_call/capability_schema_drift]"),
            _ => Err("[freeze:contract][canonical_direct_call/capability_duplicate]"),
        }
    }

    #[cfg(test)]
    const fn with_schema_version_for_test(schema_version: u8) -> Self {
        Self { schema_version }
    }
}

#[cfg(test)]
mod tests {
    use super::CanonicalDirectStaticCallCapabilityV1 as Capability;

    #[test]
    fn installs_zero_or_one_marker_from_function_need() {
        let mut no_calls = Vec::new();
        Capability::install_for_function(&mut no_calls, false).unwrap();
        assert!(no_calls.is_empty());

        let mut caller = Vec::new();
        Capability::install_for_function(&mut caller, true).unwrap();
        assert_eq!(caller, [Capability::v1()]);
    }

    #[test]
    fn repeated_emission_verification_never_adds_a_second_marker() {
        let mut rows = Vec::new();
        Capability::install_for_function(&mut rows, true).unwrap();
        Capability::verify_for_emission(&rows).unwrap();
        Capability::verify_for_emission(&rows).unwrap();
        assert_eq!(rows, [Capability::v1()]);
        assert!(Capability::install_for_function(&mut rows, true)
            .unwrap_err()
            .contains("capability_preexisting"));
    }

    #[test]
    fn emission_rejects_missing_duplicate_and_schema_drift() {
        assert!(Capability::verify_for_emission(&[])
            .unwrap_err()
            .contains("capability_missing"));
        assert!(
            Capability::verify_for_emission(&[Capability::v1(), Capability::v1()])
                .unwrap_err()
                .contains("capability_duplicate")
        );
        assert!(
            Capability::verify_for_emission(&[Capability::with_schema_version_for_test(2),])
                .unwrap_err()
                .contains("capability_schema_drift")
        );
    }
}
