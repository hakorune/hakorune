//! ring1 ArrayService provider.
//!
//! Promotion status is managed in phase-29y docs.
//! This module provides the runtime implementation used by provider_lock.

use crate::box_trait::{IntegerBox, NyashBox};
use crate::boxes::array::ArrayBox;
use crate::runtime::core_services::ArrayService;

const INVALID_ARRAY_BOX: &str = "Not an ArrayBox";

/// Create the canonical runtime ArrayBox through the ring1 array provider seam.
pub fn new_array_box() -> Box<dyn NyashBox> {
    Box::new(ArrayBox::new())
}

#[derive(Debug, Default)]
pub struct Ring1ArrayService;

impl Ring1ArrayService {
    pub fn new() -> Self {
        Self
    }

    fn array_box<'a>(&self, arr: &'a dyn NyashBox) -> Option<&'a ArrayBox> {
        arr.as_any().downcast_ref::<ArrayBox>()
    }

    fn require_array_box<'a>(&self, arr: &'a dyn NyashBox) -> Result<&'a ArrayBox, String> {
        self.array_box(arr)
            .ok_or_else(|| INVALID_ARRAY_BOX.to_string())
    }

    fn box_index(&self, index: i64) -> Box<dyn NyashBox> {
        Box::new(IntegerBox::new(index))
    }
}

impl ArrayService for Ring1ArrayService {
    fn len(&self, arr: &dyn NyashBox) -> i64 {
        self.array_box(arr).map(|a| a.len() as i64).unwrap_or(0)
    }

    fn get(&self, arr: &dyn NyashBox, index: i64) -> Option<Box<dyn NyashBox>> {
        let arr_box = self.array_box(arr)?;
        Some(arr_box.get(self.box_index(index)))
    }

    fn set(&self, arr: &dyn NyashBox, index: i64, value: Box<dyn NyashBox>) -> Result<(), String> {
        let arr_box = self.require_array_box(arr)?;
        arr_box.set(self.box_index(index), value);
        Ok(())
    }

    fn push(&self, arr: &dyn NyashBox, value: Box<dyn NyashBox>) -> Result<(), String> {
        let arr_box = self.require_array_box(arr)?;
        arr_box.push(value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_trait::IntegerBox;

    #[test]
    fn ring1_array_service_len_push_get_set() {
        let provider = Ring1ArrayService::new();
        let arr = ArrayBox::new();

        provider.push(&arr, Box::new(IntegerBox::new(10))).unwrap();
        provider.push(&arr, Box::new(IntegerBox::new(20))).unwrap();
        assert_eq!(provider.len(&arr), 2);

        provider
            .set(&arr, 1, Box::new(IntegerBox::new(30)))
            .unwrap();
        let got = provider.get(&arr, 1).unwrap();
        let got_int = got.as_any().downcast_ref::<IntegerBox>().unwrap();
        assert_eq!(got_int.value, 30);
    }

    #[test]
    fn ring1_array_service_invalid_type_contract() {
        let provider = Ring1ArrayService::new();
        let not_array = IntegerBox::new(7);

        assert_eq!(provider.len(&not_array), 0);
        assert!(provider.get(&not_array, 0).is_none());
        assert_eq!(
            provider
                .set(&not_array, 0, Box::new(IntegerBox::new(1)))
                .unwrap_err(),
            INVALID_ARRAY_BOX
        );
        assert_eq!(
            provider
                .push(&not_array, Box::new(IntegerBox::new(2)))
                .unwrap_err(),
            INVALID_ARRAY_BOX
        );
    }

    #[test]
    fn ring1_array_new_box_returns_arraybox() {
        let boxed = new_array_box();
        assert!(boxed.as_any().downcast_ref::<ArrayBox>().is_some());
    }
}
