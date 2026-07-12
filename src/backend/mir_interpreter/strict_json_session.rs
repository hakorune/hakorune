//! Invocation-owned lifetime boundary for the internal strict-JSON tree.
//!
//! This module owns only session identity and cleanup.  ProgramV0 validation,
//! path construction, budgets, and snapshot semantics stay outside it.

use super::{MirInterpreter, VMError, VMValue};
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

    /// Execute one Hako reader entry with the invocation-owned tree identity.
    ///
    /// The Hako side receives only `(session_handle, root_node)` and cannot
    /// construct, close, or retain the Rust session itself.
    pub(crate) fn execute_function_with_strict_json_session(
        &mut self,
        module: &crate::mir::MirModule,
        input: &str,
        function: &str,
    ) -> Result<VMValue, VMError> {
        crate::mir::backend_capability::enforce_mir_backend_supported(module, "mir-interpreter")
            .map_err(VMError::InvalidInstruction)?;
        let guard = self.open_strict_json_session(input)?;
        let handle = guard.handle();
        let root = guard.root_node()?;
        guard.interpreter.execute_function_with_args(
            module,
            function,
            &[VMValue::Integer(handle), VMValue::Integer(root)],
        )
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

    fn compile_root_reader_fixture() -> crate::mir::MirModule {
        let fixture = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tools/checks/fixtures/bounded_body_snapshot_root_reader_v0.hako"
        );
        let source = std::fs::read_to_string(fixture).expect("root reader fixture");
        let runner = crate::runner::NyashRunner::new(crate::cli::CliConfig::default());
        let (merged, imports) =
            crate::runner::modes::common_util::resolve::merge_prelude_text_with_imports(
                &runner, &source, fixture,
            )
            .expect("merge root reader imports");
        let prepared = crate::runner::modes::common_util::source_hint::prepare_source_minimal(
            &merged, fixture,
        )
        .expect("normalize root reader source");
        let ast = crate::parser::NyashParser::parse_from_string(&prepared).expect("parse fixture");
        crate::mir::MirCompiler::with_options(false)
            .compile_with_source_and_imports(ast, Some(fixture), imports)
            .expect("compile root reader")
            .module
    }

    fn run_root_reader(
        interpreter: &mut MirInterpreter,
        module: &crate::mir::MirModule,
        input: &str,
    ) -> Result<i64, VMError> {
        match interpreter.execute_function_with_strict_json_session(
            module,
            input,
            "SnapshotRootReaderFixtureV0Box.read/2",
        )? {
            VMValue::Integer(value) => Ok(value),
            other => Err(VMError::TypeError(format!(
                "root reader fixture returned {other:?}"
            ))),
        }
    }

    fn rust_root_error_code(input: &str) -> i64 {
        use crate::analysis::bounded_body_snapshot_v0::ProgramV0BodyViewError;
        match crate::analysis::bounded_body_snapshot_v0::read_program_v0_body(input) {
            Ok(_) => 0,
            Err(ProgramV0BodyViewError::Unsupported { .. }) => 20,
            Err(ProgramV0BodyViewError::InvalidInput { path, reason }) => {
                match (path.as_str(), reason.as_str()) {
                    ("$.version", "object.required_field_missing") => 11,
                    ("$.version", "program.version_must_be_zero") => 18,
                    ("$.kind", "program.kind_must_be_program") => 12,
                    ("$.kind", "object.required_field_missing") => 21,
                    ("$.body", "object.required_field_missing") => 13,
                    ("$", "type.expected_object.got_array") => 14,
                    ("$.future", "object.forbidden_unknown_field") => 15,
                    ("$.defs", "type.expected_array.got_object") => 16,
                    ("$.attrs", "type.expected_object.got_array") => 22,
                    ("$.body", "type.expected_array.got_object") => 17,
                    _ => 99,
                }
            }
        }
    }

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

    #[test]
    fn hako_root_reader_accepts_empty_program_and_closes_each_session() {
        let module = compile_root_reader_fixture();
        let mut interpreter = MirInterpreter::new();
        for input in [
            r#"{"version":0,"kind":"Program","body":[]}"#,
            r#"{"body":[],"attrs":{},"defs":[],"kind":"Program","version":0}"#,
        ] {
            assert_eq!(
                run_root_reader(&mut interpreter, &module, input).unwrap(),
                0
            );
            assert!(!interpreter.strict_json_session_active());
        }
    }

    #[test]
    fn hako_root_reader_owns_envelope_errors_and_nonempty_stop() {
        let module = compile_root_reader_fixture();
        let mut interpreter = MirInterpreter::new();
        for (input, expected) in [
            (r#"{"kind":"Program","body":[]}"#, 11),
            (r#"{"version":1,"kind":"Program","body":[]}"#, 18),
            (r#"{"version":0,"body":[]}"#, 21),
            (r#"{"version":0,"kind":1,"body":[]}"#, 12),
            (r#"{"version":0,"kind":"Program"}"#, 13),
            (r#"[]"#, 14),
            (r#"{"version":0,"kind":"Program","body":[],"future":1}"#, 15),
            (r#"{"version":0,"kind":"Program","body":[],"defs":{}}"#, 16),
            (r#"{"attrs":[],"kind":"Program","body":[]}"#, 11),
            (
                r#"{"version":0,"defs":{},"attrs":[],"kind":"Program","body":[]}"#,
                22,
            ),
            (r#"{"version":0,"kind":"Program","body":{}}"#, 17),
            (r#"{"version":0,"kind":"Program","body":[{}]}"#, 20),
        ] {
            assert_eq!(
                run_root_reader(&mut interpreter, &module, input).unwrap(),
                expected,
                "input={input}"
            );
            if expected != 20 {
                assert_eq!(
                    rust_root_error_code(input),
                    expected,
                    "Rust parity input={input}"
                );
            }
            assert!(!interpreter.strict_json_session_active());
        }
    }

    #[test]
    fn duplicate_key_fails_before_hako_root_reader_effects() {
        let module = compile_root_reader_fixture();
        let mut interpreter = MirInterpreter::new();
        let error = run_root_reader(
            &mut interpreter,
            &module,
            r#"{"version":0,"kind":"Program","body":[],"\u006b\u0069\u006e\u0064":"Program"}"#,
        )
        .expect_err("decoded duplicate key");
        assert!(error.to_string().contains(INPUT_TAG));
        assert!(!interpreter.strict_json_session_active());
    }
}
