use super::{ArrayBox, ArrayStorage};
use crate::runtime::exact_numeric_contract::validate_dynamic_integer;
use crate::typed_array_contract_spec::ArrayElementContractSpec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TypedArrayRuntimeContractError {
    StateConflict,
    ExistingElementMismatch { index: usize, reason: &'static str },
}

impl ArrayBox {
    pub(crate) fn claim_element_contract(
        &self,
        requested: ArrayElementContractSpec,
    ) -> Result<(), TypedArrayRuntimeContractError> {
        let mut payload = self.items.state.write();
        match payload.element_contract {
            Some(existing) if existing == requested => return Ok(()),
            Some(_) => return Err(TypedArrayRuntimeContractError::StateConflict),
            None => {}
        }
        audit_storage(&payload.storage, requested)?;
        payload.element_contract = Some(requested);
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn active_element_contract(&self) -> Option<ArrayElementContractSpec> {
        self.items.element_contract()
    }
}

fn audit_storage(
    storage: &ArrayStorage,
    requested: ArrayElementContractSpec,
) -> Result<(), TypedArrayRuntimeContractError> {
    let declared = requested.element.source_name();
    match storage {
        ArrayStorage::InlineI64(values) => audit_i64(values.iter().copied(), declared),
        ArrayStorage::Boxed(values) => {
            for (index, value) in values.iter().enumerate() {
                let Some(integer) = value.as_i64_fast() else {
                    return Err(TypedArrayRuntimeContractError::ExistingElementMismatch {
                        index,
                        reason: "runtime-type-mismatch",
                    });
                };
                validate_dynamic_integer(integer, declared).map_err(|reason| {
                    TypedArrayRuntimeContractError::ExistingElementMismatch { index, reason }
                })?;
            }
            Ok(())
        }
        ArrayStorage::Text(_)
        | ArrayStorage::InlineBool(_)
        | ArrayStorage::InlineF64(_)
        | ArrayStorage::InlineRecord(_) => {
            Err(TypedArrayRuntimeContractError::ExistingElementMismatch {
                index: 0,
                reason: "runtime-type-mismatch",
            })
        }
    }
}

fn audit_i64(
    values: impl Iterator<Item = i64>,
    declared: &str,
) -> Result<(), TypedArrayRuntimeContractError> {
    for (index, value) in values.enumerate() {
        validate_dynamic_integer(value, declared).map_err(|reason| {
            TypedArrayRuntimeContractError::ExistingElementMismatch { index, reason }
        })?;
    }
    Ok(())
}

pub(super) fn validate_boxed_element(
    contract: Option<ArrayElementContractSpec>,
    value: &dyn crate::box_trait::NyashBox,
) -> Result<(), &'static str> {
    let Some(contract) = contract else {
        return Ok(());
    };
    let Some(integer) = value.as_i64_fast() else {
        return Err("runtime-type-mismatch");
    };
    validate_dynamic_integer(integer, contract.element.source_name())
}

pub(super) fn validate_i64_element(
    contract: Option<ArrayElementContractSpec>,
    value: i64,
) -> Result<(), &'static str> {
    let Some(contract) = contract else {
        return Ok(());
    };
    validate_dynamic_integer(value, contract.element.source_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_trait::{IntegerBox, NyashBox};
    use crate::typed_array_contract_spec::{ArrayElementContractSpec, ExactArrayElementType};

    fn spec(element: ExactArrayElementType) -> ArrayElementContractSpec {
        ArrayElementContractSpec { element }
    }

    #[test]
    fn adopts_only_after_a_complete_existing_element_audit() {
        let array = ArrayBox::new_with_elements(vec![
            Box::new(IntegerBox::new(0)),
            Box::new(IntegerBox::new(255)),
        ]);
        assert_eq!(
            array.claim_element_contract(spec(ExactArrayElementType::U8)),
            Ok(())
        );
        assert_eq!(
            array.active_element_contract(),
            Some(spec(ExactArrayElementType::U8))
        );

        let invalid = ArrayBox::new_with_elements(vec![
            Box::new(IntegerBox::new(0)),
            Box::new(IntegerBox::new(256)),
        ]);
        assert_eq!(
            invalid.claim_element_contract(spec(ExactArrayElementType::U8)),
            Err(TypedArrayRuntimeContractError::ExistingElementMismatch {
                index: 1,
                reason: "out-of-range",
            })
        );
        assert_eq!(invalid.active_element_contract(), None);
    }

    #[test]
    fn same_claim_is_idempotent_and_different_claim_conflicts() {
        let array = ArrayBox::new();
        let u8_spec = spec(ExactArrayElementType::U8);
        assert_eq!(array.claim_element_contract(u8_spec), Ok(()));
        assert_eq!(array.claim_element_contract(u8_spec), Ok(()));
        assert_eq!(
            array.claim_element_contract(spec(ExactArrayElementType::U16)),
            Err(TypedArrayRuntimeContractError::StateConflict)
        );
    }

    #[test]
    fn share_preserves_identity_while_clone_and_slice_copy_contract_to_fresh_state() {
        let array = ArrayBox::new_with_elements(vec![Box::new(IntegerBox::new(7))]);
        let u8_spec = spec(ExactArrayElementType::U8);
        array.claim_element_contract(u8_spec).unwrap();

        let shared = array.share_box();
        let shared = shared.as_any().downcast_ref::<ArrayBox>().unwrap();
        assert_eq!(array.state_identity(), shared.state_identity());
        assert_eq!(shared.active_element_contract(), Some(u8_spec));

        let cloned = array.clone();
        assert_ne!(array.state_identity(), cloned.state_identity());
        assert_eq!(cloned.active_element_contract(), Some(u8_spec));

        let sliced = array.slice(Box::new(IntegerBox::new(0)), Box::new(IntegerBox::new(1)));
        let sliced = sliced.as_any().downcast_ref::<ArrayBox>().unwrap();
        assert_ne!(array.state_identity(), sliced.state_identity());
        assert_eq!(sliced.active_element_contract(), Some(u8_spec));
    }

    #[test]
    fn contracted_raw_mutations_check_before_commit() {
        let array = ArrayBox::new();
        array
            .claim_element_contract(spec(ExactArrayElementType::U8))
            .unwrap();

        assert_eq!(array.slot_append_box_raw(Box::new(IntegerBox::new(255))), 1);
        assert_eq!(
            array.slot_append_box_raw(Box::new(IntegerBox::new(256))),
            -1
        );
        assert_eq!(array.len(), 1);

        assert!(!array.slot_store_i64_raw(0, 256));
        assert!(!array.slot_insert_box_raw(0, Box::new(IntegerBox::new(-1))));
        assert_eq!(array.slot_rmw_add1_i64_raw(0), None);
        assert_eq!(array.slot_load_i64_raw(0), Some(255));
    }
}
