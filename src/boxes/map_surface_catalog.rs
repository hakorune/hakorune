use super::map_box::MapBox;
use crate::box_trait::NyashBox;
use std::fmt;

// Catalog owner for the active MapBox surface only.
// Keep compat-only `nyash.map.*` ABI exports in `map_compat.rs` separate from
// this table so phase-291x cleanup can retire source/compat drift one seam at a
// time without reopening the current route owner.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapMethodId {
    Size,
    Len,
    Has,
    Get,
    Set,
    Delete,
    Keys,
    Values,
    Clear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapSurfaceEffect {
    Read,
    WriteHeap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapSurfaceReturn {
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapExposureState {
    pub runtime_impl: bool,
    pub vm_dispatch: bool,
    pub std_sugar: bool,
    pub smoke_pinned: bool,
}

impl MapExposureState {
    pub const CURRENT_VTABLE: Self = Self {
        runtime_impl: true,
        vm_dispatch: true,
        std_sugar: false,
        smoke_pinned: true,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapMethodSpec {
    pub id: MapMethodId,
    pub canonical: &'static str,
    pub aliases: &'static [&'static str],
    pub arity: u8,
    pub slot: u16,
    pub effect: MapSurfaceEffect,
    pub returns: MapSurfaceReturn,
    pub exposure: MapExposureState,
}

impl MapMethodSpec {
    pub fn matches_name(&self, name: &str) -> bool {
        self.canonical == name || self.aliases.iter().any(|alias| *alias == name)
    }

    pub fn matches_signature(&self, name: &str, arity: usize) -> bool {
        self.arity as usize == arity && self.matches_name(name)
    }
}

pub const MAP_SURFACE_METHODS: &[MapMethodSpec] = &[
    MapMethodSpec {
        id: MapMethodId::Size,
        canonical: "size",
        aliases: &["length"],
        arity: 0,
        slot: 200,
        effect: MapSurfaceEffect::Read,
        returns: MapSurfaceReturn::Value,
        exposure: MapExposureState::CURRENT_VTABLE,
    },
    MapMethodSpec {
        id: MapMethodId::Len,
        canonical: "len",
        aliases: &[],
        arity: 0,
        slot: 201,
        effect: MapSurfaceEffect::Read,
        returns: MapSurfaceReturn::Value,
        exposure: MapExposureState::CURRENT_VTABLE,
    },
    MapMethodSpec {
        id: MapMethodId::Has,
        canonical: "has",
        aliases: &[],
        arity: 1,
        slot: 202,
        effect: MapSurfaceEffect::Read,
        returns: MapSurfaceReturn::Value,
        exposure: MapExposureState::CURRENT_VTABLE,
    },
    MapMethodSpec {
        id: MapMethodId::Get,
        canonical: "get",
        aliases: &[],
        arity: 1,
        slot: 203,
        effect: MapSurfaceEffect::Read,
        returns: MapSurfaceReturn::Value,
        exposure: MapExposureState::CURRENT_VTABLE,
    },
    MapMethodSpec {
        id: MapMethodId::Set,
        canonical: "set",
        aliases: &[],
        arity: 2,
        slot: 204,
        effect: MapSurfaceEffect::WriteHeap,
        returns: MapSurfaceReturn::Value,
        exposure: MapExposureState::CURRENT_VTABLE,
    },
    MapMethodSpec {
        id: MapMethodId::Delete,
        canonical: "delete",
        aliases: &["remove"],
        arity: 1,
        slot: 205,
        effect: MapSurfaceEffect::WriteHeap,
        returns: MapSurfaceReturn::Value,
        exposure: MapExposureState::CURRENT_VTABLE,
    },
    MapMethodSpec {
        id: MapMethodId::Keys,
        canonical: "keys",
        aliases: &[],
        arity: 0,
        slot: 206,
        effect: MapSurfaceEffect::Read,
        returns: MapSurfaceReturn::Value,
        exposure: MapExposureState::CURRENT_VTABLE,
    },
    MapMethodSpec {
        id: MapMethodId::Values,
        canonical: "values",
        aliases: &[],
        arity: 0,
        slot: 207,
        effect: MapSurfaceEffect::Read,
        returns: MapSurfaceReturn::Value,
        exposure: MapExposureState::CURRENT_VTABLE,
    },
    MapMethodSpec {
        id: MapMethodId::Clear,
        canonical: "clear",
        aliases: &[],
        arity: 0,
        slot: 208,
        effect: MapSurfaceEffect::WriteHeap,
        returns: MapSurfaceReturn::Value,
        exposure: MapExposureState::CURRENT_VTABLE,
    },
];

impl MapMethodId {
    pub fn spec(self) -> &'static MapMethodSpec {
        MAP_SURFACE_METHODS
            .iter()
            .find(|spec| spec.id == self)
            .expect("MapMethodSpec missing for MapMethodId")
    }

    pub fn canonical_name(self) -> &'static str {
        self.spec().canonical
    }

    pub fn aliases(self) -> &'static [&'static str] {
        self.spec().aliases
    }

    pub fn arity(self) -> usize {
        self.spec().arity as usize
    }

    pub fn slot(self) -> u16 {
        self.spec().slot
    }

    pub fn effect(self) -> MapSurfaceEffect {
        self.spec().effect
    }

    pub fn returns(self) -> MapSurfaceReturn {
        self.spec().returns
    }

    pub fn from_name(name: &str) -> Option<Self> {
        MAP_SURFACE_METHODS
            .iter()
            .find(|spec| spec.matches_name(name))
            .map(|spec| spec.id)
    }

    pub fn from_name_and_arity(name: &str, arity: usize) -> Option<Self> {
        MAP_SURFACE_METHODS
            .iter()
            .find(|spec| spec.matches_signature(name, arity))
            .map(|spec| spec.id)
    }

    pub fn from_slot(slot: u16) -> Option<Self> {
        MAP_SURFACE_METHODS
            .iter()
            .find(|spec| spec.slot == slot)
            .map(|spec| spec.id)
    }

    pub fn from_slot_and_arity(slot: u16, arity: usize) -> Option<Self> {
        MAP_SURFACE_METHODS
            .iter()
            .find(|spec| spec.slot == slot && spec.arity as usize == arity)
            .map(|spec| spec.id)
    }
}

pub enum MapSurfaceInvokeResult {
    Value(Box<dyn NyashBox>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapSurfaceInvokeError {
    pub method: MapMethodId,
    pub expected: usize,
    pub actual: usize,
}

impl fmt::Display for MapSurfaceInvokeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MapBox.{} expects {} args, got {}",
            self.method.canonical_name(),
            self.expected,
            self.actual
        )
    }
}

impl MapBox {
    pub fn invoke_surface(
        &self,
        method: MapMethodId,
        args: Vec<Box<dyn NyashBox>>,
    ) -> Result<MapSurfaceInvokeResult, MapSurfaceInvokeError> {
        let expected = method.arity();
        let actual = args.len();
        if actual != expected {
            return Err(MapSurfaceInvokeError {
                method,
                expected,
                actual,
            });
        }

        let mut args = args.into_iter();
        let result = match method {
            MapMethodId::Size | MapMethodId::Len => MapSurfaceInvokeResult::Value(self.size()),
            MapMethodId::Has => {
                let key = args.next().expect("validated MapBox.has key");
                MapSurfaceInvokeResult::Value(self.has(key))
            }
            MapMethodId::Get => {
                let key = args.next().expect("validated MapBox.get key");
                MapSurfaceInvokeResult::Value(self.get(key))
            }
            MapMethodId::Set => {
                let key = args.next().expect("validated MapBox.set key");
                let value = args.next().expect("validated MapBox.set value");
                MapSurfaceInvokeResult::Value(self.set(key, value))
            }
            MapMethodId::Delete => {
                let key = args.next().expect("validated MapBox.delete key");
                MapSurfaceInvokeResult::Value(self.delete(key))
            }
            MapMethodId::Keys => MapSurfaceInvokeResult::Value(self.keys()),
            MapMethodId::Values => MapSurfaceInvokeResult::Value(self.values()),
            MapMethodId::Clear => MapSurfaceInvokeResult::Value(self.clear()),
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_trait::{BoolBox, IntegerBox, StringBox};
    use crate::boxes::ArrayBox;

    #[test]
    fn map_surface_catalog_preserves_current_slots_and_aliases() {
        assert_eq!(
            MapMethodId::from_name_and_arity("size", 0),
            Some(MapMethodId::Size)
        );
        assert_eq!(
            MapMethodId::from_name_and_arity("length", 0),
            Some(MapMethodId::Size)
        );
        assert_eq!(
            MapMethodId::from_name_and_arity("len", 0),
            Some(MapMethodId::Len)
        );
        assert_eq!(
            MapMethodId::from_name_and_arity("remove", 1),
            Some(MapMethodId::Delete)
        );
        assert_eq!(MapMethodId::from_name_and_arity("length", 1), None);

        assert_eq!(
            MapMethodId::from_slot_and_arity(200, 0),
            Some(MapMethodId::Size)
        );
        assert_eq!(
            MapMethodId::from_slot_and_arity(201, 0),
            Some(MapMethodId::Len)
        );
        assert_eq!(
            MapMethodId::from_slot_and_arity(205, 1),
            Some(MapMethodId::Delete)
        );
        assert_eq!(MapMethodId::from_slot_and_arity(205, 0), None);
    }

    #[test]
    fn invoke_surface_routes_current_map_rows() {
        let map = MapBox::new();

        let set = map
            .invoke_surface(
                MapMethodId::Set,
                vec![Box::new(StringBox::new("a")), Box::new(IntegerBox::new(41))],
            )
            .expect("MapBox.set invoke");
        let MapSurfaceInvokeResult::Value(set_value) = set;
        assert_eq!(set_value.type_name(), "StringBox");

        let has = map
            .invoke_surface(MapMethodId::Has, vec![Box::new(StringBox::new("a"))])
            .expect("MapBox.has invoke");
        let MapSurfaceInvokeResult::Value(has_value) = has;
        let has_bool = has_value
            .as_any()
            .downcast_ref::<BoolBox>()
            .expect("MapBox.has returns BoolBox");
        assert!(has_bool.value);

        let get = map
            .invoke_surface(MapMethodId::Get, vec![Box::new(StringBox::new("a"))])
            .expect("MapBox.get invoke");
        let MapSurfaceInvokeResult::Value(got_value) = get;
        let got_int = got_value
            .as_any()
            .downcast_ref::<IntegerBox>()
            .expect("MapBox.get returns stored IntegerBox");
        assert_eq!(got_int.value, 41);

        let size = map
            .invoke_surface(MapMethodId::Size, vec![])
            .expect("MapBox.size invoke");
        let MapSurfaceInvokeResult::Value(size_value) = size;
        let size_int = size_value
            .as_any()
            .downcast_ref::<IntegerBox>()
            .expect("MapBox.size returns IntegerBox");
        assert_eq!(size_int.value, 1);

        let length = map
            .invoke_surface(
                MapMethodId::from_name_and_arity("length", 0).expect("MapBox.length alias"),
                vec![],
            )
            .expect("MapBox.length invoke");
        let MapSurfaceInvokeResult::Value(length_value) = length;
        let length_int = length_value
            .as_any()
            .downcast_ref::<IntegerBox>()
            .expect("MapBox.length returns IntegerBox");
        assert_eq!(length_int.value, 1);

        let len = map
            .invoke_surface(MapMethodId::Len, vec![])
            .expect("MapBox.len invoke");
        let MapSurfaceInvokeResult::Value(len_value) = len;
        let len_int = len_value
            .as_any()
            .downcast_ref::<IntegerBox>()
            .expect("MapBox.len returns IntegerBox");
        assert_eq!(len_int.value, 1);

        let keys = map
            .invoke_surface(MapMethodId::Keys, vec![])
            .expect("MapBox.keys invoke");
        let MapSurfaceInvokeResult::Value(keys_value) = keys;
        let keys_array = keys_value
            .as_any()
            .downcast_ref::<ArrayBox>()
            .expect("MapBox.keys returns ArrayBox");
        assert_eq!(keys_array.len(), 1);

        let values = map
            .invoke_surface(MapMethodId::Values, vec![])
            .expect("MapBox.values invoke");
        let MapSurfaceInvokeResult::Value(values_value) = values;
        let values_array = values_value
            .as_any()
            .downcast_ref::<ArrayBox>()
            .expect("MapBox.values returns ArrayBox");
        assert_eq!(values_array.len(), 1);

        map.invoke_surface(MapMethodId::Delete, vec![Box::new(StringBox::new("a"))])
            .expect("MapBox.delete invoke");
        assert_eq!(map.len(), 0);

        let clear = map
            .invoke_surface(MapMethodId::Clear, vec![])
            .expect("MapBox.clear invoke");
        let MapSurfaceInvokeResult::Value(clear_value) = clear;
        assert_eq!(clear_value.type_name(), "StringBox");
    }
}
