use crate::{
    hako_atomic_ptr_cas_ordered, hako_atomic_ptr_load_ordered, hako_atomic_slot_fetch_add_i64,
    hako_atomic_slot_load_i64, hako_atomic_slot_store_i64, hako_mem_alloc, hako_mem_free,
    hako_tls_cache_slot_get_i64, hako_tls_cache_slot_set_i64, hako_worker_current_id_i64,
};
use std::ffi::c_void;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

const HAKO_OK: i64 = 0;
const WORKER_COUNT: usize = 4;
const ITERATIONS_PER_WORKER: usize = 64;
const REMOTE_FREE_COUNT_SLOT: i64 = 3;
const TLS_CACHE_SLOT: i64 = 0;
const NODE_WORDS: usize = 2;
const NODE_BYTES: u64 = (NODE_WORDS * std::mem::size_of::<usize>()) as u64;

fn node_next(node: *mut c_void) -> *mut c_void {
    unsafe { node.cast::<usize>().read() as *mut c_void }
}

fn set_node_next(node: *mut c_void, next: *mut c_void) {
    unsafe {
        node.cast::<usize>().write(next as usize);
    }
}

fn node_payload(node: *mut c_void) -> usize {
    unsafe { node.cast::<usize>().add(1).read() }
}

fn set_node_payload(node: *mut c_void, payload: usize) {
    unsafe {
        node.cast::<usize>().add(1).write(payload);
    }
}

fn push_remote_free(head: &AtomicUsize, node: *mut c_void) {
    let head_ptr = (head as *const AtomicUsize).cast_mut().cast::<c_void>();
    loop {
        let observed = hako_atomic_ptr_load_ordered(head_ptr, 1);
        set_node_next(node, observed);
        let previous = hako_atomic_ptr_cas_ordered(head_ptr, observed, node, 3, 1);
        if previous == observed {
            break;
        }
    }
}

fn pop_remote_free(head: &AtomicUsize) -> *mut c_void {
    let head_ptr = (head as *const AtomicUsize).cast_mut().cast::<c_void>();
    loop {
        let observed = hako_atomic_ptr_load_ordered(head_ptr, 1);
        if observed.is_null() {
            return std::ptr::null_mut();
        }
        let next = node_next(observed);
        let previous = hako_atomic_ptr_cas_ordered(head_ptr, observed, next, 3, 1);
        if previous == observed {
            return observed;
        }
    }
}

#[test]
fn mimalloc_parallel_substrate_stress_exercises_native_worker_tls_atomic_and_remote_free() {
    assert_eq!(
        hako_atomic_slot_store_i64(REMOTE_FREE_COUNT_SLOT, 0),
        HAKO_OK
    );

    let remote_head = Arc::new(AtomicUsize::new(0));
    let mut workers = Vec::new();
    for worker_index in 0..WORKER_COUNT {
        let head = Arc::clone(&remote_head);
        workers.push(std::thread::spawn(move || {
            let tls_value = 10_000 + worker_index as i64;
            assert_eq!(hako_worker_current_id_i64(0), 0);
            assert_eq!(
                hako_tls_cache_slot_set_i64(TLS_CACHE_SLOT, tls_value),
                HAKO_OK
            );
            assert_eq!(hako_tls_cache_slot_get_i64(TLS_CACHE_SLOT), tls_value);

            for iteration in 0..ITERATIONS_PER_WORKER {
                let node = hako_mem_alloc(NODE_BYTES);
                assert!(!node.is_null());
                let payload = (worker_index << 16) | iteration;
                set_node_payload(node, payload);
                push_remote_free(&head, node);
                hako_atomic_slot_fetch_add_i64(REMOTE_FREE_COUNT_SLOT, 1);
            }

            assert_eq!(hako_tls_cache_slot_get_i64(TLS_CACHE_SLOT), tls_value);
        }));
    }

    for worker in workers {
        worker
            .join()
            .expect("native substrate stress worker must finish");
    }

    let expected_count = (WORKER_COUNT * ITERATIONS_PER_WORKER) as i64;
    assert_eq!(
        hako_atomic_slot_load_i64(REMOTE_FREE_COUNT_SLOT),
        expected_count
    );

    let mut drained = 0usize;
    let mut payload_sum = 0usize;
    loop {
        let node = pop_remote_free(&remote_head);
        if node.is_null() {
            break;
        }
        payload_sum = payload_sum.wrapping_add(node_payload(node));
        drained += 1;
        hako_mem_free(node);
    }

    assert_eq!(drained, WORKER_COUNT * ITERATIONS_PER_WORKER);
    assert_ne!(payload_sum, 0);
    assert_eq!(remote_head.load(Ordering::SeqCst), 0);
    assert_eq!(
        hako_atomic_slot_store_i64(REMOTE_FREE_COUNT_SLOT, 0),
        HAKO_OK
    );
}
