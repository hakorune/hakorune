use super::*;

impl MirInterpreter {
    pub(super) fn get_object_field(&self, key: u64, name: &str) -> Option<VMValue> {
        self.obj_fields
            .get(&key)
            .and_then(|fields| fields.get(name))
            .cloned()
    }

    pub(super) fn set_object_field(&mut self, key: u64, name: String, value: VMValue) {
        self.obj_fields.entry(key).or_default().insert(name, value);
    }

    pub(super) fn object_field_root_count(&self) -> usize {
        self.obj_fields
            .values()
            .flat_map(|fields| fields.values())
            .filter(|v| Self::is_strong_root_value(v))
            .count()
    }
}
