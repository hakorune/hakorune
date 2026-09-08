use super::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex, Weak,
};

#[derive(Debug)]
struct StoredValue {
    base: BoxBase,
    clones: Arc<AtomicUsize>,
    drop_probe: Option<(Weak<JSONBox>, Arc<Mutex<Vec<(bool, bool)>>>)>,
}

impl Drop for StoredValue {
    fn drop(&mut self) {
        if let Some((target, observations)) = &self.drop_probe {
            let target = target.upgrade().expect("destination remains live");
            let observation = match target.value.try_write() {
                Ok(value) => (true, value.get("value").is_none()),
                Err(_) => (false, false),
            };
            observations.lock().unwrap().push(observation);
        }
    }
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
        drop_probe: None,
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
    let result = json
        .set(Box::new(StringBox::new("snapshot")), Box::new(map))
        .unwrap();
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
    let snapshot = nyash_box_to_json_value(&map).unwrap();
    map.insert_key_str("value".into(), Box::new(IntegerBox::new(20)));
    assert_eq!(snapshot, serde_json::json!({"value": 10}));
    assert_eq!(
        nyash_box_to_json_value(&map).unwrap(),
        serde_json::json!({"value": 20})
    );
}

#[test]
fn nested_map_failure_preserves_destination_and_does_not_clone_children() {
    let clones = Arc::new(AtomicUsize::new(0));
    let bad = MapBox::new();
    bad.insert_key_str("unread".into(), stored(&clones));
    crate::boxes::map_box::storage_tests::poison_storage(&bad);
    let array = ArrayBox::new();
    array.push(Box::new(IntegerBox::new(1)));
    array.push(Box::new(bad));
    let outer = MapBox::new();
    outer.insert_key_str("nested".into(), Box::new(array));
    let json = JSONBox::new(serde_json::json!({ "keep": 19 }));
    assert!(matches!(
        json.set(Box::new(StringBox::new("new")), Box::new(outer)),
        Err(JsonSetError::SourceMapUnavailable)
    ));
    assert_eq!(
        *json.value.read().unwrap(),
        serde_json::json!({ "keep": 19 })
    );
    assert_eq!(clones.load(Ordering::SeqCst), 0);
}

#[test]
fn set_rejects_invalid_destination_and_keeps_source_failure_priority() {
    let not_object = JSONBox::new(serde_json::json!([1]));
    assert!(matches!(
        not_object.set(Box::new(StringBox::new("x")), Box::new(IntegerBox::new(2))),
        Err(JsonSetError::DestinationNotObject)
    ));
    assert_eq!(*not_object.value.read().unwrap(), serde_json::json!([1]));

    let json = JSONBox::new(serde_json::json!({ "keep": 9 }));
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = json.value.write().unwrap();
        panic!("test destination storage failure");
    }));
    assert!(matches!(
        json.set(Box::new(StringBox::new("x")), Box::new(IntegerBox::new(2))),
        Err(JsonSetError::DestinationUnavailable)
    ));
    let bad = MapBox::new();
    crate::boxes::map_box::storage_tests::poison_storage(&bad);
    assert!(matches!(
        json.set(Box::new(StringBox::new("x")), Box::new(bad)),
        Err(JsonSetError::SourceMapUnavailable)
    ));
    assert_eq!(
        *json.value.read().unwrap_err().into_inner(),
        serde_json::json!({ "keep": 9 })
    );
}

#[test]
fn set_disposes_input_once_outside_destination_lock_before_commit_or_refusal() {
    for initial in [serde_json::json!({}), serde_json::json!([])] {
        let accepted = initial.is_object();
        let json = Arc::new(JSONBox::new(initial));
        let observations = Arc::new(Mutex::new(Vec::new()));
        let clones = Arc::new(AtomicUsize::new(0));
        let input = Box::new(StoredValue {
            base: BoxBase::new(),
            clones: clones.clone(),
            drop_probe: Some((Arc::downgrade(&json), observations.clone())),
        });
        let result = json.set(Box::new(StringBox::new("value")), input);
        assert_eq!(result.is_ok(), accepted);
        assert_eq!(*observations.lock().unwrap(), vec![(true, true)]);
        assert_eq!(clones.load(Ordering::SeqCst), 0);
        if accepted {
            assert_eq!(
                *json.value.read().unwrap(),
                serde_json::json!({ "value": "stored" })
            );
        }
    }
}
