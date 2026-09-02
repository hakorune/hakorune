//! Invocation-owned debug/strict policy for the selected MIR emit boundary.
//!
//! The process environment is parsed at invocation ingress and the resulting
//! value is borrowed by emit code. This module owns no semantic route, target,
//! or backend policy; it only preserves the existing flag vocabulary for one
//! Builder session.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct BuilderEmitDebugPolicySnapshotV1 {
    joinir_debug: bool,
    joinir_strict: bool,
    joinir_planner_required: bool,
    local_ssa_trace: bool,
    trace_recv: bool,
    builder_debug: bool,
    static_call_trace: bool,
    static_method_trace: bool,
    call_resolve_trace: bool,
}

impl BuilderEmitDebugPolicySnapshotV1 {
    pub(in crate::mir::builder) fn from_environment() -> Self {
        Self {
            joinir_debug: crate::config::env::joinir_dev::debug_enabled(),
            joinir_strict: crate::config::env::joinir_dev::strict_enabled(),
            joinir_planner_required: crate::config::env::joinir_dev::planner_required_enabled(),
            local_ssa_trace: crate::config::env::builder_local_ssa_trace(),
            trace_recv: crate::config::env::builder_trace_recv(),
            builder_debug: crate::config::env::builder_debug_enabled(),
            static_call_trace: crate::config::env::builder_static_call_trace(),
            static_method_trace: crate::config::env::builder_static_method_trace(),
            call_resolve_trace: crate::config::env::builder_call_resolve_trace(),
        }
    }

    pub(in crate::mir::builder) const fn strict_planner_required_debug_enabled(self) -> bool {
        self.joinir_strict && self.joinir_planner_required && self.joinir_debug
    }

    pub(in crate::mir::builder) const fn joinir_debug_enabled(self) -> bool {
        self.joinir_debug
    }

    pub(in crate::mir::builder) const fn local_ssa_trace(self) -> bool {
        self.local_ssa_trace
    }

    pub(in crate::mir::builder) const fn trace_recv(self) -> bool {
        self.trace_recv
    }

    pub(in crate::mir::builder) const fn builder_debug_enabled(self) -> bool {
        self.builder_debug
    }

    pub(in crate::mir::builder) const fn static_call_trace(self) -> bool {
        self.static_call_trace
    }

    pub(in crate::mir::builder) const fn static_method_trace(self) -> bool {
        self.static_method_trace
    }

    pub(in crate::mir::builder) const fn call_resolve_trace(self) -> bool {
        self.call_resolve_trace
    }
}

impl Default for BuilderEmitDebugPolicySnapshotV1 {
    fn default() -> Self {
        Self {
            joinir_debug: false,
            joinir_strict: false,
            joinir_planner_required: false,
            local_ssa_trace: false,
            trace_recv: false,
            builder_debug: false,
            static_call_trace: false,
            static_method_trace: false,
            call_resolve_trace: false,
        }
    }
}
