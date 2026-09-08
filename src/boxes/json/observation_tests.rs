use super::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[derive(Debug)]
struct StoredValue {
    base: BoxBase,
    clones: Arc<AtomicUsize>,
}

impl BoxCore for StoredValue {
    fn box_id(&self) -> u64 {
        self.base.id
    }
    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        None
    }
    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "stored")
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for StoredValue {
    fn to_string_box(&self) -> StringBox {
        StringBox::new("stored")
    }
    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        BoolBox::new(self.box_id() == other.box_id())
    }
    fn clone_box(&self) -> Box<dyn NyashBox> {
        self.clones.fetch_add(1, Ordering::SeqCst);
        Box::new(StringBox::new("clone-result"))
    }
    fn share_box(&self) -> Box<dyn NyashBox> {
        self.clone_box()
    }
}

fn stored(clones: &Arc<AtomicUsize>) -> Box<dyn NyashBox> {
    Box::new(StoredValue {
        base: BoxBase::new(),
        clones: clones.clone(),
    })
}

#[test]
fn set_observes_nested_stored_values_without_cloning() {
    let clones = Arc::new(AtomicUsize::new(0));
    let array = ArrayBox::new();
    array.push(stored(&clones));
    array.push(Box::new(IntegerBox::new(30)));
    array.push(Box::new(BoolBox::new(true)));
    array.push(Box::new(crate::boxes::null_box::NullBox::new()));
    let map = MapBox::new();
    map.insert_key_str("array".into(), Box::new(array));
    map.insert_key_str("direct".into(), stored(&clones));
    map.insert_key_str("01".into(), Box::new(StringBox::new("a\0b")));
    assert_eq!(clones.load(Ordering::SeqCst), 0);

    let json = JSONBox::new(serde_json::json!({}));
    let result = json.set(Box::new(StringBox::new("snapshot")), Box::new(map));
    assert_eq!(result.to_string_box().value, "ok");
    assert_eq!(
        *json.value.read().unwrap(),
        serde_json::json!({
            "snapshot": {"array": ["stored", 30, true, null],
                         "direct": "stored", "01": "a\0b"}
        })
    );
    assert_eq!(clones.load(Ordering::SeqCst), 0);
}

#[test]
fn observed_json_owns_its_data_and_leaves_source_usable() {
    let map = MapBox::new();
    map.insert_key_str("value".into(), Box::new(IntegerBox::new(10)));
    let snapshot = nyash_box_to_json_value(&map);
    map.insert_key_str("value".into(), Box::new(IntegerBox::new(20)));
    assert_eq!(snapshot, serde_json::json!({"value": 10}));
    assert_eq!(
        nyash_box_to_json_value(&map),
        serde_json::json!({"value": 20})
    );
}
