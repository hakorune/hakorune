use super::super::{BasicBlockId, MirBuilder, ValueId};

/// Prepare the existing raw-PHI unanimous-origin fact without mutation.
pub(crate) fn prepare_unanimous_origin(
    builder: &MirBuilder,
    inputs: &[(BasicBlockId, ValueId)],
) -> Option<String> {
    let mut common_cls: Option<String> = None;
    for (_bb, v) in inputs {
        if let Some(c) = builder.type_ctx.value_origin_newbox.get(v).cloned() {
            match &common_cls {
                None => common_cls = Some(c),
                Some(cc) => {
                    if cc != &c {
                        return None;
                    }
                }
            }
        } else {
            return None;
        }
    }
    common_cls
}

/// Commit a prevalidated raw-PHI origin only after instruction emission.
pub(crate) fn commit_unanimous_origin(
    builder: &mut MirBuilder,
    dst: ValueId,
    prepared: Option<String>,
) {
    if let Some(origin) = prepared {
        builder.type_ctx.value_origin_newbox.insert(dst, origin);
    }
}
