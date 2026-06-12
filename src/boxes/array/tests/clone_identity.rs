use super::*;

#[test]
fn array_visible_read_shares_self_identity_without_clone_recursion() {
    let array = ArrayBox::new();
    array.push(array.share_box());

    let got = array.get_index_i64(0);

    assert_eq!(got.type_name(), "ArrayBox");
}

#[test]
fn array_clone_shares_self_identity_without_clone_recursion() {
    let array = ArrayBox::new();
    array.push(array.share_box());

    let cloned = array.clone_box();

    assert_eq!(cloned.type_name(), "ArrayBox");
}

#[test]
fn map_clone_shares_nested_collection_identity_without_clone_recursion() {
    let array = ArrayBox::new();
    let map = crate::boxes::MapBox::new();
    array.push(map.share_box());
    map.set(Box::new(StringBox::new("array")), array.share_box());

    let cloned = map.clone_box();

    assert_eq!(cloned.type_name(), "MapBox");
}
