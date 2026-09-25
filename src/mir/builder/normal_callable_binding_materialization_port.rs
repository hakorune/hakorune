//! Callable-entry physical value handoff.
//!
//! Parameter setup remains the sole allocator/publisher of MIR formal values.
//! This port only snapshots those existing values in positional source order
//! and offers them to an optional callable semantic materialization ledger.

use crate::mir::{MirBuilder, ValueId};

use super::recursive_child_lowering::RawLegacyChildLoweringPortV1;

/// Exact callable entry shape selected by the existing draft owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableEntryShapeV1 {
    Static { parameter_count: usize },
    Instance { parameter_count: usize },
    /// App-main wrapper route: declared source parameters are realized
    /// as the injector's published `variable_map` locals (the retained
    /// argv materialization contract), not function formals.  The
    /// declared names snapshot those locals in source order.
    StaticInjectedLocals { parameter_names: Box<[String]> },
}

impl CallableEntryShapeV1 {
    pub(in crate::mir::builder) fn prepare_values(
        self,
        builder: &MirBuilder,
    ) -> Result<PreparedCallableEntryValuesV1, String> {
        match self {
            Self::Static { parameter_count } => {
                PreparedCallableEntryValuesV1::static_function(builder, parameter_count)
            }
            Self::Instance { parameter_count } => {
                PreparedCallableEntryValuesV1::instance_method(builder, parameter_count)
            }
            Self::StaticInjectedLocals { parameter_names } => {
                PreparedCallableEntryValuesV1::injected_locals(builder, &parameter_names)
            }
        }
    }
}

/// Existing physical values for one callable entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct PreparedCallableEntryValuesV1 {
    receiver: Option<ValueId>,
    parameters: Box<[ValueId]>,
}

impl PreparedCallableEntryValuesV1 {
    pub(in crate::mir::builder) fn static_from_values(parameters: [ValueId; 4]) -> Self {
        Self {
            receiver: None,
            parameters: parameters.into(),
        }
    }

    pub(in crate::mir::builder) fn static_function(
        builder: &MirBuilder,
        parameter_count: usize,
    ) -> Result<Self, String> {
        let values = current_formal_values_v1(builder)?;
        if values.len() != parameter_count {
            return Err(format!(
                "[freeze:contract][callable-entry/static-arity] expected={} actual={}",
                parameter_count,
                values.len()
            ));
        }
        Ok(Self {
            receiver: None,
            parameters: values.into_boxed_slice(),
        })
    }

    pub(in crate::mir::builder) fn instance_method(
        builder: &MirBuilder,
        parameter_count: usize,
    ) -> Result<Self, String> {
        let values = current_formal_values_v1(builder)?;
        let expected = parameter_count.checked_add(1).ok_or_else(|| {
            "[freeze:contract][callable-entry/instance-arity-overflow]".to_owned()
        })?;
        if values.len() != expected {
            return Err(format!(
                "[freeze:contract][callable-entry/instance-arity] expected={} actual={}",
                expected,
                values.len()
            ));
        }
        Ok(Self {
            receiver: values.first().copied(),
            parameters: values[1..].into(),
        })
    }

    /// Snapshot the injector-published local values realizing the
    /// declared source parameters of the app-main wrapper route, in
    /// declared name order.  `variable_map` is keyed only by the
    /// declared names — map iteration order is never consulted, and
    /// `function.params` is never widened.
    pub(in crate::mir::builder) fn injected_locals(
        builder: &MirBuilder,
        parameter_names: &[String],
    ) -> Result<Self, String> {
        let variable_map = &builder.function_state.variable_ctx.variable_map;
        let mut parameters = Vec::with_capacity(parameter_names.len());
        for name in parameter_names {
            let value = variable_map.get(name).copied().ok_or_else(|| {
                format!(
                    "[freeze:contract][callable-entry/injected-local-missing] name={name}"
                )
            })?;
            parameters.push(value);
        }
        Ok(Self {
            receiver: None,
            parameters: parameters.into_boxed_slice(),
        })
    }

    pub(in crate::mir::builder) const fn receiver(&self) -> Option<ValueId> {
        self.receiver
    }

    pub(in crate::mir::builder) fn parameters(&self) -> &[ValueId] {
        &self.parameters
    }
}

fn current_formal_values_v1(builder: &MirBuilder) -> Result<Vec<ValueId>, String> {
    builder
        .function_state
        .current_function
        .as_ref()
        .map(|function| function.params.clone())
        .ok_or_else(|| "[freeze:contract][callable-entry/no-current-function]".to_owned())
}

/// Optional consumer of already-allocated callable entry values.
pub(in crate::mir::builder) trait CallableBindingMaterializationPortV1 {
    fn adopt_callable_entry_values_v1(
        &mut self,
        builder: &MirBuilder,
        shape: CallableEntryShapeV1,
    ) -> Result<(), String>;
}

/// Raw/reference lowering has no callable semantic ledger.  Parameter setup
/// remains unchanged and the positional snapshot is deliberately discarded.
impl CallableBindingMaterializationPortV1 for RawLegacyChildLoweringPortV1 {
    fn adopt_callable_entry_values_v1(
        &mut self,
        _builder: &MirBuilder,
        _shape: CallableEntryShapeV1,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injected_locals_snapshot_follows_declared_name_order() {
        let mut builder = MirBuilder::new();
        // Deliberately insert in an order that differs from the declared
        // order: BTreeMap iteration would yield `a` first, but the
        // declared names drive the snapshot.
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("a".to_string(), ValueId(41));
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("b".to_string(), ValueId(42));
        let values = CallableEntryShapeV1::StaticInjectedLocals {
            parameter_names: vec!["b".to_string(), "a".to_string()].into_boxed_slice(),
        }
        .prepare_values(&builder)
        .expect("published locals must snapshot");
        assert_eq!(values.receiver(), None);
        assert_eq!(values.parameters(), &[ValueId(42), ValueId(41)]);
    }

    #[test]
    fn injected_locals_missing_name_fails_named_boundary() {
        let mut builder = MirBuilder::new();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("args".to_string(), ValueId(41));
        let error = CallableEntryShapeV1::StaticInjectedLocals {
            parameter_names: vec!["args".to_string(), "unseen".to_string()]
                .into_boxed_slice(),
        }
        .prepare_values(&builder)
        .expect_err("a missing injected local must fail");
        assert!(
            error.contains("callable-entry/injected-local-missing"),
            "expected injected-local-missing, got: {error}"
        );
        assert!(
            error.contains("name=unseen"),
            "the failing name must be pinned in the error: {error}"
        );
    }
}
