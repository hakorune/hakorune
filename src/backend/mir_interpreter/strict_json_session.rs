//! Invocation-owned lifetime boundary for the internal strict-JSON tree.
//!
//! This module owns only session identity and cleanup.  ProgramV0 validation,
//! path construction, budgets, and snapshot semantics stay outside it.

use super::{MirInterpreter, VMError};
use crate::analysis::bounded_body_snapshot_v0::{StrictJsonArenaV0, StrictJsonNodeIdV0};

const INTERNAL_TAG: &str = "[analysis/strict_json_tree_v0/internal_bridge_contract_violation]";
const INPUT_TAG: &str = "[analysis/strict_json_tree_v0/invalid_input]";

pub(crate) struct StrictJsonSessionV0 {
    handle: i64,
    arena: StrictJsonArenaV0,
}

impl StrictJsonSessionV0 {
    fn open(handle: i64, input: &str) -> Result<Self, VMError> {
        let arena = StrictJsonArenaV0::parse(input)
            .map_err(|reason| VMError::InvalidInstruction(format!("{INPUT_TAG} {reason}")))?;
        Ok(Self { handle, arena })
    }

    pub(super) fn handle(&self) -> i64 {
        self.handle
    }

    pub(super) fn arena(&self) -> &StrictJsonArenaV0 {
        &self.arena
    }

    pub(super) fn node_id(&self, raw: i64) -> Result<StrictJsonNodeIdV0, VMError> {
        StrictJsonNodeIdV0::from_i64(raw).ok_or_else(|| {
            VMError::InvalidInstruction(format!("{INTERNAL_TAG} invalid_node_id={raw}"))
        })
    }
}

pub(super) struct StrictJsonSessionGuard<'a> {
    pub(super) interpreter: &'a mut MirInterpreter,
    handle: i64,
}

impl StrictJsonSessionGuard<'_> {
    pub(super) fn handle(&self) -> i64 {
        self.handle
    }

    pub(super) fn root_node(&self) -> Result<i64, VMError> {
        let session = self
            .interpreter
            .strict_json_session
            .as_ref()
            .ok_or_else(|| VMError::InvalidInstruction(format!("{INTERNAL_TAG} session_closed")))?;
        if session.handle() != self.handle {
            return Err(VMError::InvalidInstruction(format!(
                "{INTERNAL_TAG} stale_strict_json_session"
            )));
        }
        Ok(i64::from(session.arena().root().raw()))
    }
}

impl Drop for StrictJsonSessionGuard<'_> {
    fn drop(&mut self) {
        let should_close = self
            .interpreter
            .strict_json_session
            .as_ref()
            .is_some_and(|session| session.handle() == self.handle);
        if should_close {
            self.interpreter.strict_json_session = None;
        }
    }
}

impl MirInterpreter {
    pub(crate) fn execute_module_with_strict_json_session(
        &mut self,
        module: &crate::mir::MirModule,
        input: &str,
    ) -> Result<Box<dyn crate::box_trait::NyashBox>, VMError> {
        crate::mir::backend_capability::enforce_mir_backend_supported(module, "mir-interpreter")
            .map_err(VMError::InvalidInstruction)?;
        let guard = self.open_strict_json_session(input)?;
        guard.interpreter.execute_module(module)
    }

    pub(super) fn open_strict_json_session(
        &mut self,
        input: &str,
    ) -> Result<StrictJsonSessionGuard<'_>, VMError> {
        if self.strict_json_session.is_some() {
            return Err(VMError::InvalidInstruction(format!(
                "{INTERNAL_TAG} nested_session"
            )));
        }
        let generation = self
            .strict_json_generation
            .checked_add(1)
            .filter(|value| *value <= i64::MAX as u64)
            .ok_or_else(|| VMError::InvalidInstruction(format!("{INTERNAL_TAG} generation")))?;
        self.strict_json_generation = generation;
        let handle = generation as i64;
        self.strict_json_session = Some(StrictJsonSessionV0::open(handle, input)?);
        Ok(StrictJsonSessionGuard {
            interpreter: self,
            handle,
        })
    }

    pub(super) fn strict_json_session_active(&self) -> bool {
        self.strict_json_session.is_some()
    }

    pub(super) fn strict_json_session_for(
        &self,
        handle: i64,
    ) -> Result<&StrictJsonSessionV0, VMError> {
        let session = self.strict_json_session.as_ref().ok_or_else(|| {
            VMError::InvalidInstruction(format!("{INTERNAL_TAG} stale_strict_json_session"))
        })?;
        if session.handle() != handle {
            return Err(VMError::InvalidInstruction(format!(
                "{INTERNAL_TAG} stale_strict_json_session"
            )));
        }
        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_is_invocation_scoped_and_raii_closed() {
        let mut interpreter = MirInterpreter::new();
        assert!(!interpreter.strict_json_session_active());
        {
            let guard = interpreter
                .open_strict_json_session(r#"{"body":[]}"#)
                .expect("session");
            assert_ne!(guard.handle(), 0);
            assert_eq!(guard.root_node().expect("root"), 0);
            assert!(guard.interpreter.strict_json_session_active());
        }
        assert!(!interpreter.strict_json_session_active());
    }

    #[test]
    fn nested_session_and_stale_handle_fail_as_bridge_errors() {
        let mut interpreter = MirInterpreter::new();
        let guard = interpreter
            .open_strict_json_session(r#"{"body":[]}"#)
            .expect("session");
        let nested = guard
            .interpreter
            .open_strict_json_session(r#"{"body":[]}"#)
            .err()
            .expect("nested session must fail");
        assert!(nested.to_string().contains("nested_session"));
        let handle = guard.handle();
        assert!(guard.interpreter.strict_json_session_for(handle).is_ok());
        assert!(guard
            .interpreter
            .strict_json_session_for(handle + 1)
            .is_err());
        drop(guard);
        assert!(!interpreter.strict_json_session_active());
    }

    #[test]
    fn malformed_strict_json_is_input_failure_before_session_publish() {
        let mut interpreter = MirInterpreter::new();
        let error = interpreter
            .open_strict_json_session(r#"{"a":1,"\u0061":2}"#)
            .err()
            .expect("duplicate key");
        assert!(error.to_string().contains(INPUT_TAG));
        assert!(!interpreter.strict_json_session_active());
    }
}
