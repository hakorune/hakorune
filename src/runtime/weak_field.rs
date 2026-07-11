//! Declaration-indexed storage vocabulary for Weak fields.

use crate::box_trait::NyashBox;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredFieldKind {
    Ordinary,
    Weak,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredFieldLayoutEntry {
    pub index: u32,
    pub diagnostic_name: String,
    pub kind: DeclaredFieldKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredFieldLayout {
    pub schema_fingerprint: String,
    pub fields: Vec<DeclaredFieldLayoutEntry>,
    name_to_index: HashMap<String, u32>,
}

impl DeclaredFieldLayout {
    pub fn new(
        schema_fingerprint: String,
        fields: impl IntoIterator<Item = (String, bool)>,
    ) -> Self {
        let fields = fields
            .into_iter()
            .enumerate()
            .map(
                |(index, (diagnostic_name, is_weak))| DeclaredFieldLayoutEntry {
                    index: index as u32,
                    diagnostic_name,
                    kind: if is_weak {
                        DeclaredFieldKind::Weak
                    } else {
                        DeclaredFieldKind::Ordinary
                    },
                },
            )
            .collect::<Vec<_>>();
        let name_to_index = fields
            .iter()
            .map(|field| (field.diagnostic_name.clone(), field.index))
            .collect();
        Self {
            schema_fingerprint,
            fields,
            name_to_index,
        }
    }

    pub fn field(&self, index: u32) -> Option<&DeclaredFieldLayoutEntry> {
        self.fields.get(index as usize)
    }

    pub fn field_index(&self, name: &str) -> Option<u32> {
        self.name_to_index.get(name).copied()
    }
}

#[derive(Clone)]
pub enum WeakSlotState {
    Empty,
    Occupied(Weak<dyn NyashBox>),
}

impl std::fmt::Debug for WeakSlotState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => formatter.write_str("Empty"),
            Self::Occupied(_) => formatter.write_str("Occupied(WeakRef)"),
        }
    }
}

#[derive(Debug)]
pub struct DeclaredWeakFieldStore {
    layout: Arc<DeclaredFieldLayout>,
    slots: Mutex<Vec<WeakSlotState>>,
}

pub const CONTRACT_VIOLATION_TAG: &str = "[type/weak_field_contract_violation]";
pub const BASE_NOT_INSTANCE_TAG: &str = "[type/weak_field_contract_base_not_instance]";
pub const RUNTIME_LAYOUT_MISSING_TAG: &str = "[type/weak_field_contract_runtime_layout_missing]";

pub struct WeakFieldRuntime;

impl WeakFieldRuntime {
    pub fn write_contract(
        instance: &crate::instance_v2::InstanceBox,
        expected_schema_fingerprint: &str,
        field_index: u32,
        value: WeakSlotState,
    ) -> Result<(), String> {
        let store = instance.declared_weak_fields();
        if store.layout().schema_fingerprint != expected_schema_fingerprint {
            return Err(format!(
                "{} expected={} actual={}",
                RUNTIME_LAYOUT_MISSING_TAG,
                expected_schema_fingerprint,
                store.layout().schema_fingerprint
            ));
        }
        store.write(field_index, value)
    }

    pub fn write_dynamic(
        instance: &crate::instance_v2::InstanceBox,
        field_name: &str,
        value: WeakSlotState,
    ) -> Result<bool, String> {
        let store = instance.declared_weak_fields();
        let Some(field_index) = store.layout().field_index(field_name) else {
            return Ok(false);
        };
        let Some(field) = store.layout().field(field_index) else {
            return Err(format!("{} field={field_name}", RUNTIME_LAYOUT_MISSING_TAG));
        };
        if field.kind != DeclaredFieldKind::Weak {
            return Ok(false);
        }
        store.write(field_index, value)?;
        Ok(true)
    }

    pub fn read_dynamic(
        instance: &crate::instance_v2::InstanceBox,
        field_name: &str,
    ) -> Result<Option<WeakSlotState>, String> {
        let store = instance.declared_weak_fields();
        let Some(field_index) = store.layout().field_index(field_name) else {
            return Ok(None);
        };
        let Some(field) = store.layout().field(field_index) else {
            return Err(format!("{} field={field_name}", RUNTIME_LAYOUT_MISSING_TAG));
        };
        if field.kind != DeclaredFieldKind::Weak {
            return Ok(None);
        }
        store.read(field_index).map(Some)
    }
}

impl DeclaredWeakFieldStore {
    pub fn new(layout: Arc<DeclaredFieldLayout>) -> Self {
        Self {
            slots: Mutex::new(vec![WeakSlotState::Empty; layout.fields.len()]),
            layout,
        }
    }

    pub fn layout(&self) -> &Arc<DeclaredFieldLayout> {
        &self.layout
    }

    pub fn read(&self, index: u32) -> Result<WeakSlotState, String> {
        self.require_weak(index)?;
        self.slots
            .lock()
            .map_err(|_| "weak field slot lock poisoned".to_string())?
            .get(index as usize)
            .cloned()
            .ok_or_else(|| format!("weak field index out of range: {index}"))
    }

    pub fn write(&self, index: u32, value: WeakSlotState) -> Result<(), String> {
        self.require_weak(index)?;
        let mut slots = self
            .slots
            .lock()
            .map_err(|_| "weak field slot lock poisoned".to_string())?;
        let slot = slots
            .get_mut(index as usize)
            .ok_or_else(|| format!("weak field index out of range: {index}"))?;
        *slot = value;
        Ok(())
    }

    pub fn clear_all(&self) {
        if let Ok(mut slots) = self.slots.lock() {
            slots.fill(WeakSlotState::Empty);
        }
    }

    fn require_weak(&self, index: u32) -> Result<(), String> {
        match self.layout.field(index) {
            Some(field) if field.kind == DeclaredFieldKind::Weak => Ok(()),
            Some(field) => Err(format!(
                "declared field is not weak: {}",
                field.diagnostic_name
            )),
            None => Err(format!("weak field index out of range: {index}")),
        }
    }
}
