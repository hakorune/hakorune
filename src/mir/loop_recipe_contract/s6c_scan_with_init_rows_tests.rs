use super::*;

#[test]
fn const_text_has_result_but_is_outside_s6c_operation_rows() {
    let result = LoopValueKeyV1::new(0);
    let operation = LoopOperationV2::ConstText {
        result,
        value: "text".into(),
    };
    assert_eq!(operation_result(&operation), Some(result));
    assert!(operation_row(&operation).is_none());
}
