use super::super::*;

#[test]
fn share_preserves_state_identity_and_deep_clone_is_fresh() {
    let array = ArrayBox::new();
    let original = array.state_identity();

    let shared = array.share_box();
    let shared = shared.as_any().downcast_ref::<ArrayBox>().unwrap();
    assert_eq!(shared.state_identity(), original);
    assert_ne!(shared.box_id(), array.box_id());

    let cloned = array.clone();
    assert_ne!(cloned.state_identity(), original);
    assert_ne!(cloned.box_id(), array.box_id());
}
