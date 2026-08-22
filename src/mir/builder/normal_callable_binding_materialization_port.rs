//! Callable-entry physical value handoff.
//!
//! Parameter setup remains the sole allocator/publisher of MIR formal values.
//! This port only snapshots those existing values in positional source order
//! and offers them to an optional callable semantic materialization ledger.

use crate::mir::{MirBuilder, ValueId};

use super::recursive_child_lowering::RawLegacyChildLoweringPortV1;

/// Exact callable entry shape selected by the existing draft owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum CallableEntryShapeV1 {
    Static { parameter_count: usize },
    Instance { parameter_count: usize },
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
