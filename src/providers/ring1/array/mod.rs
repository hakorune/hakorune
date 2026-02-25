//! ring1 ArrayService provider.
//!
//! Promotion status is managed in phase-29y docs.
//! This module provides the runtime implementation used by provider_lock.

use crate::box_trait::{IntegerBox, NyashBox};
use crate::boxes::array::ArrayBox;
use crate::runtime::core_services::ArrayService;

#[derive(Debug, Default)]
pub struct Ring1ArrayService;

impl Ring1ArrayService {
    pub fn new() -> Self {
        Self
    }
}

impl ArrayService for Ring1ArrayService {
    fn len(&self, arr: &dyn NyashBox) -> i64 {
        arr.as_any()
            .downcast_ref::<ArrayBox>()
            .map(|a| a.len() as i64)
            .unwrap_or(0)
    }

    fn get(&self, arr: &dyn NyashBox, index: i64) -> Option<Box<dyn NyashBox>> {
        let arr_box = arr.as_any().downcast_ref::<ArrayBox>()?;
        let index_box = Box::new(IntegerBox::new(index));
        Some(arr_box.get(index_box))
    }

    fn set(&self, arr: &dyn NyashBox, index: i64, value: Box<dyn NyashBox>) -> Result<(), String> {
        let arr_box = arr
            .as_any()
            .downcast_ref::<ArrayBox>()
            .ok_or("Not an ArrayBox")?;
        let index_box = Box::new(IntegerBox::new(index));
        arr_box.set(index_box, value);
        Ok(())
    }

    fn push(&self, arr: &dyn NyashBox, value: Box<dyn NyashBox>) -> Result<(), String> {
        let arr_box = arr
            .as_any()
            .downcast_ref::<ArrayBox>()
            .ok_or("Not an ArrayBox")?;
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
}
