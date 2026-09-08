//! Regression: a displaced value may re-enter its Map during native Drop.
use super::*;
use std::sync::{Mutex, Weak};

type Entries = RwLock<HashMap<MapKeyDomain, Box<dyn NyashBox>>>;
type Observations = Arc<Mutex<Vec<(bool, Option<i64>)>>>;

#[derive(Debug)]
struct ReenterOnDrop {
    base: BoxBase,
    entries: Weak<Entries>,
    observations: Observations,
}

impl Drop for ReenterOnDrop {
    fn drop(&mut self) {
        let observation = self
            .entries
            .upgrade()
            .map(|entries| match entries.try_write() {
                Ok(guard) => {
                    let installed = guard
                        .get(&MapKeyDomain::from_text("target"))
                        .and_then(|value| value.as_any().downcast_ref::<IntegerBox>())
                        .map(|value| value.value);
                    (true, installed)
                }
                Err(_) => (false, None),
            })
            .unwrap_or((false, None));
        self.observations.lock().unwrap().push(observation);
    }
}

impl BoxCore for ReenterOnDrop {
    fn box_id(&self) -> u64 {
        self.base.id
    }
    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        None
    }
    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ReenterOnDrop")
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NyashBox for ReenterOnDrop {
    fn to_string_box(&self) -> StringBox {
        StringBox::new("ReenterOnDrop")
    }
    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        BoolBox::new(self.box_id() == other.box_id())
    }
    fn clone_box(&self) -> Box<dyn NyashBox> {
        panic!("probe must not be cloned")
    }
    fn share_box(&self) -> Box<dyn NyashBox> {
        panic!("probe must not be shared")
    }
}

#[test]
fn replacement_drop_can_reenter_and_observes_committed_value_once() {
    let map = MapBox::new();
    let observations = Arc::new(Mutex::new(Vec::new()));
    map.insert_key_str(
        "target".into(),
        Box::new(ReenterOnDrop {
            base: BoxBase::new(),
            entries: Arc::downgrade(&map.data),
            observations: observations.clone(),
        }),
    );
    assert!(observations.lock().unwrap().is_empty());
    map.insert_key_str("other".into(), Box::new(IntegerBox::new(7)));
    assert!(
        observations.lock().unwrap().is_empty(),
        "missing-key insert displaced nothing"
    );

    map.insert_key_str("target".into(), Box::new(IntegerBox::new(42)));
    assert_eq!(*observations.lock().unwrap(), vec![(true, Some(42))]);
    assert_eq!(map.get_scalar_i64_key_str("other"), Some(7));
    drop(map);
    assert_eq!(
        observations.lock().unwrap().len(),
        1,
        "displaced value ended once"
    );
}
