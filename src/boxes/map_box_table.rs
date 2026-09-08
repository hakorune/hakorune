//! Sole key/payload table shared by native and checked Map facades.
//! The value parameter prevents a native-visible table from admitting Owned.
use crate::boxes::map_key_domain::MapKeyDomain;
use std::collections::{hash_map, HashMap, TryReserveError};

pub(super) struct MapTable<V>(HashMap<MapKeyDomain, V>);

impl<V> MapTable<V> {
    pub(super) fn new() -> Self {
        Self(HashMap::new())
    }
    pub(super) fn len(&self) -> usize {
        self.0.len()
    }
    pub(super) fn capacity(&self) -> usize {
        self.0.capacity()
    }
    pub(super) fn get(&self, key: &MapKeyDomain) -> Option<&V> {
        self.0.get(key)
    }
    pub(super) fn contains_key(&self, key: &MapKeyDomain) -> bool {
        self.0.contains_key(key)
    }
    pub(super) fn keys(&self) -> hash_map::Keys<'_, MapKeyDomain, V> {
        self.0.keys()
    }
    pub(super) fn iter(&self) -> hash_map::Iter<'_, MapKeyDomain, V> {
        self.0.iter()
    }
    pub(super) fn insert(&mut self, key: MapKeyDomain, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
    pub(super) fn remove(&mut self, key: &MapKeyDomain) -> Option<V> {
        self.0.remove(key)
    }
    pub(super) fn drain(&mut self) -> hash_map::Drain<'_, MapKeyDomain, V> {
        self.0.drain()
    }
    pub(super) fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.0.try_reserve(additional)
    }
}

impl<V> FromIterator<(MapKeyDomain, V)> for MapTable<V> {
    fn from_iter<T: IntoIterator<Item = (MapKeyDomain, V)>>(items: T) -> Self {
        Self(items.into_iter().collect())
    }
}
