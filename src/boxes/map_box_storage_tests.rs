//! Owner-private storage failure setup; no raw access exists in production.
use super::MapBox;

pub(crate) fn poison_storage(map: &MapBox) {
    let poisoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _entries = map.data.write().unwrap();
        panic!("test Map storage failure");
    }));
    assert!(poisoned.is_err());
}
