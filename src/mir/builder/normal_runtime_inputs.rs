//! Immutable runtime inputs captured once for the selected normal ingress.
//!
//! This is deliberately separate from Raw's strict runtime snapshot. Normal
//! compilation historically treats malformed environment values as absent, so
//! its candidate lifecycle consumes this permissive receipt instead of reading
//! the process environment during lowering.

/// Route-local ambient inputs for one normal/default compilation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct NormalRuntimeInputSnapshotV1 {
    script_args: Box<[String]>,
    entry_safepoint: bool,
    cleanup_exit_policy: crate::mir::builder::control_flow::cleanup::CleanupExitPolicyV1,
}

impl NormalRuntimeInputSnapshotV1 {
    /// Snapshot the existing normal/default environment contract exactly once.
    ///
    /// A present `NYASH_SCRIPT_ARGS_JSON` always masks `HAKO_SCRIPT_ARGS_JSON`.
    /// Invalid, non-array, and empty values intentionally collapse to no
    /// arguments, matching the previous lower-side helper.
    pub(in crate::mir) fn capture_from_normal_ingress() -> Self {
        let script_args = crate::config::env::builder_script_args_json()
            .and_then(|raw| serde_json::from_str::<Vec<String>>(&raw).ok())
            .filter(|args| !args.is_empty())
            .unwrap_or_default()
            .into_boxed_slice();
        Self {
            script_args,
            entry_safepoint: crate::config::env::builder_safepoint_entry(),
            cleanup_exit_policy:
                crate::mir::builder::control_flow::cleanup::CleanupExitPolicyV1::capture_from_environment(),
        }
    }

    pub(in crate::mir::builder) const fn entry_safepoint_enabled(&self) -> bool {
        self.entry_safepoint
    }

    pub(in crate::mir::builder) fn script_args(&self) -> &[String] {
        &self.script_args
    }

    pub(in crate::mir::builder) const fn cleanup_exit_policy(
        &self,
    ) -> crate::mir::builder::control_flow::cleanup::CleanupExitPolicyV1 {
        self.cleanup_exit_policy
    }

    /// Empty inputs for disconnected and unit-test seams. The selected normal
    /// lifecycle always receives `capture_from_normal_ingress()`.
    pub(in crate::mir::builder) fn empty() -> Self {
        Self {
            script_args: Box::new([]),
            entry_safepoint: false,
            cleanup_exit_policy:
                crate::mir::builder::control_flow::cleanup::CleanupExitPolicyV1::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NormalRuntimeInputSnapshotV1;

    #[test]
    fn normal_snapshot_keeps_permissive_alias_and_safepoint_contract() {
        crate::test_support::with_env_vars(
            &[
                ("NYASH_SCRIPT_ARGS_JSON", Some("{malformed}")),
                ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["must-not-win"]"#)),
                ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some(" true ")),
            ],
            || {
                let snapshot = NormalRuntimeInputSnapshotV1::capture_from_normal_ingress();
                assert!(snapshot.script_args().is_empty());
                assert!(!snapshot.entry_safepoint_enabled());
            },
        );

        crate::test_support::with_env_vars(
            &[
                ("NYASH_SCRIPT_ARGS_JSON", None),
                ("HAKO_SCRIPT_ARGS_JSON", Some(r#"["alpha", "beta"]"#)),
                ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("On")),
            ],
            || {
                let snapshot = NormalRuntimeInputSnapshotV1::capture_from_normal_ingress();
                assert_eq!(snapshot.script_args(), ["alpha", "beta"]);
                assert!(snapshot.entry_safepoint_enabled());
            },
        );
    }

    #[test]
    fn normal_snapshot_is_immutable_after_ingress_capture() {
        let snapshot = crate::test_support::with_env_vars(
            &[
                ("NYASH_SCRIPT_ARGS_JSON", Some(r#"["before"]"#)),
                ("HAKO_SCRIPT_ARGS_JSON", None),
                ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("1")),
                ("NYASH_CLEANUP_ALLOW_RETURN", Some("1")),
                ("NYASH_CLEANUP_ALLOW_THROW", Some("0")),
            ],
            NormalRuntimeInputSnapshotV1::capture_from_normal_ingress,
        );
        crate::test_support::with_env_vars(
            &[
                ("NYASH_SCRIPT_ARGS_JSON", Some(r#"["after"]"#)),
                ("HAKO_SCRIPT_ARGS_JSON", None),
                ("NYASH_BUILDER_SAFEPOINT_ENTRY", Some("0")),
                ("NYASH_CLEANUP_ALLOW_RETURN", Some("0")),
                ("NYASH_CLEANUP_ALLOW_THROW", Some("1")),
            ],
            || {
                assert_eq!(snapshot.script_args(), ["before"]);
                assert!(snapshot.entry_safepoint_enabled());
                assert!(snapshot.cleanup_exit_policy().allows_return());
                assert!(!snapshot.cleanup_exit_policy().allows_throw());
            },
        );
    }
}
