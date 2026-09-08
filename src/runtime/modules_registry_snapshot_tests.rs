use super::*;

#[test]
fn module_root_snapshot_distinguishes_empty_from_unavailable() {
    let registry = Mutex::new(HashMap::new());
    assert!(snapshot_box_values(&registry).unwrap().is_empty());
    registry.lock().unwrap().insert(
        "root".into(),
        Box::new(crate::box_trait::IntegerBox::new(9)),
    );
    let roots = snapshot_box_values(&registry).unwrap();
    assert_eq!(roots.len(), 1);
    assert_eq!(
        roots[0]
            .as_any()
            .downcast_ref::<crate::box_trait::IntegerBox>()
            .unwrap()
            .value,
        9
    );
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = registry.lock().unwrap();
        panic!("test root snapshot failure");
    }));
    assert!(matches!(
        snapshot_box_values(&registry),
        Err(ModuleRootsUnavailable)
    ));
}
