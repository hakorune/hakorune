//! ring1 MapService provider.
//!
//! Promotion status is managed in phase-29y docs.
//! This module provides the runtime implementation used by provider_lock.

use crate::box_trait::{BoolBox, IntegerBox, NyashBox, StringBox};
use crate::boxes::map_box::MapBox;
use crate::runtime::core_services::MapService;

const INVALID_MAP_BOX: &str = "Not a MapBox";

/// Create the canonical runtime MapBox through the ring1 map provider seam.
pub fn new_map_box() -> Box<dyn NyashBox> {
    Box::new(MapBox::new())
}

#[derive(Debug, Default)]
pub struct Ring1MapService;

impl Ring1MapService {
    pub fn new() -> Self {
        Self
    }

    fn map_box<'a>(&self, map: &'a dyn NyashBox) -> Option<&'a MapBox> {
        map.as_any().downcast_ref::<MapBox>()
    }

    fn require_map_box<'a>(&self, map: &'a dyn NyashBox) -> Result<&'a MapBox, String> {
        self.map_box(map).ok_or_else(|| INVALID_MAP_BOX.to_string())
    }

    fn box_key(&self, key: &str) -> Box<dyn NyashBox> {
        Box::new(StringBox::new(key))
    }

    fn extract_size(&self, size_box: Box<dyn NyashBox>) -> i64 {
        size_box
            .as_any()
            .downcast_ref::<IntegerBox>()
            .map(|i| i.value)
            .unwrap_or(0)
    }

    fn extract_bool(&self, value_box: Box<dyn NyashBox>) -> bool {
        value_box
            .as_any()
            .downcast_ref::<BoolBox>()
            .map(|b| b.value)
            .unwrap_or(false)
    }
}

impl MapService for Ring1MapService {
    fn size(&self, map: &dyn NyashBox) -> i64 {
        self.map_box(map)
            .map(|m| self.extract_size(m.size()))
            .unwrap_or(0)
    }

    fn has(&self, map: &dyn NyashBox, key: &str) -> bool {
        let map_box = match self.map_box(map) {
            Some(m) => m,
            None => return false,
        };
        self.extract_bool(map_box.has(self.box_key(key)))
    }

    fn get(&self, map: &dyn NyashBox, key: &str) -> Option<Box<dyn NyashBox>> {
        let map_box = self.map_box(map)?;
        Some(map_box.get(self.box_key(key)))
    }

    fn set(&self, map: &dyn NyashBox, key: &str, value: Box<dyn NyashBox>) -> Result<(), String> {
        let map_box = self.require_map_box(map)?;
        map_box.set(self.box_key(key), value);
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

    #[test]
    fn ring1_map_service_invalid_type_contract() {
        let provider = Ring1MapService::new();
        let not_map = IntegerBox::new(7);

        assert_eq!(provider.size(&not_map), 0);
        assert!(!provider.has(&not_map, "k1"));
        assert!(provider.get(&not_map, "k1").is_none());
        assert_eq!(
            provider
                .set(&not_map, "k1", Box::new(IntegerBox::new(1)))
                .unwrap_err(),
            INVALID_MAP_BOX
        );
    }

    #[test]
    fn ring1_map_new_box_returns_mapbox() {
        let boxed = new_map_box();
        assert!(boxed.as_any().downcast_ref::<MapBox>().is_some());
    }
}
