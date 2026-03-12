use super::method_id::CoreMethodId;
use super::CoreBoxId;

/// SSOT for CoreMethodId metadata (name/arity/return types and policy flags).
#[derive(Debug, Clone, Copy)]
pub(super) struct CoreMethodSpec {
    pub id: CoreMethodId,
    pub box_id: CoreBoxId,
    pub name: &'static str,
    pub arity: usize,
    pub return_type_name: &'static str,
    pub is_pure: bool,
    pub allowed_in_condition: bool,
    pub allowed_in_init: bool,
    pub vtable_slot: Option<u16>,
}

pub(super) mod basic;
pub(super) mod optional;
pub(super) mod special;

pub(super) fn iter_all_specs() -> impl Iterator<Item = &'static CoreMethodSpec> {
    basic::SPECS
        .iter()
        .chain(optional::SPECS.iter())
        .chain(special::SPECS.iter())
}
