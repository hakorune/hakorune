use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

mod copy_type;
mod error;
mod finalize;
mod materialize;
mod post_success;
use error::{LocalSsaFailurePolicyV1, LocalSsaMaterializationErrorV1};
pub use finalize::{finalize_args, finalize_branch_cond, finalize_compare};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LocalKind {
    Recv,
    Arg,
    CompareOperand,
    Cond,
    FieldBase,
    Other(u8),
}

impl LocalKind {
    #[inline]
    fn can_forward_same_block_call_result_to_compare_operand(self) -> bool {
        matches!(self, LocalKind::CompareOperand)
    }

    #[inline]
    fn can_forward_same_block_field_get_to_consumer(self) -> bool {
        matches!(self, LocalKind::Arg | LocalKind::CompareOperand)
    }

    #[inline]
    fn can_forward_same_block_copy_to_receiver(self) -> bool {
        matches!(self, LocalKind::Recv)
    }

    #[inline]
    fn can_forward_same_block_copy_root_to_receiver(self) -> bool {
        matches!(self, LocalKind::Recv)
    }

    #[inline]
    fn can_forward_field_get_alias_to_consumer(self) -> bool {
        matches!(
            self,
            LocalKind::Arg | LocalKind::CompareOperand | LocalKind::FieldBase
        )
    }
}

impl LocalKind {
    #[inline]
    fn tag(self) -> u8 {
        match self {
            LocalKind::Recv => 0,
            LocalKind::Arg => 1,
            LocalKind::CompareOperand => 2,
            LocalKind::Cond => 4,
            LocalKind::FieldBase => 0, // share recv slot for bases
            LocalKind::Other(k) => k,
        }
    }
}

#[path = "analysis.rs"]
mod analysis;
use analysis::format_value_ids;
pub(crate) use analysis::strict_planner_required;

/// Ensure a value has an in-block definition and cache it per (bb, orig, kind).
/// Always emits a Copy in the current block when not cached.
pub fn ensure(builder: &mut MirBuilder, v: ValueId, kind: LocalKind) -> ValueId {
    ensure_inner(builder, v, kind, false).unwrap_or(v)
}

pub fn try_ensure(
    builder: &mut MirBuilder,
    v: ValueId,
    kind: LocalKind,
) -> Result<ValueId, String> {
    ensure_inner(
        builder,
        v,
        kind,
        crate::config::env::joinir_dev::strict_planner_required_debug_enabled()
            && kind == LocalKind::Arg,
    )
}

fn ensure_inner(
    builder: &mut MirBuilder,
    v: ValueId,
    kind: LocalKind,
    forbid_non_pure: bool,
) -> Result<ValueId, String> {
    match materialize::materialize_local_v1(
        builder,
        v,
        kind,
        forbid_non_pure,
        LocalSsaFailurePolicyV1::LegacyFacade,
    ) {
        Ok(value) => Ok(value),
        Err(LocalSsaMaterializationErrorV1::Contract(error)) => Err(error),
        Err(LocalSsaMaterializationErrorV1::BlockCreation(_))
        | Err(LocalSsaMaterializationErrorV1::InstructionEmission(_)) => unreachable!(
            "legacy LocalSSA materialization must resolve recoverable failures before returning"
        ),
    }
}

#[allow(dead_code)] // COPY0 is the first checked consumer.
fn try_materialize_local_v1(
    builder: &mut MirBuilder,
    v: ValueId,
    kind: LocalKind,
    forbid_non_pure: bool,
) -> Result<ValueId, LocalSsaMaterializationErrorV1> {
    materialize::materialize_local_v1(
        builder,
        v,
        kind,
        forbid_non_pure,
        LocalSsaFailurePolicyV1::Checked,
    )
}

#[inline]
pub fn recv(builder: &mut MirBuilder, v: ValueId) -> ValueId {
    ensure(builder, v, LocalKind::Recv)
}

#[inline]
pub fn arg(builder: &mut MirBuilder, v: ValueId) -> ValueId {
    ensure(builder, v, LocalKind::Arg)
}

#[inline]
pub fn cond(builder: &mut MirBuilder, v: ValueId) -> ValueId {
    ensure(builder, v, LocalKind::Cond)
}

#[inline]
pub fn field_base(builder: &mut MirBuilder, v: ValueId) -> ValueId {
    ensure(builder, v, LocalKind::FieldBase)
}

#[inline]
pub fn cmp_operand(builder: &mut MirBuilder, v: ValueId) -> ValueId {
    ensure(builder, v, LocalKind::CompareOperand)
}
