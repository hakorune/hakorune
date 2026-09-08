// Real kernel callers: raw integer and all three generic primitive arms.
use super::super::{array_compat::append_integer_raw, array_slot_append::array_slot_append_any};
use nyash_rust::box_trait::{BoolBox, IntegerBox, NyashBox};
use nyash_rust::boxes::{array::ArrayBox, FloatBox};
use nyash_rust::runtime::host_handles as handles;
use std::sync::{Arc, Barrier};

#[test]
fn primitive_append_callers_return_committed_lengths_and_preserve_types() {
    let _guard = crate::test_support::handle_registry_test_lock();
    let array = Arc::new(ArrayBox::new());
    let handle = handles::to_handle_arc(array.clone()) as i64;
    assert_eq!(append_integer_raw(handle, 7), 1);
    for (index, value) in [
        Arc::new(IntegerBox::new(9)) as Arc<dyn NyashBox>,
        Arc::new(BoolBox::new(true)),
        Arc::new(FloatBox::new(-1.25)),
    ]
    .into_iter()
    .enumerate()
    {
        let value_handle = handles::to_handle_arc(value);
        assert_eq!(
            array_slot_append_any(handle, value_handle as i64),
            index as i64 + 2
        );
        handles::drop_handle(value_handle);
    }
    assert_eq!(array.get_index_i64(0).as_i64_fast(), Some(7));
    assert_eq!(array.get_index_i64(1).as_i64_fast(), Some(9));
    assert_eq!(array.get_index_i64(2).as_bool_fast(), Some(true));
    assert_eq!(array.get_index_i64(3).as_f64_fast(), Some(-1.25));
    assert_eq!(append_integer_raw(0, 7), 0);
    assert_eq!(array_slot_append_any(0, 7), 0);
    let non_array = handles::to_handle_arc(Arc::new(IntegerBox::new(1)));
    assert_eq!(append_integer_raw(non_array as i64, 7), 0);
    assert_eq!(array_slot_append_any(non_array as i64, 7), 0);
    assert_eq!(array.len(), 4);
    handles::drop_handle(non_array);
    handles::drop_handle(handle as u64);
}

#[test]
fn primitive_append_callers_do_not_overwrite_concurrent_commits() {
    let _guard = crate::test_support::handle_registry_test_lock();
    const THREADS: usize = 4;
    const COUNT: usize = 128;
    for kind in 0..4 {
        let array = Arc::new(ArrayBox::new());
        let handle = handles::to_handle_arc(array.clone()) as i64;
        let value: Arc<dyn NyashBox> = match kind {
            0 | 1 => Arc::new(IntegerBox::new(91)),
            2 => Arc::new(BoolBox::new(true)),
            _ => Arc::new(FloatBox::new(1.25)),
        };
        let value_handle = handles::to_handle_arc(value);
        let start = Barrier::new(THREADS);
        let mut positions = std::thread::scope(|scope| {
            let workers: Vec<_> = (0..THREADS)
                .map(|_| {
                    let start = &start;
                    scope.spawn(move || {
                        start.wait();
                        (0..COUNT)
                            .map(|_| {
                                if kind == 0 {
                                    append_integer_raw(handle, 91)
                                } else {
                                    array_slot_append_any(handle, value_handle as i64)
                                }
                            })
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            workers
                .into_iter()
                .flat_map(|worker| worker.join().unwrap())
                .collect::<Vec<_>>()
        });
        positions.sort_unstable();
        assert_eq!(
            positions,
            (1..=(THREADS * COUNT) as i64).collect::<Vec<_>>()
        );
        assert_eq!(array.len(), THREADS * COUNT);
        for index in 0..array.len() {
            let item = array.get_index_i64(index as i64);
            match kind {
                0 | 1 => assert_eq!(item.as_i64_fast(), Some(91)),
                2 => assert_eq!(item.as_bool_fast(), Some(true)),
                _ => assert_eq!(item.as_f64_fast(), Some(1.25)),
            }
        }
        handles::drop_handle(value_handle);
        handles::drop_handle(handle as u64);
    }
}
