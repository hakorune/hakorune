use super::*;
use crate::boxes::array::runtime_contract::ArrayPrimitiveWriteError as Error;
use crate::typed_array_contract_spec::{ArrayElementContractSpec, ExactArrayElementType};

fn snapshot(array: &ArrayBox) -> (std::mem::Discriminant<ArrayStorage>, Vec<String>) {
    let state = array.items.state.read();
    let kind = std::mem::discriminant(&state.storage);
    drop(state);
    let values = (0..array.len())
        .map(|index| array.get_index_i64(index as i64).to_string_box().value)
        .collect();
    (kind, values)
}

#[test]
fn numeric_results_preserve_exact_reasons_and_shared_state() {
    use ExactArrayElementType::*;
    for (element, low, high) in [
        (I8, -128, 127),
        (I16, -32768, 32767),
        (I32, i32::MIN as i64, i32::MAX as i64),
        (I64, i64::MIN, i64::MAX),
        (U8, 0, 255),
        (U16, 0, 65535),
        (U32, 0, u32::MAX as i64),
    ] {
        let array = ArrayBox::new();
        let spec = ArrayElementContractSpec { element };
        array.claim_element_contract(spec).unwrap();
        assert_eq!(array.slot_store_i64_result(0, low), Ok(()));
        assert_eq!(array.slot_store_i64_result(1, high), Ok(()));
        assert_eq!(array.slot_store_i64_result(0, high), Ok(()));
        let shared = array.share_box();
        let shared = shared.as_any().downcast_ref::<ArrayBox>().unwrap();
        let before = snapshot(&array);
        for index in [0, 2] {
            for invalid in [low.checked_sub(1), high.checked_add(1)]
                .into_iter()
                .flatten()
            {
                let reason = if low == 0 && invalid < 0 {
                    "negative-to-unsigned"
                } else {
                    "out-of-range"
                };
                assert_eq!(
                    shared.slot_store_i64_result(index, invalid),
                    Err(Error::ElementContract { reason })
                );
                assert!(!shared.slot_store_i64_raw(index, invalid));
                assert_eq!(snapshot(&array), before);
            }
            assert_eq!(
                shared.slot_store_bool_result(index, true),
                Err(Error::ElementContract {
                    reason: "runtime-type-mismatch"
                })
            );
            assert_eq!(
                shared.slot_store_f64_result(index, 1.0),
                Err(Error::ElementContract {
                    reason: "runtime-type-mismatch"
                })
            );
            assert!(!shared.slot_store_bool_raw(index, false));
            assert!(!shared.slot_store_f64_raw(index, f64::NAN));
            assert_eq!(snapshot(&array), before);
            assert_eq!(shared.active_element_contract(), Some(spec));
        }
    }
}

#[test]
fn primitive_bounds_and_storage_rejections_preserve_state() {
    let array = ArrayBox::new();
    assert!(array.slot_store_i64_raw(0, 7));
    let before = snapshot(&array);
    for index in [-1, 2, i64::MAX] {
        assert_eq!(
            array.slot_store_i64_result(index, 3),
            Err(Error::InvalidIndex)
        );
        assert_eq!(
            array.slot_store_bool_result(index, true),
            Err(Error::InvalidIndex)
        );
        assert_eq!(
            array.slot_store_f64_result(index, 1.5),
            Err(Error::InvalidIndex)
        );
        assert!(!array.slot_store_i64_raw(index, 3));
        assert!(!array.slot_store_bool_raw(index, true));
        assert!(!array.slot_store_f64_raw(index, 1.5));
        assert_eq!(snapshot(&array), before);
    }
    let record = inline_record_test_array();
    let before = match &record.items.state.read().storage {
        ArrayStorage::InlineRecord(storage) => storage.clone(),
        _ => panic!("expected record storage"),
    };
    for index in [0, record.len() as i64] {
        assert_eq!(
            record.slot_store_i64_result(index, 3),
            Err(Error::UnsupportedStorage)
        );
        assert_eq!(
            record.slot_store_bool_result(index, true),
            Err(Error::UnsupportedStorage)
        );
        assert_eq!(
            record.slot_store_f64_result(index, 1.5),
            Err(Error::UnsupportedStorage)
        );
        assert!(!record.slot_store_i64_raw(index, 3));
        assert!(!record.slot_store_bool_raw(index, true));
        assert!(!record.slot_store_f64_raw(index, 1.5));
        match &record.items.state.read().storage {
            ArrayStorage::InlineRecord(storage) => assert_eq!(storage, &before),
            _ => panic!("rejection changed storage kind"),
        }
    }
}

#[test]
fn primitive_success_preserves_inline_and_mixed_conversion_behavior() {
    let array = ArrayBox::new();
    assert_eq!(array.slot_store_bool_result(0, true), Ok(()));
    assert_eq!(array.slot_store_bool_result(0, false), Ok(()));
    assert!(array.uses_inline_bool_slots());
    assert_eq!(array.get_index_i64(0).as_bool_fast(), Some(false));
    assert_eq!(array.slot_store_i64_result(1, 7), Ok(()));
    assert_eq!(array.get_index_i64(0).as_bool_fast(), Some(false));
    assert_eq!(array.get_index_i64(1).as_i64_fast(), Some(7));
    assert_eq!(array.slot_store_bool_result(1, true), Ok(()));
    assert_eq!(array.slot_store_f64_result(2, 1.25), Ok(()));
    assert_eq!(array.slot_store_f64_result(1, -2.5), Ok(()));
    assert_eq!(array.get_index_i64(0).as_bool_fast(), Some(false));
    assert_eq!(array.get_index_i64(1).as_f64_fast(), Some(-2.5));
    assert_eq!(array.get_index_i64(2).as_f64_fast(), Some(1.25));
    assert_eq!(array.slot_store_i64_result(0, 8), Ok(()));
    assert_eq!(array.slot_store_bool_result(3, true), Ok(()));
    assert_eq!(array.len(), 4);

    let floats = ArrayBox::new();
    for (index, value) in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
        .into_iter()
        .enumerate()
    {
        assert_eq!(floats.slot_store_f64_result(index as i64, value), Ok(()));
        assert_eq!(
            floats
                .get_index_i64(index as i64)
                .as_f64_fast()
                .unwrap()
                .to_bits(),
            value.to_bits()
        );
    }
    assert!(floats.uses_inline_f64_slots());
    assert_eq!(floats.slot_store_f64_result(0, 3.5), Ok(()));
    assert_eq!(floats.get_index_i64(0).as_f64_fast(), Some(3.5));
}
