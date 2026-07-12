//! Runtime-direct generic strict-JSON arena accessors.
//!
//! This surface is intentionally generic.  It does not know ProgramV0 field
//! names, tags, operators, paths, limits, or snapshot vocabulary.

use super::super::*;
use crate::analysis::bounded_body_snapshot_v0::StrictJsonKindV0;
use crate::backend::mir_interpreter::utils::error_helpers::ErrorBuilder;

const INTERNAL_TAG: &str = "[analysis/strict_json_tree_v0/internal_bridge_contract_violation]";

fn kind_name(kind: StrictJsonKindV0) -> &'static str {
    match kind {
        StrictJsonKindV0::Null => "Null",
        StrictJsonKindV0::Bool => "Bool",
        StrictJsonKindV0::I64 => "I64",
        StrictJsonKindV0::U64 => "U64",
        StrictJsonKindV0::F64 => "F64",
        StrictJsonKindV0::String => "String",
        StrictJsonKindV0::Array => "Array",
        StrictJsonKindV0::Object => "Object",
    }
}

impl MirInterpreter {
    pub(super) fn dispatch_strict_json_tree_extern(
        &mut self,
        extern_name: &str,
        args: &[ValueId],
    ) -> Result<VMValue, VMError> {
        let expected = match extern_name {
            "hako.analysis.strict_json_tree_v0.kind"
            | "hako.analysis.strict_json_tree_v0.object_len"
            | "hako.analysis.strict_json_tree_v0.array_len"
            | "hako.analysis.strict_json_tree_v0.string_value"
            | "hako.analysis.strict_json_tree_v0.bool_value"
            | "hako.analysis.strict_json_tree_v0.i64_value"
            | "hako.analysis.strict_json_tree_v0.u64_fits_i64"
            | "hako.analysis.strict_json_tree_v0.u64_as_i64" => 2,
            "hako.analysis.strict_json_tree_v0.object_key_at"
            | "hako.analysis.strict_json_tree_v0.object_value_at"
            | "hako.analysis.strict_json_tree_v0.array_at" => 3,
            _ => {
                return Err(
                    self.err_invalid(format!("{INTERNAL_TAG} unknown_accessor={extern_name}"))
                )
            }
        };
        if args.len() != expected {
            return Err(ErrorBuilder::arg_count_mismatch(
                extern_name,
                expected,
                args.len(),
            ));
        }
        let handle = self.strict_json_i64_arg(args[0], "session_handle")?;
        let node = self.strict_json_i64_arg(args[1], "node_id")?;
        let index = if expected == 3 {
            Some(self.strict_json_index_arg(args[2], "index")?)
        } else {
            None
        };
        let session = self.strict_json_session_for(handle)?;
        let node = session.node_id(node)?;
        let arena = session.arena();

        match extern_name {
            "hako.analysis.strict_json_tree_v0.kind" => {
                let kind = arena
                    .kind(node)
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} out_of_range_node")))?;
                Ok(VMValue::String(kind_name(kind).to_string()))
            }
            "hako.analysis.strict_json_tree_v0.object_len" => {
                let len = arena
                    .object_len(node)
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} wrong_kind=Object")))?;
                Ok(VMValue::Integer(self.strict_json_len_i64(len)?))
            }
            "hako.analysis.strict_json_tree_v0.object_key_at" => {
                let index = index
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} missing_index")))?;
                let key = arena.object_key_at(node, index).ok_or_else(|| {
                    self.err_invalid(format!("{INTERNAL_TAG} invalid_object_member"))
                })?;
                Ok(VMValue::String(key.to_string()))
            }
            "hako.analysis.strict_json_tree_v0.object_value_at" => {
                let index = index
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} missing_index")))?;
                let child = arena.object_value_at(node, index).ok_or_else(|| {
                    self.err_invalid(format!("{INTERNAL_TAG} invalid_object_member"))
                })?;
                Ok(VMValue::Integer(i64::from(child.raw())))
            }
            "hako.analysis.strict_json_tree_v0.array_len" => {
                let len = arena
                    .array_len(node)
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} wrong_kind=Array")))?;
                Ok(VMValue::Integer(self.strict_json_len_i64(len)?))
            }
            "hako.analysis.strict_json_tree_v0.array_at" => {
                let index = index
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} missing_index")))?;
                let child = arena.array_at(node, index).ok_or_else(|| {
                    self.err_invalid(format!("{INTERNAL_TAG} invalid_array_index"))
                })?;
                Ok(VMValue::Integer(i64::from(child.raw())))
            }
            "hako.analysis.strict_json_tree_v0.string_value" => {
                let value = arena
                    .string_value(node)
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} wrong_kind=String")))?;
                Ok(VMValue::String(value.to_string()))
            }
            "hako.analysis.strict_json_tree_v0.bool_value" => {
                let value = arena
                    .bool_value(node)
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} wrong_kind=Bool")))?;
                Ok(VMValue::Bool(value))
            }
            "hako.analysis.strict_json_tree_v0.i64_value" => {
                let value = arena
                    .i64_value(node)
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} wrong_kind=I64")))?;
                Ok(VMValue::Integer(value))
            }
            "hako.analysis.strict_json_tree_v0.u64_fits_i64" => {
                let value = arena
                    .u64_value(node)
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} wrong_kind=U64")))?;
                Ok(VMValue::Bool(i64::try_from(value).is_ok()))
            }
            "hako.analysis.strict_json_tree_v0.u64_as_i64" => {
                let value = arena
                    .u64_value(node)
                    .ok_or_else(|| self.err_invalid(format!("{INTERNAL_TAG} wrong_kind=U64")))?;
                let value = i64::try_from(value).map_err(|_| {
                    self.err_invalid(format!("{INTERNAL_TAG} u64_does_not_fit_i64"))
                })?;
                Ok(VMValue::Integer(value))
            }
            _ => Err(self.err_invalid(format!("{INTERNAL_TAG} unknown_accessor={extern_name}"))),
        }
    }

    fn strict_json_i64_arg(&mut self, value: ValueId, role: &str) -> Result<i64, VMError> {
        match self.reg_load(value)? {
            VMValue::Integer(value) => Ok(value),
            _ => Err(self.err_invalid(format!("{INTERNAL_TAG} {role}_must_be_i64"))),
        }
    }

    fn strict_json_index_arg(&mut self, value: ValueId, role: &str) -> Result<usize, VMError> {
        let value = self.strict_json_i64_arg(value, role)?;
        usize::try_from(value)
            .map_err(|_| self.err_invalid(format!("{INTERNAL_TAG} {role}_out_of_range")))
    }

    fn strict_json_len_i64(&self, value: usize) -> Result<i64, VMError> {
        i64::try_from(value)
            .map_err(|_| self.err_invalid(format!("{INTERNAL_TAG} length_out_of_range")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put_i64(interpreter: &mut MirInterpreter, id: u32, value: i64) {
        interpreter.write_reg(ValueId::new(id), VMValue::Integer(value));
    }

    #[test]
    fn generic_accessors_preserve_order_and_scalar_kinds() {
        let mut interpreter = MirInterpreter::new();
        let guard = interpreter
            .open_strict_json_session(r#"{"name":"猫","items":[false,9223372036854775808]}"#)
            .expect("session");
        let handle = guard.handle();
        put_i64(guard.interpreter, 1, handle);
        put_i64(guard.interpreter, 2, 0);
        put_i64(guard.interpreter, 3, 0);

        assert_eq!(
            guard
                .interpreter
                .dispatch_strict_json_tree_extern(
                    "hako.analysis.strict_json_tree_v0.kind",
                    &[ValueId::new(1), ValueId::new(2)],
                )
                .expect("kind"),
            VMValue::String("Object".to_string())
        );
        assert_eq!(
            guard
                .interpreter
                .dispatch_strict_json_tree_extern(
                    "hako.analysis.strict_json_tree_v0.object_key_at",
                    &[ValueId::new(1), ValueId::new(2), ValueId::new(3)],
                )
                .expect("key"),
            VMValue::String("name".to_string())
        );
        assert_eq!(
            guard
                .interpreter
                .dispatch_strict_json_tree_extern(
                    "hako.analysis.strict_json_tree_v0.object_value_at",
                    &[ValueId::new(1), ValueId::new(2), ValueId::new(3)],
                )
                .expect("value"),
            VMValue::Integer(1)
        );

        put_i64(guard.interpreter, 2, 2);
        put_i64(guard.interpreter, 3, 1);
        assert_eq!(
            guard
                .interpreter
                .dispatch_strict_json_tree_extern(
                    "hako.analysis.strict_json_tree_v0.array_at",
                    &[ValueId::new(1), ValueId::new(2), ValueId::new(3)],
                )
                .expect("array child"),
            VMValue::Integer(4)
        );

        put_i64(guard.interpreter, 2, 4);
        assert_eq!(
            guard
                .interpreter
                .dispatch_strict_json_tree_extern(
                    "hako.analysis.strict_json_tree_v0.u64_fits_i64",
                    &[ValueId::new(1), ValueId::new(2)],
                )
                .expect("u64 fit"),
            VMValue::Bool(false)
        );
        let error = guard
            .interpreter
            .dispatch_strict_json_tree_extern(
                "hako.analysis.strict_json_tree_v0.u64_as_i64",
                &[ValueId::new(1), ValueId::new(2)],
            )
            .expect_err("out-of-range u64");
        assert!(error.to_string().contains(INTERNAL_TAG));
    }

    #[test]
    fn wrong_kind_and_out_of_range_access_are_bridge_errors() {
        let mut interpreter = MirInterpreter::new();
        let guard = interpreter
            .open_strict_json_session(r#"[true]"#)
            .expect("session");
        put_i64(guard.interpreter, 1, guard.handle());
        put_i64(guard.interpreter, 2, 0);
        let wrong_kind = guard
            .interpreter
            .dispatch_strict_json_tree_extern(
                "hako.analysis.strict_json_tree_v0.object_len",
                &[ValueId::new(1), ValueId::new(2)],
            )
            .expect_err("wrong kind");
        assert!(wrong_kind.to_string().contains(INTERNAL_TAG));
        put_i64(guard.interpreter, 2, 99);
        let out_of_range = guard
            .interpreter
            .dispatch_strict_json_tree_extern(
                "hako.analysis.strict_json_tree_v0.kind",
                &[ValueId::new(1), ValueId::new(2)],
            )
            .expect_err("out of range");
        assert!(out_of_range.to_string().contains(INTERNAL_TAG));
    }
}
