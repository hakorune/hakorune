//! Passive module-level backend-capability witness for canonical recursion.
//!
//! P0c-MR-C0 defines and validates the marker only. No production compiler
//! ingress installs it until P0c-MR-I1.

pub(crate) const CANONICAL_RECURSIVE_CALLABLE_MODULE_CAPABILITY_V1: &str =
    "canonical_recursive_callable_module_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CanonicalRecursiveCallableModuleCapabilityV1 {
    schema_version: u8,
}

impl CanonicalRecursiveCallableModuleCapabilityV1 {
    pub(crate) const fn v1() -> Self {
        Self { schema_version: 1 }
    }

    pub(crate) const fn schema_version(self) -> u8 {
        self.schema_version
    }

    pub(crate) fn install_for_module(
        slot: &mut Option<Self>,
        required: bool,
    ) -> Result<(), &'static str> {
        if slot.is_some() {
            return Err("[freeze:contract][canonical_recursive_module/capability_preexisting]");
        }
        if required {
            *slot = Some(Self::v1());
        }
        Ok(())
    }

    pub(crate) fn verify_required(slot: Option<&Self>) -> Result<(), &'static str> {
        match slot {
            None => Err("[freeze:contract][canonical_recursive_module/capability_missing]"),
            Some(row) if row.schema_version() == 1 => Ok(()),
            Some(_) => Err("[freeze:contract][canonical_recursive_module/capability_schema_drift]"),
        }
    }

    #[cfg(test)]
    const fn with_schema_version_for_test(schema_version: u8) -> Self {
        Self { schema_version }
    }
}

#[cfg(test)]
mod tests {
    use super::CanonicalRecursiveCallableModuleCapabilityV1 as Capability;

    #[test]
    fn installs_zero_or_one_module_marker() {
        let mut absent = None;
        Capability::install_for_module(&mut absent, false).unwrap();
        assert!(absent.is_none());

        let mut required = None;
        Capability::install_for_module(&mut required, true).unwrap();
        assert_eq!(required, Some(Capability::v1()));
        assert!(Capability::install_for_module(&mut required, true)
            .unwrap_err()
            .contains("capability_preexisting"));
    }

    #[test]
    fn required_verification_rejects_missing_and_schema_drift() {
        assert!(Capability::verify_required(None)
            .unwrap_err()
            .contains("capability_missing"));
        assert!(
            Capability::verify_required(Some(&Capability::with_schema_version_for_test(2)))
                .unwrap_err()
                .contains("capability_schema_drift")
        );
        Capability::verify_required(Some(&Capability::v1())).unwrap();
    }
}
