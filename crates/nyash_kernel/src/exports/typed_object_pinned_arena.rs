//! Pinned typed-object arena for exact typed-slot lowering.
//!
//! This module is intentionally storage-only. It does not export C ABI symbols,
//! does not change the default typed-object store, and does not emit direct
//! lowering. The purpose is to provide a stable object/slot store for the
//! exact-lane backend and its view handles.

use super::typed_object::{TypedSlot, TypedSlotObject, TypedSlotStorage, TypedSlotValue};
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ptr::{self, NonNull};

const INDEX_BITS: u32 = 31;
const INDEX_MASK: i64 = (1_i64 << INDEX_BITS) - 1;
const DIRECT_SLOT_TAG_I64: u32 = 1;
const DIRECT_SLOT_TAG_U64: u32 = 2;
const DIRECT_SLOT_TAG_HANDLE: u32 = 3;
const DIRECT_SLOT_TAG_USIZE: u32 = 4;
const DIRECT_SLOT_HANDLE_TAG: usize = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DirectSlotCellV0 {
    storage_tag: u32,
    flags: u32,
    payload: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DirectSlotObjectV0 {
    type_id: i64,
    generation: u32,
    field_count: u32,
    flags: u32,
    reserved: u32,
}

#[derive(Debug)]
pub(crate) struct DirectSlotObjectV0Box {
    ptr: NonNull<DirectSlotObjectV0>,
    layout: Layout,
    field_count: usize,
}

#[derive(Debug)]
struct PinnedTypedSlotObject {
    generation: u32,
    fields: Box<[TypedSlot]>,
}

impl DirectSlotObjectV0Box {
    pub(crate) fn from_typed_object(object: TypedSlotObject) -> Option<Self> {
        let cells = object
            .fields
            .iter()
            .map(DirectSlotCellV0::from_typed_slot)
            .collect::<Vec<_>>();
        Self::new(object.type_id, 1, &cells)
    }

    pub(crate) fn new(type_id: i64, generation: u32, cells: &[DirectSlotCellV0]) -> Option<Self> {
        if generation == 0 {
            return None;
        }
        let field_count = u32::try_from(cells.len()).ok()?;
        let layout = direct_slot_object_layout(cells.len())?;
        let raw = unsafe { alloc(layout) };
        let Some(ptr) = NonNull::new(raw.cast::<DirectSlotObjectV0>()) else {
            handle_alloc_error(layout);
        };
        unsafe {
            ptr::write(
                ptr.as_ptr(),
                DirectSlotObjectV0 {
                    type_id,
                    generation,
                    field_count,
                    flags: 0,
                    reserved: 0,
                },
            );
            let cells_ptr = direct_slot_object_cells_mut_ptr(ptr.as_ptr());
            ptr::copy_nonoverlapping(cells.as_ptr(), cells_ptr, cells.len());
        }
        Some(Self {
            ptr,
            layout,
            field_count: cells.len(),
        })
    }

    #[cfg(test)]
    pub(crate) fn as_ptr(&self) -> *const DirectSlotObjectV0 {
        self.ptr.as_ptr()
    }

    pub(crate) fn handle(&self) -> Option<i64> {
        encode_direct_slot_object_handle(self.ptr)
    }

    pub(crate) fn from_handle(handle: i64) -> Option<NonNull<DirectSlotObjectV0>> {
        decode_direct_slot_object_handle(handle)
    }

    pub(crate) fn matches_handle(&self, handle: i64) -> bool {
        Self::from_handle(handle)
            .map(|ptr| ptr.as_ptr() == self.ptr.as_ptr())
            .unwrap_or(false)
    }

    pub(crate) fn cell_ptr(&self, slot: usize) -> Option<*const DirectSlotCellV0> {
        if slot >= self.field_count {
            return None;
        }
        Some(unsafe { direct_slot_object_cells_ptr(self.ptr.as_ptr()).add(slot) })
    }

    pub(crate) fn cell(&self, slot: usize) -> Option<&DirectSlotCellV0> {
        let ptr = self.cell_ptr(slot)?;
        Some(unsafe { &*ptr })
    }

    pub(crate) fn cell_mut(&mut self, slot: usize) -> Option<&mut DirectSlotCellV0> {
        if slot >= self.field_count {
            return None;
        }
        let ptr = unsafe { direct_slot_object_cells_mut_ptr(self.ptr.as_ptr()).add(slot) };
        Some(unsafe { &mut *ptr })
    }

    pub(crate) fn rmw_add_exact_unsigned_u64(&mut self, slot: usize, delta: u128) -> Option<i64> {
        self.cell_mut(slot)?.rmw_add_exact_unsigned_u64(delta)
    }

    pub(crate) fn set_exact_signed_i64(&mut self, slot: usize, value: i64) -> bool {
        self.cell_mut(slot)
            .map(|cell| cell.write_i64(value))
            .unwrap_or(false)
    }

    pub(crate) fn exact_slot_set4_i64(
        &mut self,
        start_slot: usize,
        value0: i64,
        value1: i64,
        value2: i64,
        value3: i64,
    ) -> bool {
        let Some(end_slot) = start_slot.checked_add(4) else {
            return false;
        };
        if end_slot > self.field_count {
            return false;
        }
        for slot in start_slot..end_slot {
            if self
                .cell(slot)
                .and_then(DirectSlotCellV0::read_i64)
                .is_none()
            {
                return false;
            }
        }
        self.set_exact_signed_i64(start_slot, value0)
            && self.set_exact_signed_i64(start_slot + 1, value1)
            && self.set_exact_signed_i64(start_slot + 2, value2)
            && self.set_exact_signed_i64(start_slot + 3, value3)
    }

    pub(crate) fn exact_slot_record_alloc_success(&mut self, selected_kind: i64) -> bool {
        const LAST_REASON: usize = 2;
        const LAST_OK: usize = 3;
        const SUCCESS_COUNT: usize = 5;
        const REUSABLE_SUCCESS_COUNT: usize = 7;
        const ACTIVE_SUCCESS_COUNT: usize = 8;

        if self.field_count <= ACTIVE_SUCCESS_COUNT {
            return false;
        }
        if !self.set_exact_signed_i64(LAST_REASON, 0) {
            return false;
        }
        if !self.set_exact_signed_i64(LAST_OK, 1) {
            return false;
        }
        if self.rmw_add_exact_unsigned_u64(SUCCESS_COUNT, 1).is_none() {
            return false;
        }
        if selected_kind == 1 {
            return self
                .rmw_add_exact_unsigned_u64(REUSABLE_SUCCESS_COUNT, 1)
                .is_some();
        }
        if selected_kind == 2 {
            return self
                .rmw_add_exact_unsigned_u64(ACTIVE_SUCCESS_COUNT, 1)
                .is_some();
        }
        true
    }

    pub(crate) fn exact_slot_record_release_success(
        &mut self,
        page_id: i64,
        block_id: i64,
    ) -> bool {
        const LAST_PAGE_ID: usize = 0;
        const LAST_BLOCK_ID: usize = 1;
        const LAST_REASON: usize = 2;
        const LAST_OK: usize = 3;
        const SUCCESS_COUNT: usize = 4;

        if self.field_count <= SUCCESS_COUNT {
            return false;
        }
        if !self.set_exact_signed_i64(LAST_PAGE_ID, page_id) {
            return false;
        }
        if !self.set_exact_signed_i64(LAST_BLOCK_ID, block_id) {
            return false;
        }
        if !self.set_exact_signed_i64(LAST_REASON, 0) {
            return false;
        }
        if !self.set_exact_signed_i64(LAST_OK, 1) {
            return false;
        }
        self.rmw_add_exact_unsigned_u64(SUCCESS_COUNT, 1).is_some()
    }

    #[cfg(test)]
    pub(crate) fn materialize_typed_object_snapshot(&self) -> Option<TypedSlotObject> {
        let header = unsafe { self.ptr.as_ref() };
        let cells = unsafe { direct_slot_object_cells_ptr(self.ptr.as_ptr()) };
        let mut fields = Vec::with_capacity(self.field_count);
        for slot in 0..self.field_count {
            let cell = unsafe { *cells.add(slot) };
            fields.push(cell.to_typed_slot()?);
        }
        Some(TypedSlotObject {
            type_id: header.type_id,
            fields,
        })
    }
}

impl Drop for DirectSlotObjectV0Box {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.ptr.as_ptr().cast::<u8>(), self.layout);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PinnedTypedObjectRef {
    index: usize,
    generation: u32,
}

#[derive(Debug, Default)]
pub(crate) struct PinnedTypedObjectArena {
    objects: Vec<Option<Box<PinnedTypedSlotObject>>>,
}

impl PinnedTypedObjectArena {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert(&mut self, object: TypedSlotObject) -> Option<i64> {
        let generation = 1;
        let index = self.objects.len();
        self.objects.push(Some(Box::new(PinnedTypedSlotObject {
            generation,
            fields: object.fields.into_boxed_slice(),
        })));
        encode_handle(PinnedTypedObjectRef { index, generation })
    }

    pub(crate) fn validate(&self, handle: i64) -> Option<PinnedTypedObjectRef> {
        let object_ref = decode_handle(handle)?;
        let object = self.objects.get(object_ref.index)?.as_ref()?;
        if object.generation != object_ref.generation {
            return None;
        }
        Some(object_ref)
    }

    pub(crate) fn get_field(&self, handle: i64, slot: usize) -> Option<&TypedSlot> {
        let object_ref = self.validate(handle)?;
        self.objects
            .get(object_ref.index)?
            .as_ref()?
            .fields
            .get(slot)
    }

    pub(crate) fn get_field_mut(&mut self, handle: i64, slot: usize) -> Option<&mut TypedSlot> {
        let object_ref = self.validate(handle)?;
        self.objects
            .get_mut(object_ref.index)?
            .as_mut()?
            .fields
            .get_mut(slot)
    }

    pub(crate) fn get_fields_mut(&mut self, handle: i64) -> Option<&mut [TypedSlot]> {
        let object_ref = self.validate(handle)?;
        self.objects
            .get_mut(object_ref.index)?
            .as_mut()
            .map(|object| object.fields.as_mut())
    }
}

impl DirectSlotCellV0 {
    pub(crate) fn from_typed_slot(slot: &TypedSlot) -> Self {
        match (slot.storage, &slot.value) {
            (TypedSlotStorage::I64, TypedSlotValue::I64(value)) => Self::from_i64(*value),
            (TypedSlotStorage::U64, TypedSlotValue::Unsigned(value)) => {
                Self::from_u64(u64::try_from(*value).unwrap_or(0))
            }
            (TypedSlotStorage::USize, TypedSlotValue::Unsigned(value))
                if cfg!(target_pointer_width = "64") =>
            {
                u64::try_from(*value).map(Self::from_usize).unwrap_or(Self {
                    storage_tag: 0,
                    flags: 0,
                    payload: 0,
                })
            }
            (TypedSlotStorage::Handle, TypedSlotValue::Handle(value)) => Self::from_handle(*value),
            _ => Self {
                storage_tag: 0,
                flags: 0,
                payload: 0,
            },
        }
    }

    #[allow(dead_code)]
    fn to_typed_slot(self) -> Option<TypedSlot> {
        match self.storage_tag {
            DIRECT_SLOT_TAG_I64 => Some(TypedSlot {
                storage: TypedSlotStorage::I64,
                value: TypedSlotValue::I64(self.payload as i64),
            }),
            DIRECT_SLOT_TAG_U64 => Some(TypedSlot {
                storage: TypedSlotStorage::U64,
                value: TypedSlotValue::Unsigned(self.payload as u128),
            }),
            DIRECT_SLOT_TAG_USIZE => Some(TypedSlot {
                storage: TypedSlotStorage::USize,
                value: TypedSlotValue::Unsigned(self.payload as u128),
            }),
            DIRECT_SLOT_TAG_HANDLE => Some(TypedSlot {
                storage: TypedSlotStorage::Handle,
                value: TypedSlotValue::Handle(self.payload as i64),
            }),
            _ => None,
        }
    }

    fn from_i64(value: i64) -> Self {
        Self {
            storage_tag: DIRECT_SLOT_TAG_I64,
            flags: 0,
            payload: value as u64,
        }
    }

    fn from_u64(value: u64) -> Self {
        Self {
            storage_tag: DIRECT_SLOT_TAG_U64,
            flags: 0,
            payload: value,
        }
    }

    fn from_usize(value: u64) -> Self {
        Self {
            storage_tag: DIRECT_SLOT_TAG_USIZE,
            flags: 0,
            payload: value,
        }
    }

    fn from_handle(value: i64) -> Self {
        Self {
            storage_tag: DIRECT_SLOT_TAG_HANDLE,
            flags: 0,
            payload: value as u64,
        }
    }

    fn read_i64(&self) -> Option<i64> {
        (self.storage_tag == DIRECT_SLOT_TAG_I64).then_some(self.payload as i64)
    }

    pub(crate) fn read_compat_i64(&self) -> Option<i64> {
        match self.storage_tag {
            DIRECT_SLOT_TAG_I64 | DIRECT_SLOT_TAG_HANDLE => Some(self.payload as i64),
            _ => None,
        }
    }

    fn write_i64(&mut self, value: i64) -> bool {
        if self.storage_tag != DIRECT_SLOT_TAG_I64 {
            return false;
        }
        self.payload = value as u64;
        true
    }

    pub(crate) fn write_compat_i64(&mut self, value: i64) -> bool {
        match self.storage_tag {
            DIRECT_SLOT_TAG_I64 => self.write_i64(value),
            DIRECT_SLOT_TAG_HANDLE => self.write_handle(value),
            _ => false,
        }
    }

    fn read_u64(&self) -> Option<u64> {
        self.u64_payload_tag().then_some(self.payload)
    }

    fn write_u64(&mut self, value: u64) -> bool {
        if !self.u64_payload_tag() {
            return false;
        }
        self.payload = value;
        true
    }

    fn rmw_add_exact_unsigned_u64(&mut self, delta: u128) -> Option<i64> {
        let next = u128::from(self.read_u64()?).checked_add(delta)?;
        let next_i64 = i64::try_from(next).ok()?;
        let next_u64 = u64::try_from(next).ok()?;
        if !self.write_u64(next_u64) {
            return None;
        }
        Some(next_i64)
    }

    fn u64_payload_tag(&self) -> bool {
        self.storage_tag == DIRECT_SLOT_TAG_U64
            || (cfg!(target_pointer_width = "64") && self.storage_tag == DIRECT_SLOT_TAG_USIZE)
    }

    #[cfg(test)]
    fn read_handle(&self) -> Option<i64> {
        (self.storage_tag == DIRECT_SLOT_TAG_HANDLE).then_some(self.payload as i64)
    }

    fn write_handle(&mut self, value: i64) -> bool {
        if self.storage_tag != DIRECT_SLOT_TAG_HANDLE {
            return false;
        }
        self.payload = value as u64;
        true
    }
}

fn direct_slot_object_layout(field_count: usize) -> Option<Layout> {
    let header = Layout::new::<DirectSlotObjectV0>();
    let cells = Layout::array::<DirectSlotCellV0>(field_count).ok()?;
    let (layout, _) = header.extend(cells).ok()?;
    Some(layout.pad_to_align())
}

fn direct_slot_object_field_offset() -> usize {
    let header = Layout::new::<DirectSlotObjectV0>();
    let cells = Layout::new::<DirectSlotCellV0>();
    let (_, offset) = header
        .extend(cells)
        .expect("DirectSlotObjectV0 + DirectSlotCellV0 layout must be valid");
    offset
}

unsafe fn direct_slot_object_cells_ptr(
    object: *const DirectSlotObjectV0,
) -> *const DirectSlotCellV0 {
    object
        .cast::<u8>()
        .add(direct_slot_object_field_offset())
        .cast::<DirectSlotCellV0>()
}

unsafe fn direct_slot_object_cells_mut_ptr(
    object: *mut DirectSlotObjectV0,
) -> *mut DirectSlotCellV0 {
    object
        .cast::<u8>()
        .add(direct_slot_object_field_offset())
        .cast::<DirectSlotCellV0>()
}

fn encode_direct_slot_object_handle(object: NonNull<DirectSlotObjectV0>) -> Option<i64> {
    let ptr = object.as_ptr() as usize;
    if ptr & DIRECT_SLOT_HANDLE_TAG != 0 {
        return None;
    }
    i64::try_from(ptr | DIRECT_SLOT_HANDLE_TAG).ok()
}

fn decode_direct_slot_object_handle(handle: i64) -> Option<NonNull<DirectSlotObjectV0>> {
    let payload = usize::try_from(handle).ok()?;
    if payload & DIRECT_SLOT_HANDLE_TAG != DIRECT_SLOT_HANDLE_TAG {
        return None;
    }
    let ptr = (payload & !DIRECT_SLOT_HANDLE_TAG) as *mut DirectSlotObjectV0;
    NonNull::new(ptr)
}

pub(crate) fn read_direct_slot_compat_i64(handle: i64, slot: usize) -> Option<i64> {
    let ptr = decode_direct_slot_object_handle(handle)?;
    let header = unsafe { ptr.as_ref() };
    if slot >= header.field_count as usize {
        return None;
    }
    let cells = unsafe { direct_slot_object_cells_ptr(ptr.as_ptr()) };
    let cell = unsafe { &*cells.add(slot) };
    cell.read_compat_i64()
}

pub(crate) fn read_direct_slot_i64(handle: i64, slot: usize) -> Option<i64> {
    let ptr = decode_direct_slot_object_handle(handle)?;
    let header = unsafe { ptr.as_ref() };
    if slot >= header.field_count as usize {
        return None;
    }
    let cells = unsafe { direct_slot_object_cells_ptr(ptr.as_ptr()) };
    let cell = unsafe { &*cells.add(slot) };
    cell.read_i64()
}

pub(crate) fn write_direct_slot_compat_i64(handle: i64, slot: usize, value: i64) -> Option<bool> {
    let ptr = decode_direct_slot_object_handle(handle)?;
    let header = unsafe { ptr.as_ref() };
    if slot >= header.field_count as usize {
        return None;
    }
    let cells = unsafe { direct_slot_object_cells_mut_ptr(ptr.as_ptr()) };
    let cell = unsafe { &mut *cells.add(slot) };
    Some(cell.write_compat_i64(value))
}

pub(crate) fn write_direct_slot_i64(handle: i64, slot: usize, value: i64) -> Option<bool> {
    let ptr = decode_direct_slot_object_handle(handle)?;
    let header = unsafe { ptr.as_ref() };
    if slot >= header.field_count as usize {
        return None;
    }
    let cells = unsafe { direct_slot_object_cells_mut_ptr(ptr.as_ptr()) };
    let cell = unsafe { &mut *cells.add(slot) };
    Some(cell.write_i64(value))
}

fn encode_handle(object_ref: PinnedTypedObjectRef) -> Option<i64> {
    if object_ref.generation == 0 {
        return None;
    }
    let index_payload = i64::try_from(object_ref.index).ok()?.checked_add(1)?;
    if index_payload <= 0 || index_payload > INDEX_MASK {
        return None;
    }
    let generation_payload = i64::from(object_ref.generation).checked_shl(INDEX_BITS)?;
    let payload = generation_payload | index_payload;
    payload.checked_neg()
}

fn decode_handle(handle: i64) -> Option<PinnedTypedObjectRef> {
    if handle >= 0 {
        return None;
    }
    let payload = handle.checked_neg()?;
    let index_payload = payload & INDEX_MASK;
    if index_payload == 0 {
        return None;
    }
    let generation_payload = payload >> INDEX_BITS;
    let generation = u32::try_from(generation_payload).ok()?;
    if generation == 0 {
        return None;
    }
    let index = usize::try_from(index_payload.checked_sub(1)?).ok()?;
    Some(PinnedTypedObjectRef { index, generation })
}

#[cfg(test)]
mod tests;
