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
pub(crate) enum RetainedForm {
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
) -> RetainedForm {
    if !view_enabled || slice_len <= SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES {
        RetainedForm::MustFreeze(BoundaryKind::Store)
    } else {
        RetainedForm::RetainView
    }
}

#[inline(always)]
pub(crate) fn concat_suffix_retention_class(suffix_is_empty: bool) -> RetainedForm {
    if suffix_is_empty {
        RetainedForm::ReturnHandle
    } else {
        RetainedForm::KeepTransient
    }
}

#[inline(always)]
pub(crate) fn insert_middle_retention_class(
    source_is_empty: bool,
    middle_is_empty: bool,
) -> RetainedForm {
    if middle_is_empty {
        RetainedForm::ReturnHandle
    } else if source_is_empty {
        RetainedForm::MustFreeze(BoundaryKind::Store)
    } else {
        RetainedForm::KeepTransient
    }
}

#[inline(always)]
pub(crate) fn concat3_retention_class(
    a_is_empty: bool,
    b_is_empty: bool,
    c_is_empty: bool,
    allow_handle_reuse: bool,
) -> RetainedForm {
    if allow_handle_reuse
        && ((a_is_empty && b_is_empty) || (a_is_empty && c_is_empty) || (b_is_empty && c_is_empty))
    {
        RetainedForm::ReturnHandle
    } else {
        RetainedForm::KeepTransient
    }
}

#[cfg(test)]
mod tests {
    use super::{
        concat3_retention_class, concat_suffix_retention_class, insert_middle_retention_class,
    };
    use super::{substring_retention_class, BoundaryKind, RetainedForm};

    #[test]
    fn substring_placement_distinguishes_view_and_freeze() {
        assert_eq!(
            substring_retention_class(true, 9),
            RetainedForm::RetainView
        );
        assert_eq!(
            substring_retention_class(false, 9),
            RetainedForm::MustFreeze(BoundaryKind::Store)
        );
    }

    #[test]
    fn concat_and_insert_placement_keep_transient_or_reuse_handle() {
        assert_eq!(
            concat_suffix_retention_class(true),
            RetainedForm::ReturnHandle
        );
        assert_eq!(
            concat_suffix_retention_class(false),
            RetainedForm::KeepTransient
        );
        assert_eq!(
            insert_middle_retention_class(true, false),
            RetainedForm::MustFreeze(BoundaryKind::Store)
        );
        assert_eq!(
            insert_middle_retention_class(false, true),
            RetainedForm::ReturnHandle
        );
        assert_eq!(
            insert_middle_retention_class(false, false),
            RetainedForm::KeepTransient
        );
        assert_eq!(
            concat3_retention_class(false, false, false, true),
            RetainedForm::KeepTransient
        );
        assert_eq!(
            concat3_retention_class(true, true, false, true),
            RetainedForm::ReturnHandle
        );
    }
}
