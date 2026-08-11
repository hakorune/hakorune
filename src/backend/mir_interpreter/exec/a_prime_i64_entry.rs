//! A-prime selected-entry representation boundary.
//!
//! This helper is deliberately backend-local.  The generic exact-numeric
//! parameter contract continues to accept its existing `Integer` and
//! matching `ExactNumeric` values.  The selected A-prime physical session is
//! narrower: it can use only the direct `VMValue::Integer` carrier as an
//! `ImmediateI64` witness.  No source contract, ValueId, or MirType is
//! reclassified here.

use super::super::VMValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct APrimeImmediateI64ValueV1(i64);

impl APrimeImmediateI64ValueV1 {
    pub(super) const fn value(self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum APrimeVmI64EntryRejectV1 {
    ExactNumericWrapper,
    ObjectCarrier,
    NonImmediate,
}

/// Classify one selected A-prime argument without inspecting object identity.
///
/// This is intentionally not connected to the live interpreter entry yet.
/// The named physical-session ingress will apply it after the generic entry
/// contract and before register seeding/body effects.  Until then, production
/// caller count remains zero and these are unit-level capability facts only.
pub(super) fn classify_selected_a_prime_i64_argument(
    value: &VMValue,
) -> Result<APrimeImmediateI64ValueV1, APrimeVmI64EntryRejectV1> {
    match value {
        VMValue::Integer(value) => Ok(APrimeImmediateI64ValueV1(*value)),
        VMValue::ExactNumeric(_) => Err(APrimeVmI64EntryRejectV1::ExactNumericWrapper),
        VMValue::BoxRef(_) => Err(APrimeVmI64EntryRejectV1::ObjectCarrier),
        VMValue::Float(_)
        | VMValue::Bool(_)
        | VMValue::String(_)
        | VMValue::Future(_)
        | VMValue::Void
        | VMValue::WeakBox(_) => Err(APrimeVmI64EntryRejectV1::NonImmediate),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::vm_types::ExactNumericRuntimeValue;
    use crate::box_trait::{IntegerBox, NyashBox, StringBox};
    use crate::boxes::future::FutureBox;
    use std::sync::Arc;

    #[test]
    fn accepts_only_direct_integer_carriers() {
        for value in [0, i64::MIN, i64::MAX] {
            let witness = classify_selected_a_prime_i64_argument(&VMValue::Integer(value))
                .expect("direct integer is the A-prime carrier");
            assert_eq!(witness.value(), value);
        }
    }

    #[test]
    fn rejects_exact_numeric_wrappers_without_generic_contract_change() {
        for source_name in ["i64", "u8"] {
            assert_eq!(
                classify_selected_a_prime_i64_argument(&VMValue::ExactNumeric(
                    ExactNumericRuntimeValue::new(source_name, 0),
                )),
                Err(APrimeVmI64EntryRejectV1::ExactNumericWrapper)
            );
        }
    }

    #[test]
    fn rejects_all_object_carriers_without_downcast_or_type_name_lookup() {
        for object in [
            Arc::new(IntegerBox::new(0)) as Arc<dyn NyashBox>,
            Arc::new(StringBox::new("0")) as Arc<dyn NyashBox>,
        ] {
            assert_eq!(
                classify_selected_a_prime_i64_argument(&VMValue::BoxRef(object)),
                Err(APrimeVmI64EntryRejectV1::ObjectCarrier)
            );
        }
    }

    #[test]
    fn rejects_non_immediate_runtime_variants() {
        let strong: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(1));
        let weak = Arc::downgrade(&strong);
        for value in [
            VMValue::Float(0.0),
            VMValue::Bool(false),
            VMValue::String("0".to_string()),
            VMValue::Future(FutureBox::new()),
            VMValue::Void,
            VMValue::WeakBox(weak),
        ] {
            assert_eq!(
                classify_selected_a_prime_i64_argument(&value),
                Err(APrimeVmI64EntryRejectV1::NonImmediate)
            );
        }
    }
}
