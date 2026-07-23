//! RAW-SOURCE0 ROOT0 ELIGIBILITY0: immutable Raw runtime-input snapshot.
//!
//! The compiler captures these ambient compatibility inputs once, before a
//! Raw invocation token is issued. Later lowering owners receive only this
//! typed value and must not read the environment again.

/// Exact script-argument presence captured at Raw ingress.
///
/// `Present([])` is intentionally distinct from `Absent`: the source of truth
/// records that a valid, explicitly empty JSON array was supplied even though
/// both dispositions materialize an empty runtime array.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum RawScriptArgsSnapshotV1 {
    Absent,
    Present(Box<[String]>),
}

impl RawScriptArgsSnapshotV1 {
    pub(in crate::mir) fn values(&self) -> Option<&[String]> {
        match self {
            Self::Absent => None,
            Self::Present(values) => Some(values),
        }
    }
}

/// Entry-safepoint policy captured through the existing config/env SSOT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawEntrySafepointV1 {
    Disabled,
    Enabled,
}

impl RawEntrySafepointV1 {
    pub(in crate::mir) const fn is_enabled(self) -> bool {
        matches!(self, Self::Enabled)
    }
}

/// One immutable ingress snapshot retained by the Raw source continuation.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct RawRuntimeInputSnapshotV1 {
    script_args: RawScriptArgsSnapshotV1,
    entry_safepoint: RawEntrySafepointV1,
}

impl RawRuntimeInputSnapshotV1 {
    /// Capture through the centralized environment vocabulary exactly once.
    pub(in crate::mir) fn capture() -> Result<Self, RawRuntimeInputCaptureErrorV1> {
        let script_args = match crate::config::env::builder_script_args_json() {
            None => RawScriptArgsSnapshotV1::Absent,
            Some(raw) => {
                let values = serde_json::from_str::<Vec<String>>(&raw).map_err(|error| {
                    RawRuntimeInputCaptureErrorV1::MalformedScriptArgsJson {
                        message: error.to_string().into_boxed_str(),
                    }
                })?;
                RawScriptArgsSnapshotV1::Present(values.into_boxed_slice())
            }
        };
        let entry_safepoint = match crate::config::env::builder_safepoint_entry_raw_value() {
            None => RawEntrySafepointV1::Disabled,
            Some(raw) => match raw.trim().to_ascii_lowercase().as_str() {
                "1" | "true" | "on" => RawEntrySafepointV1::Enabled,
                "0" | "false" | "off" => RawEntrySafepointV1::Disabled,
                _ => {
                    return Err(RawRuntimeInputCaptureErrorV1::MalformedEntrySafepoint {
                        value: raw.into_boxed_str(),
                    });
                }
            },
        };
        Ok(Self {
            script_args,
            entry_safepoint,
        })
    }

    pub(in crate::mir) const fn script_args(&self) -> &RawScriptArgsSnapshotV1 {
        &self.script_args
    }

    pub(in crate::mir) const fn entry_safepoint(&self) -> RawEntrySafepointV1 {
        self.entry_safepoint
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRuntimeInputCaptureErrorV1 {
    MalformedScriptArgsJson { message: Box<str> },
    MalformedEntrySafepoint { value: Box<str> },
}

impl std::fmt::Display for RawRuntimeInputCaptureErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "[freeze:contract][raw_runtime_inputs] {self:?}")
    }
}

impl std::error::Error for RawRuntimeInputCaptureErrorV1 {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvRestore {
        values: Vec<(&'static str, Option<std::ffi::OsString>)>,
    }

    impl EnvRestore {
        fn capture(keys: &[&'static str]) -> Self {
            Self {
                values: keys
                    .iter()
                    .map(|key| (*key, std::env::var_os(key)))
                    .collect(),
            }
        }
    }

    impl Drop for EnvRestore {
        fn drop(&mut self) {
            for (key, value) in &self.values {
                match value {
                    Some(value) => std::env::set_var(key, value),
                    None => std::env::remove_var(key),
                }
            }
        }
    }

    #[test]
    fn capture_distinguishes_absent_from_explicit_empty() {
        let _guard = ENV_LOCK.lock().unwrap();
        let _restore = EnvRestore::capture(&[
            "NYASH_SCRIPT_ARGS_JSON",
            "HAKO_SCRIPT_ARGS_JSON",
            "NYASH_BUILDER_SAFEPOINT_ENTRY",
        ]);
        std::env::remove_var("NYASH_SCRIPT_ARGS_JSON");
        std::env::remove_var("HAKO_SCRIPT_ARGS_JSON");
        std::env::remove_var("NYASH_BUILDER_SAFEPOINT_ENTRY");
        assert!(matches!(
            RawRuntimeInputSnapshotV1::capture().unwrap().script_args(),
            RawScriptArgsSnapshotV1::Absent
        ));

        std::env::set_var("NYASH_SCRIPT_ARGS_JSON", "[]");
        let snapshot = RawRuntimeInputSnapshotV1::capture().unwrap();
        assert!(matches!(
            snapshot.script_args(),
            RawScriptArgsSnapshotV1::Present(values) if values.is_empty()
        ));
    }

    #[test]
    fn capture_rejects_malformed_values_before_binding() {
        let _guard = ENV_LOCK.lock().unwrap();
        let _restore = EnvRestore::capture(&[
            "NYASH_SCRIPT_ARGS_JSON",
            "HAKO_SCRIPT_ARGS_JSON",
            "NYASH_BUILDER_SAFEPOINT_ENTRY",
        ]);
        std::env::set_var("NYASH_SCRIPT_ARGS_JSON", "{not-an-array}");
        std::env::remove_var("HAKO_SCRIPT_ARGS_JSON");
        std::env::remove_var("NYASH_BUILDER_SAFEPOINT_ENTRY");
        assert!(matches!(
            RawRuntimeInputSnapshotV1::capture(),
            Err(RawRuntimeInputCaptureErrorV1::MalformedScriptArgsJson { .. })
        ));

        std::env::remove_var("NYASH_SCRIPT_ARGS_JSON");
        std::env::set_var("NYASH_BUILDER_SAFEPOINT_ENTRY", "maybe");
        assert!(matches!(
            RawRuntimeInputSnapshotV1::capture(),
            Err(RawRuntimeInputCaptureErrorV1::MalformedEntrySafepoint { .. })
        ));
    }
}
