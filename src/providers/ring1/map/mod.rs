//! ring1 MapService provider.
//!
//! Promotion status is managed in phase-29y docs.
//! This module provides the runtime implementation used by provider_lock.

use crate::box_trait::{BoolBox, IntegerBox, NyashBox, StringBox};
use crate::boxes::map_box::MapBox;
use crate::runtime::core_services::MapService;

#[derive(Debug, Default)]
pub struct Ring1MapService;

impl Ring1MapService {
    pub fn new() -> Self {
        Self
    }
}

impl MapService for Ring1MapService {
    fn size(&self, map: &dyn NyashBox) -> i64 {
        map.as_any()
            .downcast_ref::<MapBox>()
            .map(|m| {
                let size_box = m.size();
                size_box
                    .as_any()
                    .downcast_ref::<IntegerBox>()
                    .map(|i| i.value)
                    .unwrap_or(0)
            })
            .unwrap_or(0)
    }

    fn has(&self, map: &dyn NyashBox, key: &str) -> bool {
        let map_box = match map.as_any().downcast_ref::<MapBox>() {
            Some(m) => m,
            None => return false,
        };
        let key_box = Box::new(StringBox::new(key));
        let result = map_box.has(key_box);
        result
            .as_any()
            .downcast_ref::<BoolBox>()
            .map(|b| b.value)
            .unwrap_or(false)
    }

    fn get(&self, map: &dyn NyashBox, key: &str) -> Option<Box<dyn NyashBox>> {
        let map_box = map.as_any().downcast_ref::<MapBox>()?;
        let key_box = Box::new(StringBox::new(key));
        Some(map_box.get(key_box))
    }

    fn set(&self, map: &dyn NyashBox, key: &str, value: Box<dyn NyashBox>) -> Result<(), String> {
        let map_box = map
            .as_any()
            .downcast_ref::<MapBox>()
            .ok_or("Not a MapBox")?;
        let key_box = Box::new(StringBox::new(key));
        map_box.set(key_box, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring1_map_service_size_set_get_has() {
        let provider = Ring1MapService::new();
        let map = MapBox::new();

        provider
            .set(&map, "k1", Box::new(StringBox::new("v1")))
            .unwrap();
        provider
            .set(&map, "k2", Box::new(IntegerBox::new(42)))
            .unwrap();

        assert_eq!(provider.size(&map), 2);
        assert!(provider.has(&map, "k1"));
        assert!(!provider.has(&map, "missing"));

        let got = provider.get(&map, "k2").unwrap();
        let got_int = got.as_any().downcast_ref::<IntegerBox>().unwrap();
        assert_eq!(got_int.value, 42);
    }
}
