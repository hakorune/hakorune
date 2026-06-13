//! Runtime reference snapshots for structured ambient `context`.
//!
//! This module owns the snapshot value shape only. The active stack and child
//! future association are owned by `runtime::global_hooks` because that module
//! already owns structured task-scope registration.

use crate::box_trait::NyashBox;

#[derive(Debug)]
pub struct ContextBindingSnapshot {
    pub name: String,
    pub value: Box<dyn NyashBox>,
}

impl Clone for ContextBindingSnapshot {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            value: self.value.clone_or_share(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ContextSnapshot {
    bindings: Vec<ContextBindingSnapshot>,
}

impl ContextSnapshot {
    pub fn new(bindings: Vec<ContextBindingSnapshot>) -> Self {
        Self { bindings }
    }

    pub fn bindings(&self) -> &[ContextBindingSnapshot] {
        &self.bindings
    }

    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    pub fn get(&self, name: &str) -> Option<&dyn NyashBox> {
        self.bindings
            .iter()
            .rev()
            .find(|binding| binding.name == name)
            .map(|binding| binding.value.as_ref())
    }
}

pub fn context_snapshot_report_fields() -> Vec<(&'static str, &'static str)> {
    vec![
        ("context_snapshot_runtime_enabled", "1"),
        ("context_snapshot_explicit_scope_only", "1"),
        ("context_snapshot_implicit_root_propagation", "0"),
        ("context_snapshot_program_json_enabled", "0"),
        ("context_snapshot_mir_lowering_enabled", "0"),
        ("context_snapshot_llvm_enabled", "0"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_trait::StringBox;

    #[test]
    fn context_snapshot_reads_latest_binding_by_name() {
        let snapshot = ContextSnapshot::new(vec![
            ContextBindingSnapshot {
                name: "request_id".to_string(),
                value: Box::new(StringBox::new("outer")),
            },
            ContextBindingSnapshot {
                name: "request_id".to_string(),
                value: Box::new(StringBox::new("inner")),
            },
        ]);

        assert_eq!(
            snapshot
                .get("request_id")
                .expect("binding should exist")
                .to_string_box()
                .value,
            "inner"
        );
    }

    #[test]
    fn context_snapshot_report_fields_keep_lowering_closed() {
        assert_eq!(
            context_snapshot_report_fields(),
            vec![
                ("context_snapshot_runtime_enabled", "1"),
                ("context_snapshot_explicit_scope_only", "1"),
                ("context_snapshot_implicit_root_propagation", "0"),
                ("context_snapshot_program_json_enabled", "0"),
                ("context_snapshot_mir_lowering_enabled", "0"),
                ("context_snapshot_llvm_enabled", "0"),
            ]
        );
    }
}
