pub const ROUTE_COMPAT: &str = "compat";
pub const ROUTE_STAGE_A: &str = ROUTE_COMPAT;

pub const LANE_DIRECT: &str = "direct";
pub const LANE_COMPAT_PROGRAM_TO_MIR: &str = "compat-program-to-mir";
pub const LANE_COMPAT_RUST_JSON_V0_BRIDGE: &str = "compat-rust-json-v0-bridge";
pub const FREEZE_COMPAT_RUST_JSON_V0_BRIDGE: &str = "runtime-route/compat-rust-json-v0-bridge";
pub const LANE_DIRECT_V0_BRIDGE: &str = "direct-v0-bridge";
pub const FREEZE_DIRECT_V0_BRIDGE_DISABLED: &str = "runtime-route/direct-v0-bridge-disabled";

pub fn format_expected_mir_non_strict_compat_disabled(source: &str) -> String {
    format!(
        "[contract][runtime-route][expected=mir-json] route={} source={} non_strict_compat=disabled require=NYASH_VM_USE_FALLBACK=1",
        ROUTE_COMPAT, source
    )
}

pub fn format_expected_mir_strict_planner_required(source: &str) -> String {
    format!(
        "[contract][runtime-route][expected=mir-json] route={} source={} got=program-json strict_planner_required=1",
        ROUTE_COMPAT, source
    )
}

pub fn format_freeze_compat_rust_json_v0_bridge(source: &str) -> String {
    format!(
        "[freeze:contract][{}] route={} source={} lane={} require=NYASH_VM_USE_FALLBACK=1",
        FREEZE_COMPAT_RUST_JSON_V0_BRIDGE, ROUTE_COMPAT, source, LANE_COMPAT_RUST_JSON_V0_BRIDGE
    )
}

pub fn format_accepted_mir(source: &str, lane: &str) -> String {
    format!(
        "[contract][runtime-route][accepted=mir-json] route={} source={} lane={}",
        ROUTE_COMPAT, source, lane
    )
}

pub fn format_freeze_direct_v0_bridge_disabled(source: &str) -> String {
    format!(
        "[freeze:contract][{}] route={} source={} lane={} status=retired",
        FREEZE_DIRECT_V0_BRIDGE_DISABLED, ROUTE_COMPAT, source, LANE_DIRECT_V0_BRIDGE
    )
}

pub fn emit_expected_mir_non_strict_compat_disabled(source: &str) {
    eprintln!("{}", format_expected_mir_non_strict_compat_disabled(source));
}

pub fn emit_expected_mir_strict_planner_required(source: &str) {
    eprintln!("{}", format_expected_mir_strict_planner_required(source));
}

pub fn emit_freeze_compat_rust_json_v0_bridge(source: &str) {
    eprintln!("{}", format_freeze_compat_rust_json_v0_bridge(source));
}

pub fn emit_accepted_mir(source: &str, lane: &str) {
    eprintln!("{}", format_accepted_mir(source, lane));
}

pub fn emit_freeze_direct_v0_bridge_disabled(source: &str) {
    eprintln!("{}", format_freeze_direct_v0_bridge_disabled(source));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_expected_mir_non_strict_is_stable() {
        let line = format_expected_mir_non_strict_compat_disabled("foo.hako");
        assert_eq!(
            line,
            "[contract][runtime-route][expected=mir-json] route=compat source=foo.hako non_strict_compat=disabled require=NYASH_VM_USE_FALLBACK=1"
        );
    }

    #[test]
    fn format_expected_mir_strict_planner_required_is_stable() {
        let line = format_expected_mir_strict_planner_required("foo.hako");
        assert_eq!(
            line,
            "[contract][runtime-route][expected=mir-json] route=compat source=foo.hako got=program-json strict_planner_required=1"
        );
    }

    #[test]
    fn format_freeze_compat_rust_json_v0_bridge_is_stable() {
        let line = format_freeze_compat_rust_json_v0_bridge("foo.hako");
        assert_eq!(
            line,
            "[freeze:contract][runtime-route/compat-rust-json-v0-bridge] route=compat source=foo.hako lane=compat-rust-json-v0-bridge require=NYASH_VM_USE_FALLBACK=1"
        );
    }

    #[test]
    fn format_accepted_mir_is_stable() {
        let line = format_accepted_mir("foo.hako", LANE_DIRECT);
        assert_eq!(
            line,
            "[contract][runtime-route][accepted=mir-json] route=compat source=foo.hako lane=direct"
        );
    }

    #[test]
    fn format_freeze_direct_v0_bridge_disabled_is_stable() {
        let line = format_freeze_direct_v0_bridge_disabled("foo.hako");
        assert_eq!(
            line,
            "[freeze:contract][runtime-route/direct-v0-bridge-disabled] route=compat source=foo.hako lane=direct-v0-bridge status=retired"
        );
    }
}
