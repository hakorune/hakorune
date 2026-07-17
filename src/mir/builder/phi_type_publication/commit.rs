use std::collections::BTreeMap;

use super::{MirType, PreparedPhiTypePublicationV1, ValueId};

/// Commit a prevalidated PHI type decision.
///
/// This operation is deliberately non-fallible. Only `Publish` mutates the
/// transient type map; all preservation/no-publication decisions are no-ops.
pub(in crate::mir::builder) fn commit_prepared_phi_type(
    value_types: &mut BTreeMap<ValueId, MirType>,
    dst: ValueId,
    prepared: PreparedPhiTypePublicationV1,
) {
    if let PreparedPhiTypePublicationV1::Publish(ty) = prepared {
        value_types.insert(dst, ty);
    }
}
