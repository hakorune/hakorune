//! Shared V2 lifecycle row vocabulary; layout and source authority stay elsewhere.

pub(super) const DEFINITION_ROLE_BIRTH_UNIT: u32 = 1;
pub(super) const DEFINITION_ROLE_ROOT_I64: u32 = 2;
pub(super) const DEFINITION_ROLE_ROOT_UNIT: u32 = 3;

pub(super) const RESULT_KIND_UNIT: u32 = 0;
pub(super) const RESULT_KIND_I64: u32 = 1;

pub(super) const CONTROL_KIND_RETURN: u32 = 4;
pub(super) const ABSENT_U32: u32 = u32::MAX;
