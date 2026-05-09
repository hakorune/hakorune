// hako.tls cache-slot exports for pure-first EXE lowering.
//
// This is a narrow allocator-substrate seam. It owns per-thread i64 storage
// only; language-level TLS cells and allocator policy remain above this layer.

use std::cell::RefCell;

const HAKO_OK: i64 = 0;
const HAKO_VALIDATION: i64 = 6;
const TLS_CACHE_SLOT_COUNT: usize = 4;

thread_local! {
    static HAKO_TLS_CACHE_SLOTS: RefCell<[i64; TLS_CACHE_SLOT_COUNT]> =
        const { RefCell::new([0; TLS_CACHE_SLOT_COUNT]) };
}

fn slot_index(slot: i64) -> Option<usize> {
    let index = usize::try_from(slot).ok()?;
    (index < TLS_CACHE_SLOT_COUNT).then_some(index)
}

#[no_mangle]
pub extern "C" fn hako_tls_cache_slot_get_i64(slot: i64) -> i64 {
    let Some(index) = slot_index(slot) else {
        return HAKO_VALIDATION;
    };
    HAKO_TLS_CACHE_SLOTS.with(|slots| slots.borrow()[index])
}

#[no_mangle]
pub extern "C" fn hako_tls_cache_slot_set_i64(slot: i64, value: i64) -> i64 {
    let Some(index) = slot_index(slot) else {
        return HAKO_VALIDATION;
    };
    HAKO_TLS_CACHE_SLOTS.with(|slots| {
        slots.borrow_mut()[index] = value;
    });
    HAKO_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_slot_roundtrip_and_validation_code_are_stable() {
        assert_eq!(hako_tls_cache_slot_set_i64(0, 0), HAKO_OK);
        assert_eq!(hako_tls_cache_slot_get_i64(0), 0);

        assert_eq!(hako_tls_cache_slot_set_i64(0, 4096), HAKO_OK);
        assert_eq!(hako_tls_cache_slot_get_i64(0), 4096);

        assert_eq!(hako_tls_cache_slot_set_i64(0, 0), HAKO_OK);
        assert_eq!(hako_tls_cache_slot_get_i64(0), 0);

        assert_eq!(
            hako_tls_cache_slot_get_i64(TLS_CACHE_SLOT_COUNT as i64),
            HAKO_VALIDATION
        );
        assert_eq!(
            hako_tls_cache_slot_set_i64(TLS_CACHE_SLOT_COUNT as i64, 1),
            HAKO_VALIDATION
        );
    }
}
