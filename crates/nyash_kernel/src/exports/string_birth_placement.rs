#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum BoundaryKind {
    ObserverOnly,
    Store,
    LoopCarry,
    AbiVisible,
    CloneShare,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TextRetentionClass {
    ReturnHandle,
    KeepTransient,
    RetainView,
    MustFreeze(BoundaryKind),
}

const SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES: usize = 8;

#[inline(always)]
pub(crate) fn substring_retention_class(
    view_enabled: bool,
    slice_len: usize,
) -> TextRetentionClass {
    if !view_enabled || slice_len <= SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES {
        TextRetentionClass::MustFreeze(BoundaryKind::Store)
    } else {
        TextRetentionClass::RetainView
    }
}

#[inline(always)]
pub(crate) fn concat_suffix_retention_class(suffix_is_empty: bool) -> TextRetentionClass {
    if suffix_is_empty {
        TextRetentionClass::ReturnHandle
    } else {
        TextRetentionClass::KeepTransient
    }
}

#[inline(always)]
pub(crate) fn insert_middle_retention_class(
    source_is_empty: bool,
    middle_is_empty: bool,
) -> TextRetentionClass {
    if middle_is_empty {
        TextRetentionClass::ReturnHandle
    } else if source_is_empty {
        TextRetentionClass::MustFreeze(BoundaryKind::Store)
    } else {
        TextRetentionClass::KeepTransient
    }
}

#[inline(always)]
pub(crate) fn concat3_retention_class(
    a_is_empty: bool,
    b_is_empty: bool,
    c_is_empty: bool,
    allow_handle_reuse: bool,
) -> TextRetentionClass {
    if allow_handle_reuse
        && ((a_is_empty && b_is_empty) || (a_is_empty && c_is_empty) || (b_is_empty && c_is_empty))
    {
        TextRetentionClass::ReturnHandle
    } else {
        TextRetentionClass::KeepTransient
    }
}

#[cfg(test)]
mod tests {
    use super::{
        concat3_retention_class, concat_suffix_retention_class, insert_middle_retention_class,
    };
    use super::{substring_retention_class, BoundaryKind, TextRetentionClass};

    #[test]
    fn substring_placement_distinguishes_view_and_freeze() {
        assert_eq!(
            substring_retention_class(true, 9),
            TextRetentionClass::RetainView
        );
        assert_eq!(
            substring_retention_class(false, 9),
            TextRetentionClass::MustFreeze(BoundaryKind::Store)
        );
    }

    #[test]
    fn concat_and_insert_placement_keep_transient_or_reuse_handle() {
        assert_eq!(
            concat_suffix_retention_class(true),
            TextRetentionClass::ReturnHandle
        );
        assert_eq!(
            concat_suffix_retention_class(false),
            TextRetentionClass::KeepTransient
        );
        assert_eq!(
            insert_middle_retention_class(true, false),
            TextRetentionClass::MustFreeze(BoundaryKind::Store)
        );
        assert_eq!(
            insert_middle_retention_class(false, true),
            TextRetentionClass::ReturnHandle
        );
        assert_eq!(
            insert_middle_retention_class(false, false),
            TextRetentionClass::KeepTransient
        );
        assert_eq!(
            concat3_retention_class(false, false, false, true),
            TextRetentionClass::KeepTransient
        );
        assert_eq!(
            concat3_retention_class(true, true, false, true),
            TextRetentionClass::ReturnHandle
        );
    }
}
