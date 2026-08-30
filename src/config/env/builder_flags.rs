//! MIR builder-related environment flags
//!
//! Centralizes builder/debug toggles to avoid direct env reads.

use super::{env_bool, env_bool_default, env_present, env_string};
use std::ffi::OsStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BuilderMethodizeIngressErrorV1 {
    RetiredExplicitCompatibility,
    NonUnicode,
    InvalidSelector(Box<str>),
}

impl std::fmt::Display for BuilderMethodizeIngressErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RetiredExplicitCompatibility => formatter.write_str(
                "[rust-methodize/retired] HAKO_MIR_BUILDER_METHODIZE=1 is no longer accepted",
            ),
            Self::NonUnicode => formatter.write_str(
                "HAKO_MIR_BUILDER_METHODIZE must be valid Unicode (expected unset or 0)",
            ),
            Self::InvalidSelector(value) => write!(
                formatter,
                "invalid HAKO_MIR_BUILDER_METHODIZE selector: {value} (expected unset or 0)"
            ),
        }
    }
}

impl std::error::Error for BuilderMethodizeIngressErrorV1 {}

/// Validate the retired Rust methodize selector without touching process state.
///
/// `None` and exact `0` select the canonical non-methodizing route. Exact `1`
/// is a named retired contract, while every other value remains malformed.
pub(crate) fn validate_builder_methodize_selector_v1(
    raw: Option<&OsStr>,
) -> Result<(), BuilderMethodizeIngressErrorV1> {
    let Some(raw) = raw else {
        return Ok(());
    };
    let value = raw
        .to_str()
        .ok_or(BuilderMethodizeIngressErrorV1::NonUnicode)?;
    match value {
        "0" => Ok(()),
        "1" => Err(BuilderMethodizeIngressErrorV1::RetiredExplicitCompatibility),
        _ => Err(BuilderMethodizeIngressErrorV1::InvalidSelector(
            value.to_owned().into_boxed_str(),
        )),
    }
}

/// Validate the selector once at a named Rust module ingress.
pub(crate) fn validate_builder_methodize_ingress_v1() -> Result<(), BuilderMethodizeIngressErrorV1>
{
    let raw = std::env::var_os("HAKO_MIR_BUILDER_METHODIZE");
    validate_builder_methodize_selector_v1(raw.as_deref())
}

const BUILDER_OPERATOR_CALL_SELECTOR_KEYS_V1: [&str; 3] = [
    "NYASH_BUILDER_OPERATOR_BOX_ALL_CALL",
    "NYASH_BUILDER_OPERATOR_BOX_ADD_CALL",
    "NYASH_BUILDER_OPERATOR_BOX_COMPARE_CALL",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuilderOperatorCallIngressPolicyV1 {
    Direct,
    RetiredExplicitCompatibility { key: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuilderOperatorCallIngressErrorV1 {
    RetiredExplicitCompatibility { key: &'static str },
    InvalidSelector { key: &'static str },
    NonUnicode { key: &'static str },
}

impl std::fmt::Display for BuilderOperatorCallIngressErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RetiredExplicitCompatibility { key } => write!(
                formatter,
                "[mir/operator-call/retired] selector={key} is no longer accepted"
            ),
            Self::InvalidSelector { key } => write!(
                formatter,
                "[mir/operator-call/invalid] selector={key} has an invalid value"
            ),
            Self::NonUnicode { key } => write!(
                formatter,
                "[mir/operator-call/non-unicode] selector={key} must be valid Unicode"
            ),
        }
    }
}

impl std::error::Error for BuilderOperatorCallIngressErrorV1 {}

fn parse_builder_operator_call_selector_v1(
    key: &'static str,
    raw: Option<&OsStr>,
) -> Result<bool, BuilderOperatorCallIngressErrorV1> {
    let Some(raw) = raw else {
        return Ok(false);
    };
    let value = raw
        .to_str()
        .ok_or(BuilderOperatorCallIngressErrorV1::NonUnicode { key })?;
    match value.to_ascii_lowercase().as_str() {
        "0" | "false" | "off" => Ok(false),
        "1" | "true" | "on" => Ok(true),
        _ => Err(BuilderOperatorCallIngressErrorV1::InvalidSelector { key }),
    }
}

pub(crate) fn builder_operator_call_policy_from_values_v1(
    all: Option<&OsStr>,
    add: Option<&OsStr>,
    compare: Option<&OsStr>,
) -> Result<BuilderOperatorCallIngressPolicyV1, BuilderOperatorCallIngressErrorV1> {
    let all =
        parse_builder_operator_call_selector_v1(BUILDER_OPERATOR_CALL_SELECTOR_KEYS_V1[0], all)?;
    let add =
        parse_builder_operator_call_selector_v1(BUILDER_OPERATOR_CALL_SELECTOR_KEYS_V1[1], add)?;
    let compare = parse_builder_operator_call_selector_v1(
        BUILDER_OPERATOR_CALL_SELECTOR_KEYS_V1[2],
        compare,
    )?;

    if all {
        return Ok(
            BuilderOperatorCallIngressPolicyV1::RetiredExplicitCompatibility {
                key: BUILDER_OPERATOR_CALL_SELECTOR_KEYS_V1[0],
            },
        );
    }
    if add {
        return Ok(
            BuilderOperatorCallIngressPolicyV1::RetiredExplicitCompatibility {
                key: BUILDER_OPERATOR_CALL_SELECTOR_KEYS_V1[1],
            },
        );
    }
    if compare {
        return Ok(
            BuilderOperatorCallIngressPolicyV1::RetiredExplicitCompatibility {
                key: BUILDER_OPERATOR_CALL_SELECTOR_KEYS_V1[2],
            },
        );
    }
    Ok(BuilderOperatorCallIngressPolicyV1::Direct)
}

/// Read all Builder operator selectors once at a compiler ingress.
///
/// Direct MIR lowering remains the only accepted route.  A truthy legacy
/// selector is reported as a typed retirement, while malformed values are
/// rejected before any compiler effects.  The fixed key order also ensures an
/// invalid selector cannot be hidden by an earlier truthy selector.
pub(crate) fn validate_builder_operator_call_ingress_v1(
) -> Result<(), BuilderOperatorCallIngressErrorV1> {
    let all = std::env::var_os(BUILDER_OPERATOR_CALL_SELECTOR_KEYS_V1[0]);
    let add = std::env::var_os(BUILDER_OPERATOR_CALL_SELECTOR_KEYS_V1[1]);
    let compare = std::env::var_os(BUILDER_OPERATOR_CALL_SELECTOR_KEYS_V1[2]);
    match builder_operator_call_policy_from_values_v1(
        all.as_deref(),
        add.as_deref(),
        compare.as_deref(),
    )? {
        BuilderOperatorCallIngressPolicyV1::Direct => Ok(()),
        BuilderOperatorCallIngressPolicyV1::RetiredExplicitCompatibility { key } => {
            Err(BuilderOperatorCallIngressErrorV1::RetiredExplicitCompatibility { key })
        }
    }
}

pub fn builder_me_call_arity_strict() -> bool {
    env_bool_default("NYASH_ME_CALL_ARITY_STRICT", true)
}

pub fn builder_static_call_trace() -> bool {
    env_bool("NYASH_STATIC_CALL_TRACE")
}

pub fn builder_static_method_trace() -> bool {
    env_bool("NYASH_STATIC_METHOD_TRACE")
}

pub fn builder_conservative_phi_trace() -> bool {
    env_bool("NYASH_CONSERVATIVE_PHI_TRACE")
}

pub fn builder_type_registry_trace() -> bool {
    env_bool("NYASH_TYPE_REGISTRY_TRACE")
}

pub fn builder_source_file_hint() -> Option<String> {
    env_string("NYASH_SOURCE_FILE_HINT")
}

pub fn builder_router_trace() -> bool {
    env_string("NYASH_ROUTER_TRACE")
        .map(|v| matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "on" | "yes"))
        .unwrap_or(false)
}

pub fn builder_loopform_debug() -> bool {
    env_present("NYASH_LOOPFORM_DEBUG")
}

pub fn builder_use_type_registry() -> bool {
    env_bool("NYASH_USE_TYPE_REGISTRY")
}

pub fn builder_rewrite_known_default() -> Option<String> {
    env_string("NYASH_REWRITE_KNOWN_DEFAULT")
}

pub fn builder_rewrite_instance_mode() -> Option<String> {
    env_string("NYASH_BUILDER_REWRITE_INSTANCE")
}

pub fn builder_dev_rewrite_userbox() -> bool {
    env_bool("NYASH_DEV_REWRITE_USERBOX")
}

pub fn builder_dev_rewrite_new_origin() -> bool {
    env_bool("NYASH_DEV_REWRITE_NEW_ORIGIN")
}

pub fn builder_typefacts_debug() -> bool {
    env_present("NYASH_TYPEFACTS_DEBUG")
}

pub fn builder_birth_inject_builtins() -> bool {
    env_bool("NYASH_DEV_BIRTH_INJECT_BUILTINS")
}

pub fn builder_p3d_debug() -> bool {
    env_present("NYASH_P3D_DEBUG")
}

pub fn builder_p4_debug() -> bool {
    env_present("NYASH_P4_DEBUG")
}

pub fn builder_p3c_debug() -> bool {
    env_present("NYASH_P3C_DEBUG")
}

pub fn builder_local_ssa_trace() -> bool {
    env_bool("NYASH_LOCAL_SSA_TRACE")
}

pub fn builder_schedule_trace() -> bool {
    env_bool("NYASH_SCHEDULE_TRACE")
}

pub fn builder_block_schedule_verify() -> bool {
    env_bool("NYASH_BLOCK_SCHEDULE_VERIFY")
}

pub fn builder_trace_recv() -> bool {
    env_bool("NYASH_BUILDER_TRACE_RECV")
}

pub fn builder_mir_compile_trace() -> bool {
    env_bool("NYASH_MIR_COMPILE_TRACE")
}

pub fn builder_mir_type_trace() -> bool {
    env_bool("NYASH_MIR_TYPE_TRACE")
}

pub fn builder_debug_enabled() -> bool {
    env_present("NYASH_BUILDER_DEBUG")
}

pub fn builder_debug_limit() -> Option<usize> {
    env_string("NYASH_BUILDER_DEBUG_LIMIT").and_then(|s| s.parse::<usize>().ok())
}

pub fn builder_201a_debug() -> bool {
    env_present("NYASH_201A_DEBUG")
}

pub fn builder_if_trace() -> bool {
    env_bool("NYASH_IF_TRACE")
}

pub fn builder_build_static_main_entry() -> bool {
    env_present("NYASH_BUILD_STATIC_MAIN_ENTRY")
}

pub fn builder_script_args_json() -> Option<String> {
    env_string("NYASH_SCRIPT_ARGS_JSON").or_else(|| env_string("HAKO_SCRIPT_ARGS_JSON"))
}

pub fn builder_trycatch_debug() -> bool {
    env_bool("NYASH_DEBUG_TRYCATCH")
}

pub fn builder_boxcall_type_debug() -> bool {
    env_bool("NYASH_BOXCALL_TYPE_DEBUG")
}

pub fn builder_boxcall_type_trace() -> bool {
    env_bool("NYASH_BOXCALL_TYPE_TRACE")
}

pub fn builder_debug_kpi_known() -> bool {
    env_bool("NYASH_DEBUG_KPI_KNOWN")
}

pub fn builder_debug_sample_every() -> Option<usize> {
    env_string("NYASH_DEBUG_SAMPLE_EVERY").and_then(|s| s.parse::<usize>().ok())
}

pub fn builder_pin_trace() -> bool {
    env_bool("NYASH_PIN_TRACE")
}

pub fn builder_callee_resolve_trace() -> bool {
    env_bool("NYASH_CALLEE_RESOLVE_TRACE")
}

pub fn builder_debug_annotation() -> bool {
    env_bool("NYASH_DEBUG_ANNOTATION")
}

pub fn builder_debug_param_receiver() -> bool {
    env_bool("NYASH_DEBUG_PARAM_RECEIVER")
}

pub fn builder_call_resolve_trace() -> bool {
    env_bool("NYASH_CALL_RESOLVE_TRACE")
}

pub fn builder_unified_call_mode() -> Option<String> {
    env_string("NYASH_MIR_UNIFIED_CALL")
}

pub fn builder_trace_normalize() -> bool {
    env_present("NYASH_TRACE_NORMALIZE")
}

pub fn builder_trace_varmap() -> bool {
    env_present("NYASH_TRACE_VARMAP")
}

pub fn builder_option_c_debug() -> bool {
    env_present("NYASH_OPTION_C_DEBUG")
}

pub fn builder_capture_debug() -> bool {
    env_present("NYASH_CAPTURE_DEBUG")
}

pub fn builder_carrier_phi_debug() -> bool {
    env_bool("NYASH_CARRIER_PHI_DEBUG")
}

pub fn builder_safepoint_entry() -> bool {
    env_bool("NYASH_BUILDER_SAFEPOINT_ENTRY")
}

/// Raw source binding needs to distinguish an absent value from a present but
/// malformed one.  Keep the raw spelling in the centralized env vocabulary;
/// the Raw ingress owns strict validation and never reads the process env
/// after token issuance.
pub fn builder_safepoint_entry_raw_value() -> Option<String> {
    env_string("NYASH_BUILDER_SAFEPOINT_ENTRY")
}

#[cfg(test)]
mod methodize_ingress_tests {
    use super::{validate_builder_methodize_selector_v1, BuilderMethodizeIngressErrorV1};
    use std::ffi::OsStr;

    #[test]
    fn selector_matrix_is_finite_and_strict() {
        assert_eq!(validate_builder_methodize_selector_v1(None), Ok(()));
        assert_eq!(
            validate_builder_methodize_selector_v1(Some(OsStr::new("0"))),
            Ok(())
        );
        assert_eq!(
            validate_builder_methodize_selector_v1(Some(OsStr::new("1"))),
            Err(BuilderMethodizeIngressErrorV1::RetiredExplicitCompatibility)
        );

        for value in ["", "true", "on", "false", "off", "01", " 1 ", "garbage"] {
            assert!(matches!(
                validate_builder_methodize_selector_v1(Some(OsStr::new(value))),
                Err(BuilderMethodizeIngressErrorV1::InvalidSelector(_))
            ));
        }
    }

    #[cfg(unix)]
    #[test]
    fn non_unicode_selector_is_not_treated_as_unset() {
        use std::os::unix::ffi::OsStrExt;
        let raw = OsStr::from_bytes(b"\xff");
        assert_eq!(
            validate_builder_methodize_selector_v1(Some(raw)),
            Err(BuilderMethodizeIngressErrorV1::NonUnicode)
        );
    }
}

#[cfg(test)]
mod operator_call_ingress_tests {
    use super::{
        builder_operator_call_policy_from_values_v1, BuilderOperatorCallIngressErrorV1,
        BuilderOperatorCallIngressPolicyV1,
    };
    use std::ffi::OsStr;

    fn policy(
        all: Option<&str>,
        add: Option<&str>,
        compare: Option<&str>,
    ) -> Result<BuilderOperatorCallIngressPolicyV1, BuilderOperatorCallIngressErrorV1> {
        builder_operator_call_policy_from_values_v1(
            all.map(OsStr::new),
            add.map(OsStr::new),
            compare.map(OsStr::new),
        )
    }

    #[test]
    fn selector_matrix_is_finite_and_direct_by_default() {
        assert_eq!(
            policy(None, None, None),
            Ok(BuilderOperatorCallIngressPolicyV1::Direct)
        );
        for value in ["0", "false", "FALSE", "off", "Off"] {
            assert_eq!(
                policy(Some(value), Some(value), Some(value)),
                Ok(BuilderOperatorCallIngressPolicyV1::Direct)
            );
        }
        assert_eq!(
            policy(Some("1"), None, None),
            Ok(
                BuilderOperatorCallIngressPolicyV1::RetiredExplicitCompatibility {
                    key: "NYASH_BUILDER_OPERATOR_BOX_ALL_CALL"
                }
            )
        );
        assert_eq!(
            policy(None, Some("TRUE"), None),
            Ok(
                BuilderOperatorCallIngressPolicyV1::RetiredExplicitCompatibility {
                    key: "NYASH_BUILDER_OPERATOR_BOX_ADD_CALL"
                }
            )
        );
        assert_eq!(
            policy(None, None, Some("on")),
            Ok(
                BuilderOperatorCallIngressPolicyV1::RetiredExplicitCompatibility {
                    key: "NYASH_BUILDER_OPERATOR_BOX_COMPARE_CALL"
                }
            )
        );
    }

    #[test]
    fn malformed_and_non_unicode_values_cannot_hide_behind_truthy_values() {
        assert_eq!(
            policy(Some("1"), Some("garbage"), None),
            Err(BuilderOperatorCallIngressErrorV1::InvalidSelector {
                key: "NYASH_BUILDER_OPERATOR_BOX_ADD_CALL"
            })
        );
        assert_eq!(
            policy(Some("1"), None, Some(" 1 ")),
            Err(BuilderOperatorCallIngressErrorV1::InvalidSelector {
                key: "NYASH_BUILDER_OPERATOR_BOX_COMPARE_CALL"
            })
        );
    }

    #[cfg(unix)]
    #[test]
    fn operator_non_unicode_selector_is_not_treated_as_unset() {
        use std::os::unix::ffi::OsStrExt;
        let raw = OsStr::from_bytes(b"\xff");
        assert_eq!(
            super::builder_operator_call_policy_from_values_v1(None, None, Some(raw)),
            Err(BuilderOperatorCallIngressErrorV1::NonUnicode {
                key: "NYASH_BUILDER_OPERATOR_BOX_COMPARE_CALL"
            })
        );
    }
}
